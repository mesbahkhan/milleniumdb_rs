
use std::sync::Arc;
use std::time::{SystemTime, Duration};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time;
use std::error::Error;


use tokio::sync::{mpsc, Mutex};


use crate::network::listener::Listener;
use crate::query::query_contexts::QueryContext;

pub const DEFAULT_PORT: u16 = 8080;

pub struct Server {
    //thread_info_vec_mutex: Mutex<()>,
    query_contexts: Vec<Arc<Mutex<QueryContext>>>,
    pub shutdown_server: Arc<Mutex<bool>>,
    interrupt: Arc<Mutex<mpsc::Receiver<bool>>>,
}

impl Server {

    pub fn new() -> Arc<Mutex<Self>>  {
        Arc::new(Mutex::new(Self {
            shutdown_server: Arc::new(Mutex::new(false)),
            query_contexts: Vec::new(),
            interrupt: Arc::new(Mutex::new(mpsc::channel(1).1)),
            //thread_info_vec_mutex: Mutex::new(()),
        }))
    }

    pub async fn execute_timeouts(&self) {
        let shutdown_server = self.shutdown_server.clone();
        let query_contexts = self.query_contexts.clone();
        tokio::spawn(async move {
            loop {
                let now = SystemTime::now();
                {
                    // Asynchronously lock the mutex guarding the shutdown flag
                    let shutdown = shutdown_server.lock().await;
                    if *shutdown {
                        break; // Exit the loop if the server is shutting down
                    }

                    // Asynchronously access each QueryContext
                    for query_ctx in query_contexts.iter() {
                        let mut qc = query_ctx.lock().await; // Await the lock
                        if qc.thread_info.timeout <= now {
                            qc.thread_info.interruption_requested = true;
                        }
                    }
                }
                // Asynchronously wait before the next iteration
                time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    pub async fn run(
        server: Arc<Mutex<Self>>,      
        port: u16,
        //worker_threads: usize, 
        _timeout: Duration) -> Result<(), Box<dyn Error>> {       

            let server_clone_for_signals = server.clone();
            let server_clone_for_listener = server.clone();

        let handle_interrupt = tokio::spawn(async move {
            Server::handle_signals(server_clone_for_signals).await;
        });

        let server_loop = tokio::spawn(async move {
            Server::start_listener(
                server_clone_for_listener,
                port).await; 
        });
        
        handle_interrupt.await?;
        server_loop.await?;

        // tokio::select! {
        //      _ = server_loop => {},
        //      _ = handle_interrupt => {},
        //  };

        Ok(())
    }

    async fn start_listener(
        server: Arc<Mutex<Self>>,         
        port: u16 ) -> Option<Result<(), Box<std::io::Error>>> {
        

        let server_clone = server.clone();

        let io_context_result = 
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build();

        let io_context = match io_context_result {
            Ok(runtime) => Arc::new(runtime),
            Err(e) => return Some(Err(Box::new(e))),
                    };        

        let server_weak = 
            Arc::downgrade(&server);

        let handle = tokio::spawn(async move {     
    
        // Define the port and endpoint here...
        //let port = 1234; // Change to your desired port
        let endpoint = format!(
            "127.0.0.1:{}", 
            port)
                .parse()
                .unwrap(); // Change to your desired endpoint
    
        // Create a new instance of your Listener struct
        let listener_result = 
            Listener::new(
                server_weak, 
                io_context, 
                endpoint, 
                Duration::from_secs(10))
                    .await;
            
        if let Err(err) = listener_result {
            eprintln!("Failed to create listener: {}", err);
            return; // Exit the task if we can't create the listener
        }
    
        let listener = listener_result.unwrap();
    
        listener.run().await;
    
                });  
            
    
        // Create and launch a listening port
            
        if let Err(e) = handle.await {
        eprintln!("Listen spawn failed: {:?}", e);
                }
        None
    }
    
       

    async fn handle_signals(server: Arc<Mutex<Server>>) {
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to bind SIGINT handler");
        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to bind SIGTERM handler");
    
        tokio::select! {
            _ = sigint.recv() => println!("Received SIGINT"),
            _ = sigterm.recv() => println!("Received SIGTERM"),
        }
    
        let server_guard = server.lock().await;
        // Directly set the shutdown_server boolean value
        *server_guard.shutdown_server.lock().await = true;
        drop(server_guard);
        // Additional shutdown logic here...
    }

}
           


   
    

        // tokio::select! {
        //     _ = server_loop => {},
        //     _ = handle_interrupt => {},
        // };
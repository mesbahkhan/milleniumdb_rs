use std::sync::Arc;
use std::time::{SystemTime, Duration};
use tokio::runtime::Runtime;
use tokio::time;
use std::error::Error;

use tokio::net::TcpListener;

use tokio::sync::{mpsc, Mutex};


use crate::network::listener::Listener;
use crate::query::query_contexts::QueryContext;

pub const DEFAULT_PORT: u16 = 8080;

pub struct Server {
    thread_info_vec_mutex: Mutex<()>,
    query_contexts: Vec<Arc<Mutex<QueryContext>>>,
    shutdown_server: Arc<Mutex<bool>>,
    interrupt: Arc<Mutex<mpsc::Receiver<bool>>>,
}

impl Server {

    pub fn new() -> Self {
        Self {
            shutdown_server: Arc::new(Mutex::new(false)),
            query_contexts: Vec::new(),
            interrupt: Arc::new(Mutex::new(mpsc::channel(1).1)),
            thread_info_vec_mutex: Mutex::new(()),
        }
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
        &mut self, 
        port: u16, 
        worker_threads: usize, 
        _timeout: Duration) -> Result<(), Box<dyn Error>> {
        
        // Create and launch a listening port
        // Start the query service
        tokio::spawn(async move {

      
            // let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)).await {
            //     Ok(listener) => listener,
            //     Err(e) => {
            //         eprintln!("Failed to bind to port 1234: {}", e);
            //         return; // Exit the task if we can't bind to the port
            //     }
            // };

            // println!("Listening to port:{}", port);

            // loop {
            //     // Accept connections and process them
            //     let (socket, _) = listener.accept().await.unwrap();
            //     //process_query(socket).await;
            // }
            let io_context = Arc::new(tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .map_err(|e| Box::new(e) as Box<dyn Error>)); // Initialize your IO context

            // Define the port and endpoint here...
            //let port = 1234; // Change to your desired port
            let endpoint = format!("127.0.0.1:{}", port).parse().unwrap(); // Change to your desired endpoint

            // Create a new instance of your Listener struct
            let listener_result = Listener::new(Arc::downgrade(&self.thread_info_vec_mutex), io_context.clone(), endpoint, Duration::from_secs(10)).await;
            if let Err(err) = listener_result {
                eprintln!("Failed to create listener: {}", err);
                return; // Exit the task if we can't create the listener
            }
            let listener = listener_result.unwrap();

            // Run the listener
            if let Err(err) = listener.run().await {
                eprintln!("Listener error: {}", err);
            }
            
        });
    
        self.query_contexts.resize_with(
            worker_threads, || Arc::new(Mutex::new(QueryContext::new())));
    
        let (tx, mut rx) = mpsc::channel::<bool>(1);        
        
        // Capture SIGINT and SIGTERM to perform a clean shutdown        
    
        let shutdown_server_interrupt_clone = self.shutdown_server.clone();

        let handle_interrupt = tokio::spawn(async move {
            while rx.recv().await.is_some() {
                let mut shutdown = shutdown_server_interrupt_clone.lock().await;
                *shutdown = true;
            }
        });
    
        let shutdown_server_clone2 = self.shutdown_server.clone();

        let server_loop = tokio::spawn(async move {
            loop {
                let shutdown = shutdown_server_clone2.lock().await;
                if *shutdown {
                    break;
                }
                // Assume handling connections asynchronously
            }
        });
        
        let mut handles = Vec::new();
        // Run the I/O service on the requested number of threads
        for i in 0..worker_threads {
            let query_ctx = Arc::clone(&self.query_contexts[i]);
            let handle = tokio::spawn(async move {
                // Run your task here
            });
            handles.push(handle);
        }
        
        println!("SPARQL Server running on port {}", port);
        println!("To terminate press CTRL-C");
        
        self.execute_timeouts().await;

        // Block until all the threads exit
        // Wait for all tasks to complete
        for handle in handles {
        handle.await?;
        }

        // Wait for either server loop or interrupt handler to finish
        tokio::select! {
            _ = server_loop => {},
            _ = handle_interrupt => {},
        };
        

        Ok(())
    }
    
}
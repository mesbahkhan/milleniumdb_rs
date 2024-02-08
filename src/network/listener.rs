use std::sync::{Arc, Weak};
use std::error::Error;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

use crate::query::query_contexts::QueryContext;
use crate::network::sparql_servers::Server;
use crate::network::session::Session;

pub struct Listener {
    server: Weak<Mutex<Server>>,
    io_context: Arc<tokio::runtime::Runtime>,
    acceptor: Arc<Mutex<tokio::net::TcpListener>>,
    timeout: Duration,
}

impl Listener {
    pub async fn new(
        server: Weak<Mutex<Server>>,
        io_context: Arc<tokio::runtime::Runtime>,
        endpoint: std::net::SocketAddr,
        timeout: Duration,
    ) -> Result<Self, Box<dyn Error>> {

        let listener = TcpListener::bind(&endpoint).await?;

        Ok(Self {
            server,
            io_context,
            acceptor: Arc::new(Mutex::new(listener)),
            timeout,
        })
    }

    pub async fn run(&self) {

        println!(
            "Listening on port {}", 
            self.acceptor
                .lock()
                .await
                    .local_addr()
                    .unwrap()
                        .port());

        loop {
            let acceptor = Arc::clone(&self.acceptor);
            
            match acceptor.lock().await.accept().await {
                Ok((socket, _)) => {
                    self.handle_connection(socket).await;
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            };
        }
    }

    async fn handle_connection(&self, socket: TcpStream) {
        let server = match self.server.upgrade() {
            Some(server) => server,
            None => {
                eprintln!("Error: Server no longer exists");
                return;
            }
        };

        let timeout = self.timeout;
        let query_ctx = Arc::new(Mutex::new(QueryContext::new()));
        let server_weak = self.server.clone();
        tokio::spawn(async move {
            // Create session and run it
            let session = Session::new(server_weak, socket, timeout);
            session.run().await;
        });

    }
}


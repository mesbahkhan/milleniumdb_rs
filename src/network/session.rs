use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use crate::query::query_contexts::QueryContext;
//use crate::query::executor::query_executor::QueryExecutor;
//use crate::network::response_type::ResponseType;
use crate::network::sparql_servers::Server;

pub struct Session {
    server: Weak<Mutex<Server>>,
    stream: TcpStream,
    timeout: Duration,
}

impl Session {
    pub fn new(
        server: Weak<Mutex<Server>>,
        stream: TcpStream,
        timeout: Duration,
    ) -> Self {
        Self {
            server,
            stream,
            timeout,
        }
    }

    pub async fn run(mut self) {
        loop {
            let result = timeout(self.timeout, self.do_read()).await;
            match result {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Session timed out");
                    break;
                }
            }
        }
    }

    async fn do_read(&mut self) {
        let mut buffer = vec![0; 1024];
        match self.stream.read(&mut buffer).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return;
                }
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                // Process the request
                println!("Received request: {}", request);
                // Write response
                let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
                if let Err(e) = self.stream.write_all(response.as_bytes()).await {
                    eprintln!("Error writing response: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
            }
        }
    }
}

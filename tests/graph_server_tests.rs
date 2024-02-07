use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::time::Duration;
use tokio::time::sleep;


use milleniumdb_rs::server::graph_server_orchestrator::startup_server;

#[tokio::test]
async fn test_graph_server() {
    // Start the server in a separate task
    let server_handle = tokio::spawn(async {
        // Replace this with the actual function to start your server
        startup_server().await;
    });

    // Give the server some time to start
    sleep(Duration::from_secs(1)).await;

    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:1234").await.unwrap();

    // Send a test query
    stream.write_all(b"EXPECTED_QUERY").await.unwrap();

    // Read the response
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.unwrap();

    // Convert buffer to a string and trim to remove extra whitespace
    let response = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

    // Check the response
    assert_eq!(response, "Query received and processed.");

    // Stop the server
    server_handle.abort();
}
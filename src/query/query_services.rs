
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn process_query(
    mut socket: TcpStream) {

    let mut buffer = [0; 1024];

    if let Ok(n) = socket.read(&mut buffer).await {
         
        // Convert buffer to a string and trim to remove extra whitespace
        let query = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

        // Check if the query matches expected format (modify this according to your needs)
        let response = if query == "EXPECTED_QUERY" {

            "Query received and processed.".to_string()

        } else {

            "Syntax Error: Invalid query format.".to_string()

        };

        // Send the response back to the client
        if let Err(e) = socket.write_all(response.as_bytes()).await {

            eprintln!("Failed to send response: {}", e);
            
        }
    } else {

        eprintln!("Failed to read from socket.");

    }
}



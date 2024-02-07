use tokio::net::TcpListener;

use crate::query::query_services::process_query;
use crate::import::import_services::load_data_into_database;

use std::error::Error;

pub async fn startup_server() -> Result<(), Box<dyn Error>> {
    // Initialize the graph database (replace with actual database initialization)

    // Start the query service
    let query_service = tokio::spawn(async {

        // Define the listener on port 1234
        let listener = match TcpListener::bind("127.0.0.1:1234").await {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind to port 1234: {}", e);
                return; // Exit the task if we can't bind to the port
            }
        };

        println!("Listening on port 1234");

        loop {
            // Accept connections and process them
            let (socket, _) = listener.accept().await.unwrap();
            process_query(socket).await;
        }
        
    });

    // Start the data loading service
    let data_loading_service = tokio::spawn(async {
        // Implement data loading logic here
        load_data_into_database().await;
        
    });

    // You can handle the join results here (omitted for brevity)
    let _ = tokio::join!(query_service, data_loading_service);
    Ok(())
}




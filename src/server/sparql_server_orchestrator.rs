use crate::network::sparql_servers::Server;
use crate::import::import_services::load_data_into_database;

use std::error::Error;

pub async fn startup_server() -> Result<(), Box<dyn Error>> {
    // Initialize the SPARQL server
    let server = Server::new();

    // Load data into the server
    load_data_into_database().await;

    

    // Start the server on a specific port with a specified number of worker threads and a timeout
    Server::run(
        server, 
        1234, 
        4,
        tokio::time::Duration::from_secs(30)).await?;


    Ok(())
}
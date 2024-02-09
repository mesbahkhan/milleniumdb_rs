use crate::network::sparql_servers::Server;
use crate::import::import_services::load_data_into_database;

use std::error::Error;

pub async fn startup_server() -> Result<(), Box<dyn Error>> {
    
    // Initialize the SPARQL server
    let server = Server::new();

    // Load data into the server
    load_data_into_database().await;    

    // Start the server on a specific port with a specified number of worker threads and a timeout
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
         //Initialization and output code remains the same
         //let result = startup_server().await;

        Server::run(
            server, 
            1234,
            tokio::time::Duration::from_secs(30)).await.unwrap();

        });

    Ok(())
}
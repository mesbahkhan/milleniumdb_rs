mod server;
mod query;
mod import;
mod network;

use clap::{Parser, ValueHint, Error};
use clap::error::ErrorKind; 
use std::path::PathBuf;
use std::process;



use server::sparql_server_orchestrator::startup_server;

#[derive(Parser, Debug)]
#[command(about = "MillenniumDB server", long_about = None)]
struct ServerConfig {
    #[arg(short, long, value_hint = ValueHint::DirPath)]
    db_folder: PathBuf,

    #[arg(short, long, default_value_t = 8080, value_parser = parse_positive_number::<u16>)]
    port: u16,

    #[arg(short = 't', long, default_value_t = 60, value_parser = parse_positive_number::<u64>)]
    timeout: u64,

    #[arg(long, default_value_t = 2, value_parser = parse_positive_number::<u64>)]
    string_initial_populate_size: u64,

    #[arg(long, default_value_t = 1024, value_parser = parse_positive_number::<u64>)]
    buffer_size: u64,

    #[arg(long, default_value_t = 256, value_parser = parse_positive_number::<u64>)]
    private_buffer_size: u64,

    #[arg(long, default_value_t = 4, value_parser = parse_positive_number::<u8>)]
    threads: u8,

    #[arg(short, long, default_value_t = 0, value_parser = parse_positive_number::<u64>)]
    limit: u64,
}

fn parse_positive_number<T: std::str::FromStr + std::cmp::PartialOrd + Copy + Default>(s: &str) -> Result<T, clap::Error>
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match s.parse::<T>() {
        Ok(v) if v > T::default() => Ok(v),
        Ok(_) => Err(Error::raw(ErrorKind::ValueValidation, "Must be a positive number")),
        Err(e) => Err(Error::raw(ErrorKind::ValueValidation, e.to_string())),
    }
}

#[tokio::main]
async fn main() {

    let config = ServerConfig::parse();

    if let Err(e) = validate_db_folder(&config.db_folder) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    // Initialization and output code remains the same
    match startup_server().await {
        Ok(_) => {
            println!("Server started successfully.");
            // Continue with your server logic here
        },
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            process::exit(1); // Exit the program if the server fails to start
        }
    }
}

fn validate_db_folder(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        Err(String::from("Database folder does not exist"))
    } else if !path.is_dir() {
        Err(String::from("Database folder is not a directory"))
    } else {
        Ok(()) // This indicates that the validation passed successfully
    }
}

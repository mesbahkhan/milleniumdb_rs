use std::error::Error;
use std::fmt;

// Define a custom error type for connection exceptions
#[derive(Debug)]
pub struct ConnectionException {
    message: String,
}

impl ConnectionException {
    // Constructor for ConnectionException
    pub fn new(message: &str) -> Self {
        ConnectionException {
            message: message.to_string(),
        }
    }
}

// Implement Display trait to format the error message
impl fmt::Display for ConnectionException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Implement Error trait for ConnectionException
impl Error for ConnectionException {}

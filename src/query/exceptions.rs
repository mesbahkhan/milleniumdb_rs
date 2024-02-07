use std::error::Error;
use std::fmt;

// Used when the query execution has a timeout
#[derive(Debug)]
pub struct InterruptedException;

impl fmt::Display for InterruptedException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interruption request received.")
    }
}

impl Error for InterruptedException {}

// Used when the query received is not a valid query
#[derive(Debug)]
pub struct QueryException {
    message: String,
}

impl QueryException {
    // Constructor for QueryException
    pub fn new(message: &str) -> Self {
        QueryException {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for QueryException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for QueryException {}

// Used when the query received is not a valid query (syntax error)
pub type QueryParsingException = QueryException;

// Used when the query received is not a valid query (semantic error)
#[derive(Debug)]
pub struct QuerySemanticException {
    message: String,
}

impl QuerySemanticException {
    // Constructor for QuerySemanticException
    pub fn new(msg: &str) -> Self {
        QuerySemanticException {
            message: format!("Bad query semantic: `{}`.", msg),
        }
    }
}

impl fmt::Display for QuerySemanticException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for QuerySemanticException {}

// Used when the query is correct but something went wrong during the execution.
// This exception does not mean there is a bug or something wrong with the logic, but
// an expected border-case error. For example being out of available buffers.
#[derive(Debug)] // Add Debug trait derivation here
pub struct QueryExecutionException {
    message: String,
}

impl QueryExecutionException {
    // Constructor for QueryExecutionException
    pub fn new(msg: &str) -> Self {
        QueryExecutionException {
            message: format!("Error in query execution: `{}`.", msg),
        }
    }
}

impl fmt::Display for QueryExecutionException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for QueryExecutionException {}

// Very similar to std::logic_error. It reports errors that are a consequence of faulty logic
// within the program such as violating logical preconditions or class invariants.
// It is a clear sign of a bug in the implementation or corrupted data.
#[derive(Debug)]
pub struct LogicException {
    message: String,
}

impl LogicException {
    // Constructor for LogicException
    pub fn new(msg: &str) -> Self {
        LogicException {
            message: format!("Logic Error: `{}`.", msg),
        }
    }
}

impl fmt::Display for LogicException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LogicException {}

// Used when the query received needs a feature that is not supported yet, but may be supported
// in the future
#[derive(Debug)]
pub struct NotSupportedException {
    message: String,
}

impl NotSupportedException {
    // Constructor for NotSupportedException
    pub fn new(operation: &str) -> Self {
        NotSupportedException {
            message: format!("Operation `{}` not supported yet.", operation),
        }
    }
}

impl fmt::Display for NotSupportedException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for NotSupportedException {}

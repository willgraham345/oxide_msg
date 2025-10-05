//! Error types for the Oxide messaging framework

use std::fmt;

/// Result type for Oxide operations
pub type Result<T> = std::result::Result<T, OxideError>;

/// Errors that can occur in the Oxide messaging framework
#[derive(Debug)]
pub enum OxideError {
    /// ZeroMQ error
    Zmq(zmq::Error),
    /// Serialization/deserialization error
    Serialization(String),
    /// Invalid configuration
    Configuration(String),
    /// Connection error
    Connection(String),
    /// Send error
    Send(String),
    /// Receive error
    Receive(String),
}

impl fmt::Display for OxideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OxideError::Zmq(e) => write!(f, "ZeroMQ error: {}", e),
            OxideError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            OxideError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            OxideError::Connection(msg) => write!(f, "Connection error: {}", msg),
            OxideError::Send(msg) => write!(f, "Send error: {}", msg),
            OxideError::Receive(msg) => write!(f, "Receive error: {}", msg),
        }
    }
}

impl std::error::Error for OxideError {}

impl From<zmq::Error> for OxideError {
    fn from(err: zmq::Error) -> Self {
        OxideError::Zmq(err)
    }
}

impl From<serde_json::Error> for OxideError {
    fn from(err: serde_json::Error) -> Self {
        OxideError::Serialization(err.to_string())
    }
}

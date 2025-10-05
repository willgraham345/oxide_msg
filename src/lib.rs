//! # Oxide Message Framework
//!
//! A Rust messaging framework built on top of ZeroMQ, providing high-level
//! abstractions for common messaging patterns.
//!
//! ## Features
//!
//! - Publisher/Subscriber pattern
//! - Request/Reply pattern
//! - Push/Pull pattern
//! - Easy-to-use API with error handling
//! - Support for serialization with JSON

pub mod error;
pub mod patterns;
pub mod message;

pub use error::{Result, OxideError};
pub use message::Message;
pub use patterns::{Publisher, Subscriber, Requester, Replier, Pusher, Puller};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        Publisher, Subscriber, Requester, Replier, Pusher, Puller,
        Message, Result, OxideError
    };
}

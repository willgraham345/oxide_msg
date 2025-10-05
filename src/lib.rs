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
pub mod message;
pub mod patterns;

pub use error::{OxideError, Result};
pub use message::Message;
pub use patterns::{Publisher, Puller, Pusher, Replier, Requester, Subscriber};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        Message, OxideError, Publisher, Puller, Pusher, Replier, Requester, Result, Subscriber,
    };
}

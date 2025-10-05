//! Push/Pull (pipeline) messaging pattern

use crate::error::{Result, OxideError};
use crate::message::Message;
use zmq::{Context, Socket};

/// Pusher for the push/pull pattern (sends tasks to workers)
pub struct Pusher {
    socket: Socket,
}

impl Pusher {
    /// Create a new pusher that binds to the specified address
    pub fn new_bind(address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(zmq::PUSH)?;
        socket.bind(address)?;
        Ok(Self { socket })
    }

    /// Create a new pusher that connects to the specified address
    pub fn new_connect(address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(zmq::PUSH)?;
        socket.connect(address)?;
        Ok(Self { socket })
    }

    /// Push a message to workers
    pub fn push(&self, message: &Message) -> Result<()> {
        let bytes = message.to_bytes()?;
        self.socket
            .send(&bytes, 0)
            .map_err(|e| OxideError::Send(e.to_string()))?;
        Ok(())
    }
}

/// Puller for the push/pull pattern (receives tasks from pushers)
pub struct Puller {
    socket: Socket,
}

impl Puller {
    /// Create a new puller that binds to the specified address
    pub fn new_bind(address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(zmq::PULL)?;
        socket.bind(address)?;
        Ok(Self { socket })
    }

    /// Create a new puller that connects to the specified address
    pub fn new_connect(address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(zmq::PULL)?;
        socket.connect(address)?;
        Ok(Self { socket })
    }

    /// Pull a message (blocking)
    pub fn pull(&self) -> Result<Message> {
        let bytes = self
            .socket
            .recv_bytes(0)
            .map_err(|e| OxideError::Receive(e.to_string()))?;
        Message::from_bytes(&bytes)
    }

    /// Pull a message with timeout
    pub fn pull_timeout(&self, timeout_ms: i32) -> Result<Option<Message>> {
        self.socket
            .set_rcvtimeo(timeout_ms)
            .map_err(|e| OxideError::Configuration(e.to_string()))?;

        match self.socket.recv_bytes(0) {
            Ok(bytes) => Ok(Some(Message::from_bytes(&bytes)?)),
            Err(zmq::Error::EAGAIN) => Ok(None),
            Err(e) => Err(OxideError::Receive(e.to_string())),
        }
    }

    /// Try to pull a message without blocking
    pub fn try_pull(&self) -> Result<Option<Message>> {
        match self.socket.recv_bytes(zmq::DONTWAIT) {
            Ok(bytes) => Ok(Some(Message::from_bytes(&bytes)?)),
            Err(zmq::Error::EAGAIN) => Ok(None),
            Err(e) => Err(OxideError::Receive(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_pipeline_basic() {
        let address = "tcp://127.0.0.1:5557";

        // Start puller in a separate thread
        let worker_handle = thread::spawn(move || {
            // Give pusher time to bind
            thread::sleep(Duration::from_millis(100));
            
            let puller = Puller::new_connect(address).unwrap();
            let task = puller.pull_timeout(1000).unwrap();
            assert!(task.is_some());
            let msg = task.unwrap();
            assert_eq!(msg.topic, "task");
            assert_eq!(msg.payload["id"], 1);
        });

        // Create pusher
        let pusher = Pusher::new_bind(address).unwrap();
        // Give puller time to connect
        thread::sleep(Duration::from_millis(200));

        let task = Message::new("task", json!({"id": 1, "data": "process this"}));
        pusher.push(&task).unwrap();

        worker_handle.join().unwrap();
    }
}

//! Publisher/Subscriber messaging pattern

use crate::error::{Result, OxideError};
use crate::message::Message;
use zmq::{Context, Socket};

/// Publisher for the pub/sub pattern
pub struct Publisher {
    socket: Socket,
}

impl Publisher {
    /// Create a new publisher that binds to the specified address
    pub fn new(address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(zmq::PUB)?;
        socket.bind(address)?;
        Ok(Self { socket })
    }

    /// Publish a message
    pub fn publish(&self, message: &Message) -> Result<()> {
        let bytes = message.to_bytes()?;
        self.socket
            .send(&bytes, 0)
            .map_err(|e| OxideError::Send(e.to_string()))?;
        Ok(())
    }

    /// Publish raw bytes with a topic prefix
    pub fn publish_raw(&self, topic: &str, data: &[u8]) -> Result<()> {
        // Send topic as first frame
        self.socket
            .send(topic.as_bytes(), zmq::SNDMORE)
            .map_err(|e| OxideError::Send(e.to_string()))?;
        // Send data as second frame
        self.socket
            .send(data, 0)
            .map_err(|e| OxideError::Send(e.to_string()))?;
        Ok(())
    }
}

/// Subscriber for the pub/sub pattern
pub struct Subscriber {
    socket: Socket,
}

impl Subscriber {
    /// Create a new subscriber that connects to the specified address
    pub fn new(address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(zmq::SUB)?;
        socket.connect(address)?;
        Ok(Self { socket })
    }

    /// Subscribe to messages with a specific topic prefix
    /// Use an empty string to subscribe to all messages
    pub fn subscribe(&self, topic: &str) -> Result<()> {
        self.socket
            .set_subscribe(topic.as_bytes())
            .map_err(|e| OxideError::Configuration(e.to_string()))?;
        Ok(())
    }

    /// Unsubscribe from a topic
    pub fn unsubscribe(&self, topic: &str) -> Result<()> {
        self.socket
            .set_unsubscribe(topic.as_bytes())
            .map_err(|e| OxideError::Configuration(e.to_string()))?;
        Ok(())
    }

    /// Receive a message (blocking)
    pub fn receive(&self) -> Result<Message> {
        let bytes = self
            .socket
            .recv_bytes(0)
            .map_err(|e| OxideError::Receive(e.to_string()))?;
        Message::from_bytes(&bytes)
    }

    /// Receive a message with timeout in milliseconds
    /// Returns None if timeout expires
    pub fn receive_timeout(&self, timeout_ms: i32) -> Result<Option<Message>> {
        self.socket
            .set_rcvtimeo(timeout_ms)
            .map_err(|e| OxideError::Configuration(e.to_string()))?;
        
        match self.socket.recv_bytes(0) {
            Ok(bytes) => Ok(Some(Message::from_bytes(&bytes)?)),
            Err(zmq::Error::EAGAIN) => Ok(None),
            Err(e) => Err(OxideError::Receive(e.to_string())),
        }
    }

    /// Check if a message is available without blocking
    pub fn try_receive(&self) -> Result<Option<Message>> {
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
    fn test_pubsub_basic() {
        let address = "tcp://127.0.0.1:5555";
        
        // Create publisher in a separate thread
        let pub_handle = thread::spawn(move || {
            let publisher = Publisher::new(address).unwrap();
            // Give subscriber time to connect
            thread::sleep(Duration::from_millis(100));
            
            let msg = Message::new("test", json!({"data": "hello"}));
            publisher.publish(&msg).unwrap();
        });

        // Create subscriber
        let subscriber = Subscriber::new(address).unwrap();
        subscriber.subscribe("").unwrap(); // Subscribe to all
        
        pub_handle.join().unwrap();
        
        // Try to receive with timeout
        let received = subscriber.receive_timeout(1000).unwrap();
        assert!(received.is_some());
        let msg = received.unwrap();
        assert_eq!(msg.topic, "test");
    }
}

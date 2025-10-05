//! Request/Reply messaging pattern

use crate::error::{Result, OxideError};
use crate::message::Message;
use zmq::{Context, Socket};

/// Requester for the request/reply pattern (client side)
pub struct Requester {
    socket: Socket,
}

impl Requester {
    /// Create a new requester that connects to the specified address
    pub fn new(address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(zmq::REQ)?;
        socket.connect(address)?;
        Ok(Self { socket })
    }

    /// Send a request and wait for a reply
    pub fn request(&self, message: &Message) -> Result<Message> {
        let bytes = message.to_bytes()?;
        self.socket
            .send(&bytes, 0)
            .map_err(|e| OxideError::Send(e.to_string()))?;

        let reply_bytes = self
            .socket
            .recv_bytes(0)
            .map_err(|e| OxideError::Receive(e.to_string()))?;
        Message::from_bytes(&reply_bytes)
    }

    /// Send a request and wait for a reply with timeout
    pub fn request_timeout(&self, message: &Message, timeout_ms: i32) -> Result<Option<Message>> {
        let bytes = message.to_bytes()?;
        self.socket
            .send(&bytes, 0)
            .map_err(|e| OxideError::Send(e.to_string()))?;

        self.socket
            .set_rcvtimeo(timeout_ms)
            .map_err(|e| OxideError::Configuration(e.to_string()))?;

        match self.socket.recv_bytes(0) {
            Ok(reply_bytes) => Ok(Some(Message::from_bytes(&reply_bytes)?)),
            Err(zmq::Error::EAGAIN) => Ok(None),
            Err(e) => Err(OxideError::Receive(e.to_string())),
        }
    }
}

/// Replier for the request/reply pattern (server side)
pub struct Replier {
    socket: Socket,
}

impl Replier {
    /// Create a new replier that binds to the specified address
    pub fn new(address: &str) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(zmq::REP)?;
        socket.bind(address)?;
        Ok(Self { socket })
    }

    /// Receive a request (blocking)
    pub fn receive(&self) -> Result<Message> {
        let bytes = self
            .socket
            .recv_bytes(0)
            .map_err(|e| OxideError::Receive(e.to_string()))?;
        Message::from_bytes(&bytes)
    }

    /// Receive a request with timeout
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

    /// Send a reply
    pub fn reply(&self, message: &Message) -> Result<()> {
        let bytes = message.to_bytes()?;
        self.socket
            .send(&bytes, 0)
            .map_err(|e| OxideError::Send(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_reqrep_basic() {
        let address = "tcp://127.0.0.1:5556";

        // Start replier in a separate thread
        let server_handle = thread::spawn(move || {
            let replier = Replier::new(address).unwrap();
            // Give requester time to connect
            thread::sleep(Duration::from_millis(100));

            let request = replier.receive().unwrap();
            assert_eq!(request.topic, "ping");

            let reply = Message::new("pong", json!({"status": "ok"}));
            replier.reply(&reply).unwrap();
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(200));

        // Create requester
        let requester = Requester::new(address).unwrap();
        let request = Message::new("ping", json!({"data": "test"}));
        let reply = requester.request(&request).unwrap();

        assert_eq!(reply.topic, "pong");
        assert_eq!(reply.payload["status"], "ok");

        server_handle.join().unwrap();
    }
}

//! Message types and serialization

use serde::{Deserialize, Serialize};
use crate::error::{Result, OxideError};

/// A message that can be sent through the Oxide framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message topic or identifier
    pub topic: String,
    /// Message payload as JSON
    pub payload: serde_json::Value,
}

impl Message {
    /// Create a new message
    pub fn new(topic: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            topic: topic.into(),
            payload,
        }
    }

    /// Create a message from a serializable value
    pub fn from_value<T: Serialize>(topic: impl Into<String>, value: &T) -> Result<Self> {
        let payload = serde_json::to_value(value)
            .map_err(|e| OxideError::Serialization(e.to_string()))?;
        Ok(Self {
            topic: topic.into(),
            payload,
        })
    }

    /// Serialize the message to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self).map_err(|e| OxideError::Serialization(e.to_string()))
    }

    /// Deserialize a message from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(|e| OxideError::Serialization(e.to_string()))
    }

    /// Deserialize the payload to a specific type
    pub fn payload_as<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        serde_json::from_value(self.payload.clone())
            .map_err(|e| OxideError::Serialization(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_message_creation() {
        let msg = Message::new("test_topic", json!({"key": "value"}));
        assert_eq!(msg.topic, "test_topic");
        assert_eq!(msg.payload, json!({"key": "value"}));
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::new("test", json!({"data": 42}));
        let bytes = msg.to_bytes().unwrap();
        let decoded = Message::from_bytes(&bytes).unwrap();
        assert_eq!(msg.topic, decoded.topic);
        assert_eq!(msg.payload, decoded.payload);
    }

    #[test]
    fn test_payload_deserialization() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestData {
            value: i32,
        }

        let data = TestData { value: 42 };
        let msg = Message::from_value("test", &data).unwrap();
        let decoded: TestData = msg.payload_as().unwrap();
        assert_eq!(data, decoded);
    }
}

# Oxide Message Framework

A Rust messaging framework built on top of ZeroMQ, providing high-level abstractions for common messaging patterns. Designed for work with OpenC3 and general-purpose distributed systems.

## Features

- **Multiple Messaging Patterns**
  - Publisher/Subscriber (Pub/Sub)
  - Request/Reply (Req/Rep)
  - Push/Pull (Pipeline)
  
- **Easy-to-use API** with Rust error handling
- **JSON serialization** support via serde
- **Type-safe messaging** with generic payload support
- **Non-blocking and timeout** operations available

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
oxide_msg = "0.1"
```

## Quick Start

### Publisher/Subscriber Pattern

**Publisher:**
```rust
use oxide_msg::prelude::*;
use serde_json::json;

let publisher = Publisher::new("tcp://127.0.0.1:5555")?;
let message = Message::new("sensor_data", json!({
    "temperature": 25.5,
    "humidity": 60.0
}));
publisher.publish(&message)?;
```

**Subscriber:**
```rust
use oxide_msg::prelude::*;

let subscriber = Subscriber::new("tcp://127.0.0.1:5555")?;
subscriber.subscribe("")?; // Subscribe to all topics

loop {
    let message = subscriber.receive()?;
    println!("Received: {:?}", message);
}
```

### Request/Reply Pattern

**Server (Replier):**
```rust
use oxide_msg::prelude::*;
use serde_json::json;

let replier = Replier::new("tcp://127.0.0.1:5556")?;

loop {
    let request = replier.receive()?;
    let reply = Message::new("response", json!({"status": "ok"}));
    replier.reply(&reply)?;
}
```

**Client (Requester):**
```rust
use oxide_msg::prelude::*;
use serde_json::json;

let requester = Requester::new("tcp://127.0.0.1:5556")?;
let request = Message::new("query", json!({"action": "get_data"}));
let reply = requester.request(&request)?;
println!("Reply: {:?}", reply);
```

### Push/Pull (Pipeline) Pattern

**Task Distributor (Pusher):**
```rust
use oxide_msg::prelude::*;
use serde_json::json;

let pusher = Pusher::new_bind("tcp://127.0.0.1:5557")?;
for i in 0..10 {
    let task = Message::new("task", json!({"id": i}));
    pusher.push(&task)?;
}
```

**Worker (Puller):**
```rust
use oxide_msg::prelude::*;

let puller = Puller::new_connect("tcp://127.0.0.1:5557")?;
loop {
    if let Some(task) = puller.pull_timeout(5000)? {
        println!("Processing task: {:?}", task);
        // Process the task...
    }
}
```

## Examples

The repository includes several complete examples:

- **pubsub_example.rs** - Publisher/Subscriber pattern with sensor data
- **reqrep_example.rs** - Request/Reply pattern with echo and add operations
- **pipeline_example.rs** - Push/Pull pattern with task distribution

Run examples with:
```bash
# Publisher/Subscriber
cargo run --example pubsub_example publisher
cargo run --example pubsub_example subscriber

# Request/Reply
cargo run --example reqrep_example server
cargo run --example reqrep_example client

# Pipeline
cargo run --example pipeline_example sink
cargo run --example pipeline_example worker
cargo run --example pipeline_example ventilator
```

## Architecture

The framework is organized into the following modules:

- `error` - Error types and result handling
- `message` - Message structure and serialization
- `patterns` - Messaging pattern implementations
  - `pubsub` - Publisher/Subscriber pattern
  - `reqrep` - Request/Reply pattern
  - `pipeline` - Push/Pull pattern

## Message Structure

Messages in Oxide are structured with a topic and a JSON payload:

```rust
pub struct Message {
    pub topic: String,
    pub payload: serde_json::Value,
}
```

You can create messages from any serializable type:

```rust
#[derive(Serialize)]
struct SensorData {
    temperature: f64,
    humidity: f64,
}

let data = SensorData { temperature: 25.5, humidity: 60.0 };
let message = Message::from_value("sensors", &data)?;
```

## Testing

Run the test suite with:
```bash
cargo test
```

All tests use unique ports to avoid conflicts and include proper synchronization.

## Dependencies

- **zmq** (0.10) - ZeroMQ bindings for Rust
- **serde** (1.0) - Serialization framework
- **serde_json** (1.0) - JSON support

## License

See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

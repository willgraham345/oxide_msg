# Oxide Message Framework - Usage Guide

This guide provides detailed information on how to use the Oxide messaging framework.

## Core Concepts

### Messages

All messages in Oxide consist of:
- **topic**: A string identifier for the message type
- **payload**: A JSON value containing the message data

```rust
use oxide_msg::Message;
use serde_json::json;

let msg = Message::new("my_topic", json!({
    "key": "value",
    "number": 42
}));
```

### Error Handling

The framework uses Result types for all operations:

```rust
use oxide_msg::Result;

fn send_message() -> Result<()> {
    let publisher = Publisher::new("tcp://127.0.0.1:5555")?;
    let msg = Message::new("test", json!({}));
    publisher.publish(&msg)?;
    Ok(())
}
```

## Messaging Patterns

### 1. Publisher/Subscriber (Pub/Sub)

Best for: Broadcasting messages to multiple subscribers.

**Key Characteristics:**
- One publisher, multiple subscribers
- Fire-and-forget (no acknowledgment)
- Subscribers can filter by topic
- Late joiners miss early messages (slow joiner problem)

**Example:**

```rust
// Publisher
let publisher = Publisher::new("tcp://127.0.0.1:5555")?;
publisher.publish(&Message::new("topic", json!({"data": "value"})))?;

// Subscriber
let subscriber = Subscriber::new("tcp://127.0.0.1:5555")?;
subscriber.subscribe("")?; // Subscribe to all
let msg = subscriber.receive()?;
```

**Best Practices:**
- Start subscribers before publishers to avoid missing messages
- Use specific topics to filter messages
- Use `receive_timeout()` for non-blocking operation

### 2. Request/Reply (Req/Rep)

Best for: Client-server communication requiring responses.

**Key Characteristics:**
- Synchronous request-response
- One request, one reply
- Automatic load balancing with multiple repliers
- Client blocks until reply received

**Example:**

```rust
// Server (Replier)
let replier = Replier::new("tcp://127.0.0.1:5556")?;
loop {
    let request = replier.receive()?;
    let reply = Message::new("response", json!({"status": "ok"}));
    replier.reply(&reply)?;
}

// Client (Requester)
let requester = Requester::new("tcp://127.0.0.1:5556")?;
let reply = requester.request(&Message::new("request", json!({})))?;
```

**Best Practices:**
- Use `request_timeout()` to avoid blocking indefinitely
- Keep request/reply payloads small
- Multiple repliers can share the same address for load balancing

### 3. Push/Pull (Pipeline)

Best for: Distributing tasks to multiple workers.

**Key Characteristics:**
- Load-balanced task distribution
- One-way communication (no replies)
- Fair queuing among workers
- Multiple pushers and pullers supported

**Example:**

```rust
// Task distributor
let pusher = Pusher::new_bind("tcp://127.0.0.1:5557")?;
pusher.push(&Message::new("task", json!({"id": 1})))?;

// Worker
let puller = Puller::new_connect("tcp://127.0.0.1:5557")?;
let task = puller.pull()?;
// Process task...
```

**Best Practices:**
- Use `new_bind()` for task distributors
- Use `new_connect()` for workers
- Use multiple workers for parallel processing
- Add a sink (result collector) for collecting results

## Advanced Features

### Timeouts

All receive operations support timeouts:

```rust
// Timeout in milliseconds
if let Some(msg) = subscriber.receive_timeout(1000)? {
    println!("Received: {:?}", msg);
} else {
    println!("Timeout!");
}
```

### Non-blocking Operations

Try to receive without blocking:

```rust
match subscriber.try_receive()? {
    Some(msg) => println!("Got message: {:?}", msg),
    None => println!("No message available"),
}
```

### Custom Serialization

Use custom types with messages:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CustomData {
    field1: String,
    field2: i32,
}

let data = CustomData {
    field1: "hello".to_string(),
    field2: 42,
};

let msg = Message::from_value("custom", &data)?;
let decoded: CustomData = msg.payload_as()?;
```

## Network Addresses

Oxide supports various ZeroMQ transport protocols:

- **TCP**: `tcp://127.0.0.1:5555` (network communication)
- **IPC**: `ipc:///tmp/socket` (inter-process on same machine)
- **Inproc**: `inproc://myqueue` (in-process, same program)

## Common Patterns

### Publisher with Multiple Topics

```rust
let publisher = Publisher::new("tcp://127.0.0.1:5555")?;

loop {
    publisher.publish(&Message::new("topic1", json!({"data": 1})))?;
    publisher.publish(&Message::new("topic2", json!({"data": 2})))?;
}
```

### Subscriber Filtering

```rust
let subscriber = Subscriber::new("tcp://127.0.0.1:5555")?;
subscriber.subscribe("topic1")?; // Only receive topic1 messages
```

### Worker Pool

```rust
// Start multiple workers
for worker_id in 0..5 {
    thread::spawn(move || {
        let puller = Puller::new_connect("tcp://127.0.0.1:5557").unwrap();
        loop {
            let task = puller.pull().unwrap();
            process_task(task);
        }
    });
}
```

## Troubleshooting

### Slow Joiner Problem

If subscribers miss messages, start them before the publisher and add a delay:

```rust
let publisher = Publisher::new("tcp://127.0.0.1:5555")?;
thread::sleep(Duration::from_millis(100)); // Give subscribers time to connect
publisher.publish(&msg)?;
```

### Port Already in Use

Ensure only one process binds to a specific address:

```rust
// Only one of these should exist
let publisher1 = Publisher::new("tcp://127.0.0.1:5555")?; // OK
let publisher2 = Publisher::new("tcp://127.0.0.1:5555")?; // ERROR
```

### Connection Refused

Ensure the server (bind) starts before clients (connect):

```rust
// Start server first
let replier = Replier::new("tcp://127.0.0.1:5556")?; // Binds

// Then connect client
let requester = Requester::new("tcp://127.0.0.1:5556")?; // Connects
```

## Performance Tips

1. **Reuse connections** - Create sockets once and reuse them
2. **Batch operations** - Send multiple messages in sequence
3. **Use appropriate pattern** - Choose the right pattern for your use case
4. **Monitor queues** - ZeroMQ has high-water marks to prevent memory issues
5. **Use binary protocols** - For performance-critical apps, consider msgpack or protobuf

## Testing

When testing, use unique ports for each test to avoid conflicts:

```rust
#[test]
fn test_my_feature() {
    let publisher = Publisher::new("tcp://127.0.0.1:15555").unwrap();
    // ...
}
```

## Further Reading

- [ZeroMQ Guide](https://zguide.zeromq.org/)
- [Rust ZeroMQ bindings](https://github.com/erickt/rust-zmq)
- [serde documentation](https://serde.rs/)

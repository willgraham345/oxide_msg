//! Publisher/Subscriber pattern example
//!
//! Run the publisher with: cargo run --example pubsub_example publisher
//! Run the subscriber with: cargo run --example pubsub_example subscriber

use oxide_msg::prelude::*;
use serde_json::json;
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match mode {
        "publisher" => run_publisher(),
        "subscriber" => run_subscriber(),
        _ => {
            println!("Usage: {} [publisher|subscriber]", args[0]);
            println!("\nPublisher: Publishes messages on tcp://127.0.0.1:5555");
            println!("Subscriber: Subscribes to messages on tcp://127.0.0.1:5555");
            Ok(())
        }
    }
}

fn run_publisher() -> Result<()> {
    println!("Starting publisher on tcp://127.0.0.1:5555");
    let publisher = Publisher::new("tcp://127.0.0.1:5555")?;

    // Give subscribers time to connect
    thread::sleep(Duration::from_millis(500));

    let mut count = 0;
    loop {
        let message = Message::new(
            "sensor_data",
            json!({
                "timestamp": count,
                "temperature": 20.0 + (count as f64 * 0.1),
                "humidity": 50.0 + (count as f64 * 0.2)
            }),
        );

        publisher.publish(&message)?;
        println!("Published message #{}: {:?}", count, message);

        count += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

fn run_subscriber() -> Result<()> {
    println!("Starting subscriber connecting to tcp://127.0.0.1:5555");
    let subscriber = Subscriber::new("tcp://127.0.0.1:5555")?;

    // Subscribe to all messages
    subscriber.subscribe("")?;
    println!("Subscribed to all topics, waiting for messages...");

    loop {
        match subscriber.receive() {
            Ok(message) => {
                println!("Received message: {:?}", message);
                println!("  Topic: {}", message.topic);
                println!("  Payload: {}", message.payload);
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }

    Ok(())
}

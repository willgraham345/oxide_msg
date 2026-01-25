//! Request/Reply pattern example
//!
//! Run the server with: cargo run --example reqrep_example server
//! Run the client with: cargo run --example reqrep_example client

use oxide_msg::prelude::*;
use serde_json::json;
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match mode {
        "server" => run_server(),
        "client" => run_client(),
        _ => {
            println!("Usage: {} [server|client]", args[0]);
            println!("\nServer: Listens for requests on tcp://127.0.0.1:5556");
            println!("Client: Sends requests to tcp://127.0.0.1:5556");
            Ok(())
        }
    }
}

fn run_server() -> Result<()> {
    println!("Starting server on tcp://127.0.0.1:5556");
    let replier = Replier::new("tcp://127.0.0.1:5556")?;

    println!("Server ready, waiting for requests...");
    let mut request_count = 0;

    loop {
        match replier.receive() {
            Ok(request) => {
                request_count += 1;
                println!("\n[Request #{}]", request_count);
                println!("  Topic: {}", request.topic);
                println!("  Payload: {}", request.payload);

                // Process the request
                let reply = match request.topic.as_str() {
                    "echo" => Message::new(
                        "echo_reply",
                        json!({
                            "original": request.payload,
                            "request_count": request_count
                        }),
                    ),
                    "add" => {
                        let a = request.payload["a"].as_i64().unwrap_or(0);
                        let b = request.payload["b"].as_i64().unwrap_or(0);
                        Message::new("add_reply", json!({"result": a + b}))
                    }
                    _ => Message::new("error", json!({"message": "Unknown request type"})),
                };

                replier.reply(&reply)?;
                println!("  Sent reply: {:?}", reply);
            }
            Err(e) => {
                eprintln!("Error receiving request: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn run_client() -> Result<()> {
    println!("Connecting to server at tcp://127.0.0.1:5556");
    let requester = Requester::new("tcp://127.0.0.1:5556")?;

    // Give server time to start
    thread::sleep(Duration::from_millis(100));

    // Send echo request
    println!("\nSending echo request...");
    let echo_request = Message::new("echo", json!({"message": "Hello, Server!"}));
    let echo_reply = requester.request(&echo_request)?;
    println!("Received reply: {:?}", echo_reply);

    // Send add request
    println!("\nSending add request...");
    let add_request = Message::new("add", json!({"a": 42, "b": 58}));
    let add_reply = requester.request(&add_request)?;
    println!("Received reply: {:?}", add_reply);

    println!("\nAll requests completed successfully!");
    Ok(())
}

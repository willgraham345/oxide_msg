//! Push/Pull (pipeline) pattern example
//!
//! Run the ventilator with: cargo run --example pipeline_example ventilator
//! Run the worker with: cargo run --example pipeline_example worker
//! Run the sink with: cargo run --example pipeline_example sink

use oxide_msg::prelude::*;
use serde_json::json;
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match mode {
        "ventilator" => run_ventilator(),
        "worker" => run_worker(),
        "sink" => run_sink(),
        _ => {
            println!("Usage: {} [ventilator|worker|sink]", args[0]);
            println!("\nVentilator: Sends tasks to workers on tcp://127.0.0.1:5557");
            println!("Worker: Processes tasks and sends results to sink");
            println!("Sink: Collects results from workers on tcp://127.0.0.1:5558");
            Ok(())
        }
    }
}

fn run_ventilator() -> Result<()> {
    println!("Starting ventilator (task distributor)");
    let pusher = Pusher::new_bind("tcp://127.0.0.1:5557")?;

    // Give workers time to connect
    thread::sleep(Duration::from_millis(500));

    println!("Distributing tasks to workers...");
    for task_id in 0..10 {
        let task = Message::new(
            "task",
            json!({
                "id": task_id,
                "workload": (task_id % 5) + 1  // Workload from 1 to 5 seconds
            }),
        );

        pusher.push(&task)?;
        println!("Sent task #{}: workload={}", task_id, (task_id % 5) + 1);
    }

    println!("All tasks distributed!");
    Ok(())
}

fn run_worker() -> Result<()> {
    let worker_id = std::process::id();
    println!("Starting worker {} connecting to ventilator", worker_id);
    
    let puller = Puller::new_connect("tcp://127.0.0.1:5557")?;
    let pusher = Pusher::new_connect("tcp://127.0.0.1:5558")?;

    println!("Worker {} ready, waiting for tasks...", worker_id);

    loop {
        match puller.pull_timeout(5000)? {
            Some(task) => {
                let task_id = task.payload["id"].as_i64().unwrap_or(0);
                let workload = task.payload["workload"].as_i64().unwrap_or(1);

                println!(
                    "Worker {} processing task #{} (workload: {}s)",
                    worker_id, task_id, workload
                );

                // Simulate work
                thread::sleep(Duration::from_secs(workload as u64));

                // Send result to sink
                let result = Message::new(
                    "result",
                    json!({
                        "task_id": task_id,
                        "worker_id": worker_id,
                        "completed": true
                    }),
                );

                pusher.push(&result)?;
                println!("Worker {} completed task #{}", worker_id, task_id);
            }
            None => {
                println!("Worker {} timed out waiting for tasks, shutting down", worker_id);
                break;
            }
        }
    }

    Ok(())
}

fn run_sink() -> Result<()> {
    println!("Starting sink (result collector)");
    let puller = Puller::new_bind("tcp://127.0.0.1:5558")?;

    println!("Sink ready, waiting for results...");
    let mut completed_tasks = 0;

    loop {
        match puller.pull_timeout(10000)? {
            Some(result) => {
                completed_tasks += 1;
                let task_id = result.payload["task_id"].as_i64().unwrap_or(0);
                let worker_id = result.payload["worker_id"].as_i64().unwrap_or(0);

                println!(
                    "Received result #{}: task_id={}, worker_id={}",
                    completed_tasks, task_id, worker_id
                );
            }
            None => {
                println!("Sink timed out waiting for results");
                break;
            }
        }
    }

    println!("Total tasks completed: {}", completed_tasks);
    Ok(())
}

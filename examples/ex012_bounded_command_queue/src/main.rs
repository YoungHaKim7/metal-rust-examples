//! Example 012 — Limit in-flight command buffers.
//!
//! `new_command_queue(Some(2))` asks Metal for a queue that keeps at most two
//! command buffers alive at once. That bound is a scheduling hint you can use
//! to keep latency predictable. Every creation path in metal-rust returns a
//! `Result`, and runtimes that cannot satisfy the request return an error with
//! `ErrorKind::Unsupported` instead of panicking.
//!
//! Run with:
//! ```text
//! cargo run -p ex012_bounded_command_queue
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ErrorKind};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let queue = match device.new_command_queue(Some(2)) {
        Ok(queue) => queue,
        Err(error) if error.kind() == ErrorKind::Unsupported => {
            println!("Bounded queues are unsupported here; example ends gracefully.");
            return;
        }
        Err(error) => {
            println!("Unexpected queue error: {error}");
            return;
        }
    };
    println!("Created a bounded command queue.");
    let first = queue.command_buffer().expect("first command buffer");
    first.set_label(Some("bounded-queue/cb-1"));
    let second = queue.command_buffer().expect("second command buffer");
    second.set_label(Some("bounded-queue/cb-2"));
    first.commit().wait().expect("first submission completes");
    second.commit().wait().expect("second submission completes");
    println!("Both command buffers submitted and waited for.");
}

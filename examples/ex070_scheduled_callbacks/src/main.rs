//! Example 070 — on_scheduled and wait_until_scheduled.
//!
//! Scheduling is the moment Metal assigns the work to GPU hardware.
//! `on_scheduled()` fires a callback at that point and
//! `wait_until_scheduled()` blocks for it — useful for throttling submissions
//! before the full completion wait.
//!
//! Run with:
//! ```text
//! cargo run -p ex070_scheduled_callbacks
//! ```

#![forbid(unsafe_code)]

use std::{
    sync::mpsc::{Receiver, channel},
    time::Duration,
};

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    // Errors are !Send as well, so the callback ships a summary string.
    let (sender, receiver): (_, Receiver<String>) = channel();
    command_buffer
        .on_scheduled(move |result| {
            let message = match result {
                Ok(()) => "scheduled".to_string(),
                Err(error) => format!("scheduling failed: {error}"),
            };
            let _ = sender.send(message);
        })
        .expect("scheduled handler registered");
    let submitted = command_buffer.commit();
    submitted
        .wait_until_scheduled()
        .expect("scheduling succeeds");
    println!("wait_until_scheduled returned; the GPU has picked the work up.");
    let completed = submitted.wait().expect("submission completes");
    println!(
        "Full completion status ok: {}",
        completed.status() == metal_rust::CommandBufferStatus::Completed
    );
    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(message) => println!("The on_scheduled callback fired: {message}."),
        Err(timeout) => println!("Callback did not arrive in time: {timeout}"),
    }
}

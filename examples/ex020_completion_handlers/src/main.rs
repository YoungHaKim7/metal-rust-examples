//! Example 020 — on_complete callbacks with channels.
//!
//! `on_complete()` registers a handler that runs when the GPU finishes. The
//! handler must be `Send + 'static`, so it cannot borrow GPU objects; the
//! idiomatic bridge back to the main thread is an `std::sync::mpsc` channel.
//! The handler receives `Result<(), Error>` for the submission itself.
//!
//! Run with:
//! ```text
//! cargo run -p ex020_completion_handlers
//! ```

#![forbid(unsafe_code)]

use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let queue = device.new_command_queue(None).expect("command queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    // GPU objects are !Send, so the callback summarizes the result and ships
    // a plain String across the channel instead of a Library or a Buffer.
    let (sender, receiver): (_, Receiver<String>) = channel();
    command_buffer
        .on_complete(move |result| {
            let message = match result {
                Ok(()) => "GPU work succeeded".to_string(),
                Err(error) => format!("GPU work failed: {error}"),
            };
            let _ = sender.send(message);
        })
        .expect("completion handler registration");
    command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(message) => println!("Completion handler delivered: {message}"),
        Err(timeout) => println!("Handler did not fire within five seconds: {timeout}"),
    }
    println!("Channels keep the callback world and the owning thread apart.");
}

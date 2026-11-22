//! Example 011 — Create and label a command queue.
//!
//! A `CommandQueue` serializes GPU work. `new_command_queue(None)` asks Metal
//! for the default queue; `set_label(Some(...))` names it so the GPU debugger
//! and Instruments show readable labels instead of hex addresses.
//!
//! Run with:
//! ```text
//! cargo run -p ex011_first_command_queue
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let queue = match device.new_command_queue(None) {
        Ok(queue) => queue,
        Err(error) => {
            println!("Command queues are unavailable on this runtime: {error}");
            return;
        }
    };
    queue.set_label(Some("examples/first-command-queue"));
    println!("Queue label : {}", queue.label().unwrap_or_default());
    println!("Owning dev. : {}", queue.device().name());
    println!("The queue is now ready to hand out command buffers.");
}

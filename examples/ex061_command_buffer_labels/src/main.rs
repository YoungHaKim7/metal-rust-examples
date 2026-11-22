//! Example 061 — Labels, device and queue back-references.
//!
//! CommandBuffers carry labels into the GPU debugger and know where they came
//! from: `device()` and `command_queue()` walk back up the ownership chain
//! without global state.
//!
//! Run with:
//! ```text
//! cargo run -p ex061_command_buffer_labels
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let queue = device.new_command_queue(None).expect("queue");
    queue.set_label(Some("examples/labels-queue"));
    let command_buffer = queue.command_buffer().expect("command buffer");
    command_buffer.set_label(Some("examples/labels/cb-1"));
    println!(
        "Command buffer label: {}",
        command_buffer.label().unwrap_or_default()
    );
    println!("Its device          : {}", command_buffer.device().name());
    println!(
        "Its queue label     : {}",
        command_buffer.command_queue().label().unwrap_or_default()
    );
    command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    println!("Labeled buffers show up nicely in Xcode's GPU capture.");
}

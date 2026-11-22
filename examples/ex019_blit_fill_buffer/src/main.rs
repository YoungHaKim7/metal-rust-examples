//! Example 019 — Fill buffer ranges on the GPU.
//!
//! `fill_buffer(buffer, range, value)` paints a byte pattern across a checked
//! range without involving the CPU. It is the fastest way to clear or stamp
//! GPU-only buffers.
//!
//! Run with:
//! ```text
//! cargo run -p ex019_blit_fill_buffer
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ResourceOptions};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let buffer = device
        .new_buffer(32, ResourceOptions::PRIVATE)
        .expect("private buffer");
    let queue = device.new_command_queue(None).expect("command queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit encoder");
        blit.fill_buffer(&buffer, 0..16, 0xAA)
            .expect("fill first range");
        blit.fill_buffer(&buffer, 16..32, 0x00)
            .expect("fill second range");
        let readback = blit.read_buffer(&buffer, 0, 32).expect("staged readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let bytes = completed
        .resolve_buffer(readback)
        .expect("resolve readback");
    println!("First  half: {:?}", &bytes[..16]);
    println!("Second half: {:?}", &bytes[16..]);
    assert!(bytes[..16].iter().all(|byte| *byte == 0xAA));
    assert!(bytes[16..].iter().all(|byte| *byte == 0x00));
    println!("The GPU filled both ranges exactly as requested.");
}

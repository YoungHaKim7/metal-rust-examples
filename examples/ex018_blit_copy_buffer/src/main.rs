//! Example 018 — GPU-side buffer copies with readback.
//!
//! Blit encoders move data on the GPU. `copy_buffer()` copies a checked byte
//! range, and `read_buffer()` records a staged readback that is resolved
//! against a *completed* submission — the library's answer to GPU
//! synchronization: you may only read bytes from a submission the GPU has
//! provably finished.
//!
//! Run with:
//! ```text
//! cargo run -p ex018_blit_copy_buffer
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ResourceOptions};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let source = device
        .new_buffer_with_bytes(&[10, 20, 30, 40, 50, 60, 70, 80], ResourceOptions::SHARED)
        .expect("source buffer");
    let destination = device
        .new_buffer(64, ResourceOptions::SHARED)
        .expect("destination buffer");
    let queue = device.new_command_queue(None).expect("command queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit encoder");
        blit.copy_buffer(&source, 0, &destination, 16, 8)
            .expect("checked copy");
        let readback = blit
            .read_buffer(&destination, 16, 8)
            .expect("staged readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let bytes = completed
        .resolve_buffer(readback)
        .expect("readback resolves");
    println!("Copied bytes on the GPU: {bytes:?}");
    assert_eq!(bytes, vec![10, 20, 30, 40, 50, 60, 70, 80]);
    println!("GPU copy succeeded and the staged readback matches the source.");
}

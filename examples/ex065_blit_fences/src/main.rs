//! Example 065 — update_fence and wait_for_fence in blit passes.
//!
//! Fences guard GPU-side resource access between encoder passes. One blit
//! pass updates the fence after its copies; the next pass waits on it before
//! its own work runs.
//!
//! Run with:
//! ```text
//! cargo run -p ex065_blit_fences
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ResourceOptions};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let fence = device.new_fence().expect("fence");
    let source = device
        .new_buffer_with_bytes(&[9, 9, 9, 9], ResourceOptions::SHARED)
        .expect("source");
    let destination = device
        .new_buffer(4, ResourceOptions::SHARED)
        .expect("destination");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let blit = command_buffer.blit_encoder().expect("first blit");
        blit.copy_buffer(&source, 0, &destination, 0, 4)
            .expect("copy");
        blit.update_fence(&fence).expect("update fence");
        blit.end_encoding();
        let second = command_buffer.blit_encoder().expect("second blit");
        second.wait_for_fence(&fence).expect("wait for fence");
        let readback = second.read_buffer(&destination, 0, 4).expect("readback");
        second.end_encoding();
        let completed = command_buffer
            .commit()
            .wait()
            .expect("submission completes");
        let bytes = completed.resolve_buffer(readback).expect("resolve");
        println!("Fence-ordered copy result: {bytes:?}");
        assert_eq!(bytes, vec![9, 9, 9, 9]);
    }
    println!("The second pass ran only after the fence was updated.");
}

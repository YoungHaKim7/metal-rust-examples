//! Example 090 — Encoder borrows and end_encoding.
//!
//! Encoders borrow `&mut CommandBuffer` for their whole lifetime, so only one
//! encoder is active at a time and Rust's borrow checker enforces the rule
//! Metal demands. `end_encoding(self)` consumes the encoder explicitly;
//! dropping it has the same effect.
//!
//! Run with:
//! ```text
//! cargo run -p ex090_encoder_lifetimes
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    metal::ComputePassDescriptor,
    {Device, ResourceOptions},
};

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void single(device uint* out [[buffer(0)]],
                   uint id [[thread_position_in_grid]]) {
    if (id == 0) {
        out[0] += 1u;
    }
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("single").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let buffer = device
        .new_buffer(4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    // Scoped encoder: the borrow ends when the block ends, and the
    // command buffer becomes usable again for the next encoder.
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&buffer, 0, 0).expect("bind");
        encoder.dispatch_1d(1, 1).expect("dispatch");
        encoder.end_encoding(); // consumes the encoder, releasing the borrow
    }
    let second = command_buffer
        .compute_encoder(&ComputePassDescriptor::new())
        .expect("second encoder");
    second.set_pipeline(&pipeline);
    second.set_buffer(&buffer, 0, 0).expect("bind");
    second.dispatch_1d(1, 1).expect("dispatch");
    drop(second); // implicit end_encoding via Drop
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&buffer, 0, 4).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let bytes = completed.resolve_buffer(readback).expect("resolve");
    let total = u32::from_ne_bytes(bytes.try_into().expect("4 bytes"));
    println!("Two encoders ran (one explicit end, one drop): counter = {total}");
    assert_eq!(total, 2);
}

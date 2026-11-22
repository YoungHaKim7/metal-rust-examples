//! Example 067 — Sequential command buffers.
//!
//! Two command buffers tell the story of GPU ordering. The first writes with
//! a compute kernel; only after its `wait()` completes does the second buffer
//! record its staged readback, guaranteeing the data is ready.
//!
//! Run with:
//! ```text
//! cargo run -p ex067_submission_ordering
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void stamp(device uint* out [[buffer(0)]],
                  uint id [[thread_position_in_grid]]) {
    out[id] = id + 100u;
}
"#;

fn decode_u32(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_ne_bytes(chunk.try_into().expect("4-byte chunk")))
        .collect()
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("stamp").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let buffer = device
        .new_buffer(16 * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    {
        let mut writer = queue.command_buffer().expect("writer buffer");
        {
            let encoder = writer
                .compute_encoder(&ComputePassDescriptor::new())
                .expect("encoder");
            encoder.set_pipeline(&pipeline);
            encoder.set_buffer(&buffer, 0, 0).expect("bind");
            encoder.dispatch_1d(16, 16).expect("dispatch");
            encoder.end_encoding();
        }
        writer.commit().wait().expect("writer completes");
    }
    println!("First submission finished; the data is now safe to read.");
    let mut reader = queue.command_buffer().expect("reader buffer");
    let readback = {
        let blit = reader.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&buffer, 0, 16 * 4).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = reader.commit().wait().expect("reader completes");
    let values = decode_u32(&completed.resolve_buffer(readback).expect("resolve"));
    println!("Read back after ordering: {:?}", &values[..4]);
    assert_eq!(values[0], 100);
    assert_eq!(values[15], 115);
}

//! Example 062 — Push/pop debug group boundaries.
//!
//! Debug groups nest around encoder work so captures read like function
//! calls. `insert_debug_capture_boundary()` marks a point the debugger can
//! break on. Groups must stay balanced: push and pop in pairs.
//!
//! Run with:
//! ```text
//! cargo run -p ex062_debug_groups
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::ComputePassDescriptor;
use metal_rust::{Device, ResourceOptions};

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void noop_fill(device uint* out [[buffer(0)]],
                      uint id [[thread_position_in_grid]]) {
    out[id] = 0;
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
    let function = library.function("noop_fill").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let buffer = device
        .new_buffer(64, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    queue.insert_debug_capture_boundary();
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    command_buffer.push_debug_group("examples/phase-one");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&buffer, 0, 0).expect("bind");
        encoder.dispatch_1d(64, 32).expect("dispatch");
        encoder.end_encoding();
    }
    command_buffer.pop_debug_group();
    command_buffer.push_debug_group("examples/phase-two");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&buffer, 0, 0).expect("bind");
        encoder.dispatch_1d(64, 32).expect("dispatch");
        encoder.end_encoding();
    }
    command_buffer.pop_debug_group();
    command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    println!("Two named debug groups were recorded around the compute passes.");
}

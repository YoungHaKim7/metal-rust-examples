//! Example 033 — dispatch_threadgroups with grid positions.
//!
//! `dispatch_threadgroups(groups, threads_per_group)` thinks in threadgroups:
//! you launch a grid of groups and the kernel learns where it sits through
//! `threadgroup_position_in_grid`. This example paints one value per group so
//! the group layout becomes visible.
//!
//! Run with:
//! ```text
//! cargo run -p ex033_threadgroup_grid
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::Size;
use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const GROUPS: usize = 8;
const THREADS_PER_GROUP: usize = 32;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void paint_groups(device uint* out [[buffer(0)]],
                         uint group [[threadgroup_position_in_grid]],
                         uint lid [[thread_position_in_threadgroup]]) {
    if (lid == 0) {
        out[group] = group * 10;
    }
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
    let function = library.function("paint_groups").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let output = device
        .new_buffer(GROUPS * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&output, 0, 0).expect("bind");
        encoder
            .dispatch_threadgroups(Size::new(GROUPS, 1, 1), Size::new(THREADS_PER_GROUP, 1, 1))
            .expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&output, 0, GROUPS * 4).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let values = decode_u32(&completed.resolve_buffer(readback).expect("resolve"));
    println!("One painted value per threadgroup: {values:?}");
}

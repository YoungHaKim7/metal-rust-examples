//! Example 038 — Shared memory reduction with barriers.
//!
//! Each threadgroup sums 32 values in threadgroup memory (shared on-chip
//! storage), synchronizes with `threadgroup_barrier`, and writes one partial
//! sum. 8 groups reduce 256 inputs to 8 partials the CPU adds up.
//!
//! Run with:
//! ```text
//! cargo run -p ex038_threadgroup_reduction
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const GROUPS: usize = 8;
const THREADS_PER_GROUP: usize = 32;
const COUNT: usize = GROUPS * THREADS_PER_GROUP;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void reduce_groups(device const uint* input [[buffer(0)]],
                          device uint* partials [[buffer(1)]],
                          uint gid [[thread_position_in_grid]],
                          uint lid [[thread_position_in_threadgroup]]) {
    threadgroup uint scratch[32];
    scratch[lid] = input[gid];
    threadgroup_barrier(mem_flags::mem_threadgroup);
    for (uint stride = 16u; stride > 0u; stride >>= 1u) {
        if (lid < stride) {
            scratch[lid] += scratch[lid + stride];
        }
        threadgroup_barrier(mem_flags::mem_threadgroup);
    }
    if (lid == 0) {
        partials[gid / 32u] = scratch[0];
    }
}
"#;

fn encode(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

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
    let function = library.function("reduce_groups").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let inputs: Vec<u32> = (0..COUNT).map(|index| (index % 1000) as u32).collect();
    let input_buffer = device
        .new_buffer_with_bytes(&encode(&inputs), ResourceOptions::SHARED)
        .expect("buffer");
    let partial_buffer = device
        .new_buffer(GROUPS * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&input_buffer, 0, 0).expect("bind input");
        encoder
            .set_buffer(&partial_buffer, 0, 1)
            .expect("bind partials");
        encoder
            .dispatch_threadgroups(
                metal_rust::Size::new(GROUPS, 1, 1),
                metal_rust::Size::new(THREADS_PER_GROUP, 1, 1),
            )
            .expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_buffer(&partial_buffer, 0, GROUPS * 4)
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let partials = decode_u32(&completed.resolve_buffer(readback).expect("resolve"));
    println!("GPU partial sums: {partials:?}");
    let gpu_total: u32 = partials.iter().sum();
    let cpu_total: u32 = inputs.iter().sum();
    println!("GPU total {gpu_total} vs CPU total {cpu_total}");
    assert_eq!(gpu_total, cpu_total);
}

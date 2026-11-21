//! Example 095 — Parallel reduction to a scalar.
//!
//! Threadgroups compute 32-term partial products, write one u32 partial sum
//! per group, and the CPU finishes the job. Integer math makes the GPU and
//! CPU answers exactly comparable.
//!
//! Run with:
//! ```text
//! cargo run -p ex095_dot_product_reduction
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const GROUPS: usize = 8;
const THREADS_PER_GROUP: usize = 32;
const COUNT: usize = GROUPS * THREADS_PER_GROUP;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void dot_partial(device const uint* a [[buffer(0)]],
                        device const uint* b [[buffer(1)]],
                        device uint* partials [[buffer(2)]],
                        uint gid [[thread_position_in_grid]],
                        uint lid [[thread_position_in_threadgroup]]) {
    threadgroup uint scratch[32];
    scratch[lid] = a[gid] * b[gid];
    threadgroup_barrier(mem_flags::mem_threadgroup);
    for (uint stride = 16u; stride > 0u; stride >>= 1u) {
        if (lid < stride) {
            scratch[lid] += scratch[lid + stride];
        }
        threadgroup_barrier(mem_flags::mem_threadgroup);
    }
    if (lid == 0u) {
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
    let function = library.function("dot_partial").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let a: Vec<u32> = (0..COUNT).map(|index| (index % 64) as u32).collect();
    let b: Vec<u32> = (0..COUNT).map(|index| ((index * 3) % 32) as u32).collect();
    let buffer_a = device
        .new_buffer_with_bytes(&encode(&a), ResourceOptions::SHARED)
        .expect("buffer");
    let buffer_b = device
        .new_buffer_with_bytes(&encode(&b), ResourceOptions::SHARED)
        .expect("buffer");
    let buffer_p = device
        .new_buffer(GROUPS * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&buffer_a, 0, 0).expect("bind a");
        encoder.set_buffer(&buffer_b, 0, 1).expect("bind b");
        encoder.set_buffer(&buffer_p, 0, 2).expect("bind partials");
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
            .read_buffer(&buffer_p, 0, GROUPS * 4)
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let partials = decode_u32(&completed.resolve_buffer(readback).expect("resolve"));
    let gpu_total: u32 = partials.iter().sum();
    let cpu_total: u32 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
    println!("GPU partial sums: {partials:?}");
    println!("GPU total {gpu_total}; CPU total {cpu_total}");
    assert_eq!(gpu_total, cpu_total);
    println!("The reduction is exact because u32 addition is associative.");
}

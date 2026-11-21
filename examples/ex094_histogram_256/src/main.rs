//! Example 094 — Atomics-driven byte histogram.
//!
//! One atomic increment per input byte builds a 256-bin histogram on the GPU.
//! Metal zero-initializes buffers, so the bins start clean, and the readback
//! totals must sum to the input length.
//!
//! Run with:
//! ```text
//! cargo run -p ex094_histogram_256
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const COUNT: usize = 4096;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void histogram(device const uchar* input [[buffer(0)]],
                      device atomic_uint* bins [[buffer(1)]],
                      uint id [[thread_position_in_grid]]) {
    uint bucket = uint(input[id]);
    atomic_fetch_add_explicit(&bins[bucket], 1u, memory_order_relaxed);
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
    let function = library.function("histogram").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let input: Vec<u8> = (0..COUNT)
        .map(|index| ((index * 37 + 11) % 256) as u8)
        .collect();
    let input_buffer = device
        .new_buffer_with_bytes(&input, ResourceOptions::SHARED)
        .expect("buffer");
    let bins = device
        .new_buffer(256 * 4, ResourceOptions::SHARED)
        .expect("bins");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&input_buffer, 0, 0).expect("bind input");
        encoder.set_buffer(&bins, 0, 1).expect("bind bins");
        encoder.dispatch_1d(COUNT, 64).expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&bins, 0, 256 * 4).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let histogram = decode_u32(&completed.resolve_buffer(readback).expect("resolve"));
    let total: u32 = histogram.iter().sum();
    println!("Histogram built with device atomics over {COUNT} bytes.");
    println!("Bin sums back to {total} ({COUNT} expected).");
    assert_eq!(total as usize, COUNT);
    let busiest = histogram
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| **count)
        .map(|(index, _)| index)
        .expect("256 bins");
    println!("Busiest bucket: {busiest} with {} hits", histogram[busiest]);
}

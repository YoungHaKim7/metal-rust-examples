//! Example 031 — Classic GPU vector addition.
//!
//! The "hello world" of GPU computing: add two arrays element-wise. Rust
//! encodes the inputs, the kernel adds them, and a staged blit readback
//! returns the bytes only after the submission completed. GPU results are
//! verified against a CPU loop.
//!
//! Run with:
//! ```text
//! cargo run -p ex031_vector_add
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const COUNT: usize = 64;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void add_arrays(device const float* input_a [[buffer(0)]],
                       device const float* input_b [[buffer(1)]],
                       device float* output [[buffer(2)]],
                       uint id [[thread_position_in_grid]]) {
    output[id] = input_a[id] + input_b[id];
}
"#;

/// Encodes f32 values into bytes for buffer transport.
fn encode(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

/// Decodes bytes back into f32 values.
fn decode(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_ne_bytes(chunk.try_into().expect("4-byte chunk")))
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
    let function = library.function("add_arrays").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let left: Vec<f32> = (0..COUNT).map(|index| index as f32).collect();
    let right: Vec<f32> = (0..COUNT).map(|index| (index * index) as f32).collect();
    let input_a = device
        .new_buffer_with_bytes(&encode(&left), ResourceOptions::SHARED)
        .expect("buffer");
    let input_b = device
        .new_buffer_with_bytes(&encode(&right), ResourceOptions::SHARED)
        .expect("buffer");
    let output = device
        .new_buffer(COUNT * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&input_a, 0, 0).expect("bind input_a");
        encoder.set_buffer(&input_b, 0, 1).expect("bind input_b");
        encoder.set_buffer(&output, 0, 2).expect("bind output");
        encoder.dispatch_1d(COUNT, 32).expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit encoder");
        let readback = blit
            .read_buffer(&output, 0, COUNT * 4)
            .expect("staged readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let results = decode(&completed.resolve_buffer(readback).expect("resolve"));
    for index in 0..4 {
        println!("{} + {} = {}", left[index], right[index], results[index]);
    }
    let expected: Vec<f32> = left.iter().zip(&right).map(|(a, b)| a + b).collect();
    assert_eq!(results, expected);
    println!("All {} sums match the CPU reference.", COUNT);
}

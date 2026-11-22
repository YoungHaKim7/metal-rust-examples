//! Example 035 — sin, sqrt and pow on the GPU.
//!
//! MSL ships the full math library. This kernel evaluates sin, sqrt and pow
//! for a table of inputs and the CPU compares the results. With fast math
//! disabled the two answers agree to floating-point precision.
//!
//! Run with:
//! ```text
//! cargo run -p ex035_gpu_math_functions
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const COUNT: usize = 64;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void math_table(device const float* input [[buffer(0)]],
                       device float4* output [[buffer(1)]],
                       uint id [[thread_position_in_grid]]) {
    float x = input[id];
    output[id] = float4(sin(x), sqrt(fabs(x)), pow(fabs(x), 2.0f), 0.0f);
}
"#;

fn encode(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

fn decode_f32(bytes: &[u8]) -> Vec<f32> {
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
    let function = library.function("math_table").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let inputs: Vec<f32> = (0..COUNT).map(|index| index as f32 * 0.1).collect();
    let input_buffer = device
        .new_buffer_with_bytes(&encode(&inputs), ResourceOptions::SHARED)
        .expect("buffer");
    let output_buffer = device
        .new_buffer(COUNT * 16, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&input_buffer, 0, 0).expect("bind");
        encoder.set_buffer(&output_buffer, 0, 1).expect("bind");
        encoder.dispatch_1d(COUNT, 32).expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_buffer(&output_buffer, 0, COUNT * 16)
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let flat = decode_f32(&completed.resolve_buffer(readback).expect("resolve"));
    let mut worst = 0.0f32;
    for index in 0..COUNT {
        let x = inputs[index];
        let expected = [x.sin(), x.abs().sqrt(), x.abs().powi(2)];
        for lane in 0..3 {
            worst = worst.max((flat[index * 4 + lane] - expected[lane]).abs());
        }
    }
    println!("Evaluated sin/sqrt/pow for {COUNT} inputs.");
    println!("Largest deviation from the CPU reference: {worst:.3e}");
}

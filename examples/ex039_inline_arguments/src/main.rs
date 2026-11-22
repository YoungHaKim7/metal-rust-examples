//! Example 039 — set_bytes for small kernel arguments.
//!
//! Buffers are overkill for a handful of scalars. `set_bytes(bytes, index)`
//! embeds the value directly into the command stream — no allocation, no
//! binding lifetime, just bytes. The kernel reads it as a `constant` binding.
//!
//! Run with:
//! ```text
//! cargo run -p ex039_inline_arguments
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const COUNT: usize = 64;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Params {
    float scale;
    float bias;
};

kernel void affine(device const float* input [[buffer(0)]],
                   device float* output [[buffer(1)]],
                   constant Params& params [[buffer(2)]],
                   uint id [[thread_position_in_grid]]) {
    output[id] = input[id] * params.scale + params.bias;
}
"#;

#[repr(C)]
struct Params {
    scale: f32,
    bias: f32,
}

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
    let function = library.function("affine").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let inputs: Vec<f32> = (0..COUNT).map(|index| index as f32).collect();
    let input_buffer = device
        .new_buffer_with_bytes(&encode(&inputs), ResourceOptions::SHARED)
        .expect("buffer");
    let output_buffer = device
        .new_buffer(COUNT * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let params = Params {
        scale: 3.0,
        bias: 1.5,
    };
    let param_bytes: Vec<u8> = params
        .scale
        .to_ne_bytes()
        .into_iter()
        .chain(params.bias.to_ne_bytes())
        .collect();
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&input_buffer, 0, 0).expect("bind input");
        encoder
            .set_buffer(&output_buffer, 0, 1)
            .expect("bind output");
        encoder
            .set_bytes(&param_bytes, 2)
            .expect("inline parameters");
        encoder.dispatch_1d(COUNT, 32).expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_buffer(&output_buffer, 0, COUNT * 4)
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let outputs = decode_f32(&completed.resolve_buffer(readback).expect("resolve"));
    println!("input 0.0 -> {}", outputs[0]);
    println!("input 5.0 -> {}", outputs[5]);
    assert!((outputs[5] - (5.0 * params.scale + params.bias)).abs() < 1e-5);
}

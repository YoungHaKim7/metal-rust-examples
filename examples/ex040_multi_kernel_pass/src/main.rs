//! Example 040 — Two kernels with a memory barrier.
//!
//! Two compute encoders can live in one command buffer. Work inside one
//! encoder is ordered; the explicit `memory_barrier(BarrierScope::BarrierScopeBuffers)`
//! makes the hand-off between the passes unambiguous for the driver.
//! Pass one squares values; pass two flips their sign.
//!
//! Run with:
//! ```text
//! cargo run -p ex040_multi_kernel_pass
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::{BarrierScope, ComputePassDescriptor};
use metal_rust::{Device, ResourceOptions};

const COUNT: usize = 64;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void square_it(device const float* input [[buffer(0)]],
                      device float* output [[buffer(1)]],
                      uint id [[thread_position_in_grid]]) {
    output[id] = input[id] * input[id];
}

kernel void negate_it(device float* data [[buffer(0)]],
                      uint id [[thread_position_in_grid]]) {
    data[id] = -data[id];
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
    let square = library.function("square_it").expect("square exists");
    let negate = library.function("negate_it").expect("negate exists");
    let square_pipeline = device.new_compute_pipeline(&square).expect("pipeline");
    let negate_pipeline = device.new_compute_pipeline(&negate).expect("pipeline");
    let inputs: Vec<f32> = (0..COUNT).map(|index| index as f32).collect();
    let scratch = device
        .new_buffer(COUNT * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let input_buffer = device
        .new_buffer_with_bytes(&encode(&inputs), ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&square_pipeline);
        encoder.set_buffer(&input_buffer, 0, 0).expect("bind");
        encoder.set_buffer(&scratch, 0, 1).expect("bind");
        encoder.dispatch_1d(COUNT, 32).expect("dispatch");
        encoder.end_encoding();
        let second = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        second.set_pipeline(&negate_pipeline);
        second.set_buffer(&scratch, 0, 0).expect("bind");
        second
            .memory_barrier(BarrierScope::BarrierScopeBuffers)
            .expect("barrier");
        second.dispatch_1d(COUNT, 32).expect("dispatch");
        second.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&scratch, 0, COUNT * 4).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let outputs = decode_f32(&completed.resolve_buffer(readback).expect("resolve"));
    println!("input 3.0 -> squared -> negated = {}", outputs[3]);
    assert!((outputs[3] + 9.0).abs() < 1e-5);
    println!("Both passes ran in one command buffer, separated by a barrier.");
}

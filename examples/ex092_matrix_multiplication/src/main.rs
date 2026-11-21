//! Example 092 — 8x8 f32 matmul with CPU verification.
//!
//! Each thread computes one output cell from a row and a column. The 2D
//! dispatch matches the matrix shape, and the CPU reference check proves the
//! GPU result. `f32` sums run in the same order on both sides.
//!
//! Run with:
//! ```text
//! cargo run -p ex092_matrix_multiplication
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const N: usize = 8;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void matmul(device const float* a [[buffer(0)]],
                   device const float* b [[buffer(1)]],
                   device float* c [[buffer(2)]],
                   uint2 pos [[thread_position_2d]]) {
    float sum = 0.0;
    for (uint k = 0u; k < 8u; k += 1u) {
        sum += a[pos.y * 8u + k] * b[k * 8u + pos.x];
    }
    c[pos.y * 8u + pos.x] = sum;
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
    let function = library.function("matmul").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let a: Vec<f32> = (0..N * N)
        .map(|index| ((index * 7) % 13) as f32 * 0.5)
        .collect();
    let b: Vec<f32> = (0..N * N)
        .map(|index| ((index * 5) % 11) as f32 * 0.25)
        .collect();
    let buffer_a = device
        .new_buffer_with_bytes(&encode(&a), ResourceOptions::SHARED)
        .expect("buffer");
    let buffer_b = device
        .new_buffer_with_bytes(&encode(&b), ResourceOptions::SHARED)
        .expect("buffer");
    let buffer_c = device
        .new_buffer(N * N * 4, ResourceOptions::SHARED)
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
        encoder.set_buffer(&buffer_c, 0, 2).expect("bind c");
        encoder
            .dispatch_threads(
                metal_rust::Size::new(N, N, 1),
                metal_rust::Size::new(N, N, 1),
            )
            .expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&buffer_c, 0, N * N * 4).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let c_gpu = decode_f32(&completed.resolve_buffer(readback).expect("resolve"));
    let mut worst = 0.0f32;
    for row in 0..N {
        for column in 0..N {
            let mut expected = 0.0f32;
            for k in 0..N {
                expected += a[row * N + k] * b[k * N + column];
            }
            worst = worst.max((c_gpu[row * N + column] - expected).abs());
        }
    }
    println!("GPU multiplied two 8x8 matrices; worst deviation from CPU: {worst:.3e}");
    assert!(worst < 1e-4);
}

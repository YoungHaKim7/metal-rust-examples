//! Example 086 — Fixed-layout Rust structs for MSL.
//!
//! Shader interfaces need fixed memory layout. `#[repr(C)]` gives Rust
//! structs C ordering, `static_assertions`-style size checks catch drift at
//! compile time, and the kernel then trusts the bytes. This example round-trips
//! a small Vertex struct through a buffer.
//!
//! Run with:
//! ```text
//! cargo run -p ex086_repr_c_layout
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Vertex {
    float2 position;
    float weight;
};

kernel void normalize_weight(device Vertex* vertices [[buffer(0)]],
                             uint id [[thread_position_in_grid]]) {
    vertices[id].weight = vertices[id].weight * 0.5;
}
"#;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    weight: f32,
}

/// The MSL struct above is 12 bytes: 8 for the float2, 4 for the float.
const EXPECTED_VERTEX_SIZE: usize = 12;

fn main() {
    assert_eq!(std::mem::size_of::<Vertex>(), EXPECTED_VERTEX_SIZE);
    println!("Compile-time layout check passed: Vertex is {EXPECTED_VERTEX_SIZE} bytes.");
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library
        .function("normalize_weight")
        .expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let vertices = [
        Vertex {
            position: [0.0, 0.0],
            weight: 1.0,
        },
        Vertex {
            position: [1.0, 0.0],
            weight: 2.0,
        },
    ];
    let bytes: Vec<u8> = vertices
        .iter()
        .flat_map(|vertex| {
            vertex
                .position
                .into_iter()
                .chain([vertex.weight])
                .flat_map(|value| value.to_ne_bytes())
        })
        .collect();
    let buffer = device
        .new_buffer_with_bytes(&bytes, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&buffer, 0, 0).expect("bind");
        encoder.dispatch_1d(2, 1).expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_buffer(&buffer, 0, 2 * EXPECTED_VERTEX_SIZE)
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let data = completed.resolve_buffer(readback).expect("resolve");
    let weights: Vec<f32> = data
        .chunks_exact(4)
        .map(|chunk| f32::from_ne_bytes(chunk.try_into().expect("4-byte chunk")))
        .collect();
    println!("Weights after the kernel: {} {}", weights[2], weights[5]);
}

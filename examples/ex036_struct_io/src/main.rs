//! Example 036 — Repr(C) structs between Rust and MSL.
//!
//! Structs cross the boundary byte by byte. A `#[repr(C)]` Rust struct with
//! the same field order as its MSL twin maps 1:1. Here a Particle array is
//! integrated one step on the GPU: position += velocity * dt.
//!
//! Run with:
//! ```text
//! cargo run -p ex036_struct_io
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const COUNT: usize = 32;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Particle {
    float2 position;
    float2 velocity;
};

kernel void integrate(device Particle* particles [[buffer(0)]],
                      constant float& dt [[buffer(1)]],
                      uint id [[thread_position_in_grid]]) {
    particles[id].position += particles[id].velocity * dt;
}
"#;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Particle {
    position: [f32; 2],
    velocity: [f32; 2],
}

/// Encodes the particle array plus the dt uniform into bytes.
fn encode_particles(particles: &[Particle]) -> Vec<u8> {
    particles
        .iter()
        .flat_map(|particle| {
            particle
                .position
                .into_iter()
                .chain(particle.velocity)
                .flat_map(|value| value.to_ne_bytes())
        })
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
    let function = library.function("integrate").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let particles: Vec<Particle> = (0..COUNT)
        .map(|index| Particle {
            position: [index as f32, 0.0],
            velocity: [1.0, index as f32 * 0.1],
        })
        .collect();
    let dt = 2.0f32;
    let mut bytes = encode_particles(&particles);
    bytes.extend_from_slice(&dt.to_ne_bytes());
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
        encoder.set_buffer(&buffer, 0, 0).expect("bind particles");
        encoder
            .set_buffer(&buffer, COUNT * 16, 1)
            .expect("bind dt tail");
        encoder.dispatch_1d(COUNT, 32).expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&buffer, 0, COUNT * 16).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let updated = completed.resolve_buffer(readback).expect("resolve");
    let chunks: Vec<[u8; 4]> = updated
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .collect();
    let first_x = f32::from_ne_bytes(chunks[0]);
    println!("Particle 0 moved from x=0.0 to x={first_x} (expected 2.0).");
    let tenth_y = f32::from_ne_bytes(chunks[10 * 4 + 1]);
    println!("Particle 10 moved from y=0.0 to y={tenth_y} (expected 0.2).");
}

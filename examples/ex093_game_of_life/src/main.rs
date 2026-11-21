//! Example 093 — Conway's Life steps on the GPU.
//!
//! Two buffers ping-pong: each submission reads the current world and writes
//! the next generation. Cells wrap at the edges, so the glider cycles
//! forever. ASCII output shows three generations.
//!
//! Run with:
//! ```text
//! cargo run -p ex093_game_of_life
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const SIZE: usize = 16;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void life_step(device const uchar* current [[buffer(0)]],
                      device uchar* next [[buffer(1)]],
                      uint2 pos [[thread_position_2d]]) {
    uint width = 16u;
    int neighbors = 0;
    for (int dy = -1; dy <= 1; dy += 1) {
        for (int dx = -1; dx <= 1; dx += 1) {
            if (dx == 0 && dy == 0) {
                continue;
            }
            uint x = (pos.x + uint(dx) + width) % width;
            uint y = (pos.y + uint(dy) + width) % width;
            neighbors += current[y * width + x] != 0 ? 1 : 0;
        }
    }
    uchar alive = current[pos.y * width + pos.x];
    uchar result = 0;
    if (alive != 0 && (neighbors == 2 || neighbors == 3)) {
        result = 1;
    }
    if (alive == 0 && neighbors == 3) {
        result = 1;
    }
    next[pos.y * width + pos.x] = result;
}
"#;

fn glider() -> Vec<u8> {
    let mut world = vec![0u8; SIZE * SIZE];
    world[1 * SIZE + 2] = 1;
    world[2 * SIZE + 3] = 1;
    world[3 * SIZE + 1] = 1;
    world[3 * SIZE + 2] = 1;
    world[3 * SIZE + 3] = 1;
    world
}

fn print_world(label: &str, world: &[u8]) {
    println!("{label}");
    for row in 0..SIZE {
        let mut line = String::new();
        for column in 0..SIZE {
            line.push(if world[row * SIZE + column] != 0 {
                '#'
            } else {
                '.'
            });
        }
        println!("{line}");
    }
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("life_step").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let current = device
        .new_buffer_with_bytes(&glider(), ResourceOptions::SHARED)
        .expect("current");
    let next = device
        .new_buffer(SIZE * SIZE, ResourceOptions::SHARED)
        .expect("next");
    let queue = device.new_command_queue(None).expect("queue");
    print_world("Generation 0 (glider):", &glider());
    for generation in 1..=3 {
        let mut command_buffer = queue.command_buffer().expect("command buffer");
        {
            let encoder = command_buffer
                .compute_encoder(&ComputePassDescriptor::new())
                .expect("encoder");
            encoder.set_pipeline(&pipeline);
            encoder.set_buffer(&current, 0, 0).expect("bind current");
            encoder.set_buffer(&next, 0, 1).expect("bind next");
            encoder
                .dispatch_threads(
                    metal_rust::Size::new(SIZE, SIZE, 1),
                    metal_rust::Size::new(16, 16, 1),
                )
                .expect("dispatch");
            encoder.end_encoding();
        }
        let readback = {
            let blit = command_buffer.blit_encoder().expect("blit");
            let readback = blit.read_buffer(&next, 0, SIZE * SIZE).expect("readback");
            blit.end_encoding();
            readback
        };
        let completed = command_buffer
            .commit()
            .wait()
            .expect("submission completes");
        let world = completed.resolve_buffer(readback).expect("resolve");
        print_world(&format!("Generation {generation}:"), &world);
        current.write(0, &world).expect("swap generations");
    }
    println!("Each generation was simulated entirely on the GPU.");
}

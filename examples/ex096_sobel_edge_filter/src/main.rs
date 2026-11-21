//! Example 096 — Sobel edge detection on a pattern.
//!
//! The kernel applies the classic 3x3 Sobel operator: gradients in x and y
//! combine into an edge magnitude per pixel. A generated circle-with-noise
//! input keeps the example deterministic, and ASCII shows the edges.
//!
//! Run with:
//! ```text
//! cargo run -p ex096_sobel_edge_filter
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const SIZE: usize = 16;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void sobel(device const uchar* input [[buffer(0)]],
                  device uchar* edges [[buffer(1)]],
                  uint2 pos [[thread_position_2d]]) {
    uint width = 16u;
    uint height = 16u;
    if (pos.x == 0u || pos.y == 0u || pos.x == width - 1u || pos.y == height - 1u) {
        edges[pos.y * width + pos.x] = 0u;
        return;
    }
    int gx[3][3] = {{-1, 0, 1}, {-2, 0, 2}, {-1, 0, 1}};
    int gy[3][3] = {{-1, -2, -1}, {0, 0, 0}, {1, 2, 1}};
    int sum_x = 0;
    int sum_y = 0;
    for (int dy = -1; dy <= 1; dy += 1) {
        for (int dx = -1; dx <= 1; dx += 1) {
            uint x = pos.x + uint(dx);
            uint y = pos.y + uint(dy);
            int value = int(input[y * width + x]);
            sum_x += gx[dy + 1][dx + 1] * value;
            sum_y += gy[dy + 1][dx + 1] * value;
        }
    }
    int magnitude = abs(sum_x) + abs(sum_y);
    edges[pos.y * width + pos.x] = convert_uchar_sat(magnitude);
}
"#;

fn pattern() -> Vec<u8> {
    let mut pixels = vec![0u8; SIZE * SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as i32 - 8;
            let dy = y as i32 - 8;
            if dx * dx + dy * dy < 36 {
                pixels[y * SIZE + x] = 220;
            }
        }
    }
    pixels
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("sobel").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let input = device
        .new_buffer_with_bytes(&pattern(), ResourceOptions::SHARED)
        .expect("input");
    let edges = device
        .new_buffer(SIZE * SIZE, ResourceOptions::SHARED)
        .expect("edges");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&input, 0, 0).expect("bind input");
        encoder.set_buffer(&edges, 0, 1).expect("bind edges");
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
        let readback = blit.read_buffer(&edges, 0, SIZE * SIZE).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let magnitudes = completed.resolve_buffer(readback).expect("resolve");
    println!("Sobel edge magnitudes (ASCII; brighter = stronger edge):");
    for row in 0..SIZE {
        let mut line = String::new();
        for column in 0..SIZE {
            let ramp = " .:-=+*#%@";
            let value = magnitudes[row * SIZE + column] as usize;
            let step = usize::min(value * ramp.len() / 512, ramp.len() - 1);
            line.push(ramp.as_bytes()[step] as char);
        }
        println!("{line}");
    }
    println!("The circle's rim lights up while flat regions stay dark.");
}

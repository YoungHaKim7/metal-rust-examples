//! Example 091 — Compute-rendered fractal saved to PPM.
//!
//! A kernel iterates the Mandelbrot escape test per pixel and writes RGBA
//! bytes into a buffer. The CPU reads the bytes back through staged readback
//! and writes a binary PPM image to the temp directory.
//!
//! Run with:
//! ```text
//! cargo run -p ex091_mandelbrot_fractal
//! ```

#![forbid(unsafe_code)]

use std::{env, fs};

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const WIDTH: usize = 128;
const HEIGHT: usize = 96;
const MAX_ITERATIONS: u32 = 96;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

constant float2 kCenter = float2(-0.6, 0.0);
constant float kScale = 3.0;

kernel void mandelbrot(device uchar* pixels [[buffer(0)]],
                       uint2 pos [[thread_position_2d]]) {
    float width = 128.0;
    float height = 96.0;
    float2 uv = (float2(pos) / float2(width, height)) - 0.5;
    float2 c = kCenter + uv * kScale;
    float2 z = float2(0.0);
    uint iterations = 0u;
    while (iterations < 96u && dot(z, z) < 4.0) {
        z = float2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        iterations += 1u;
    }
    float t = float(iterations) / 96.0;
    uchar red = convert_uchar_sat(t * 255.0);
    uchar green = convert_uchar_sat(sqrt(t) * 255.0);
    uchar blue = convert_uchar_sat((1.0 - t) * 255.0);
    uint index = (pos.y * 128u + pos.x) * 4u;
    pixels[index + 0u] = red;
    pixels[index + 1u] = green;
    pixels[index + 2u] = blue;
    pixels[index + 3u] = 255u;
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("mandelbrot").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let pixels = device
        .new_buffer(WIDTH * HEIGHT * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&pixels, 0, 0).expect("bind");
        encoder
            .dispatch_threads(
                metal_rust::Size::new(WIDTH, HEIGHT, 1),
                metal_rust::Size::new(64, 16, 1),
            )
            .expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_buffer(&pixels, 0, WIDTH * HEIGHT * 4)
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let rgba = completed.resolve_buffer(readback).expect("resolve");
    let mut ppm = format!("P6\n{WIDTH} {HEIGHT}\n255\n").into_bytes();
    for pixel in rgba.chunks_exact(4) {
        ppm.extend_from_slice(&pixel[..3]);
    }
    let path = env::temp_dir().join("metal-rust-mandelbrot.ppm");
    fs::write(&path, &ppm).expect("write PPM");
    println!(
        "Rendered a {WIDTH}x{HEIGHT} Mandelbrot on the GPU (max {MAX_ITERATIONS} iterations per pixel)."
    );
    println!("Saved {}", path.display());
}

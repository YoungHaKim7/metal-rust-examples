//! Example 034 — 2D dispatch writing a gradient.
//!
//! `dispatch_threads(threads, threads_per_group)` takes a `Size`, so 2D work
//! is natural: the kernel reads `thread_position_2d` and writes an RGBA byte
//! per pixel. We print the gradient as ASCII luminance.
//!
//! Run with:
//! ```text
//! cargo run -p ex034_dispatch_threads_2d
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::Size;
use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const WIDTH: usize = 16;
const HEIGHT: usize = 8;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void gradient(device uchar* pixels [[buffer(0)]],
                     uint2 pos [[thread_position_2d]]) {
    uint index = (pos.y * 16u + pos.x) * 4u;
    pixels[index + 0] = convert_uchar_sat(pos.x * 16u);
    pixels[index + 1] = convert_uchar_sat(pos.y * 32u);
    pixels[index + 2] = 64u;
    pixels[index + 3] = 255u;
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
    let function = library.function("gradient").expect("function exists");
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
            .dispatch_threads(Size::new(WIDTH, HEIGHT, 1), Size::new(16, 8, 1))
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
    let bytes = completed.resolve_buffer(readback).expect("resolve");
    println!("2D gradient rendered by the GPU (one ASCII ramp per row):");
    for row in 0..HEIGHT {
        let mut line = String::new();
        for column in 0..WIDTH {
            let red = bytes[(row * WIDTH + column) * 4] as u32;
            let ramp = " .:-=+*#%@";
            let step = usize::min((red as usize) * ramp.len() / 256, ramp.len() - 1);
            line.push(ramp.as_bytes()[step] as char);
        }
        println!("{line}");
    }
}

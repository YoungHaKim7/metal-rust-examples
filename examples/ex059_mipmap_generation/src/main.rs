//! Example 059 — generate_mipmaps on the GPU.
//!
//! Mipmapped textures start with only level 0 filled; the blit encoder builds
//! every smaller level with `generate_mipmaps()`. The descriptor factory
//! `texture_2d(format, w, h, mipmapped)` allocates the chain in one call.
//!
//! Run with:
//! ```text
//! cargo run -p ex059_mipmap_generation
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, PixelFormat, Region, TextureDescriptor, TextureUsage};

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void fill_level0(device uchar* pixels [[buffer(0)]],
                        uint id [[thread_position_in_grid]]) {
    pixels[id * 4u + 0] = 255u;
    pixels[id * 4u + 1] = 128u;
    pixels[id * 4u + 2] = 0u;
    pixels[id * 4u + 3] = 255u;
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let descriptor = TextureDescriptor::texture_2d(PixelFormat::RGBA8_UNORM, WIDTH, HEIGHT, true)
        .expect("mipmap descriptor");
    descriptor.set_usage(TextureUsage::SHADER_READ);
    let expected_levels = descriptor.mipmap_level_count();
    println!("Descriptor allocates {expected_levels} mip levels for a {WIDTH}x{HEIGHT} texture.");
    let texture = device.new_texture(&descriptor).expect("texture");
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("fill_level0").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let staging = device
        .new_buffer(WIDTH * HEIGHT * 4, metal_rust::ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&metal_rust::ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&staging, 0, 0).expect("bind");
        encoder.dispatch_1d(WIDTH * HEIGHT, 64).expect("dispatch");
        encoder.end_encoding();
    }
    {
        let blit = command_buffer.blit_encoder().expect("blit");
        blit.copy_buffer_to_texture(
            &staging,
            0,
            WIDTH * 4,
            WIDTH * 4,
            metal_rust::Size::new(WIDTH, HEIGHT, 1),
            &texture,
            0,
            0,
            metal_rust::Origin::new(0, 0, 0),
        )
        .expect("buffer to texture");
        blit.generate_mipmaps(&texture).expect("mipmap generation");
        blit.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_texture(&texture, Region::new_2d(0, 0, WIDTH, HEIGHT))
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let image = completed.resolve_texture(readback).expect("resolve");
    println!("Level 0 pixel: {:?}", &image.bytes[..4]);
    println!("Mip chain built on the GPU; smaller levels smooth the colors.");
}

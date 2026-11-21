//! Example 097 — Full render pipeline exported to PPM.
//!
//! One image file proves the whole graphics pipeline: compile, pipeline,
//! render pass into a texture, staged readback with row padding handled, then
//! a binary PPM written from Rust.
//!
//! Run with:
//! ```text
//! cargo run -p ex097_render_to_ppm
//! ```

#![forbid(unsafe_code)]

use std::{env, fs};

use metal_rust::{
    ClearColor, Device, PixelFormat, Region, RenderPassDescriptor, ResourceOptions,
    TextureDescriptor, TextureUsage, Viewport,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float2 uv;
};

vertex VertexOut fullscreen(uint id [[vertex_id]]) {
    constexpr float2 corners[3] = {
        float2(-1.0, -1.0), float2(3.0, -1.0), float2(-1.0, 3.0)
    };
    return VertexOut{float4(corners[id], 0.0, 1.0), corners[id] * 0.5 + 0.5};
}

fragment float4 shade(VertexOut in [[stage_in]]) {
    float3 sky = float3(0.15, 0.35, 0.85);
    float3 ground = float3(0.9, 0.6, 0.2);
    float blend = smoothstep(0.2, 0.8, in.uv.y);
    return float4(mix(ground, sky, blend), 1.0);
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
    let vertex = library.function("fullscreen").expect("vertex exists");
    let fragment = library.function("shade").expect("fragment exists");
    let descriptor = metal_rust::RenderPipelineDescriptor::new(
        &vertex,
        Some(&fragment),
        PixelFormat::RGBA8_UNORM,
    );
    let pipeline = device.new_render_pipeline(&descriptor).expect("pipeline");
    let texture = device
        .new_texture(
            &TextureDescriptor::new_2d(
                PixelFormat::RGBA8_UNORM,
                WIDTH,
                HEIGHT,
                ResourceOptions::SHARED,
                TextureUsage::RENDER_TARGET,
            )
            .expect("descriptor"),
        )
        .expect("texture");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let pass = RenderPassDescriptor::new();
        pass.set_color_attachment(0, &texture, ClearColor::new(0.0, 0.0, 0.0, 1.0))
            .expect("attach");
        let encoder = command_buffer.render_encoder(&pass).expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder
            .set_viewport(Viewport::new(
                0.0,
                0.0,
                WIDTH as f64,
                HEIGHT as f64,
                0.0,
                1.0,
            ))
            .expect("viewport");
        encoder
            .draw_primitives(metal_rust::PrimitiveType::Triangle, 0, 3)
            .expect("draw");
        encoder.end_encoding();
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
    let mut ppm = format!("P6\n{WIDTH} {HEIGHT}\n255\n").into_bytes();
    for row in 0..HEIGHT {
        let row_start = row * image.bytes_per_row;
        for column in 0..WIDTH {
            let index = row_start + column * 4;
            ppm.extend_from_slice(&image.bytes[index..index + 3]);
        }
    }
    let path = env::temp_dir().join("metal-rust-render.ppm");
    fs::write(&path, &ppm).expect("write PPM");
    println!(
        "Rendered {WIDTH}x{HEIGHT} pixels; bytes/row reported as {}.",
        image.bytes_per_row
    );
    println!("Wrote {}", path.display());
}

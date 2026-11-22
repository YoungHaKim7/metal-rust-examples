//! Example 044 — Vertex uniforms via set_vertex_bytes.
//!
//! Uniforms are tiny per-draw constants. Binding them with
//! `set_vertex_bytes()` avoids allocating buffers; here a single offset vec2
//! slides the same triangle to a different screen position between passes.
//!
//! Run with:
//! ```text
//! cargo run -p ex044_uniform_constants
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    ClearColor, Device, PixelFormat, Region, RenderPassDescriptor, ResourceOptions,
    TextureDescriptor, TextureUsage, Viewport,
};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Uniforms {
    float2 offset;
};

vertex float4 offset_vertex(uint id [[vertex_id]],
                            constant Uniforms& uniforms [[buffer(1)]]) {
    constexpr float2 corners[3] = {
        float2(-1.0, -1.0), float2(3.0, -1.0), float2(-1.0, 3.0)
    };
    return float4(corners[id] + uniforms.offset, 0.0, 1.0);
}

fragment float4 flat_fragment() {
    return float4(1.0, 1.0, 0.0, 1.0);
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
    let vertex = library.function("offset_vertex").expect("vertex exists");
    let fragment = library.function("flat_fragment").expect("fragment exists");
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
    for (index, offset) in [[0.0f32, 0.0f32], [0.5, 0.5]].into_iter().enumerate() {
        let pass = RenderPassDescriptor::new();
        let load = if index == 0 {
            ClearColor::new(0.0, 0.0, 0.0, 1.0)
        } else {
            ClearColor::new(0.0, 0.0, 1.0, 1.0)
        };
        pass.set_color_attachment(0, &texture, load)
            .expect("attach");
        let encoder = command_buffer.render_encoder(&pass).expect("encoder");
        encoder.set_pipeline(&pipeline);
        let uniform_bytes: Vec<u8> = offset
            .into_iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        encoder
            .set_vertex_bytes(&uniform_bytes, 1)
            .expect("uniform");
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
    println!("Final pixel at (0,0): {:?}", &image.bytes[..4]);
    println!("The same geometry was drawn twice, shifted by the uniform.");
}

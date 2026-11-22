//! Example 046 — Interpolated gradients from position.
//!
//! The fragment function can compute colors procedurally from the
//! interpolated position attribute. Each pixel blends from black to yellow
//! along the diagonal; the readback prints the ramp as ASCII.
//!
//! Run with:
//! ```text
//! cargo run -p ex046_gradient_triangle
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    ClearColor, Device, PixelFormat, Region, RenderPassDescriptor, ResourceOptions,
    TextureDescriptor, TextureUsage, Viewport,
};

const WIDTH: usize = 16;
const HEIGHT: usize = 8;

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

fragment float4 gradient(VertexOut in [[stage_in]]) {
    return float4(in.uv.x, in.uv.y, 0.0, 1.0);
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
    let fragment = library.function("gradient").expect("fragment exists");
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
    println!("Red channel gradient (one ASCII ramp per row):");
    for row in 0..HEIGHT {
        let mut line = String::new();
        for column in 0..WIDTH {
            let index = row * image.bytes_per_row + column * 4;
            let ramp = " .:-=+*#%@";
            let step = usize::min(
                (image.bytes[index] as usize) * ramp.len() / 256,
                ramp.len() - 1,
            );
            line.push(ramp.as_bytes()[step] as char);
        }
        println!("{line}");
    }
}

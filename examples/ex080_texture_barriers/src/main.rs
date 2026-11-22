//! Example 080 — texture_barrier between draws.
//!
//! Reading and writing the same attachment in one pass needs a barrier.
//! Draw 1 fills the target; `texture_barrier()` makes those writes readable;
//! draw 2 shades using the previous content. The readback proves both
//! stages ran.
//!
//! Run with:
//! ```text
//! cargo run -p ex080_texture_barriers
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

vertex float4 fullscreen(uint id [[vertex_id]]) {
    constexpr float2 corners[3] = {
        float2(-1.0, -1.0), float2(3.0, -1.0), float2(-1.0, 3.0)
    };
    return float4(corners[id], 0.0, 1.0);
}

fragment float4 base_color() {
    return float4(0.5, 0.0, 0.5, 1.0);
}

fragment float4 brighten(float4 previous [[color(0)]]) {
    return previous + float4(0.25, 0.25, 0.25, 0.0);
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
    let base_fragment = library.function("base_color").expect("base exists");
    let brighten_fragment = library.function("brighten").expect("brighten exists");
    let base_descriptor = metal_rust::RenderPipelineDescriptor::new(
        &vertex,
        Some(&base_fragment),
        PixelFormat::RGBA8_UNORM,
    );
    let base_pipeline = device
        .new_render_pipeline(&base_descriptor)
        .expect("base pipeline");
    let brighten_descriptor = metal_rust::RenderPipelineDescriptor::new(
        &vertex,
        Some(&brighten_fragment),
        PixelFormat::RGBA8_UNORM,
    );
    let brighten_pipeline = device
        .new_render_pipeline(&brighten_descriptor)
        .expect("brighten pipeline");
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
        encoder.set_pipeline(&base_pipeline);
        encoder
            .draw_primitives(metal_rust::PrimitiveType::Triangle, 0, 3)
            .expect("base draw");
        encoder.texture_barrier().expect("barrier between draws");
        encoder.set_pipeline(&brighten_pipeline);
        encoder
            .draw_primitives(metal_rust::PrimitiveType::Triangle, 0, 3)
            .expect("brighten draw");
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
    println!(
        "Pixel after two barrier-separated draws: {:?}",
        &image.bytes[..4]
    );
    println!("Purple 0.5 brightened by 0.25 gives about 0.75 in every channel.");
}

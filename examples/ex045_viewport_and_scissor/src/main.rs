//! Example 045 — Viewport and scissor rectangles.
//!
//! The viewport maps NDC coordinates onto pixels; the scissor rect discards
//! fragments outside a box. Two passes with different settings show both: the
//! full-frame triangle lands everywhere, the scissored one only in the left
//! half.
//!
//! Run with:
//! ```text
//! cargo run -p ex045_viewport_and_scissor
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::ScissorRect;
use metal_rust::{
    ClearColor, Device, PixelFormat, Region, RenderPassDescriptor, ResourceOptions,
    TextureDescriptor, TextureUsage, Viewport,
};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

vertex float4 big_triangle(uint id [[vertex_id]]) {
    constexpr float2 corners[3] = {
        float2(-1.0, -1.0), float2(3.0, -1.0), float2(-1.0, 3.0)
    };
    return float4(corners[id], 0.0, 1.0);
}

fragment float4 green() {
    return float4(0.0, 1.0, 0.0, 1.0);
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
    let vertex = library.function("big_triangle").expect("vertex exists");
    let fragment = library.function("green").expect("fragment exists");
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
        let scissored = command_buffer.render_encoder(&pass).expect("encoder");
        scissored.set_pipeline(&pipeline);
        scissored
            .set_viewport(Viewport::new(
                0.0,
                0.0,
                WIDTH as f64,
                HEIGHT as f64,
                0.0,
                1.0,
            ))
            .expect("viewport");
        scissored
            .set_scissor_rect(ScissorRect {
                x: 0,
                y: 0,
                width: WIDTH / 2,
                height: HEIGHT,
            })
            .expect("scissor");
        scissored
            .draw_primitives(metal_rust::PrimitiveType::Triangle, 0, 3)
            .expect("draw");
        scissored.end_encoding();
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
    let left = (3 * image.bytes_per_row + 1 * 4) as usize;
    let right = (3 * image.bytes_per_row + 7 * 4) as usize;
    println!("Left half pixel : {:?}", &image.bytes[left..left + 4]);
    println!("Right half pixel: {:?}", &image.bytes[right..right + 4]);
    println!("The scissor rect confined the second draw to the left half.");
}

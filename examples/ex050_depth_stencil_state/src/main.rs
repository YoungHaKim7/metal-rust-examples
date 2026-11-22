//! Example 050 — Depth-stencil states without a window.
//!
//! DepthStencilState objects are built from descriptors and bound per pass
//! with `set_depth_stencil_state()`. This example configures an always-pass
//! compare function, binds it while drawing overlapping triangles, and prints
//! the resulting pixels.
//!
//! Run with:
//! ```text
//! cargo run -p ex050_depth_stencil_state
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::{CompareFunction, DepthStencilDescriptor};
use metal_rust::{
    ClearColor, Device, PixelFormat, Region, RenderPassDescriptor, ResourceOptions,
    TextureDescriptor, TextureUsage, Viewport,
};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

vertex float4 layered(uint id [[vertex_id]],
                      constant float& depth [[buffer(1)]]) {
    constexpr float2 corners[3] = {
        float2(-1.0, -1.0), float2(3.0, -1.0), float2(-1.0, 3.0)
    };
    return float4(corners[id], depth, 1.0);
}

fragment float4 shade(float4 in [[position]], constant float& depth [[buffer(1)]]) {
    if (depth > 0.5) {
        return float4(1.0, 0.0, 0.0, 1.0);
    }
    return float4(0.0, 0.0, 1.0, 1.0);
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let descriptor_state = DepthStencilDescriptor::new().expect("depth stencil descriptor");
    descriptor_state
        .set_depth_compare_function(CompareFunction::CompareFunctionAlways)
        .expect("compare function");
    let depth_stencil = device
        .new_depth_stencil_state(&descriptor_state)
        .expect("depth stencil state");
    println!("Created a DepthStencilState with CompareFunctionAlways.");
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let vertex = library.function("layered").expect("vertex exists");
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
            .set_depth_stencil_state(Some(&depth_stencil))
            .expect("depth stencil");
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
        let depth_far = [0.8f32];
        let depth_near = [0.2f32];
        let depth_far_bytes: Vec<u8> = depth_far
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let depth_near_bytes: Vec<u8> = depth_near
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        encoder
            .set_vertex_bytes(&depth_far_bytes, 1)
            .expect("depth uniform far");
        encoder
            .draw_primitives(metal_rust::PrimitiveType::Triangle, 0, 3)
            .expect("far draw");
        encoder
            .set_vertex_bytes(&depth_near_bytes, 1)
            .expect("depth uniform near");
        encoder
            .draw_primitives(metal_rust::PrimitiveType::Triangle, 0, 3)
            .expect("near draw");
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
    let center = ((HEIGHT / 2) * image.bytes_per_row + (WIDTH / 2) * 4) as usize;
    println!(
        "Center pixel after both layers: {:?}",
        &image.bytes[center..center + 4]
    );
}

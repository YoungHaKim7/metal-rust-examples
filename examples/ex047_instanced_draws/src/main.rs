//! Example 047 — draw_primitives_instanced with base_instance.
//!
//! Instancing redraws the same geometry N times; `instance_id` distinguishes
//! the copies. Four quads land in the four quadrants, each tinted by its
//! instance number.
//!
//! Run with:
//! ```text
//! cargo run -p ex047_instanced_draws
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

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut instanced_quad(uint vertex_id [[vertex_id]],
                                uint instance [[instance_id]]) {
    float2 corner[6] = {
        float2(-0.45, -0.45), float2(0.45, -0.45), float2(0.45, 0.45),
        float2(-0.45, -0.45), float2(0.45, 0.45), float2(-0.45, 0.45)
    };
    float2 center = float2(instance % 2u, instance / 2u) * 0.9 - 0.45;
    VertexOut out;
    out.position = float4(corner[vertex_id] * 0.4 + center, 0.0, 1.0);
    float3 tint[4] = {float3(1, 0, 0), float3(0, 1, 0), float3(0, 0, 1), float3(1, 1, 0)};
    out.color = float4(tint[instance], 1.0);
    return out;
}

fragment float4 quad_fragment(VertexOut in [[stage_in]]) {
    return in.color;
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
    let vertex = library.function("instanced_quad").expect("vertex exists");
    let fragment = library.function("quad_fragment").expect("fragment exists");
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
            .draw_primitives_instanced(metal_rust::PrimitiveType::Triangle, 0, 6, 4, 0)
            .expect("instanced draw");
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
    for (index, (x, y)) in [(1, 1), (6, 1), (1, 6), (6, 6)].into_iter().enumerate() {
        let offset = (y * image.bytes_per_row + x * 4) as usize;
        println!(
            "Quadrant {index} pixel: {:?}",
            &image.bytes[offset..offset + 4]
        );
    }
    println!("Four instances, four colors, one draw call.");
}

//! Example 043 — Per-vertex color interpolation.
//!
//! Vertex data uploaded with `set_vertex_bytes()` flows straight into the
//! vertex function. The rasterizer interpolates the color attribute across the
//! triangle, so the center pixel is the blend of all three corners.
//!
//! Run with:
//! ```text
//! cargo run -p ex043_vertex_colors
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

struct VertexIn {
    float2 position;
    float4 color;
};

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut colored_vertex(const device VertexIn* vertices [[buffer(0)]],
                                uint id [[vertex_id]]) {
    VertexOut out;
    out.position = float4(vertices[id].position, 0.0, 1.0);
    out.color = vertices[id].color;
    return out;
}

fragment float4 colored_fragment(VertexOut in [[stage_in]]) {
    return in.color;
}
"#;

#[repr(C)]
struct VertexIn {
    position: [f32; 2],
    color: [f32; 4],
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let vertex = library.function("colored_vertex").expect("vertex exists");
    let fragment = library
        .function("colored_fragment")
        .expect("fragment exists");
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
    let vertices = [
        VertexIn {
            position: [-1.0, -1.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        VertexIn {
            position: [3.0, -1.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        VertexIn {
            position: [-1.0, 3.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
    ];
    let bytes: Vec<u8> = vertices
        .iter()
        .flat_map(|vertex| {
            vertex
                .position
                .into_iter()
                .chain(vertex.color)
                .flat_map(|value| value.to_ne_bytes())
        })
        .collect();
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let pass = RenderPassDescriptor::new();
        pass.set_color_attachment(0, &texture, ClearColor::new(0.0, 0.0, 0.0, 1.0))
            .expect("attach");
        let encoder = command_buffer.render_encoder(&pass).expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_vertex_bytes(&bytes, 0).expect("vertex bytes");
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
    let center = ((HEIGHT / 2) * image.bytes_per_row + (WIDTH / 2) * 4) as usize;
    let pixel = &image.bytes[center..center + 4];
    println!("Center pixel (blend of red, green, blue): {pixel:?}");
}

//! Example 048 — draw_indexed_primitives with UInt16 indices.
//!
//! A quad only needs four vertices, but two triangles: an index buffer tells
//! Metal how to replay them. Indices travel in a Buffer with
//! `IndexType::IndexTypeUInt16`.
//!
//! Run with:
//! ```text
//! cargo run -p ex048_indexed_draw
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::IndexType;
use metal_rust::{
    ClearColor, Device, PixelFormat, Region, RenderPassDescriptor, ResourceOptions,
    TextureDescriptor, TextureUsage, Viewport,
};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

vertex float4 quad_vertex(const device float2* positions [[buffer(0)]],
                          uint id [[vertex_id]]) {
    return float4(positions[id], 0.0, 1.0);
}

fragment float4 cyan() {
    return float4(0.0, 1.0, 1.0, 1.0);
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
    let vertex = library.function("quad_vertex").expect("vertex exists");
    let fragment = library.function("cyan").expect("fragment exists");
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
    let positions: Vec<u8> = [[-1.0f32, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]]
        .iter()
        .flat_map(|pair| pair.into_iter().flat_map(|value| value.to_ne_bytes()))
        .collect();
    let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];
    let index_bytes: Vec<u8> = indices
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect();
    let vertex_buffer = device
        .new_buffer_with_bytes(&positions, ResourceOptions::SHARED)
        .expect("vertices");
    let index_buffer = device
        .new_buffer_with_bytes(&index_bytes, ResourceOptions::SHARED)
        .expect("indices");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let pass = RenderPassDescriptor::new();
        pass.set_color_attachment(0, &texture, ClearColor::new(0.0, 0.0, 0.0, 1.0))
            .expect("attach");
        let encoder = command_buffer.render_encoder(&pass).expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder
            .set_vertex_buffer(&vertex_buffer, 0, 0)
            .expect("vertex buffer");
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
            .draw_indexed_primitives(
                metal_rust::PrimitiveType::Triangle,
                6,
                IndexType::IndexTypeUInt16,
                &index_buffer,
                0,
                1,
                0,
                0,
            )
            .expect("indexed draw");
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
    println!("Center pixel: {:?}", &image.bytes[center..center + 4]);
    assert_eq!(&image.bytes[center..center + 4], &[0, 255, 255, 255]);
    println!("Two indexed triangles filled the quad.");
}

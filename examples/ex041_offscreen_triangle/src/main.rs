//! Example 041 — Render into a texture and read pixels back.
//!
//! Rendering needs no window: a texture with RENDER_TARGET usage becomes the
//! color attachment. After the pass, a staged `read_texture()` hands the
//! pixels to the CPU once the submission completed. BGRA8_UNORM stores blue
//! first, so pure red appears as `[0, 0, 255, 255]`.
//!
//! Run with:
//! ```text
//! cargo run -p ex041_offscreen_triangle
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
};

vertex VertexOut triangle_vertex(uint id [[vertex_id]]) {
    constexpr float2 corners[3] = {
        float2(-1.0, -1.0), float2(3.0, -1.0), float2(-1.0, 3.0)
    };
    return VertexOut{float4(corners[id], 0.0, 1.0)};
}

fragment float4 triangle_fragment() {
    return float4(1.0, 0.0, 0.0, 1.0);
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
    let vertex = library.function("triangle_vertex").expect("vertex exists");
    let fragment = library
        .function("triangle_fragment")
        .expect("fragment exists");
    let descriptor = metal_rust::RenderPipelineDescriptor::new(
        &vertex,
        Some(&fragment),
        PixelFormat::BGRA8_UNORM,
    );
    let pipeline = device
        .new_render_pipeline(&descriptor)
        .expect("render pipeline");
    let texture_descriptor = TextureDescriptor::new_2d(
        PixelFormat::BGRA8_UNORM,
        WIDTH,
        HEIGHT,
        ResourceOptions::SHARED,
        TextureUsage::RENDER_TARGET,
    )
    .expect("render target descriptor");
    let texture = device
        .new_texture(&texture_descriptor)
        .expect("render target");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let pass = RenderPassDescriptor::new();
        pass.set_color_attachment(0, &texture, ClearColor::new(0.0, 0.0, 0.0, 1.0))
            .expect("attachment 0");
        let encoder = command_buffer
            .render_encoder(&pass)
            .expect("render encoder");
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
            .expect("texture readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let image = completed
        .resolve_texture(readback)
        .expect("resolve texture");
    println!(
        "Read back {}x{} pixels (bytes/row {})",
        image.width, image.height, image.bytes_per_row
    );
    println!("Top-left pixel: {:?}", &image.bytes[..4]);
    assert_eq!(&image.bytes[..4], &[0, 0, 255, 255]);
    println!("The triangle covered the whole render target with red.");
}

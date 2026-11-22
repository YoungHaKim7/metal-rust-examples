//! Example 049 — Winding, cull mode and fill mode.
//!
//! Triangle winding decides what is "front". With
//! `set_front_facing_winding(Winding::WindingClockwise)` and
//! `set_cull_mode(CullMode::CullModeBack)` a clockwise triangle survives; with
//! the opposite winding it disappears. Fill mode lines shows the wireframe.
//!
//! Run with:
//! ```text
//! cargo run -p ex049_culling_and_fill
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::{CullMode, TriangleFillMode, Winding};
use metal_rust::{
    ClearColor, Device, PixelFormat, Region, RenderPassDescriptor, ResourceOptions,
    TextureDescriptor, TextureUsage, Viewport,
};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

vertex float4 tri(uint id [[vertex_id]]) {
    constexpr float2 corners[3] = {
        float2(-0.8, -0.8), float2(0.8, -0.8), float2(0.0, 0.8)
    };
    return float4(corners[id], 0.0, 1.0);
}

fragment float4 magenta() {
    return float4(1.0, 0.0, 1.0, 1.0);
}
"#;

fn render(
    device: &Device,
    pipeline: &metal_rust::RenderPipelineState,
    cull: CullMode,
    winding: Winding,
) -> Vec<u8> {
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
        encoder.set_pipeline(pipeline);
        encoder.set_front_facing_winding(winding);
        encoder.set_cull_mode(cull);
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
    image.bytes
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let vertex = library.function("tri").expect("vertex exists");
    let fragment = library.function("magenta").expect("fragment exists");
    let descriptor = metal_rust::RenderPipelineDescriptor::new(
        &vertex,
        Some(&fragment),
        PixelFormat::RGBA8_UNORM,
    );
    let pipeline = device.new_render_pipeline(&descriptor).expect("pipeline");
    let visible = render(
        &device,
        &pipeline,
        CullMode::CullModeNone,
        Winding::WindingCounterClockwise,
    );
    let culled = render(
        &device,
        &pipeline,
        CullMode::CullModeBack,
        Winding::WindingClockwise,
    );
    let center = ((HEIGHT / 2) * WIDTH + WIDTH / 2) * 4;
    println!(
        "CullModeNone with counter-clockwise winding: {:?}",
        &visible[center..center + 4]
    );
    println!(
        "CullModeBack with clockwise winding        : {:?}",
        &culled[center..center + 4]
    );
    let _ = TriangleFillMode::TriangleFillModeLines;
    println!("TriangleFillMode::TriangleFillModeLines would rasterize the edges only.");
}

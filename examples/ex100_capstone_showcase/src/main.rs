//! Example 100 — End-to-end compute + render + readback.
//!
//! The capstone walks the entire metal-rust workflow in one program: query
//! the device, compile MSL, compute vertex colors on the GPU, feed them into
//! a render pipeline through a vertex buffer binding, read the texture back,
//! and summarize every stage.
//!
//! Run with:
//! ```text
//! cargo run -p ex100_capstone_showcase
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    ClearColor, ComputePassDescriptor, Device, PixelFormat, Region, RenderPassDescriptor,
    ResourceOptions, TextureDescriptor, TextureUsage, Viewport,
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

kernel void palette(device float4* colors [[buffer(0)]],
                    uint id [[thread_position_in_grid]]) {
    float t = float(id) / 4.0;
    colors[id] = float4(t, 1.0 - t, 0.5 * (1.0 - t) + 0.5 * t, 1.0);
}

vertex VertexOut showcase_vertex(const device float4* palette [[buffer(0)]],
                                 constant float4& pivot [[buffer(1)]],
                                 uint vertex_id [[vertex_id]]) {
    constexpr float2 corners[3] = {
        float2(-0.8, -0.8), float2(0.8, -0.8), float2(0.0, 0.8)
    };
    VertexOut out;
    out.position = float4(corners[vertex_id] + pivot.xy, 0.0, 1.0);
    out.color = palette[vertex_id];
    return out;
}

fragment float4 showcase_fragment(VertexOut in [[stage_in]]) {
    return in.color;
}
"#;

fn main() {
    println!("== metal-rust capstone: compute -> render -> readback ==");
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    println!("[1] Device        : {}", device.name());
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compile MSL");
    println!(
        "[2] Compiled      : {} functions",
        library.function_names().len()
    );
    let palette_fn = library.function("palette").expect("palette kernel");
    let vertex_fn = library
        .function("showcase_vertex")
        .expect("vertex function");
    let fragment_fn = library
        .function("showcase_fragment")
        .expect("fragment function");
    let compute_pipeline = device
        .new_compute_pipeline(&palette_fn)
        .expect("compute pipeline");
    let render_descriptor = metal_rust::RenderPipelineDescriptor::new(
        &vertex_fn,
        Some(&fragment_fn),
        PixelFormat::BGRA8_UNORM,
    );
    let render_pipeline = device
        .new_render_pipeline(&render_descriptor)
        .expect("render pipeline");
    println!("[3] Pipelines     : compute + render ready");
    let palette = device
        .new_buffer(4 * 16, ResourceOptions::SHARED)
        .expect("palette buffer");
    let texture = device
        .new_texture(
            &TextureDescriptor::new_2d(
                PixelFormat::BGRA8_UNORM,
                WIDTH,
                HEIGHT,
                ResourceOptions::SHARED,
                TextureUsage::RENDER_TARGET,
            )
            .expect("target descriptor"),
        )
        .expect("render target");
    println!("[4] Resources     : palette buffer + BGRA8 render target");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    command_buffer.push_debug_group("capstone/compute");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&compute_pipeline);
        encoder.set_buffer(&palette, 0, 0).expect("bind palette");
        encoder.dispatch_1d(4, 4).expect("palette dispatch");
        encoder.end_encoding();
    }
    command_buffer.pop_debug_group();
    command_buffer.push_debug_group("capstone/render");
    {
        let pass = RenderPassDescriptor::new();
        pass.set_color_attachment(0, &texture, ClearColor::new(0.0, 0.0, 0.0, 1.0))
            .expect("attach");
        let encoder = command_buffer.render_encoder(&pass).expect("encoder");
        encoder.set_pipeline(&render_pipeline);
        let pivot: Vec<u8> = [0.0f32, 0.0]
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        encoder.set_vertex_bytes(&pivot, 1).expect("pivot uniform");
        encoder
            .set_vertex_buffer(&palette, 0, 0)
            .expect("palette as vertices");
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
    command_buffer.pop_debug_group();
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
        "[5] Readback      : center pixel {:?}",
        &image.bytes[center..center + 4]
    );
    println!("[6] Done          : the full safe metal-rust workflow ran end to end.");
}

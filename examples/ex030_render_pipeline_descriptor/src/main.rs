//! Example 030 — Vertex/fragment pipeline descriptors.
//!
//! `RenderPipelineDescriptor::new(vertex, fragment, color_format)` is the
//! fast path: vertex function, optional fragment function and the color
//! attachment format in one call. The pipeline must match the pixel format of
//! the textures it draws into.
//!
//! Run with:
//! ```text
//! cargo run -p ex030_render_pipeline_descriptor
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, PixelFormat, RenderPipelineDescriptor};

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut triangle_vertex(uint id [[vertex_id]]) {
    float2 corners[3] = {float2(-0.6, -0.6), float2(0.6, -0.6), float2(0.0, 0.6)};
    return VertexOut{float4(corners[id], 0.0, 1.0), float4(0.2, 0.5, 1.0, 1.0)};
}

fragment float4 triangle_fragment(VertexOut in [[stage_in]]) {
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
    let vertex = library.function("triangle_vertex").expect("vertex exists");
    let fragment = library
        .function("triangle_fragment")
        .expect("fragment exists");
    let descriptor =
        RenderPipelineDescriptor::new(&vertex, Some(&fragment), PixelFormat::BGRA8_UNORM);
    descriptor.set_label(Some("examples/triangle"));
    println!(
        "Descriptor has vertex function: {}",
        descriptor.vertex_function().is_some()
    );
    match device.new_render_pipeline(&descriptor) {
        Ok(pipeline) => println!(
            "Render pipeline built for BGRA8_UNORM output (max vertex threads {})",
            pipeline.max_total_threads_per_threadgroup()
        ),
        Err(error) => println!("Render pipeline unavailable: {error}"),
    }
}

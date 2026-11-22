//! Example 030 (include test) — Vertex/fragment pipeline descriptors.
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

const SOURCE: &str = include_str!("../assets/render_pipeline.metal");

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

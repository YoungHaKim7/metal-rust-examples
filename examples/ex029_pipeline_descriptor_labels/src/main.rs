//! Example 029 — ComputePipelineDescriptor labels.
//!
//! Descriptor objects exist before Metal does: their setters validate values
//! and fail fast with `InvalidArgument` errors. Labels help Xcode's GPU
//! debugger show readable pipeline names.
//!
//! Run with:
//! ```text
//! cargo run -p ex029_pipeline_descriptor_labels
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::ComputePipelineDescriptor;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void labelled(device uint* out [[buffer(0)]],
                     uint id [[thread_position_in_grid]]) {
    out[id] = 42;
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    if let Ok(descriptor) = ComputePipelineDescriptor::new() {
        descriptor
            .set_label("examples/labelled-pipeline")
            .expect("label is valid");
        let stored = descriptor.label().expect("label readable");
        println!("Descriptor label: {}", stored.as_deref().unwrap_or(""));
        match device.new_compute_pipeline_from_descriptor(
            &descriptor,
            metal_rust::metal::PipelineOption::PipelineOptionNone,
        ) {
            Ok((pipeline, _)) => println!(
                "Built pipeline from descriptor (max threads {})",
                pipeline.max_total_threads_per_threadgroup()
            ),
            Err(error) => println!("Descriptor pipeline rejected (no function set): {error}"),
        }
    } else {
        println!("ComputePipelineDescriptor creation is unavailable on this runtime.");
    }
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("labelled").expect("function exists");
    let pipeline = device
        .new_compute_pipeline(&function)
        .expect("direct pipeline");
    println!(
        "Direct pipeline for comparison: max threads {}",
        pipeline.max_total_threads_per_threadgroup()
    );
}

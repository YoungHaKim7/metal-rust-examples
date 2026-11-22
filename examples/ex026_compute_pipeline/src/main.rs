//! Example 026 — Compute pipelines and their limits.
//!
//! A `ComputePipelineState` is the executable form of a kernel. Its limits —
//! execution width, maximum threads per threadgroup and static threadgroup
//! memory — tell you how to size dispatches before you encode them.
//!
//! Run with:
//! ```text
//! cargo run -p ex026_compute_pipeline
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void scale(device const float* input [[buffer(0)]],
                  device float* output [[buffer(1)]],
                  uint id [[thread_position_in_grid]]) {
    output[id] = input[id] * 2.0;
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
    let function = library.function("scale").expect("scale exists");
    let pipeline = match device.new_compute_pipeline(&function) {
        Ok(pipeline) => pipeline,
        Err(error) => {
            println!("Pipeline creation unavailable: {error}");
            return;
        }
    };
    println!("Execution width           : {}", pipeline.execution_width());
    println!(
        "Max threads per threadgroup: {}",
        pipeline.max_total_threads_per_threadgroup()
    );
    println!(
        "Static threadgroup memory : {} bytes",
        pipeline.static_threadgroup_memory_length()
    );
}

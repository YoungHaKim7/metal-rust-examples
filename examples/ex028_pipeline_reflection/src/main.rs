//! Example 028 — Reflection from compute pipelines.
//!
//! Reflection describes the arguments a kernel uses. Ask for it with
//! `new_compute_pipeline_with_reflection(function, options)`; the safe API
//! returns the pipeline plus `Option<ComputePipelineReflection>`. The
//! `PipelineOption` flags select how much information is produced.
//!
//! Run with:
//! ```text
//! cargo run -p ex028_pipeline_reflection
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::PipelineOption;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void reflect(device const float4* vectors [[buffer(0)]],
                    device float* sums [[buffer(1)]],
                    uint id [[thread_position_in_grid]]) {
    sums[id] = dot(vectors[id], float4(1.0, 1.0, 1.0, 1.0));
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
    let function = library.function("reflect").expect("reflect exists");
    match device.new_compute_pipeline_with_reflection(&function, PipelineOption::PipelineOptionNone)
    {
        Ok((pipeline, reflection)) => {
            println!(
                "Pipeline compiled (max threads {})",
                pipeline.max_total_threads_per_threadgroup()
            );
            match reflection {
                Some(_) => println!("Reflection data was produced for this pipeline."),
                None => println!("Driver produced no reflection for these options."),
            }
        }
        Err(error) => println!("Reflection pipeline unavailable: {error}"),
    }
}

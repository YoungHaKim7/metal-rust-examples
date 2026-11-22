//! Example 027 — Asynchronous compute pipeline compilation.
//!
//! Pipelines can take milliseconds to prepare (Metal may run its optimizer).
//! `new_compute_pipeline_async()` builds the pipeline off-thread and delivers
//! it through your handler, so the main thread stays responsive.
//!
//! Run with:
//! ```text
//! cargo run -p ex027_async_pipeline
//! ```

#![forbid(unsafe_code)]

use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use metal_rust::Device;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void busy(device float* out [[buffer(0)]],
                 uint id [[thread_position_in_grid]]) {
    out[id] = sqrt(float(id));
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
    let function = library.function("busy").expect("busy exists");
    // ComputePipelineState is !Send: the handler summarizes it and sends a
    // String instead of moving the pipeline across threads.
    let (sender, receiver): (_, Receiver<String>) = channel();
    device
        .new_compute_pipeline_async(&function, move |result| {
            let message = match result {
                Ok(pipeline) => format!(
                    "pipeline ready, max threads per threadgroup {}",
                    pipeline.max_total_threads_per_threadgroup()
                ),
                Err(error) => format!("async pipeline failed: {error}"),
            };
            let _ = sender.send(message);
        })
        .expect("async pipeline started");
    println!("Pipeline compilation dispatched asynchronously.");
    match receiver.recv_timeout(Duration::from_secs(30)) {
        Ok(message) => println!("Async pipeline: {message}"),
        Err(timeout) => println!("Pipeline did not arrive in time: {timeout}"),
    }
}

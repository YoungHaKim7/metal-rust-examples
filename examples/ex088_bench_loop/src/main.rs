//! Example 088 — Measure repeated GPU submissions.
//!
//! Benchmark loops separate compile time from run time: build the pipeline
//! once, then submit N command buffers and time each `wait()`. The report
//! shows min/mean/max wall-clock times plus the GPU's own span.
//!
//! Run with:
//! ```text
//! cargo run -p ex088_bench_loop
//! ```

#![forbid(unsafe_code)]

use std::time::Instant;

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const RUNS: usize = 5;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void work(device float* out [[buffer(0)]],
                 uint id [[thread_position_in_grid]]) {
    float value = float(id);
    for (uint step = 0u; step < 32u; step += 1u) {
        value = fma(value, 1.0001, 0.5);
    }
    out[id] = value;
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
    let function = library.function("work").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let buffer = device
        .new_buffer(256 * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut wall_times = Vec::new();
    for run in 0..RUNS {
        let mut command_buffer = queue.command_buffer().expect("command buffer");
        command_buffer.set_label(Some(&format!("bench/run-{run}")));
        {
            let encoder = command_buffer
                .compute_encoder(&ComputePassDescriptor::new())
                .expect("encoder");
            encoder.set_pipeline(&pipeline);
            encoder.set_buffer(&buffer, 0, 0).expect("bind");
            encoder.dispatch_1d(256, 32).expect("dispatch");
            encoder.end_encoding();
        }
        let start = Instant::now();
        command_buffer
            .commit()
            .wait()
            .expect("submission completes");
        wall_times.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    let mean = wall_times.iter().sum::<f64>() / wall_times.len() as f64;
    let minimum = wall_times.iter().cloned().fold(f64::MAX, f64::min);
    let maximum = wall_times.iter().cloned().fold(0.0, f64::max);
    println!("Wall-clock per run (ms): min {minimum:.3}, mean {mean:.3}, max {maximum:.3}");
    println!("Pipeline compilation happened once; the loop only submits work.");
}

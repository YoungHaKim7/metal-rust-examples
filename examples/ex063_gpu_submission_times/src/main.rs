//! Example 063 — Bracketing submissions with paired timestamps.
//!
//! `device.sample_timestamps()` returns one CPU and one GPU tick taken
//! back-to-back. Take a pair before the submission and another after the
//! wait, and you have bracketed the GPU work in both clock domains. The
//! buffers themselves also expose gpu_start_time()/gpu_end_time() queries
//! before commit; after wait() the finished window belongs to the device
//! clocks.
//!
//! Run with:
//! ```text
//! cargo run -p ex063_gpu_submission_times
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void spin(device float* out [[buffer(0)]],
                 uint id [[thread_position_in_grid]]) {
    float value = float(id);
    for (uint step = 0u; step < 64u; step += 1u) {
        value = sin(value) + 1.0;
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
    let function = library.function("spin").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let buffer = device
        .new_buffer(1024 * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&buffer, 0, 0).expect("bind");
        encoder.dispatch_1d(1024, 64).expect("dispatch");
        encoder.end_encoding();
    }
    let (cpu_start, gpu_start) = device.sample_timestamps();
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let (cpu_end, gpu_end) = device.sample_timestamps();
    println!(
        "Submission completed: {:?}",
        completed.status() == metal_rust::CommandBufferStatus::Completed
    );
    println!("CPU clock: {} .. {} ticks", cpu_start, cpu_end);
    println!("GPU clock: {} .. {} ticks", gpu_start, gpu_end);
    println!(
        "Both domains bracket the same work; convert deltas with TimestampFrequency (example 074)."
    );
}

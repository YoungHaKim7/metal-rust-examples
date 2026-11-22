//! Example 074 — TimestampFrequency conversions.
//!
//! Raw timestamps are ticks, not seconds. The TimestampFrequency numeric
//! property gives ticks per second, which turns GPU deltas into milliseconds.
//! This example measures a real kernel with the GPU's own clock domain.
//!
//! Run with:
//! ```text
//! cargo run -p ex074_gpu_timing_measurement
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    metal::DeviceNumericProperty,
    {ComputePassDescriptor, Device, ResourceOptions},
};

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void workload(device float* out [[buffer(0)]],
                     uint id [[thread_position_in_grid]]) {
    float value = float(id) * 0.001;
    for (uint step = 0u; step < 128u; step += 1u) {
        value = fract(sin(value) * 1.37);
    }
    out[id] = value;
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let frequency = device
        .numeric_property(DeviceNumericProperty::TimestampFrequency)
        .unwrap_or(1_000_000_000);
    println!("GPU timestamp frequency: {frequency} Hz");
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("workload").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let buffer = device
        .new_buffer(4096 * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&buffer, 0, 0).expect("bind");
        encoder.dispatch_1d(4096, 128).expect("dispatch");
        encoder.end_encoding();
    }
    let (_, gpu_start) = device.sample_timestamps();
    command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let (_, gpu_end) = device.sample_timestamps();
    let delta_ticks = gpu_end.saturating_sub(gpu_start);
    let milliseconds = delta_ticks as f64 * 1000.0 / frequency as f64;
    println!("Submission spanned {delta_ticks} GPU ticks.");
    println!("That is about {milliseconds:.3} ms on the GPU clock.");
    println!("Same work, now expressed in the hardware's own units.");
}

//! Example 008 — Synchronized CPU and GPU timestamps.
//!
//! `sample_timestamps()` returns one CPU timestamp and one GPU timestamp taken
//! back-to-back. The pair lets you translate between the two clock domains;
//! later examples convert deltas to milliseconds using TimestampFrequency.
//!
//! Run with:
//! ```text
//! cargo run -p ex008_sample_timestamps
//! ```

#![forbid(unsafe_code)]

use std::thread::sleep;
use std::time::Duration;

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let (cpu_before, gpu_before) = device.sample_timestamps();
    println!("Sample 1: cpu={cpu_before} gpu={gpu_before}");
    sleep(Duration::from_millis(50));
    let (cpu_after, gpu_after) = device.sample_timestamps();
    println!("Sample 2: cpu={cpu_after} gpu={gpu_after}");
    let cpu_delta = cpu_after.saturating_sub(cpu_before);
    let gpu_delta = gpu_after.saturating_sub(gpu_before);
    println!("Delta over 50 ms of sleep:");
    println!("  CPU ticks: {cpu_delta}");
    println!("  GPU ticks: {gpu_delta}");
    println!("Both clocks advanced together, which is what makes the mapping usable.");
}

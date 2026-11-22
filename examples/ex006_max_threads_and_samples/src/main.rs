//! Example 006 — Threadgroup limits and sample counts.
//!
//! Before dispatching work you must know how many threads fit in one
//! threadgroup and which multisample counts the GPU rasterizes. Both answers
//! come straight from the device.
//!
//! Run with:
//! ```text
//! cargo run -p ex006_max_threads_and_samples
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let maximum = device.max_threads_per_threadgroup();
    println!("Max threads per threadgroup:");
    println!("  width : {}", maximum.width);
    println!("  height: {}", maximum.height);
    println!("  depth : {}", maximum.depth);
    println!("Multisample support:");
    for sample_count in [1usize, 2, 4, 8] {
        let supported = device.supports_texture_sample_count(sample_count);
        println!(
            "  {sample_count} sample(s): {}",
            if supported { "yes" } else { "no" }
        );
    }
}

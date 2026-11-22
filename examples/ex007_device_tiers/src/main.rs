//! Example 007 — Argument-buffer and read/write texture tiers.
//!
//! Metal ranks advanced features in tiers: argument buffers tier 2 enables
//! thousands of resources per pass, and read/write texture tier 2 allows a
//! single texture to be sampled and written by the same shader. This example
//! reports both tiers plus counter-sampling and rasterization-rate-map
//! support.
//!
//! Run with:
//! ```text
//! cargo run -p ex007_device_tiers
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::{ArgumentBuffersTier, CounterSamplingPoint, ReadWriteTextureTier};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let argument_tier = match device.argument_buffers_tier() {
        ArgumentBuffersTier::ArgumentBuffersTier1 => "tier 1",
        ArgumentBuffersTier::ArgumentBuffersTier2 => "tier 2",
        _ => "unknown",
    };
    let rw_tier = match device.read_write_texture_tier() {
        ReadWriteTextureTier::ReadWriteTextureTier1 => "tier 1",
        ReadWriteTextureTier::ReadWriteTextureTier2 => "tier 2",
        ReadWriteTextureTier::ReadWriteTextureTierNone => "none",
        _ => "unknown",
    };
    println!("Argument buffers     : {argument_tier}");
    println!("Read/write textures  : {rw_tier}");
    match device
        .supports_counter_sampling(CounterSamplingPoint::CounterSamplingPointAtDispatchBoundary)
    {
        Ok(true) => println!("Counter sampling     : supported at dispatch boundaries"),
        Ok(false) => println!("Counter sampling     : not supported"),
        Err(error) => println!("Counter sampling     : unavailable ({error})"),
    }
    match device.supports_rasterization_rate_map(2) {
        Ok(true) => println!("Rasterization maps   : supported for 2 layers"),
        Ok(false) => println!("Rasterization maps   : not supported for 2 layers"),
        Err(error) => println!("Rasterization maps   : unavailable ({error})"),
    }
}

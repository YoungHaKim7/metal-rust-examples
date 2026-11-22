//! Example 057 — Build sampler state objects.
//!
//! Descriptors are blueprints; `new_sampler_state()` bakes one into a
//! SamplerState the GPU can bind. States are immutable after creation, which
//! is why Metal can cache and reuse them.
//!
//! Run with:
//! ```text
//! cargo run -p ex057_sampler_states
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::{SamplerAddressMode, SamplerDescriptor, SamplerMinMagFilter};

fn build_descriptor() -> Result<SamplerDescriptor, metal_rust::Error> {
    let sampler = SamplerDescriptor::new()?;
    sampler.set_normalized_coordinates(true)?;
    sampler.set_s_address_mode(SamplerAddressMode::SamplerAddressModeRepeat)?;
    sampler.set_t_address_mode(SamplerAddressMode::SamplerAddressModeRepeat)?;
    sampler.set_min_filter(SamplerMinMagFilter::SamplerMinMagFilterNearest)?;
    sampler.set_mag_filter(SamplerMinMagFilter::SamplerMinMagFilterNearest)?;
    Ok(sampler)
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let descriptor = match build_descriptor() {
        Ok(descriptor) => descriptor,
        Err(error) => {
            println!("Sampler descriptors are unavailable here: {error}");
            return;
        }
    };
    match device.new_sampler_state(&descriptor) {
        Ok(_state) => println!("SamplerState created from the descriptor."),
        Err(error) => println!("Sampler state creation failed: {error}"),
    }
}

//! Example 056 — Address modes and LOD clamps.
//!
//! SamplerDescriptor setters validate values: NaN LOD clamps are rejected
//! with an error before Metal sees them, and address modes must be declared
//! constants. This descriptor is later consumed by new_sampler_state().
//!
//! Run with:
//! ```text
//! cargo run -p ex056_sampler_descriptors
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::{SamplerAddressMode, SamplerDescriptor, SamplerMinMagFilter};

fn main() {
    let Ok(sampler) = SamplerDescriptor::new() else {
        println!("SamplerDescriptor creation is unavailable on this runtime.");
        return;
    };
    sampler
        .set_normalized_coordinates(true)
        .expect("normalized coordinates");
    sampler
        .set_s_address_mode(SamplerAddressMode::SamplerAddressModeRepeat)
        .expect("address mode");
    sampler
        .set_t_address_mode(SamplerAddressMode::SamplerAddressModeClampToEdge)
        .expect("address mode");
    sampler
        .set_min_filter(SamplerMinMagFilter::SamplerMinMagFilterLinear)
        .expect("min filter");
    sampler
        .set_mag_filter(SamplerMinMagFilter::SamplerMinMagFilterLinear)
        .expect("mag filter");
    println!(
        "Normalized coordinates: {}",
        sampler.normalized_coordinates().expect("read")
    );
    if sampler.set_lod_min_clamp(f32::NAN).is_err() {
        println!("NaN LOD clamp rejected by the safe wrapper.");
    }
    sampler.set_lod_min_clamp(0.0).expect("finite clamp");
    sampler.set_lod_max_clamp(4.0).expect("finite clamp");
    println!(
        "LOD clamp range: {}..{}",
        sampler.lod_min_clamp().expect("min"),
        sampler.lod_max_clamp().expect("max")
    );
}

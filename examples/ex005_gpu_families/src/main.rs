//! Example 005 — GPU family support checks.
//!
//! `supports_family()` is the modern way to gate features. Apple families run
//! from A7 GPUs to today's Apple silicon; Common and Mac families cover Intel
//! and AMD GPUs. The safe API returns `Result<bool, Error>` and re-validates
//! each family constant against the runtime.
//!
//! Run with:
//! ```text
//! cargo run -p ex005_gpu_families
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::GPUFamily;

/// Families probed by this example.
const FAMILIES: [(GPUFamily, &str); 6] = [
    (GPUFamily::GPUFamilyApple5, "Apple 5 (A11 class)"),
    (GPUFamily::GPUFamilyApple7, "Apple 7 (M1 class)"),
    (GPUFamily::GPUFamilyApple8, "Apple 8 (M2 class)"),
    (GPUFamily::GPUFamilyMac2, "Mac 2"),
    (GPUFamily::GPUFamilyCommon3, "Common 3"),
    (GPUFamily::GPUFamilyMetal4, "Metal 4"),
];

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    println!("GPU family support for: {}", device.name());
    for (family, label) in FAMILIES {
        match device.supports_family(family) {
            Ok(true) => println!("  [yes] {label}"),
            Ok(false) => println!("  [no ] {label}"),
            Err(error) => println!("  [ ? ] {label} (unavailable: {error})"),
        }
    }
}

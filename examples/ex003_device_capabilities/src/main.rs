//! Example 003 — Boolean device capability matrix.
//!
//! `Device::capability()` answers one yes/no question at a time. The safe API
//! re-checks selector availability at runtime, so unknown capabilities on older
//! runtimes return an error instead of crashing.
//!
//! Run with:
//! ```text
//! cargo run -p ex003_device_capabilities
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::DeviceCapability;

/// The capability list this example probes, in report order.
const CAPABILITIES: [(DeviceCapability, &str); 8] = [
    (DeviceCapability::UnifiedMemory, "Unified memory"),
    (DeviceCapability::Headless, "Headless"),
    (DeviceCapability::LowPower, "Low power"),
    (DeviceCapability::Removable, "Removable"),
    (
        DeviceCapability::DynamicLibraries,
        "Dynamic Metal libraries",
    ),
    (DeviceCapability::FunctionPointers, "Function pointers"),
    (DeviceCapability::RayTracing, "Ray tracing"),
    (
        DeviceCapability::BarycentricCoordinates,
        "Barycentric coordinates",
    ),
];

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    println!("Capability report for: {}", device.name());
    for (capability, label) in CAPABILITIES {
        match device.capability(capability) {
            Ok(true) => println!("  [yes] {label}"),
            Ok(false) => println!("  [no ] {label}"),
            Err(error) => println!("  [ ? ] {label} (unavailable: {error})"),
        }
    }
}

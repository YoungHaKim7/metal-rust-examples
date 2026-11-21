//! Example 089 — Iterator-driven capability report.
//!
//! Rust iterators turn a capability probe into a table without loops of
//! copy-paste. Collect (name, Result<bool>) pairs, map them to markers, and
//! zip the columns for aligned output.
//!
//! Run with:
//! ```text
//! cargo run -p ex089_feature_matrix
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, metal::DeviceCapability};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let probes = [
        ("unified memory", DeviceCapability::UnifiedMemory),
        ("ray tracing", DeviceCapability::RayTracing),
        ("dynamic libraries", DeviceCapability::DynamicLibraries),
        ("function pointers", DeviceCapability::FunctionPointers),
        ("headless", DeviceCapability::Headless),
        ("removable", DeviceCapability::Removable),
    ];
    let rows = probes
        .iter()
        .map(|(label, capability)| {
            let verdict = match device.capability(*capability) {
                Ok(true) => "[yes]",
                Ok(false) => "[no ]",
                Err(_) => "[ ? ]",
            };
            format!("{verdict} {label}")
        })
        .collect::<Vec<_>>();
    println!("Capability matrix for {}", device.name());
    for row in rows {
        println!("  {row}");
    }
    let supported = probes
        .iter()
        .filter(|(_, capability)| device.capability(*capability).unwrap_or(false))
        .count();
    println!(
        "{supported} of {} probed capabilities are supported.",
        probes.len()
    );
}

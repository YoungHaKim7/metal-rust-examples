//! Example 004 — Numeric device properties without narrowing.
//!
//! Metal reports sizes and limits as platform-specific integers. The safe API
//! wraps them in `DeviceNumericProperty` and returns them as `u64`, so nothing
//! is silently truncated on the way from Objective-C to Rust.
//!
//! Run with:
//! ```text
//! cargo run -p ex004_device_numeric_properties
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::DeviceNumericProperty;

/// Numeric properties probed by this example with display labels.
const PROPERTIES: [(DeviceNumericProperty, &str); 6] = [
    (
        DeviceNumericProperty::MaxBufferLength,
        "Max buffer length (bytes)",
    ),
    (
        DeviceNumericProperty::RecommendedMaxWorkingSetSize,
        "Recommended working set (bytes)",
    ),
    (
        DeviceNumericProperty::MaxThreadgroupMemoryLength,
        "Max threadgroup memory (bytes)",
    ),
    (
        DeviceNumericProperty::TimestampFrequency,
        "GPU timestamp frequency (Hz)",
    ),
    (DeviceNumericProperty::RegistryId, "Registry id"),
    (
        DeviceNumericProperty::CurrentAllocatedSize,
        "Currently allocated",
    ),
];

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    println!("Numeric properties of: {}", device.name());
    for (property, label) in PROPERTIES {
        match device.numeric_property(property) {
            Ok(value) => println!("  {label}: {value}"),
            Err(error) => println!("  {label}: unavailable ({error})"),
        }
    }
}

//! Example 002 — Device identity: name, architecture and location.
//!
//! The facade exposes checked accessors instead of raw selectors. `name()`
//! returns an owned `String`, `architecture()` returns a checked `Result`, and
//! `location()` reports where the GPU sits (built in, slot, external).
//!
//! Run with:
//! ```text
//! cargo run -p ex002_device_identity
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::DeviceLocation;

/// Maps a DeviceLocation constant to human-readable text.
fn location_name(location: DeviceLocation) -> &'static str {
    match location {
        DeviceLocation::DeviceLocationBuiltIn => "built in",
        DeviceLocation::DeviceLocationSlot => "slot",
        DeviceLocation::DeviceLocationExternal => "external",
        DeviceLocation::DeviceLocationUnspecified => "unspecified",
        _ => "unknown",
    }
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    println!("Device name   : {}", device.name());
    match device.architecture() {
        Ok(_) => println!("Architecture  : reported by Metal"),
        Err(error) => println!("Architecture  : unavailable ({error})"),
    }
    println!("Location      : {}", location_name(device.location()));
}

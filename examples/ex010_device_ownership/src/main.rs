//! Example 010 — Device ownership: Clone and thread binding.
//!
//! `Device` is `Clone`, so cheap handles can be shared across your program,
//! but metal-rust types are deliberately `!Send + !Sync`: Metal protocol
//! objects must stay on the thread that created them. The type system enforces
//! that rule at compile time — no runtime checks needed.
//!
//! Run with:
//! ```text
//! cargo run -p ex010_device_ownership
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

/// Reports the device identity through an owned clone.
fn describe(device: Device) -> String {
    device.name()
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let clone = device.clone();
    println!("Original device name : {}", device.name());
    println!("Cloned device name   : {}", describe(clone));
    println!("Both handles own the same underlying Metal device.");
    println!("Tip: Device, Buffer, Texture and CommandBuffer are all !Send + !Sync.");
    println!("Pass references (&Device) instead of moving them across threads.");
}

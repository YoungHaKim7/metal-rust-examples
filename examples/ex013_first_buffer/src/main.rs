//! Example 013 — Allocate a shared buffer.
//!
//! `new_buffer(length, options)` allocates GPU-visible memory. With
//! `ResourceOptions::SHARED`, CPU and GPU see one coherent allocation, which
//! is the right default on Apple silicon. `length()` and `storage_mode()` read
//! the allocation back.
//!
//! Run with:
//! ```text
//! cargo run -p ex013_first_buffer
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::StorageMode;
use metal_rust::{Device, ResourceOptions};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let buffer = match device.new_buffer(256, ResourceOptions::SHARED) {
        Ok(buffer) => buffer,
        Err(error) => {
            println!("Buffer allocation failed: {error}");
            return;
        }
    };
    println!("Buffer length     : {} bytes", buffer.length());
    let mode = match buffer.storage_mode() {
        StorageMode::Shared => "shared",
        StorageMode::Managed => "managed",
        StorageMode::Private => "private",
        StorageMode::Memoryless => "memoryless",
    };
    println!("Storage mode      : {mode}");
    println!("Allocation is owned by the Buffer value; drop releases it.");
}

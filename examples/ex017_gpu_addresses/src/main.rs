//! Example 017 — Buffer GPU addresses and identity.
//!
//! Every buffer has a GPU virtual address. Addresses are useful for argument
//! buffers and indirect encoders; two live buffers always get different
//! addresses on the same device.
//!
//! Run with:
//! ```text
//! cargo run -p ex017_gpu_addresses
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ResourceOptions};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let mut seen = Vec::new();
    for index in 0..3 {
        let buffer = device
            .new_buffer_with_bytes(&[index as u8; 32], ResourceOptions::SHARED)
            .expect("buffer allocation");
        println!(
            "Buffer {index}: length={} gpu_address=0x{:x}",
            buffer.length(),
            buffer.gpu_address()
        );
        seen.push(buffer.gpu_address());
    }
    seen.sort();
    seen.dedup();
    println!("Distinct addresses observed: {}", seen.len());
}

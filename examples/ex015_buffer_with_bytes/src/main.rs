//! Example 015 — Initialize buffers from Rust data.
//!
//! `new_buffer_with_bytes(bytes, options)` allocates and fills a buffer in one
//! step. Rust slices become GPU-visible memory: encode your data with
//! `to_ne_bytes()` and hand the flat byte array to Metal.
//!
//! Run with:
//! ```text
//! cargo run -p ex015_buffer_with_bytes
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ResourceOptions};

/// Encodes a slice of f32 values into little-endian native bytes.
fn encode_f32(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let samples = [1.0f32, 2.0, 3.0, 4.0, 5.5, -0.25];
    let bytes = encode_f32(&samples);
    let buffer = device
        .new_buffer_with_bytes(&bytes, ResourceOptions::SHARED)
        .expect("initialized buffer");
    println!(
        "Created a buffer of {} bytes ({} f32 samples)",
        buffer.length(),
        samples.len()
    );
    match buffer.write(0, &bytes) {
        Ok(()) => println!("In-place refresh with buffer.write() also works."),
        Err(error) => println!("Refresh failed: {error}"),
    }
}

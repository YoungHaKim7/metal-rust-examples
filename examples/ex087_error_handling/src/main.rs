//! Example 087 — Result pipelines and error kinds.
//!
//! Real programs chain many GPU calls. Model each stage as a function
//! returning Result, use `?` for the happy path, and classify failures with
//! `error.kind()` where you can recover. This example builds a small
//! pipeline: allocate, validate, submit.
//!
//! Run with:
//! ```text
//! cargo run -p ex087_error_handling
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ErrorKind};

/// Allocates a buffer or classifies the failure.
fn allocate(device: &Device, length: usize) -> Result<metal_rust::Buffer, String> {
    device
        .new_buffer(length, metal_rust::ResourceOptions::SHARED)
        .map_err(|error| match error.kind() {
            ErrorKind::InvalidArgument => format!("allocation of {length} bytes is invalid"),
            ErrorKind::Unsupported => "buffers are unavailable on this runtime".to_string(),
            ErrorKind::Metal => format!("driver rejected the allocation: {error}"),
        })
}

/// Validates the payload before writing; a deliberate guard rail.
fn validate(payload: &[u8], length: usize) -> Result<(), String> {
    if payload.len() > length {
        return Err(format!(
            "payload {} exceeds buffer {}",
            payload.len(),
            length
        ));
    }
    Ok(())
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let pipeline = || -> Result<String, String> {
        let buffer = allocate(&device, 32)?;
        let payload = [1u8, 2, 3, 4];
        validate(&payload, buffer.length())?;
        buffer
            .write(0, &payload)
            .map_err(|error| format!("write failed: {error}"))?;
        Ok(format!("pipeline succeeded with {} bytes", payload.len()))
    }();
    match pipeline {
        Ok(message) => println!("Happy path: {message}"),
        Err(message) => println!("Pipeline error: {message}"),
    }
    match allocate(&device, 0) {
        Ok(_) => println!("Zero-byte allocation accepted."),
        Err(message) => println!("Guard rail: {message}"),
    }
}

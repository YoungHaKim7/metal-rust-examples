//! Example 099 — MetalFX temporal upscaling setup.
//!
//! Temporal upscaling combines color, depth and motion vectors across frames.
//! The descriptor takes four formats, and the facade exposes the scale range
//! the device accepts so engines can clamp dynamic resolution early.
//!
//! Run with:
//! ```text
//! cargo run -p ex099_metalfx_temporal
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ErrorKind, PixelFormat};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let descriptor = match metal_rust::TemporalScalerDescriptor::new() {
        Ok(descriptor) => descriptor,
        Err(error) if error.kind() == ErrorKind::Unsupported => {
            println!("MetalFX temporal is not available on this runtime; ending gracefully.");
            return;
        }
        Err(error) => {
            println!("Temporal scaler descriptor failed: {error}");
            return;
        }
    };
    descriptor
        .configure(
            PixelFormat::BGRA8_UNORM,
            PixelFormat::DEPTH32_FLOAT,
            PixelFormat::RG16_FLOAT,
            PixelFormat::BGRA8_UNORM,
            1280,
            720,
            2560,
            1440,
        )
        .expect("valid temporal configuration");
    match descriptor.supported_input_content_min_scale(&device) {
        Ok(minimum) => println!("Minimum input scale: {minimum:.2}"),
        Err(error) if error.kind() == ErrorKind::Unsupported => {
            println!("Scale queries unsupported here; ending gracefully.");
            return;
        }
        Err(error) => {
            println!("Scale query failed: {error}");
            return;
        }
    }
    let maximum = descriptor
        .supported_input_content_max_scale(&device)
        .expect("max scale queried after min");
    println!("Maximum input scale: {maximum:.2}");
    if descriptor.supports_device(&device) {
        let scaler = match descriptor.new_scaler(&device) {
            Ok(scaler) => scaler,
            Err(error) if error.kind() == ErrorKind::Unsupported => return,
            Err(error) => {
                println!("Scaler creation failed: {error}");
                return;
            }
        };
        scaler
            .set_input_content_size(1280, 720)
            .expect("content size");
        println!(
            "Temporal scaler ready; output usage bits: {}",
            scaler.output_texture_usage()
        );
    } else {
        println!("This device does not support MetalFX temporal scaling.");
    }
}

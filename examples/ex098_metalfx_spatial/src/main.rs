//! Example 098 — MetalFX spatial upscaling setup.
//!
//! MetalFX renders at a lower resolution and reconstructs detail for higher
//! resolutions. The descriptor validates the source/output formats and
//! sizes, `supports_device()` gates hardware availability, and the scaler
//! reports which texture usage it needs for outputs.
//!
//! Run with:
//! ```text
//! cargo run -p ex098_metalfx_spatial
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ErrorKind, PixelFormat};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let descriptor = match metal_rust::SpatialScalerDescriptor::new() {
        Ok(descriptor) => descriptor,
        Err(error) if error.kind() == ErrorKind::Unsupported => {
            println!("MetalFX is not available on this runtime; ending gracefully.");
            return;
        }
        Err(error) => {
            println!("Spatial scaler descriptor failed: {error}");
            return;
        }
    };
    descriptor
        .configure(
            PixelFormat::BGRA8_UNORM,
            PixelFormat::BGRA8_UNORM,
            1280,
            720,
            2560,
            1440,
        )
        .expect("valid spatial configuration");
    if descriptor.supports_device(&device) {
        let scaler = match descriptor.new_scaler(&device) {
            Ok(scaler) => scaler,
            Err(error) if error.kind() == ErrorKind::Unsupported => {
                println!("The device passed the check but the scaler refused; ok.");
                return;
            }
            Err(error) => {
                println!("Scaler creation failed: {error}");
                return;
            }
        };
        println!("Spatial scaler created for 1280x720 -> 2560x1440.");
        println!(
            "Required output texture usage bits: {}",
            scaler.output_texture_usage()
        );
        scaler
            .set_input_content_size(1280, 720)
            .expect("content size");
        println!("Input content size updated; the scaler is ready to encode.");
    } else {
        println!("This device does not support MetalFX spatial scaling.");
    }
}

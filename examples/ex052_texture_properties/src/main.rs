//! Example 052 — Texture dimensions and pixel formats.
//!
//! Textures answer questions about themselves: `width()`, `height()` and
//! `pixel_format()`. Create several in different formats and compare.
//!
//! Run with:
//! ```text
//! cargo run -p ex052_texture_properties
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, PixelFormat, ResourceOptions, TextureDescriptor, TextureUsage};

fn create(
    device: &Device,
    format: PixelFormat,
    width: usize,
    height: usize,
) -> Option<metal_rust::Texture> {
    let descriptor = TextureDescriptor::new_2d(
        format,
        width,
        height,
        ResourceOptions::SHARED,
        TextureUsage::SHADER_READ,
    )
    .ok()?;
    device.new_texture(&descriptor).ok()
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let plans = [
        (PixelFormat::RGBA8_UNORM, 64usize, 64usize, "RGBA8_UNORM"),
        (PixelFormat::BGRA8_UNORM, 32, 32, "BGRA8_UNORM"),
        (PixelFormat::RGBA16_FLOAT, 16, 16, "RGBA16_FLOAT"),
        (PixelFormat::DEPTH32_FLOAT, 8, 8, "DEPTH32_FLOAT"),
    ];
    for (format, width, height, label) in plans {
        match create(&device, format, width, height) {
            Some(texture) => println!(
                "{label:>13}: {}x{} created",
                texture.width(),
                texture.height()
            ),
            None => println!("{label:>13}: unavailable on this runtime"),
        }
    }
}

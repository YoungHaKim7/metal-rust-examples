//! Example 060 — Cross-process shareable textures.
//!
//! `new_shared_texture()` creates a texture whose backing store can cross
//! process boundaries; pair it with `new_shared_texture_handle()` to export
//! the identity. `is_shareable()` and the descriptor's storage mode tell you
//! what Metal granted.
//!
//! Run with:
//! ```text
//! cargo run -p ex060_shared_textures
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, PixelFormat, ResourceOptions, TextureDescriptor, TextureUsage};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let descriptor = TextureDescriptor::new_2d(
        PixelFormat::RGBA8_UNORM,
        32,
        32,
        ResourceOptions::SHARED,
        TextureUsage::SHADER_READ,
    )
    .expect("descriptor");
    match device.new_shared_texture(&descriptor) {
        Ok(texture) => {
            println!(
                "Shared texture created: {}x{}",
                texture.width(),
                texture.height()
            );
            println!("Shareable: {}", texture.is_shareable());
            match texture.new_shared_texture_handle() {
                Ok(_handle) => println!("Exported a SharedTextureHandle for other processes."),
                Err(error) => println!("Handle export unavailable: {error}"),
            }
        }
        Err(error) => println!("Shared textures unavailable on this runtime: {error}"),
    }
}

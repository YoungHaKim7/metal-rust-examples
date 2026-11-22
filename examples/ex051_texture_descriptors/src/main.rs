//! Example 051 — Descriptor factories and validation.
//!
//! `TextureDescriptor::new_2d(format, width, height, storage, usage)` checks
//! every argument before Metal is touched: a zero width is rejected with
//! `InvalidArgument` and never reaches the driver. The convenience factories
//! (`texture_2d`, `texture_cube`, `texture_buffer`) pick Metal defaults for
//! you.
//!
//! Run with:
//! ```text
//! cargo run -p ex051_texture_descriptors
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    Device, ErrorKind, PixelFormat, ResourceOptions, TextureDescriptor, TextureUsage,
};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let zero_width = TextureDescriptor::new_2d(
        PixelFormat::RGBA8_UNORM,
        0,
        1,
        ResourceOptions::SHARED,
        TextureUsage::SHADER_READ,
    );
    match zero_width {
        Ok(_) => println!("Unexpected: zero width accepted."),
        Err(error) if error.kind() == ErrorKind::InvalidArgument => {
            println!("Zero width rejected before Metal was called.");
        }
        Err(error) => println!("Rejected with another error: {error}"),
    }
    let descriptor = TextureDescriptor::new_2d(
        PixelFormat::RGBA8_UNORM,
        64,
        64,
        ResourceOptions::SHARED,
        TextureUsage::SHADER_READ.union(TextureUsage::RENDER_TARGET),
    )
    .expect("valid descriptor");
    descriptor
        .set_mipmap_level_count(4)
        .expect("mip count valid");
    println!(
        "Descriptor: {}x{}, {} mip levels",
        descriptor.dimensions().0,
        descriptor.dimensions().1,
        descriptor.mipmap_level_count()
    );
    let convenience =
        TextureDescriptor::texture_2d(PixelFormat::RGBA8_UNORM, 32, 32, true).expect("factory");
    println!(
        "texture_2d factory descriptor has {} mip levels",
        convenience.mipmap_level_count()
    );
    let _ = &device; // device is available when you want to allocate for real
}

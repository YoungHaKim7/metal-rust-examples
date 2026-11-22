//! Example 054 — replace_region_2d pixel uploads.
//!
//! `replace_region_2d(region, mipmap_level, bytes, bytes_per_row)` copies CPU
//! bytes into a texture region. The byte stride must match the format's row
//! size; a checkerboard pattern makes any mistake visible in the readback.
//!
//! Run with:
//! ```text
//! cargo run -p ex054_texture_upload
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    Device, Origin, PixelFormat, Region, ResourceOptions, TextureDescriptor, TextureUsage,
};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

/// Builds an RGBA checkerboard with 8-byte squares... well, pixels here.
fn checkerboard() -> Vec<u8> {
    let mut pixels = Vec::with_capacity(WIDTH * HEIGHT * 4);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let light = (x + y) % 2 == 0;
            let value = if light { 255u8 } else { 30u8 };
            pixels.extend_from_slice(&[value, value, value, 255]);
        }
    }
    pixels
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let descriptor = TextureDescriptor::new_2d(
        PixelFormat::RGBA8_UNORM,
        WIDTH,
        HEIGHT,
        ResourceOptions::SHARED,
        TextureUsage::SHADER_READ,
    )
    .expect("descriptor");
    let texture = device.new_texture(&descriptor).expect("texture");
    let pixels = checkerboard();
    texture
        .replace_region_2d(Region::new_2d(0, 0, WIDTH, HEIGHT), 0, &pixels, WIDTH * 4)
        .expect("upload");
    println!("Uploaded {} bytes at level 0.", pixels.len());
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_texture(
                &texture,
                Region::new(
                    Origin::new(0, 0, 0),
                    metal_rust::Size::new(WIDTH, HEIGHT, 1),
                ),
            )
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let image = completed.resolve_texture(readback).expect("resolve");
    let pixel = |x: usize, y: usize| -> [u8; 4] {
        let index = y * image.bytes_per_row + x * 4;
        image.bytes[index..index + 4].try_into().expect("4 bytes")
    };
    println!(
        "Pixel (0,0)={:?} pixel (1,0)={:?}",
        pixel(0, 0),
        pixel(1, 0)
    );
    assert_eq!(pixel(0, 0)[0], 255);
    assert_eq!(pixel(1, 0)[0], 30);
    println!("Checkerboard survived the GPU round trip.");
}

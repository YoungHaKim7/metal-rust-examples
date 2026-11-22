//! Example 042 — Color attachment clears without draws.
//!
//! A render pass can contain zero draws and still do useful work: each color
//! attachment is cleared to its ClearColor at load time. This is how engines
//! reset the framebuffer between frames.
//!
//! Run with:
//! ```text
//! cargo run -p ex042_clear_pass
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    ClearColor, Device, PixelFormat, Region, RenderPassDescriptor, ResourceOptions,
    TextureDescriptor, TextureUsage,
};

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let texture_descriptor = TextureDescriptor::new_2d(
        PixelFormat::RGBA8_UNORM,
        WIDTH,
        HEIGHT,
        ResourceOptions::SHARED,
        TextureUsage::RENDER_TARGET,
    )
    .expect("descriptor");
    let texture = device.new_texture(&texture_descriptor).expect("texture");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let pass = RenderPassDescriptor::new();
        pass.set_color_attachment(0, &texture, ClearColor::new(0.1, 0.2, 0.3, 1.0))
            .expect("attachment 0");
        let encoder = command_buffer
            .render_encoder(&pass)
            .expect("render encoder");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_texture(&texture, Region::new_2d(0, 0, WIDTH, HEIGHT))
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let image = completed.resolve_texture(readback).expect("resolve");
    let pixel = &image.bytes[..4];
    println!("Cleared pixel (RGBA8_UNORM): {pixel:?}");
    assert_eq!(pixel[0], 26); // 0.1 * 255
    assert_eq!(pixel[1], 51); // 0.2 * 255
    assert_eq!(pixel[2], 77); // 0.3 * 255
    println!("Load action cleared every pixel to the requested color.");
}

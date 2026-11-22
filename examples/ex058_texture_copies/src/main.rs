//! Example 058 — copy_texture and copy_texture_region.
//!
//! Blit encoders copy whole textures or checked sub-regions. This example
//! fills a texture from the CPU, copies it whole, then copies a 4x4 corner
//! region into a second texture and verifies both results.
//!
//! Run with:
//! ```text
//! cargo run -p ex058_texture_copies
//! ```

#![forbid(unsafe_code)]

use metal_rust::{
    Device, Origin, PixelFormat, Region, ResourceOptions, TextureDescriptor, TextureUsage,
};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let make_descriptor = || {
        TextureDescriptor::new_2d(
            PixelFormat::RGBA8_UNORM,
            WIDTH,
            HEIGHT,
            ResourceOptions::SHARED,
            TextureUsage::SHADER_READ,
        )
        .expect("descriptor")
    };
    let source = device.new_texture(&make_descriptor()).expect("source");
    let whole_copy = device.new_texture(&make_descriptor()).expect("whole copy");
    let corner_copy = device.new_texture(&make_descriptor()).expect("corner copy");
    let pixels: Vec<u8> = (0..WIDTH * HEIGHT)
        .flat_map(|index| {
            let value = (index % 251) as u8;
            [value, value, value, 255u8]
        })
        .collect();
    let expected = |x: usize, y: usize| -> [u8; 4] {
        let value = ((y * WIDTH + x) % 251) as u8;
        [value, value, value, 255]
    };
    source
        .replace_region_2d(Region::new_2d(0, 0, WIDTH, HEIGHT), 0, &pixels, WIDTH * 4)
        .expect("upload");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    let read_whole;
    let read_corner;
    {
        let blit = command_buffer.blit_encoder().expect("blit");
        blit.copy_texture(&source, &whole_copy).expect("whole copy");
        blit.copy_texture_region(
            &source,
            0,
            0,
            Region::new_2d(0, 0, 4, 4),
            &corner_copy,
            0,
            0,
            Origin::new(4, 4, 0),
        )
        .expect("region copy");
        read_whole = blit
            .read_texture(&whole_copy, Region::new_2d(0, 0, WIDTH, HEIGHT))
            .expect("read whole");
        read_corner = blit
            .read_texture(&corner_copy, Region::new_2d(0, 0, WIDTH, HEIGHT))
            .expect("read corner");
        blit.end_encoding();
    }
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let whole = completed
        .resolve_texture(read_whole)
        .expect("resolve whole");
    let corner = completed
        .resolve_texture(read_corner)
        .expect("resolve corner");
    let fetch = |image: &metal_rust::TextureReadbackData, x: usize, y: usize| -> [u8; 4] {
        let index = y * image.bytes_per_row + x * 4;
        image.bytes[index..index + 4].try_into().expect("4 bytes")
    };
    println!(
        "Whole copy (3,3): {:?} (expected {:?})",
        fetch(&whole, 3, 3),
        expected(3, 3)
    );
    println!(
        "Region copy lands at (4,4): {:?} (expected source (0,0) = {:?})",
        fetch(&corner, 4, 4),
        expected(0, 0)
    );
    assert_eq!(fetch(&whole, 3, 3), expected(3, 3));
    assert_eq!(fetch(&corner, 4, 4), expected(0, 0));
    println!("Both GPU copies verified byte for byte.");
}

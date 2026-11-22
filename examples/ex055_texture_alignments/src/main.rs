//! Example 055 — Linear texture alignment queries.
//!
//! `device.texture_alignments(format)` returns the (linear alignment, bytes
//! per pixel at level 0) pair Metal expects for uploads. Plan your
//! replace_region_2d() strides from these numbers, not from guesswork.
//!
//! Run with:
//! ```text
//! cargo run -p ex055_texture_alignments
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, PixelFormat};

const FORMATS: [(PixelFormat, &str); 5] = [
    (PixelFormat::RGBA8_UNORM, "RGBA8_UNORM"),
    (PixelFormat::BGRA8_UNORM, "BGRA8_UNORM"),
    (PixelFormat::R16_FLOAT, "R16_FLOAT"),
    (PixelFormat::RGBA16_FLOAT, "RGBA16_FLOAT"),
    (PixelFormat::DEPTH32_FLOAT, "DEPTH32_FLOAT"),
];

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    for (format, label) in FORMATS {
        let (linear_alignment, bytes_per_pixel) = device.texture_alignments(format);
        println!("{label:>13}: bytes/pixel {bytes_per_pixel}, linear alignment {linear_alignment}");
    }
    println!("A 5-pixel-wide RGBA8 row needs 20 bytes, padded up to the alignment.");
}

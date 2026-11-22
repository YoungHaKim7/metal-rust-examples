//! Example 009 — Heap size and alignment queries.
//!
//! Metal heaps own sub-allocations. Before placing resources you can ask Metal
//! how much space a buffer or texture would need inside a heap, including the
//! required alignment. The safe API returns a `SizeAndAlign` struct with plain
//! `usize` fields.
//!
//! Run with:
//! ```text
//! cargo run -p ex009_heap_allocations
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, PixelFormat, ResourceOptions, TextureDescriptor, TextureUsage};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    for length in [256usize, 4096, 1_048_576] {
        match device.heap_buffer_size_and_align(length, ResourceOptions::SHARED) {
            Ok(size_and_align) => println!(
                "Buffer of {length:>8} bytes -> size {}, align {}",
                size_and_align.size, size_and_align.align
            ),
            Err(error) => println!("Buffer of {length} bytes -> error: {error}"),
        }
    }
    let descriptor = TextureDescriptor::new_2d(
        PixelFormat::RGBA8_UNORM,
        64,
        64,
        ResourceOptions::SHARED,
        TextureUsage::SHADER_READ,
    )
    .expect("valid texture descriptor");
    match device.heap_texture_size_and_align(&descriptor) {
        Ok(size_and_align) => println!(
            "64x64 RGBA8 texture -> size {}, align {}",
            size_and_align.size, size_and_align.align
        ),
        Err(error) => println!("Texture size query failed: {error}"),
    }
}

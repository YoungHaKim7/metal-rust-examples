//! Example 078 — CAMetalLayer drawable ownership.
//!
//! Presentation goes through CAMetalLayer: `Layer::with_device()` creates a
//! layer bound to your GPU, `next_drawable()` leases the next swapchain image
//! if Core Animation has one, and `present_drawable()` hands it back after
//! rendering. Without a window the lease simply fails — handled gracefully.
//!
//! Run with:
//! ```text
//! cargo run -p ex078_layer_drawable
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ErrorKind, PixelFormat};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let layer = match metal_rust::Layer::with_device(&device) {
        Ok(layer) => layer,
        Err(error) if error.kind() == ErrorKind::InvalidArgument => {
            println!("Layers need a window server connection; skipping on this runtime.");
            return;
        }
        Err(error) => {
            println!("Layer creation failed: {error}");
            return;
        }
    };
    layer.set_pixel_format(PixelFormat::BGRA8_UNORM);
    println!("Layer pixel format set to BGRA8_UNORM.");
    match layer.device() {
        Some(owned_device) => println!("Layer is bound to: {}", owned_device.name()),
        None => println!("Layer has no device yet."),
    }
    match layer.next_drawable() {
        Ok(drawable) => {
            println!("Leased a drawable from the swapchain.");
            let queue = device.new_command_queue(None).expect("queue");
            let mut command_buffer = queue.command_buffer().expect("command buffer");
            command_buffer.present_drawable(&drawable);
            command_buffer
                .commit()
                .wait()
                .expect("presentation completes");
            println!("Presented the drawable back to Core Animation.");
        }
        Err(error) => {
            println!(
                "No drawable without a live window; error kind is Metal: {}",
                error.kind() == ErrorKind::Metal
            );
        }
    }
}

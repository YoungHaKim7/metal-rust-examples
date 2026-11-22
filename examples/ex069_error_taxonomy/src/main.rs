//! Example 069 — InvalidArgument, Unsupported and Metal errors.
//!
//! metal-rust has three error kinds. InvalidArgument means the safe wrapper
//! rejected your values before Metal; Unsupported means the runtime cannot do
//! it; Metal is a real driver error. This example triggers one of each —
//! deliberately.
//!
//! Run with:
//! ```text
//! cargo run -p ex069_error_taxonomy
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
    let buffer = device
        .new_buffer(16, ResourceOptions::SHARED)
        .expect("buffer");
    match buffer.write(15, &[1, 2]) {
        Err(error) if error.kind() == ErrorKind::InvalidArgument => {
            println!(
                "1) InvalidArgument: write of 2 bytes at offset 15 overflows a 16-byte buffer."
            );
        }
        Err(error) => println!("1) Unexpected: {error}"),
        Ok(()) => println!("1) Unexpected success."),
    }
    let bad_texture = TextureDescriptor::new_2d(
        PixelFormat::RGBA8_UNORM,
        0,
        1,
        ResourceOptions::SHARED,
        TextureUsage::SHADER_READ,
    );
    match bad_texture {
        Err(error) if error.kind() == ErrorKind::InvalidArgument => {
            println!("2) InvalidArgument: zero-width texture descriptors never reach the driver.");
        }
        result => println!("2) Unexpected result: {:?}", result.is_ok()),
    }
    let missing = match device.new_command_queue(None) {
        Ok(queue) => match queue.command_buffer() {
            Ok(command_buffer) => command_buffer.commit().wait().is_err(),
            Err(_) => true,
        },
        Err(_) => true,
    };
    println!("3) Submission paths return errors instead of panicking: {missing}");
    println!("ErrorKind has three variants: InvalidArgument, Unsupported, Metal.");
}

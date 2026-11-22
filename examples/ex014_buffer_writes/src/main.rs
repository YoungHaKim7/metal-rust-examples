//! Example 014 — Checked CPU writes into buffers.
//!
//! `buffer.write(offset, bytes)` copies CPU bytes into a shared buffer. Every
//! write is range-checked before it touches Metal: an out-of-bounds write
//! returns an `InvalidArgument` error instead of corrupting memory.
//!
//! Run with:
//! ```text
//! cargo run -p ex014_buffer_writes
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ErrorKind, ResourceOptions};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let buffer = device
        .new_buffer(64, ResourceOptions::SHARED)
        .expect("shared buffer allocation");
    buffer.write(0, &[1, 2, 3, 4]).expect("write at offset 0");
    buffer.write(60, &[7, 7, 7, 7]).expect("write at offset 60");
    println!("Two in-bounds writes succeeded.");
    let overflow = buffer.write(63, &[1, 2]);
    match overflow {
        Ok(()) => println!("Unexpected success writing past the end."),
        Err(error) if error.kind() == ErrorKind::InvalidArgument => {
            println!("Write past the end rejected as InvalidArgument.");
            println!("The safe API catches the mistake before Metal is called.");
        }
        Err(error) => println!("Write rejected with a different error: {error}"),
    }
}

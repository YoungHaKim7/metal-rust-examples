//! Example 001 — Hello Metal: create the default GPU device.
//!
//! Every metal-rust program starts with `Device::system_default()`. It returns
//! an owned `Device` when macOS can provide a Metal-capable GPU, or `None`
//! when it cannot (for example inside a headless VM). All metal-rust types are
//! safe Rust: no unsafe code, no raw Objective-C pointers, no manual
//! retain/release.
//!
//! Run with:
//! ```text
//! cargo run -p ex001_hello_metal
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    println!("Hello Metal!");
    println!("Device name : {}", device.name());
    println!("Created a safe, owned Device handle with zero unsafe code.");
}

//! Example 016 — SHARED, MANAGED, PRIVATE and MEMORYLESS storage.
//!
//! ResourceOptions selects where memory lives. SHARED is coherent on Apple
//! silicon, MANAGED keeps dual copies on Intel GPUs, PRIVATE is GPU-only, and
//! MEMORYLESS exists only inside render passes. Writes are allowed where the
//! CPU has access; a PRIVATE buffer rejects them.
//!
//! Run with:
//! ```text
//! cargo run -p ex016_storage_modes
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::StorageMode;
use metal_rust::{Device, ResourceOptions};

/// One allocation per storage mode with a display label.
const MODES: [(ResourceOptions, &str); 4] = [
    (ResourceOptions::SHARED, "SHARED"),
    (ResourceOptions::MANAGED, "MANAGED"),
    (ResourceOptions::PRIVATE, "PRIVATE"),
    (ResourceOptions::MEMORYLESS, "MEMORYLESS"),
];

fn mode_name(mode: StorageMode) -> &'static str {
    match mode {
        StorageMode::Shared => "shared",
        StorageMode::Managed => "managed",
        StorageMode::Private => "private",
        StorageMode::Memoryless => "memoryless",
    }
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    for (options, label) in MODES {
        match device.new_buffer(64, options) {
            Ok(buffer) => {
                let write_result = buffer.write(0, &[1, 2, 3, 4]);
                let writable = match write_result {
                    Ok(()) => "cpu writes accepted",
                    Err(_) => "cpu writes rejected",
                };
                println!(
                    "{label:>9} -> storage_mode={:<10} {}",
                    mode_name(buffer.storage_mode()),
                    writable
                );
            }
            Err(error) => println!("{label:>9} -> allocation unavailable ({error})"),
        }
    }
    println!("MEMORYLESS buffers are only valid inside render passes on some runtimes.");
}

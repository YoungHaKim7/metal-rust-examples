//! Example 079 — add_debug_marker ranges.
//!
//! Debug markers name byte ranges inside buffers; the GPU debugger renders
//! them as annotations. `add_debug_marker(name, range)` checks the range and
//! `remove_all_debug_markers()` clears the board.
//!
//! Run with:
//! ```text
//! cargo run -p ex079_buffer_debug_markers
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ResourceOptions};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let buffer = device
        .new_buffer(256, ResourceOptions::SHARED)
        .expect("buffer");
    buffer
        .add_debug_marker("vertices", 0..128)
        .expect("marker range");
    buffer
        .add_debug_marker("indices", 128..256)
        .expect("marker range");
    println!("Two markers placed: 'vertices' owns the first half, 'indices' the second.");
    buffer.remove_all_debug_markers();
    println!("Markers cleared; the buffer itself is untouched.");
    if buffer.add_debug_marker("overflow", 200..300).is_err() {
        println!("Ranges are checked: a marker past the end is rejected.");
    }
}

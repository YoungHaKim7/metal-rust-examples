//! Example 023 — Enumerate functions in a library.
//!
//! A Library bundles compiled MSL functions. `function_names()` lists what the
//! source contained, and `function(name)` returns a checked `Result<Function>`
//! for each entry. Lookups of unknown names fail with an error instead of a
//! nil pointer.
//!
//! Run with:
//! ```text
//! cargo run -p ex023_library_functions
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void fill_zero(device uint* out [[buffer(0)]],
                      uint id [[thread_position_in_grid]]) {
    out[id] = 0;
}

kernel void fill_one(device uint* out [[buffer(0)]],
                     uint id [[thread_position_in_grid]]) {
    out[id] = 1;
}

vertex float4 plain_vertex(uint id [[vertex_id]]) {
    return float4(0.0, 0.0, 0.0, 1.0);
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("three functions compile");
    let names = library.function_names();
    println!("Functions in library: {names:?}");
    for name in &names {
        match library.function(name) {
            Ok(function) => println!("  {name}: resolved as {}", function.name()),
            Err(error) => println!("  {name}: lookup failed ({error})"),
        }
    }
}

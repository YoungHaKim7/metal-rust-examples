//! Example 021 — Compile MSL source at runtime.
//!
//! Metal compiles the Metal Shading Language (MSL) at runtime, just like a
//! JIT. `new_library_from_source(source, options)` takes a Rust string and
//! returns an owned `Library`. `None` compiles with Metal defaults;
//! `Some(&CompileOptions::strict())` enables strict math and logging.
//!
//! Run with:
//! ```text
//! cargo run -p ex021_compile_inline_msl
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

/// A minimal but complete MSL kernel.
const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void ping(device uint* out [[buffer(0)]],
                 uint id [[thread_position_in_grid]]) {
    if (id == 0) {
        out[0] = 1;
    }
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    match device.new_library_from_source(SOURCE, None) {
        Ok(library) => {
            println!("Compiled an MSL library at runtime.");
            println!("Library device : {}", library.device().name());
            println!("Library type   : {}", library.library_type().as_raw());
        }
        Err(error) => println!("Compilation failed: {error}"),
    }
}

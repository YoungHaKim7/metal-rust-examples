//! Example 022 — CompileOptions: strict, fast-math, logging.
//!
//! `CompileOptions` controls the MSL compiler. `strict()` turns on strict math
//! and logging; `new()` starts from Metal defaults and lets you toggle
//! individual switches. Getters read the values back so you can confirm what
//! will be compiled.
//!
//! Run with:
//! ```text
//! cargo run -p ex022_compile_options
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void noop() {}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let relaxed = metal_rust::CompileOptions::new();
    relaxed.set_fast_math_enabled(true);
    relaxed.set_logging_enabled(false);
    println!(
        "Custom options   : fast_math={}",
        relaxed.fast_math_enabled()
    );
    let strict = metal_rust::CompileOptions::strict();
    println!(
        "Strict options   : fast_math={}",
        strict.fast_math_enabled()
    );
    match device.new_library_from_source(SOURCE, Some(&strict)) {
        Ok(library) => println!(
            "Compiled with strict options: {} functions",
            library.function_names().len()
        ),
        Err(error) => println!("Strict compilation failed: {error}"),
    }
}

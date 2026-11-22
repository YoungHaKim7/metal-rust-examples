//! Example 024 — Function names, types and constants.
//!
//! `Function` exposes metadata without running anything: its name, its
//! `FunctionType` (vertex, fragment, kernel) and the function constants the
//! source declares. Function constants let one shader serve many variants.
//!
//! Run with:
//! ```text
//! cargo run -p ex024_function_metadata
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;
use metal_rust::metal::FunctionType;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

constant bool use_correction [[function_constant(0)]];

kernel void tune(device float* out [[buffer(0)]],
                 uint id [[thread_position_in_grid]]) {
    float value = 1.0;
    if (use_correction) {
        value = 0.5;
    }
    out[id] = value;
}
"#;

fn type_name(function_type: FunctionType) -> &'static str {
    if function_type == FunctionType::FunctionTypeKernel {
        "kernel"
    } else if function_type == FunctionType::FunctionTypeVertex {
        "vertex"
    } else if function_type == FunctionType::FunctionTypeFragment {
        "fragment"
    } else {
        "other"
    }
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("tune").expect("tune exists");
    println!("Function name      : {}", function.name());
    println!(
        "Function type      : {}",
        type_name(function.function_type())
    );
    let constants = function.function_constant_names();
    println!("Function constants : {constants:?}");
}

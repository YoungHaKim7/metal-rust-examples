//! Example 077 — metal-cpp version compatibility.
//!
//! metal-rust tracks Apple's metal-cpp release numbers. The METAL_CPP_VERSION
//! constants state which reference this build follows, and
//! metal_cpp_supports_version() answers "is this runtime at least version X?"
//! with lexicographic comparison.
//!
//! Run with:
//! ```text
//! cargo run -p ex077_metal_cpp_version
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::{
    METAL_CPP_VERSION_MAJOR, METAL_CPP_VERSION_MINOR, METAL_CPP_VERSION_PATCH,
    metal_cpp_supports_version,
};

fn main() {
    println!(
        "Built against metal-cpp {}.{}.{}",
        METAL_CPP_VERSION_MAJOR, METAL_CPP_VERSION_MINOR, METAL_CPP_VERSION_PATCH
    );
    let current = metal_cpp_supports_version(
        METAL_CPP_VERSION_MAJOR,
        METAL_CPP_VERSION_MINOR,
        METAL_CPP_VERSION_PATCH,
    );
    println!("Supports own version: {current}");
    println!(
        "Supports an older reference: {}",
        metal_cpp_supports_version(METAL_CPP_VERSION_MAJOR - 1, u32::MAX, u32::MAX)
    );
    println!(
        "Supports a newer reference: {}",
        metal_cpp_supports_version(
            METAL_CPP_VERSION_MAJOR,
            METAL_CPP_VERSION_MINOR,
            METAL_CPP_VERSION_PATCH + 1
        )
    );
}

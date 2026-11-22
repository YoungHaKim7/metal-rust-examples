//! Example 084 — PackedFloat3, 4x3 and quaternion.
//!
//! MSL's packed vector types map to checked Rust structs: PackedFloat3
//! indexes like an array, PackedFloat4x3 stores column-major matrices, and
//! PackedFloatQuaternion keeps rotations compact for instance buffers.
//!
//! Run with:
//! ```text
//! cargo run -p ex084_packed_float_math
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::{PackedFloat3, PackedFloat4x3, PackedFloatQuaternion};

fn main() {
    let mut vector = PackedFloat3::from_components(1.0, 2.0, 3.0);
    println!("Vector component 1: {}", vector[1]);
    vector[1] = 4.0;
    println!("Vector after write: {:?}", vector.into_array());
    let matrix = PackedFloat4x3::new([
        PackedFloat3::new([1.0, 0.0, 0.0]),
        PackedFloat3::new([0.0, 1.0, 0.0]),
        PackedFloat3::new([0.0, 0.0, 1.0]),
        PackedFloat3::new([4.0, 5.0, 6.0]),
    ]);
    println!("Translation column: {:?}", matrix.columns[3].into_array());
    let identity_quaternion = PackedFloatQuaternion::from_components(0.0, 0.0, 0.0, 1.0);
    println!("Identity quaternion w: {}", identity_quaternion[3]);
    println!("These types were designed to match MSL memory layouts exactly.");
}

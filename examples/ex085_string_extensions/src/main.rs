//! Example 085 — StringExt and Array helpers.
//!
//! Small conveniences make Cocoa strings ergonomic: `character(index)`
//! returns an Option<char> instead of panicking, and `Array<T>` behaves like
//! a Vec while owned by the Foundation world.
//!
//! Run with:
//! ```text
//! cargo run -p ex085_string_extensions
//! ```

#![forbid(unsafe_code)]

use metal_rust::foundation::{Array, StringExt};

fn main() {
    let framework = "Metal";
    println!("framework.character(1) = {:?}", framework.character(1));
    println!("framework.character(99) = {:?}", framework.character(99));
    let values: Array<u32> = vec![10, 20, 30].into();
    println!("Array length: {}", values.len());
    let cloned = values.clone();
    let total: u32 = cloned.into_iter().sum();
    println!("Array sum via into_iter: {total}");
}

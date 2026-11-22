//! Example 082 — Number, URL and error domains.
//!
//! Foundation bridges Rust primitives into Cocoa types: Number wraps numeric
//! values, URL models file locations, and COCOA_ERROR_DOMAIN names the error
//! domain every NSError uses.
//!
//! Run with:
//! ```text
//! cargo run -p ex082_foundation_values
//! ```

#![forbid(unsafe_code)]

use std::env;

use metal_rust::foundation::{COCOA_ERROR_DOMAIN, Number, URL};

fn main() {
    let number = Number::from(42_u64);
    println!("Number as u64 : {}", number.as_u64());
    let path = env::temp_dir().join("metal-rust-example-082");
    let url = URL::from_file_path(&path).expect("UTF-8 temporary path");
    println!("URL file path : {}", url.file_path().display());
    println!("Cocoa domain  : {}", COCOA_ERROR_DOMAIN.as_str());
    let _ = std::fs::remove_file(&path);
}

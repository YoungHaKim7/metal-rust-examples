//! Example 076 — Capture scopes for devices.
//!
//! Capture scopes group commands under one name in the trace UI.
//! `new_scope_for_device()` builds one, and `with_scope()` runs your closure
//! inside it, returning the closure's value.
//!
//! Run with:
//! ```text
//! cargo run -p ex076_capture_scopes
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    match metal_rust::CaptureSession::new_scope_for_device(&device) {
        Ok(scope) => {
            let answer = scope.with_scope(|| 6 * 7).expect("scope closure runs");
            println!("with_scope returned {answer} for the capture scope.");
            println!(
                "Scope device: {}",
                scope.device().expect("scope device").name()
            );
        }
        Err(error) => println!("Capture scopes unavailable here: {error}"),
    }
}

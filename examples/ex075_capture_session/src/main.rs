//! Example 075 — GPU trace capture to a file.
//!
//! CaptureSession records a .gputrace you can open in Xcode. The path must be
//! absolute; capture is runtime-gated, so older systems return Unsupported
//! and the example exits cleanly.
//!
//! Run with:
//! ```text
//! cargo run -p ex075_capture_session
//! ```

#![forbid(unsafe_code)]

use std::path::PathBuf;

use metal_rust::{Device, ErrorKind};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let path = PathBuf::from("/tmp").join(format!("metal-rust-{}.gputrace", std::process::id()));
    let session = match metal_rust::CaptureSession::start(&device, &path) {
        Ok(session) => session,
        Err(error) if error.kind() == ErrorKind::Unsupported => {
            println!("GPU capture is not supported on this runtime; skipping.");
            return;
        }
        Err(error) => {
            println!("Capture failed to start: {error}");
            return;
        }
    };
    println!("Capturing to {}", path.display());
    session.finish();
    println!("Capture session finished; open the trace file in Xcode.");
    let _ = std::fs::remove_file(&path);
}

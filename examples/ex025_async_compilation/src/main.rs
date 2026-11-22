//! Example 025 — Asynchronous library compilation.
//!
//! Compiling large shaders takes time. `new_library_from_source_async()`
//! starts the compile off-thread and calls your handler once with the
//! `Result<Library, Error>`. The handler must be `Send + 'static`; an mpsc
//! channel hands the Library back to the main thread.
//!
//! Run with:
//! ```text
//! cargo run -p ex025_async_compilation
//! ```

#![forbid(unsafe_code)]

use std::{
    sync::mpsc::{Receiver, channel},
    time::Duration,
};

use metal_rust::Device;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void async_fill(device float* out [[buffer(0)]],
                       uint id [[thread_position_in_grid]]) {
    out[id] = 0.5;
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    // The Library is !Send, so the handler consumes it on its own thread and
    // reports the outcome as a plain String across the channel.
    let (sender, receiver): (_, Receiver<String>) = channel();
    device
        .new_library_from_source_async(SOURCE, None, move |result| {
            let message = match result {
                Ok(library) => {
                    let functions = library.function_names().len();
                    format!("compiled a library with {functions} functions")
                }
                Err(error) => format!("compilation failed: {error}"),
            };
            let _ = sender.send(message);
        })
        .expect("async compile started");
    println!("Compilation started; doing other work while Metal compiles...");
    match receiver.recv_timeout(Duration::from_secs(30)) {
        Ok(message) => println!("Async result: {message}"),
        Err(timeout) => println!("Compilation did not finish in time: {timeout}"),
    }
}

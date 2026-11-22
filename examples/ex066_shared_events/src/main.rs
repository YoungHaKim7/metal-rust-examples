//! Example 066 — Shareable events and handles.
//!
//! SharedEvents can cross process boundaries through a SharedEventHandle.
//! This example creates one, exports the handle, and polls
//! wait_until_signaled_value() with a short timeout — nobody signals it, so
//! the poll times out and reports false.
//!
//! Run with:
//! ```text
//! cargo run -p ex066_shared_events
//! ```

#![forbid(unsafe_code)]

use std::time::Duration;

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let shared_event = device.new_shared_event().expect("shared event");
    // The handle stays alive for this scope; dropping it releases the export.
    let _handle = shared_event.new_shared_event_handle().expect("handle");
    println!("Exported a SharedEventHandle that another process can import.");
    let signaled = shared_event
        .wait_until_signaled_value(42, Duration::from_millis(200))
        .expect("wait call itself succeeds");
    println!("Signaled after 200 ms: {signaled}");
    if !signaled {
        println!("Nothing signaled value 42, so the poll returned false — as designed.");
    }
}

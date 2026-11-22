//! Example 073 — Device counter sets.
//!
//! `counter_sets()` lists the named counter groups the hardware supports
//! (timestamps, stage utilization, statistics). The safe API returns owned
//! values; the count alone tells you what the driver exposes.
//!
//! Run with:
//! ```text
//! cargo run -p ex073_counter_sets
//! ```

#![forbid(unsafe_code)]

use metal_rust::Device;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    match device.counter_sets() {
        Ok(sets) => println!("The device reports {} counter set(s).", sets.len()),
        Err(error) => println!("Counter sets unavailable: {error}"),
    }
    println!("Counter sets feed sample buffers and Instruments profiles.");
}

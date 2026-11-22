//! Example 072 — MAX_TIMESTAMP_COUNTERS bounds.
//!
//! Heap sizes are checked in Rust first: zero is invalid and anything above
//! MAX_TIMESTAMP_COUNTERS is invalid. Both mistakes return InvalidArgument
//! errors and never reach the driver.
//!
//! Run with:
//! ```text
//! cargo run -p ex072_counter_heap_limits
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ErrorKind, MAX_TIMESTAMP_COUNTERS};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    println!("MAX_TIMESTAMP_COUNTERS = {MAX_TIMESTAMP_COUNTERS}");
    let zero = device
        .new_timestamp_counter_heap(0)
        .err()
        .expect("zero is rejected");
    println!(
        "Heap of 0 counters -> {:?}",
        zero.kind() == ErrorKind::InvalidArgument
    );
    let too_large = device
        .new_timestamp_counter_heap(MAX_TIMESTAMP_COUNTERS + 1)
        .err()
        .expect("over-limit is rejected");
    println!(
        "Heap of MAX+1 counters -> {:?}",
        too_large.kind() == ErrorKind::InvalidArgument
    );
    println!("Both bounds were enforced in safe Rust, before Metal.");
}

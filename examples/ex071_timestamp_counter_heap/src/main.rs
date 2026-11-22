//! Example 071 — Metal 4 counter heaps.
//!
//! Metal 4 replaces per-encoder counters with timestamp counter heaps.
//! `new_timestamp_counter_heap(count)` allocates one; the facade rejects zero
//! counts before Metal is consulted and returns `Unsupported` on older
//! runtimes.
//!
//! Run with:
//! ```text
//! cargo run -p ex071_timestamp_counter_heap
//! ```

#![forbid(unsafe_code)]

use metal_rust::{Device, ErrorKind};

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    match device.new_timestamp_counter_heap(2) {
        Ok(heap) => {
            println!(
                "Allocated a Metal 4 counter heap holding {} counters.",
                heap.count()
            );
        }
        Err(error) if error.kind() == ErrorKind::Unsupported => {
            println!(
                "This runtime does not expose Metal 4 counter heaps; example ends gracefully."
            );
        }
        Err(error) => println!("Unexpected heap error: {error}"),
    }
}

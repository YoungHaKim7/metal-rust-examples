//! Example 037 — Device atomics in kernels.
//!
//! Atomics let thousands of threads update one counter without races. The
//! kernel does `atomic_fetch_add_explicit` once per thread; after the
//! submission the readback must equal the thread count exactly.
//!
//! Run with:
//! ```text
//! cargo run -p ex037_atomic_counters
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const THREADS: usize = 1024;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void tally(device atomic_uint* counter [[buffer(0)]],
                  uint id [[thread_position_in_grid]]) {
    atomic_fetch_add_explicit(counter, 1u, memory_order_relaxed);
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("tally").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let counter = device
        .new_buffer(4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&counter, 0, 0).expect("bind counter");
        encoder.dispatch_1d(THREADS, 32).expect("dispatch");
        encoder.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&counter, 0, 4).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let bytes = completed.resolve_buffer(readback).expect("resolve");
    let total = u32::from_ne_bytes(bytes.try_into().expect("4 bytes"));
    println!("{THREADS} threads incremented one atomic counter.");
    println!("Final value: {total}");
    assert_eq!(total as usize, THREADS);
}

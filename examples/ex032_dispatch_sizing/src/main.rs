//! Example 032 — dispatch_1d with threads per group.
//!
//! `dispatch_1d(threads, threads_per_group)` launches a one-dimensional grid.
//! The kernel records `thread_position_in_grid` and its threadgroup so you can
//! see how Metal laid the work out. 1024 threads with 32 per group make 32
//! threadgroups.
//!
//! Run with:
//! ```text
//! cargo run -p ex032_dispatch_sizing
//! ```

#![forbid(unsafe_code)]

use metal_rust::{ComputePassDescriptor, Device, ResourceOptions};

const THREADS: usize = 1024;
const THREADS_PER_GROUP: usize = 32;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void record(device uint* grid_ids [[buffer(0)]],
                   device uint* group_ids [[buffer(1)]],
                   uint id [[thread_position_in_grid]],
                   uint group [[threadgroup_position_in_grid]]) {
    grid_ids[id] = id;
    group_ids[id] = group;
}
"#;

fn decode_u32(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_ne_bytes(chunk.try_into().expect("4-byte chunk")))
        .collect()
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let function = library.function("record").expect("function exists");
    let pipeline = device.new_compute_pipeline(&function).expect("pipeline");
    let grid_ids = device
        .new_buffer(THREADS * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let group_ids = device
        .new_buffer(THREADS * 4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&pipeline);
        encoder.set_buffer(&grid_ids, 0, 0).expect("bind");
        encoder.set_buffer(&group_ids, 0, 1).expect("bind");
        encoder
            .dispatch_1d(THREADS, THREADS_PER_GROUP)
            .expect("dispatch");
        encoder.end_encoding();
    }
    let read_grid = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_buffer(&grid_ids, 0, THREADS * 4)
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let read_group = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit
            .read_buffer(&group_ids, 0, THREADS * 4)
            .expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let grids = decode_u32(&completed.resolve_buffer(read_grid).expect("resolve"));
    let groups = decode_u32(&completed.resolve_buffer(read_group).expect("resolve"));
    println!("Thread 0  -> grid id {}, group {}", grids[0], groups[0]);
    println!("Thread 33 -> grid id {}, group {}", grids[33], groups[33]);
    println!(
        "Thread 1023 -> grid id {}, group {}",
        grids[1023], groups[1023]
    );
    println!(
        "Expected {} threadgroups for {} / {}.",
        THREADS / THREADS_PER_GROUP,
        THREADS,
        THREADS_PER_GROUP
    );
}

//! Example 064 — encode_wait and encode_signal_event.
//!
//! Events carry values between command encoders. Pass one signals the event;
//! pass two waits for it before its dispatch runs. Both halves live in one
//! command buffer, and the GPU orders them through the event.
//!
//! Run with:
//! ```text
//! cargo run -p ex064_encode_events
//! ```

#![forbid(unsafe_code)]

use metal_rust::metal::ComputePassDescriptor;
use metal_rust::{Device, ResourceOptions};

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void write_flag(device uint* out [[buffer(0)]],
                       uint id [[thread_position_in_grid]]) {
    if (id == 0) {
        out[0] = 7;
    }
}

kernel void double_it(device const uint* in [[buffer(0)]],
                      device uint* out [[buffer(1)]],
                      uint id [[thread_position_in_grid]]) {
    if (id == 0) {
        out[0] = in[0] * 2u;
    }
}
"#;

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let event = device.new_event().expect("event");
    event.set_label("examples/flag-event").expect("event label");
    let library = device
        .new_library_from_source(SOURCE, None)
        .expect("compiles");
    let write_fn = library.function("write_flag").expect("write exists");
    let double_fn = library.function("double_it").expect("double exists");
    let write_pipeline = device.new_compute_pipeline(&write_fn).expect("pipeline");
    let double_pipeline = device.new_compute_pipeline(&double_fn).expect("pipeline");
    let scratch = device
        .new_buffer(4, ResourceOptions::SHARED)
        .expect("buffer");
    let result = device
        .new_buffer(4, ResourceOptions::SHARED)
        .expect("buffer");
    let queue = device.new_command_queue(None).expect("queue");
    let mut command_buffer = queue.command_buffer().expect("command buffer");
    {
        let encoder = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        encoder.set_pipeline(&write_pipeline);
        encoder.set_buffer(&scratch, 0, 0).expect("bind");
        encoder.dispatch_1d(1, 1).expect("dispatch");
        encoder.end_encoding();
        command_buffer
            .encode_signal_event(&event, 1)
            .expect("signal");
        command_buffer.encode_wait(&event, 1).expect("wait");
        let second = command_buffer
            .compute_encoder(&ComputePassDescriptor::new())
            .expect("encoder");
        second.set_pipeline(&double_pipeline);
        second.set_buffer(&scratch, 0, 0).expect("bind");
        second.set_buffer(&result, 0, 1).expect("bind");
        second.dispatch_1d(1, 1).expect("dispatch");
        second.end_encoding();
    }
    let readback = {
        let blit = command_buffer.blit_encoder().expect("blit");
        let readback = blit.read_buffer(&result, 0, 4).expect("readback");
        blit.end_encoding();
        readback
    };
    let completed = command_buffer
        .commit()
        .wait()
        .expect("submission completes");
    let bytes = completed.resolve_buffer(readback).expect("resolve");
    let doubled = u32::from_ne_bytes(bytes.try_into().expect("4 bytes"));
    println!("Event-ordered result: scratch=7 -> doubled={doubled}");
    assert_eq!(doubled, 14);
}

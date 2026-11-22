//! Example 068 — CommandBufferStatus across a submission.
//!
//! A command buffer walks through NotEnqueued, Enqueued, Committed, Scheduled
//! and finally Completed (or Error). The facade splits those states across
//! types: `CommandBuffer` before commit, `SubmittedCommandBuffer` after, and
//! `CompletedCommandBuffer` after wait().
//!
//! Run with:
//! ```text
//! cargo run -p ex068_status_transitions
//! ```

#![forbid(unsafe_code)]

use metal_rust::{CommandBufferStatus, Device};

fn status_name(status: CommandBufferStatus) -> &'static str {
    match status {
        CommandBufferStatus::NotEnqueued => "not enqueued",
        CommandBufferStatus::Enqueued => "enqueued",
        CommandBufferStatus::Committed => "committed",
        CommandBufferStatus::Scheduled => "scheduled",
        CommandBufferStatus::Completed => "completed",
        CommandBufferStatus::Error => "error",
        CommandBufferStatus::Unknown(_) => "unknown",
    }
}

fn main() {
    let Some(device) = Device::system_default() else {
        println!("No Metal device found; this example requires macOS with Metal support.");
        return;
    };
    let queue = device.new_command_queue(None).expect("queue");
    let command_buffer = queue.command_buffer().expect("command buffer");
    println!("Before commit  : {}", status_name(command_buffer.status()));
    let submitted = command_buffer.commit();
    println!("After commit   : {}", status_name(submitted.status()));
    let completed = submitted.wait().expect("submission completes");
    println!("After wait     : {}", status_name(completed.status()));
    assert_eq!(completed.status(), CommandBufferStatus::Completed);
    println!("The type changes mark the transitions as tightly as the values do.");
}

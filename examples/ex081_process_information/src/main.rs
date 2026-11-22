//! Example 081 — NSProcessInfo through Foundation.
//!
//! The foundation module wraps Cocoa essentials without Objective-C. This
//! example reads the process identifier, processor counts and physical memory
//! through the safe ProcessInfo facade.
//!
//! Run with:
//! ```text
//! cargo run -p ex081_process_information
//! ```

#![forbid(unsafe_code)]

use metal_rust::foundation::ProcessInfo;

fn main() {
    let process = ProcessInfo::current();
    println!("Process id          : {}", process.process_identifier());
    println!("Processor count     : {}", process.processor_count());
    println!("Active processors   : {}", process.active_processor_count());
    println!("Physical memory     : {} bytes", process.physical_memory());
    println!("The same values NSProcessInfo would give you, without leaving Rust.");
}

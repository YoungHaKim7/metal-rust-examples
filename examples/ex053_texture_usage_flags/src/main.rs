//! Example 053 — TextureUsage bit flags.
//!
//! TextureUsage is a checked bit set. `union()` combines hints, and the
//! UNKNOWN flag is the absorbing element: unioning anything with UNKNOWN
//! yields UNKNOWN, which is how the wrapper keeps invalid states detectable.
//!
//! Run with:
//! ```text
//! cargo run -p ex053_texture_usage_flags
//! ```

#![forbid(unsafe_code)]

use metal_rust::TextureUsage;

fn main() {
    let shader_read = TextureUsage::SHADER_READ;
    let render_target = TextureUsage::RENDER_TARGET;
    let shader_write = TextureUsage::SHADER_WRITE;
    let combined = shader_read.union(render_target).union(shader_write);
    println!(
        "SHADER_READ | RENDER_TARGET | SHADER_WRITE = raw {}",
        combined.as_raw()
    );
    println!(
        "UNKNOWN absorbs everything: {}",
        shader_read.union(TextureUsage::UNKNOWN) == TextureUsage::UNKNOWN
    );
    println!(
        "RENDER_TARGET alone stays itself: {}",
        render_target.union(TextureUsage::UNKNOWN) == TextureUsage::UNKNOWN
    );
    let descriptor_ready = shader_read.union(render_target);
    println!(
        "A render target you also sample from needs usage raw value {}.",
        descriptor_ready.as_raw()
    );
}

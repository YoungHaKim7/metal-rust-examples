//! Example 083 — NSNotificationCenter observers.
//!
//! NotificationCenter delivers named notifications. `add_observer()` returns
//! a Registration whose Drop unregisters the observer — RAII replaces manual
//! removeObserver calls.
//!
//! Run with:
//! ```text
//! cargo run -p ex083_notification_center
//! ```

#![forbid(unsafe_code)]

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use metal_rust::foundation::NotificationCenter;

fn main() {
    let center = NotificationCenter::default_center();
    let deliveries = Arc::new(AtomicUsize::new(0));
    let callback_deliveries = Arc::clone(&deliveries);
    let registration = center.add_observer("MetalRustExample083", move |notification| {
        println!("Received notification: {}", notification.name());
        callback_deliveries.fetch_add(1, Ordering::SeqCst);
    });
    center.post("MetalRustExample083");
    println!(
        "Deliveries while registered: {}",
        deliveries.load(Ordering::SeqCst)
    );
    drop(registration);
    center.post("MetalRustExample083");
    println!(
        "Deliveries after unregister: {} (drop removed the observer)",
        deliveries.load(Ordering::SeqCst)
    );
}

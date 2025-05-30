//! Framework adapters for css-in-rust
//!
//! This module provides adapters for different UI frameworks,
//! allowing seamless integration with their styling systems.

#[cfg(feature = "dioxus")]
pub mod dioxus;

#[cfg(feature = "dioxus")]
pub use self::dioxus::*;

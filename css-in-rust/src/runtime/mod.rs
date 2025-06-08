//! Runtime CSS management functionality
//!
//! This module provides runtime style injection and management capabilities.

pub mod injector;
pub mod manager;
pub mod provider;

pub use injector::InjectionEnvironment;
pub use injector::{InjectionError, StyleInjector};
pub use manager::{StyleManager, StyleManagerConfig};
pub use provider::{
    clear_all_styles, current_environment, generate_style_html, init, init_with_provider,
    remove_style,
};
pub use provider::{inject_style, ProviderType, StyleProvider};

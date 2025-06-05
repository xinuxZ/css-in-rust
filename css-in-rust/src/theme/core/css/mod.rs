pub mod dependency;
pub mod generator;
pub mod variables;

// Re-exports
pub use dependency::{Dependency, DependencyGraph, DependencyTracker, DependencyType};
pub use generator::CssGenerator;
pub use variables::{CssVariables, InjectionStrategy};

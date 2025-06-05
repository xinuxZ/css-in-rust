pub mod css_generator;
pub mod definitions;
pub mod resolver;
pub mod simple_system;
pub mod system;
pub mod values;

// Re-export commonly used types
pub use definitions::{
    ColorValue, DimensionUnit, DimensionValue, TokenMetadata, TokenReference, TokenType, TokenValue,
};
pub use simple_system::{ThemeVariant, TokenSystem};
pub use values::{DesignTokens, TokenStore, TokenValuesImpl};

// Common traits
pub trait TokenDefinitions {
    fn get_token_value(&self, path: &str) -> Option<TokenValue>;
    fn set_token_value(&mut self, path: &str, value: TokenValue);
    fn get_metadata(&self, path: &str) -> Option<TokenMetadata>;
}

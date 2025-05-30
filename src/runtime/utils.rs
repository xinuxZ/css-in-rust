//! Utility functions for CSS-in-Rust runtime
//!
//! This module provides utility functions for class name generation,
//! hashing, and validation.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a unique class name based on CSS content
///
/// # Arguments
/// * `css_content` - The CSS content to generate a class name for
///
/// # Returns
/// A unique class name in the format "css-{hash}"
///
/// # Examples
/// ```
/// use css_in_rust::runtime::utils::generate_class_name;
///
/// let css = ".test { color: red; }";
/// let class_name = generate_class_name(css);
/// assert!(class_name.starts_with("css-"));
/// ```
pub fn generate_class_name(css_content: &str) -> String {
    let hash = generate_hash(css_content);
    format!("css-{:08x}", hash)
}

/// Generate a hash for the given content
///
/// # Arguments
/// * `content` - The content to hash
///
/// # Returns
/// A 64-bit hash value
///
/// # Examples
/// ```
/// use css_in_rust::runtime::utils::generate_hash;
///
/// let content = "some content";
/// let hash1 = generate_hash(content);
/// let hash2 = generate_hash(content);
/// assert_eq!(hash1, hash2); // Same content produces same hash
/// ```
pub fn generate_hash(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

/// Check if a class name is valid according to CSS standards
///
/// # Arguments
/// * `class_name` - The class name to validate
///
/// # Returns
/// `true` if the class name is valid, `false` otherwise
///
/// # Examples
/// ```
/// use css_in_rust::runtime::utils::is_valid_class_name;
///
/// assert!(is_valid_class_name("valid-class"));
/// assert!(!is_valid_class_name("123invalid"));
/// ```
pub fn is_valid_class_name(class_name: &str) -> bool {
    if class_name.is_empty() {
        return false;
    }

    let first_char = class_name.chars().next().unwrap();
    if first_char.is_ascii_digit() || first_char == '.' || first_char == '#' {
        return false;
    }

    class_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Sanitize a class name to make it valid
///
/// # Arguments
/// * `class_name` - The class name to sanitize
///
/// # Returns
/// A sanitized class name that is valid according to CSS standards
///
/// # Examples
/// ```
/// use css_in_rust::runtime::utils::sanitize_class_name;
///
/// assert_eq!(sanitize_class_name("123invalid"), "_23invalid");
/// assert_eq!(sanitize_class_name(".invalid"), "_invalid");
/// ```
pub fn sanitize_class_name(class_name: &str) -> String {
    if class_name.is_empty() {
        return "css".to_string();
    }

    let mut result = String::new();
    let mut chars = class_name.chars();

    // Handle first character
    if let Some(first_char) = chars.next() {
        if first_char.is_ascii_digit() || first_char == '.' || first_char == '#' {
            result.push('_');
            if first_char != '.' && first_char != '#' {
                result.push(first_char);
            }
        } else if first_char.is_ascii_alphanumeric() || first_char == '-' || first_char == '_' {
            result.push(first_char);
        } else {
            result.push('_');
        }
    }

    // Handle remaining characters
    for c in chars {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
            result.push(c);
        } else {
            result.push('_');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_class_name() {
        let css = ".test { color: red; }";
        let class_name = generate_class_name(css);

        assert!(class_name.starts_with("css-"));
        assert_eq!(class_name.len(), 12); // "css-" + 8 hex chars

        // Same CSS should generate same class name
        let class_name2 = generate_class_name(css);
        assert_eq!(class_name, class_name2);
    }

    #[test]
    fn test_generate_hash() {
        let content = "test content";
        let hash1 = generate_hash(content);
        let hash2 = generate_hash(content);
        assert_eq!(hash1, hash2);

        let hash3 = generate_hash("different content");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_is_valid_class_name() {
        assert!(is_valid_class_name("valid-class"));
        assert!(is_valid_class_name("_valid"));
        assert!(is_valid_class_name("valid123"));

        assert!(!is_valid_class_name("123invalid"));
        assert!(!is_valid_class_name(""));
        assert!(!is_valid_class_name(".invalid"));
        assert!(!is_valid_class_name("#invalid"));
    }

    #[test]
    fn test_sanitize_class_name() {
        assert_eq!(sanitize_class_name("valid-class"), "valid-class");
        assert_eq!(sanitize_class_name("123invalid"), "_23invalid");
        assert_eq!(sanitize_class_name(".invalid"), "_invalid");
        assert_eq!(sanitize_class_name("#invalid"), "_invalid");
        assert_eq!(sanitize_class_name(""), "css");
        assert_eq!(sanitize_class_name("test@class"), "test_class");
    }
}

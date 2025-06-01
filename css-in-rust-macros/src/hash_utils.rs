use sha2::{Digest, Sha256};

/// Calculate SHA-256 hash of CSS content for unique class names
pub fn calculate_css_hash(css: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(css.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

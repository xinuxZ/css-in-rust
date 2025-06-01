use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Global CSS cache for storing processed CSS classes
static CSS_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

/// Initialize the CSS cache
fn get_css_cache() -> &'static Mutex<HashMap<String, String>> {
    CSS_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Check if CSS is already cached and return the class name if found
pub fn get_cached_css(css_hash: &str) -> Option<String> {
    if let Ok(cache) = get_css_cache().lock() {
        cache.get(css_hash).cloned()
    } else {
        None
    }
}

/// Cache the CSS with its hash and class name
pub fn cache_css(css_hash: String, class_name: String) {
    if let Ok(mut cache) = get_css_cache().lock() {
        cache.insert(css_hash, class_name);
    }
}

/// Clear CSS cache (useful for testing)
#[allow(dead_code)]
pub fn clear_css_cache() {
    let cache = CSS_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut cache_guard) = cache.lock() {
        cache_guard.clear();
    }
}

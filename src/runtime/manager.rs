//! Global style manager
//!
//! This module provides a global style manager that coordinates style injection,
//! caching, and lifecycle management across the application.

use crate::core::{CssError, Result};
use crate::runtime::provider::{StyleProvider, StyleProviderFactory, StyleProviderInfo};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Global style manager instance
static GLOBAL_MANAGER: OnceLock<Arc<StyleManager>> = OnceLock::new();

/// Style manager configuration
#[derive(Debug, Clone)]
pub struct StyleManagerConfig {
    /// Enable style caching
    pub enable_cache: bool,
    /// Maximum number of cached styles
    pub max_cache_size: usize,
    /// Enable style deduplication
    pub enable_deduplication: bool,
    /// Enable development mode features
    pub dev_mode: bool,
    /// Custom provider type
    pub provider_type: Option<crate::runtime::provider::ProviderType>,
}

impl Default for StyleManagerConfig {
    fn default() -> Self {
        Self {
            enable_cache: true,
            max_cache_size: 1000,
            enable_deduplication: true,
            dev_mode: cfg!(debug_assertions),
            provider_type: None,
        }
    }
}

/// Information about a managed style
#[derive(Debug, Clone)]
pub struct StyleInfo {
    /// Unique identifier for the style
    pub id: String,
    /// Original CSS content
    pub css: String,
    /// Generated class name
    pub class_name: String,
    /// Hash of the CSS content
    pub hash: String,
    /// Whether the style is currently injected
    pub is_injected: bool,
    /// Reference count for this style
    pub ref_count: usize,
    /// Timestamp when the style was created
    pub created_at: std::time::SystemTime,
    /// Timestamp when the style was last used
    pub last_used: std::time::SystemTime,
}

/// Style manager for coordinating style injection and caching
pub struct StyleManager {
    /// Style provider for the current environment
    provider: Box<dyn StyleProvider + Send + Sync>,
    /// Cache of managed styles
    styles: Mutex<HashMap<String, StyleInfo>>,
    /// Configuration
    config: StyleManagerConfig,
    /// Style hash to ID mapping for deduplication
    hash_to_id: Mutex<HashMap<String, String>>,
}

impl StyleManager {
    /// Create a new style manager with default configuration
    pub fn new() -> Self {
        Self::with_config(StyleManagerConfig::default())
    }

    /// Create a new style manager with custom configuration
    pub fn with_config(config: StyleManagerConfig) -> Self {
        let provider = if let Some(provider_type) = &config.provider_type {
            StyleProviderFactory::create_by_type(provider_type.clone())
                .unwrap_or_else(|_| StyleProviderFactory::create_default())
        } else {
            StyleProviderFactory::create_default()
        };

        Self {
            provider,
            styles: Mutex::new(HashMap::new()),
            config,
            hash_to_id: Mutex::new(HashMap::new()),
        }
    }

    /// Inject styles and return the generated class name
    pub fn inject_style(&self, css: &str) -> Result<String> {
        let hash = self.calculate_hash(css);

        // Check for existing style with same hash if deduplication is enabled
        if self.config.enable_deduplication {
            if let Ok(hash_map) = self.hash_to_id.lock() {
                if let Some(existing_id) = hash_map.get(&hash) {
                    if let Ok(mut styles) = self.styles.lock() {
                        if let Some(style_info) = styles.get_mut(existing_id) {
                            style_info.ref_count += 1;
                            style_info.last_used = std::time::SystemTime::now();
                            return Ok(style_info.class_name.clone());
                        }
                    }
                }
            }
        }

        // Generate unique ID and class name
        let id = format!("css_{}", &hash[..8]);
        let class_name = format!("css-{}", &hash[..8]);

        // Inject the style
        let scoped_css = self.scope_css(css, &class_name)?;
        self.provider.inject_styles(&scoped_css)?;

        // Store style information
        let now = std::time::SystemTime::now();
        let style_info = StyleInfo {
            id: id.clone(),
            css: css.to_string(),
            class_name: class_name.clone(),
            hash: hash.clone(),
            is_injected: true,
            ref_count: 1,
            created_at: now,
            last_used: now,
        };

        if let Ok(mut styles) = self.styles.lock() {
            // Check cache size limit
            if self.config.enable_cache && styles.len() >= self.config.max_cache_size {
                self.evict_least_used(&mut styles)?;
            }

            styles.insert(id.clone(), style_info);
        }

        // Update hash mapping
        if self.config.enable_deduplication {
            if let Ok(mut hash_map) = self.hash_to_id.lock() {
                hash_map.insert(hash, id);
            }
        }

        Ok(class_name)
    }

    /// Remove a style by its ID
    pub fn remove_style(&self, id: &str) -> Result<()> {
        let mut should_remove = false;

        if let Ok(mut styles) = self.styles.lock() {
            if let Some(style_info) = styles.get_mut(id) {
                style_info.ref_count = style_info.ref_count.saturating_sub(1);

                if style_info.ref_count == 0 {
                    self.provider.remove_styles(id)?;
                    should_remove = true;

                    // Remove from hash mapping
                    if self.config.enable_deduplication {
                        if let Ok(mut hash_map) = self.hash_to_id.lock() {
                            hash_map.remove(&style_info.hash);
                        }
                    }
                }
            }

            if should_remove {
                styles.remove(id);
            }
        }

        Ok(())
    }

    /// Clear all styles
    pub fn clear_all_styles(&self) -> Result<()> {
        self.provider.clear_all_styles()?;

        if let Ok(mut styles) = self.styles.lock() {
            styles.clear();
        }

        if let Ok(mut hash_map) = self.hash_to_id.lock() {
            hash_map.clear();
        }

        Ok(())
    }

    /// Get information about a specific style
    pub fn get_style_info(&self, id: &str) -> Result<Option<StyleInfo>> {
        if let Ok(styles) = self.styles.lock() {
            Ok(styles.get(id).cloned())
        } else {
            Err(CssError::runtime_error("Failed to acquire styles lock"))
        }
    }

    /// Check if a style is currently injected
    pub fn is_style_injected(&self, id: &str) -> bool {
        if let Ok(styles) = self.styles.lock() {
            styles.get(id).map(|info| info.is_injected).unwrap_or(false)
        } else {
            false
        }
    }

    /// Get all managed styles
    pub fn get_all_styles(&self) -> Result<Vec<StyleInfo>> {
        if let Ok(styles) = self.styles.lock() {
            Ok(styles.values().cloned().collect())
        } else {
            Err(CssError::runtime_error("Failed to acquire styles lock"))
        }
    }

    /// Get style manager statistics
    pub fn get_stats(&self) -> Result<StyleManagerStats> {
        let provider_info = self.provider.get_style_info()?;

        if let Ok(styles) = self.styles.lock() {
            let total_ref_count = styles.values().map(|info| info.ref_count).sum();
            let cache_hit_rate = if self.config.enable_deduplication {
                if let Ok(hash_map) = self.hash_to_id.lock() {
                    if total_ref_count > 0 {
                        (total_ref_count - hash_map.len()) as f64 / total_ref_count as f64
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            } else {
                0.0
            };

            Ok(StyleManagerStats {
                total_styles: styles.len(),
                injected_styles: styles.values().filter(|info| info.is_injected).count(),
                total_ref_count,
                cache_hit_rate,
                provider_info,
                config: self.config.clone(),
            })
        } else {
            Err(CssError::runtime_error("Failed to acquire styles lock"))
        }
    }

    /// Calculate hash for CSS content
    fn calculate_hash(&self, css: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(css.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Scope CSS with a class name
    fn scope_css(&self, css: &str, class_name: &str) -> Result<String> {
        // Simple scoping - prepend class name to each selector
        // In a real implementation, this would use a proper CSS parser
        let lines: Vec<&str> = css.lines().collect();
        let mut scoped_css = String::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('@') {
                scoped_css.push_str(line);
                scoped_css.push('\n');
            } else if trimmed.contains('{') {
                // This is a selector line
                let parts: Vec<&str> = trimmed.split('{').collect();
                if parts.len() >= 2 {
                    let selector = parts[0].trim();
                    let rest = parts[1..].join("{");

                    // Scope the selector
                    let scoped_selector = if selector == "&" {
                        format!(".{}", class_name)
                    } else if selector.starts_with('&') {
                        format!(".{}{}", class_name, &selector[1..])
                    } else {
                        format!(".{} {}", class_name, selector)
                    };

                    scoped_css.push_str(&format!("{} {{{}", scoped_selector, rest));
                    scoped_css.push('\n');
                } else {
                    scoped_css.push_str(line);
                    scoped_css.push('\n');
                }
            } else {
                scoped_css.push_str(line);
                scoped_css.push('\n');
            }
        }

        Ok(scoped_css)
    }

    /// Evict least recently used styles from cache
    fn evict_least_used(&self, styles: &mut HashMap<String, StyleInfo>) -> Result<()> {
        if styles.is_empty() {
            return Ok(());
        }

        // Find the least recently used style
        let mut oldest_time = std::time::SystemTime::now();
        let mut oldest_id = String::new();

        for (id, info) in styles.iter() {
            if info.ref_count == 0 && info.last_used < oldest_time {
                oldest_time = info.last_used;
                oldest_id = id.clone();
            }
        }

        if !oldest_id.is_empty() {
            self.provider.remove_styles(&oldest_id)?;
            styles.remove(&oldest_id);

            // Remove from hash mapping
            if self.config.enable_deduplication {
                if let Ok(mut hash_map) = self.hash_to_id.lock() {
                    hash_map.retain(|_, id| id != &oldest_id);
                }
            }
        }

        Ok(())
    }
}

impl Default for StyleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Style manager statistics
#[derive(Debug, Clone)]
pub struct StyleManagerStats {
    /// Total number of managed styles
    pub total_styles: usize,
    /// Number of currently injected styles
    pub injected_styles: usize,
    /// Total reference count across all styles
    pub total_ref_count: usize,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Provider information
    pub provider_info: StyleProviderInfo,
    /// Manager configuration
    pub config: StyleManagerConfig,
}

/// Get the global style manager instance
pub fn get_global_manager() -> &'static Arc<StyleManager> {
    GLOBAL_MANAGER.get_or_init(|| Arc::new(StyleManager::new()))
}

/// Initialize the global style manager with custom configuration
pub fn init_global_manager(config: StyleManagerConfig) -> Result<()> {
    GLOBAL_MANAGER
        .set(Arc::new(StyleManager::with_config(config)))
        .map_err(|_| CssError::runtime_error("Global style manager already initialized"))?;
    Ok(())
}

/// Inject styles using the global manager
pub fn inject_style(css: &str) -> Result<String> {
    get_global_manager().inject_style(css)
}

/// Remove styles using the global manager
pub fn remove_style(id: &str) -> Result<()> {
    get_global_manager().remove_style(id)
}

/// Clear all styles using the global manager
pub fn clear_all_styles() -> Result<()> {
    get_global_manager().clear_all_styles()
}

/// Get style information using the global manager
pub fn get_style_info(id: &str) -> Result<Option<StyleInfo>> {
    get_global_manager().get_style_info(id)
}

/// Check if a style is injected using the global manager
pub fn is_style_injected(id: &str) -> bool {
    get_global_manager().is_style_injected(id)
}

/// Get all styles using the global manager
pub fn get_all_styles() -> Result<Vec<StyleInfo>> {
    get_global_manager().get_all_styles()
}

/// Get style manager statistics
pub fn get_stats() -> Result<StyleManagerStats> {
    get_global_manager().get_stats()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::provider::ProviderType;

    #[test]
    fn test_style_manager_creation() {
        let manager = StyleManager::new();
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_styles, 0);
        assert_eq!(stats.injected_styles, 0);
    }

    #[test]
    fn test_style_injection() {
        let config = StyleManagerConfig {
            provider_type: Some(ProviderType::Memory),
            ..Default::default()
        };
        let manager = StyleManager::with_config(config);

        let css = ".test { color: red; }";
        let class_name = manager.inject_style(css).unwrap();

        assert!(class_name.starts_with("css-"));

        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_styles, 1);
        assert_eq!(stats.injected_styles, 1);
    }

    #[test]
    fn test_style_deduplication() {
        let config = StyleManagerConfig {
            provider_type: Some(ProviderType::Memory),
            enable_deduplication: true,
            ..Default::default()
        };
        let manager = StyleManager::with_config(config);

        let css = ".test { color: red; }";
        let class_name1 = manager.inject_style(css).unwrap();
        let class_name2 = manager.inject_style(css).unwrap();

        assert_eq!(class_name1, class_name2);

        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_styles, 1);
        assert_eq!(stats.total_ref_count, 2);
    }

    #[test]
    fn test_style_removal() {
        let config = StyleManagerConfig {
            provider_type: Some(ProviderType::Memory),
            ..Default::default()
        };
        let manager = StyleManager::with_config(config);

        let css = ".test { color: red; }";
        let class_name = manager.inject_style(css).unwrap();

        let styles = manager.get_all_styles().unwrap();
        assert_eq!(styles.len(), 1);

        let style_id = &styles[0].id;
        manager.remove_style(style_id).unwrap();

        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_styles, 0);
    }

    #[test]
    fn test_css_scoping() {
        let manager = StyleManager::new();

        let css = "color: red; font-size: 16px;";
        let scoped = manager.scope_css(css, "test-class").unwrap();

        assert!(scoped.contains(".test-class"));
    }

    #[test]
    fn test_global_manager() {
        let css = ".global-test { color: blue; }";
        let class_name = inject_style(css).unwrap();

        assert!(class_name.starts_with("css-"));
        assert!(is_style_injected(&class_name));

        let stats = get_stats().unwrap();
        assert!(stats.total_styles > 0);
    }
}

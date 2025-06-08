/// 缓存系统模块
///
/// 本模块提供样式和主题的缓存功能，用于优化性能和减少重复计算。
/// 包括样式缓存、组件缓存和缓存管理器等组件。
///
/// # 子模块
///
/// - `cache_entity`: 缓存实体定义，表示可缓存的不同类型的值
/// - `cache_manager`: 缓存管理器，负责管理和协调不同类型的缓存
/// - `component_cache`: 组件样式缓存，专门用于缓存组件的样式
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::cache::{StyleCache, StyleCacheKey, CachedStyle};
///
/// // 创建样式缓存
/// let mut cache = StyleCache::new();
///
/// // 创建缓存键和值
/// let key = StyleCacheKey {
///     component: "Button".to_string(),
///     variant: Some("primary".to_string()),
///     state: None,
/// };
///
/// let style = CachedStyle {
///     class_name: "btn-primary".to_string(),
///     css: ".btn-primary { color: blue; }".to_string(),
///     variables: vec!["--primary-color".to_string()],
/// };
///
/// // 缓存样式
/// cache.set(key.clone(), style);
///
/// // 获取缓存的样式
/// if let Some(cached_style) = cache.get(&key) {
///     println!("缓存的类名: {}", cached_style.class_name);
///     println!("缓存的 CSS: {}", cached_style.css);
/// }
/// ```
use std::collections::HashMap;
use std::hash::Hash;

pub mod cache_entity;
pub mod cache_manager;
pub mod component_cache;

// Re-exports
/// 从 component_cache 模块重新导出的类型和函数
pub use component_cache::{
    compute_props_hash, compute_style_hash, compute_theme_hash, CacheStats, CachedComponentStyle,
    ComponentCacheKey, ComponentStyleCache,
};

/// 从 cache_entity 模块重新导出的类型和常量
pub use cache_entity::{
    CacheEntity, CacheValue, CssVarCacheValue, StyleCacheValue, TokenCacheValue, CSS_VAR_PREFIX,
    STYLE_PREFIX, TOKEN_PREFIX,
};

/// 从 cache_manager 模块重新导出的类型
pub use cache_manager::{CacheManager, MemoryUsage};

/// 样式缓存键
///
/// 用于唯一标识缓存中的样式项，包括组件名称、变体和状态。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::cache::StyleCacheKey;
///
/// let key = StyleCacheKey {
///     component: "Button".to_string(),
///     variant: Some("primary".to_string()),
///     state: Some("hover".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct StyleCacheKey {
    /// 组件名称
    pub component: String,
    /// 变体名称
    pub variant: Option<String>,
    /// 状态名称
    pub state: Option<String>,
}

/// 缓存的样式
///
/// 表示缓存中存储的样式信息，包括类名、CSS 内容和依赖的变量。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::cache::CachedStyle;
///
/// let style = CachedStyle {
///     class_name: "btn-primary".to_string(),
///     css: ".btn-primary { color: blue; }".to_string(),
///     variables: vec!["--primary-color".to_string()],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct CachedStyle {
    /// CSS 类名
    pub class_name: String,
    /// CSS 样式
    pub css: String,
    /// 依赖的变量
    pub variables: Vec<String>,
}

/// 样式缓存
///
/// 提供样式的缓存存储和检索功能，用于优化性能和减少重复计算。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::cache::{StyleCache, StyleCacheKey, CachedStyle};
///
/// // 创建样式缓存
/// let mut cache = StyleCache::new();
///
/// // 缓存样式
/// let key = StyleCacheKey {
///     component: "Button".to_string(),
///     variant: None,
///     state: None,
/// };
///
/// let style = CachedStyle {
///     class_name: "btn-primary".to_string(),
///     css: ".btn-primary { color: blue; }".to_string(),
///     variables: vec!["--primary-color".to_string()],
/// };
///
/// cache.set(key.clone(), style);
///
/// // 检查缓存命中
/// if let Some(cached_style) = cache.get(&key) {
///     println!("找到缓存的样式: {}", cached_style.class_name);
/// }
/// ```
pub struct StyleCache {
    /// 缓存映射
    cache: HashMap<StyleCacheKey, CachedStyle>,
}

impl StyleCache {
    /// 创建新的样式缓存
    ///
    /// 初始化一个空的样式缓存实例。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `StyleCache` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::StyleCache;
    ///
    /// let cache = StyleCache::new();
    /// ```
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 获取缓存的样式
    ///
    /// 根据提供的键查找缓存中的样式。
    ///
    /// # 参数
    ///
    /// * `key` - 样式缓存键
    ///
    /// # 返回值
    ///
    /// 如果找到缓存的样式，返回 `Some(&CachedStyle)`，否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::{StyleCache, StyleCacheKey};
    ///
    /// let cache = StyleCache::new();
    /// let key = StyleCacheKey {
    ///     component: "Button".to_string(),
    ///     variant: None,
    ///     state: None,
    /// };
    ///
    /// if let Some(style) = cache.get(&key) {
    ///     println!("找到缓存的样式: {}", style.class_name);
    /// } else {
    ///     println!("未找到缓存的样式");
    /// }
    /// ```
    pub fn get(&self, key: &StyleCacheKey) -> Option<&CachedStyle> {
        self.cache.get(key)
    }

    /// 设置缓存的样式
    ///
    /// 将样式添加到缓存中，如果键已存在则替换现有的样式。
    ///
    /// # 参数
    ///
    /// * `key` - 样式缓存键
    /// * `style` - 要缓存的样式
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::{StyleCache, StyleCacheKey, CachedStyle};
    ///
    /// let mut cache = StyleCache::new();
    /// let key = StyleCacheKey {
    ///     component: "Button".to_string(),
    ///     variant: Some("primary".to_string()),
    ///     state: None,
    /// };
    ///
    /// let style = CachedStyle {
    ///     class_name: "btn-primary".to_string(),
    ///     css: ".btn-primary { color: blue; }".to_string(),
    ///     variables: vec!["--primary-color".to_string()],
    /// };
    ///
    /// cache.set(key, style);
    /// ```
    pub fn set(&mut self, key: StyleCacheKey, style: CachedStyle) {
        self.cache.insert(key, style);
    }

    /// 清除缓存
    ///
    /// 移除缓存中的所有样式。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::StyleCache;
    ///
    /// let mut cache = StyleCache::new();
    /// // ... 添加一些缓存项 ...
    ///
    /// // 清除所有缓存
    /// cache.clear();
    /// ```
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// 移除缓存项
    ///
    /// 从缓存中移除指定键的样式。
    ///
    /// # 参数
    ///
    /// * `key` - 要移除的样式的缓存键
    ///
    /// # 返回值
    ///
    /// 如果找到并移除了样式，返回 `Some(CachedStyle)`，否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::{StyleCache, StyleCacheKey};
    ///
    /// let mut cache = StyleCache::new();
    /// // ... 添加一些缓存项 ...
    ///
    /// let key = StyleCacheKey {
    ///     component: "Button".to_string(),
    ///     variant: None,
    ///     state: None,
    /// };
    ///
    /// if let Some(removed_style) = cache.remove(&key) {
    ///     println!("移除了样式: {}", removed_style.class_name);
    /// }
    /// ```
    pub fn remove(&mut self, key: &StyleCacheKey) -> Option<CachedStyle> {
        self.cache.remove(key)
    }
}

impl Default for StyleCache {
    /// 创建默认的样式缓存
    ///
    /// # 返回值
    ///
    /// 返回一个新的默认 `StyleCache` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::StyleCache;
    ///
    /// let cache = StyleCache::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_cache_operations() {
        let mut cache = StyleCache::new();
        let key = StyleCacheKey {
            component: "Button".to_string(),
            variant: None,
            state: None,
        };
        let style = CachedStyle {
            class_name: "btn-primary".to_string(),
            css: ".btn-primary { color: blue; }".to_string(),
            variables: vec!["--primary-color".to_string()],
        };

        // Test set and get
        cache.set(key.clone(), style.clone());
        let cached = cache.get(&key);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().class_name, "btn-primary");

        // Test remove
        cache.remove(&key);
        assert!(cache.get(&key).is_none());

        // Test clear
        cache.set(key.clone(), style);
        assert!(!cache.get(&key).is_none());
        cache.clear();
        assert!(cache.get(&key).is_none());
    }
}

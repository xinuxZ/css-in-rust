use std::collections::HashMap;
use std::hash::Hash;

/// 样式缓存键
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
pub struct StyleCache {
    /// 缓存映射
    cache: HashMap<StyleCacheKey, CachedStyle>,
}

impl StyleCache {
    /// 创建新的样式缓存
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 获取缓存的样式
    pub fn get(&self, key: &StyleCacheKey) -> Option<&CachedStyle> {
        self.cache.get(key)
    }

    /// 设置缓存的样式
    pub fn set(&mut self, key: StyleCacheKey, style: CachedStyle) {
        self.cache.insert(key, style);
    }

    /// 清除缓存
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// 移除缓存项
    pub fn remove(&mut self, key: &StyleCacheKey) -> Option<CachedStyle> {
        self.cache.remove(key)
    }
}

impl Default for StyleCache {
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

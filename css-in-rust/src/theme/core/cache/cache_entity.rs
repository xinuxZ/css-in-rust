use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 缓存前缀常量
pub const STYLE_PREFIX: &str = "style";
pub const TOKEN_PREFIX: &str = "token";
pub const CSS_VAR_PREFIX: &str = "cssvar";

/// 缓存值类型
#[derive(Clone, Debug)]
pub enum CacheValue {
    Style(StyleCacheValue),
    Token(TokenCacheValue),
    CssVar(CssVarCacheValue),
}

/// 样式缓存值
#[derive(Clone, Debug)]
pub struct StyleCacheValue {
    pub style_str: String,
    pub style_id: String,
    pub effect_style: Option<String>,
    pub order: i32,
}

/// 令牌缓存值
#[derive(Clone, Debug)]
pub struct TokenCacheValue {
    pub token_key: String,
    pub token_hash: String,
    pub token_data: Value,
}

/// CSS变量缓存值
#[derive(Clone, Debug)]
pub struct CssVarCacheValue {
    pub css_var_str: String,
    pub css_var_id: String,
}

/// 缓存实体，管理所有类型的缓存
pub struct CacheEntity {
    cache: Arc<RwLock<HashMap<String, (CacheValue, usize)>>>, // 值和引用计数
    token_keys: Arc<RwLock<HashMap<String, Vec<String>>>>,    // 跟踪令牌使用情况
}

impl CacheEntity {
    /// 创建新的缓存实体
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            token_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 获取缓存值
    pub fn get(&self, prefix: &str, key: &str) -> Option<CacheValue> {
        let cache_key = format!("{}:{}", prefix, key);
        let cache = self.cache.read().unwrap();
        cache.get(&cache_key).map(|(value, _)| value.clone())
    }

    /// 设置缓存值
    pub fn set(&self, prefix: &str, key: &str, value: CacheValue) {
        let cache_key = format!("{}:{}", prefix, key);
        let mut cache = self.cache.write().unwrap();

        // 更新或插入
        if let Some((_, count)) = cache.get_mut(&cache_key) {
            *count += 1;
        } else {
            cache.insert(cache_key, (value, 1));
        }
    }

    /// 增加引用计数
    pub fn increment(&self, prefix: &str, key: &str) -> bool {
        let cache_key = format!("{}:{}", prefix, key);
        let mut cache = self.cache.write().unwrap();

        if let Some((_, count)) = cache.get_mut(&cache_key) {
            *count += 1;
            true
        } else {
            false
        }
    }

    /// 减少引用计数，返回是否应该移除
    pub fn decrement(&self, prefix: &str, key: &str) -> bool {
        let cache_key = format!("{}:{}", prefix, key);
        let mut cache = self.cache.write().unwrap();

        if let Some((_, count)) = cache.get_mut(&cache_key) {
            *count -= 1;
            *count == 0
        } else {
            false
        }
    }

    /// 移除缓存
    pub fn remove(&self, prefix: &str, key: &str) {
        let cache_key = format!("{}:{}", prefix, key);
        let mut cache = self.cache.write().unwrap();
        cache.remove(&cache_key);
    }

    /// 跟踪令牌使用
    pub fn track_token(&self, token_key: &str, style_key: &str) {
        let mut token_keys = self.token_keys.write().unwrap();
        token_keys
            .entry(token_key.to_string())
            .or_insert_with(Vec::new)
            .push(style_key.to_string());
    }

    /// 清理与令牌相关的样式
    pub fn clean_token_styles(&self, token_key: &str) -> Vec<String> {
        let mut removed_styles = Vec::new();
        let token_keys = self.token_keys.read().unwrap();

        if let Some(style_keys) = token_keys.get(token_key) {
            for style_key in style_keys {
                if self.decrement(STYLE_PREFIX, style_key) {
                    self.remove(STYLE_PREFIX, style_key);
                    removed_styles.push(style_key.clone());
                }
            }
        }

        removed_styles
    }

    /// 获取所有缓存键
    pub fn get_all_keys(&self) -> Vec<(String, String)> {
        let cache = self.cache.read().unwrap();
        cache
            .keys()
            .filter_map(|key| {
                let parts: Vec<&str> = key.split(':').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// 获取指定前缀的所有缓存键
    pub fn get_keys_by_prefix(&self, prefix: &str) -> Vec<String> {
        let cache = self.cache.read().unwrap();
        cache
            .keys()
            .filter_map(|key| {
                if key.starts_with(&format!("{}:", prefix)) {
                    let parts: Vec<&str> = key.split(':').collect();
                    if parts.len() == 2 {
                        Some(parts[1].to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    /// 清理所有缓存
    pub fn clear_all(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();

        let mut token_keys = self.token_keys.write().unwrap();
        token_keys.clear();
    }
}

impl Default for CacheEntity {
    fn default() -> Self {
        Self::new()
    }
}

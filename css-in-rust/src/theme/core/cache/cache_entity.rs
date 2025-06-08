use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 缓存前缀常量
pub const STYLE_PREFIX: &str = "style";
pub const TOKEN_PREFIX: &str = "token";
pub const CSS_VAR_PREFIX: &str = "cssvar";

/// 缓存值类型
///
/// 表示可以存储在缓存中的不同类型的值。
///
/// # 变体
///
/// * `Style` - 样式缓存值
/// * `Token` - 令牌缓存值
/// * `CssVar` - CSS变量缓存值
#[derive(Clone, Debug)]
pub enum CacheValue {
    Style(StyleCacheValue),
    Token(TokenCacheValue),
    CssVar(CssVarCacheValue),
}

/// 样式缓存值
///
/// 存储样式相关的缓存信息。
///
/// # 字段
///
/// * `style_str` - 样式内容字符串
/// * `style_id` - 样式标识符
/// * `effect_style` - 可选的效果样式
/// * `order` - 样式优先级顺序
#[derive(Clone, Debug)]
pub struct StyleCacheValue {
    pub style_str: String,
    pub style_id: String,
    pub effect_style: Option<String>,
    pub order: i32,
}

/// 令牌缓存值
///
/// 存储设计令牌相关的缓存信息。
///
/// # 字段
///
/// * `token_key` - 令牌键名
/// * `token_hash` - 令牌哈希值
/// * `token_data` - 令牌数据
#[derive(Clone, Debug)]
pub struct TokenCacheValue {
    pub token_key: String,
    pub token_hash: String,
    pub token_data: Value,
}

/// CSS变量缓存值
///
/// 存储CSS变量相关的缓存信息。
///
/// # 字段
///
/// * `css_var_str` - CSS变量内容字符串
/// * `css_var_id` - CSS变量标识符
#[derive(Clone, Debug)]
pub struct CssVarCacheValue {
    pub css_var_str: String,
    pub css_var_id: String,
}

/// 缓存实体，管理所有类型的缓存
///
/// 提供存储和检索不同类型缓存值的功能，支持引用计数和令牌跟踪。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::cache::cache_entity::{CacheEntity, CacheValue, StyleCacheValue, STYLE_PREFIX};
///
/// // 创建缓存实体
/// let cache = CacheEntity::new();
///
/// // 存储样式缓存
/// let style = StyleCacheValue {
///     style_str: ".button { color: blue; }".to_string(),
///     style_id: "btn-primary".to_string(),
///     effect_style: None,
///     order: 0,
/// };
/// cache.set(STYLE_PREFIX, "button", CacheValue::Style(style));
///
/// // 获取样式缓存
/// if let Some(CacheValue::Style(cached_style)) = cache.get(STYLE_PREFIX, "button") {
///     println!("缓存的样式: {}", cached_style.style_str);
/// }
/// ```
pub struct CacheEntity {
    cache: Arc<RwLock<HashMap<String, (CacheValue, usize)>>>, // 值和引用计数
    token_keys: Arc<RwLock<HashMap<String, Vec<String>>>>,    // 跟踪令牌使用情况
}

impl CacheEntity {
    /// 创建新的缓存实体
    ///
    /// 初始化一个空的缓存实体，准备存储各种类型的缓存值。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `CacheEntity` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::CacheEntity;
    ///
    /// let cache = CacheEntity::new();
    /// ```
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            token_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 获取缓存值
    ///
    /// 根据前缀和键名获取缓存的值。
    ///
    /// # 参数
    ///
    /// * `prefix` - 缓存前缀，如 "style"、"token" 或 "cssvar"
    /// * `key` - 缓存键名
    ///
    /// # 返回值
    ///
    /// 如果找到缓存值，返回 `Some(CacheValue)`，否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::{CacheEntity, STYLE_PREFIX};
    ///
    /// let cache = CacheEntity::new();
    /// // ... 存储一些缓存值 ...
    ///
    /// if let Some(value) = cache.get(STYLE_PREFIX, "button") {
    ///     // 处理缓存值
    /// }
    /// ```
    pub fn get(&self, prefix: &str, key: &str) -> Option<CacheValue> {
        let cache_key = format!("{}:{}", prefix, key);
        let cache = self.cache.read().unwrap();
        cache.get(&cache_key).map(|(value, _)| value.clone())
    }

    /// 设置缓存值
    ///
    /// 存储一个缓存值，如果键已存在则更新引用计数。
    ///
    /// # 参数
    ///
    /// * `prefix` - 缓存前缀，如 "style"、"token" 或 "cssvar"
    /// * `key` - 缓存键名
    /// * `value` - 要存储的缓存值
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::{CacheEntity, CacheValue, StyleCacheValue, STYLE_PREFIX};
    ///
    /// let cache = CacheEntity::new();
    /// let style = StyleCacheValue {
    ///     style_str: ".button { color: blue; }".to_string(),
    ///     style_id: "btn-primary".to_string(),
    ///     effect_style: None,
    ///     order: 0,
    /// };
    ///
    /// cache.set(STYLE_PREFIX, "button", CacheValue::Style(style));
    /// ```
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
    ///
    /// 增加指定缓存项的引用计数。
    ///
    /// # 参数
    ///
    /// * `prefix` - 缓存前缀，如 "style"、"token" 或 "cssvar"
    /// * `key` - 缓存键名
    ///
    /// # 返回值
    ///
    /// 如果找到并增加了引用计数，返回 `true`，否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::{CacheEntity, STYLE_PREFIX};
    ///
    /// let cache = CacheEntity::new();
    /// // ... 存储一些缓存值 ...
    ///
    /// if cache.increment(STYLE_PREFIX, "button") {
    ///     println!("引用计数已增加");
    /// }
    /// ```
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

    /// 减少引用计数
    ///
    /// 减少指定缓存项的引用计数，如果引用计数降为零，返回 `true` 表示可以移除该项。
    ///
    /// # 参数
    ///
    /// * `prefix` - 缓存前缀，如 "style"、"token" 或 "cssvar"
    /// * `key` - 缓存键名
    ///
    /// # 返回值
    ///
    /// 如果引用计数降为零，返回 `true`，否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::{CacheEntity, STYLE_PREFIX};
    ///
    /// let cache = CacheEntity::new();
    /// // ... 存储一些缓存值 ...
    ///
    /// if cache.decrement(STYLE_PREFIX, "button") {
    ///     // 引用计数为零，可以移除
    ///     cache.remove(STYLE_PREFIX, "button");
    /// }
    /// ```
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
    ///
    /// 从缓存中移除指定的项。
    ///
    /// # 参数
    ///
    /// * `prefix` - 缓存前缀，如 "style"、"token" 或 "cssvar"
    /// * `key` - 缓存键名
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::{CacheEntity, STYLE_PREFIX};
    ///
    /// let cache = CacheEntity::new();
    /// // ... 存储一些缓存值 ...
    ///
    /// cache.remove(STYLE_PREFIX, "button");
    /// ```
    pub fn remove(&self, prefix: &str, key: &str) {
        let cache_key = format!("{}:{}", prefix, key);
        let mut cache = self.cache.write().unwrap();
        cache.remove(&cache_key);
    }

    /// 跟踪令牌使用
    ///
    /// 记录令牌与样式之间的依赖关系，用于后续的缓存失效处理。
    ///
    /// # 参数
    ///
    /// * `token_key` - 令牌键名
    /// * `style_key` - 样式键名
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::CacheEntity;
    ///
    /// let cache = CacheEntity::new();
    /// cache.track_token("primary-color", "button-style");
    /// ```
    pub fn track_token(&self, token_key: &str, style_key: &str) {
        let mut token_keys = self.token_keys.write().unwrap();
        token_keys
            .entry(token_key.to_string())
            .or_insert_with(Vec::new)
            .push(style_key.to_string());
    }

    /// 清理与令牌相关的样式
    ///
    /// 当令牌发生变化时，清理所有依赖该令牌的样式缓存。
    ///
    /// # 参数
    ///
    /// * `token_key` - 令牌键名
    ///
    /// # 返回值
    ///
    /// 返回被移除的样式键名列表。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::CacheEntity;
    ///
    /// let cache = CacheEntity::new();
    /// // ... 存储一些缓存值并跟踪令牌使用 ...
    ///
    /// let removed_styles = cache.clean_token_styles("primary-color");
    /// println!("移除了 {} 个样式", removed_styles.len());
    /// ```
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
    ///
    /// 返回所有缓存键的前缀和名称对。
    ///
    /// # 返回值
    ///
    /// 返回包含 (前缀, 键名) 元组的向量。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::CacheEntity;
    ///
    /// let cache = CacheEntity::new();
    /// // ... 存储一些缓存值 ...
    ///
    /// let all_keys = cache.get_all_keys();
    /// for (prefix, key) in all_keys {
    ///     println!("缓存键: {}:{}", prefix, key);
    /// }
    /// ```
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
    ///
    /// 返回具有指定前缀的所有缓存键名。
    ///
    /// # 参数
    ///
    /// * `prefix` - 缓存前缀，如 "style"、"token" 或 "cssvar"
    ///
    /// # 返回值
    ///
    /// 返回包含键名的向量。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::{CacheEntity, STYLE_PREFIX};
    ///
    /// let cache = CacheEntity::new();
    /// // ... 存储一些缓存值 ...
    ///
    /// let style_keys = cache.get_keys_by_prefix(STYLE_PREFIX);
    /// for key in style_keys {
    ///     println!("样式键: {}", key);
    /// }
    /// ```
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
    ///
    /// 移除所有缓存项和令牌跟踪信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::cache_entity::CacheEntity;
    ///
    /// let cache = CacheEntity::new();
    /// // ... 存储一些缓存值 ...
    ///
    /// cache.clear_all();
    /// ```
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

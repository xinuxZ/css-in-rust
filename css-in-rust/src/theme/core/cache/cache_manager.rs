use crate::theme::core::cache::cache_entity::{
    CacheEntity, CacheValue, CssVarCacheValue, StyleCacheValue, TokenCacheValue, CSS_VAR_PREFIX,
    STYLE_PREFIX, TOKEN_PREFIX,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

/// 缓存管理器
///
/// 管理多层缓存，包括样式缓存、令牌缓存和CSS变量缓存
pub struct CacheManager {
    /// 全局缓存实体
    global_cache: Arc<CacheEntity>,
    /// 当前容器ID
    container_id: String,
    /// 内存使用统计
    memory_usage: Arc<Mutex<MemoryUsage>>,
}

/// 内存使用统计
#[derive(Debug, Default, Clone)]
pub struct MemoryUsage {
    /// 样式缓存大小
    pub style_cache_size: usize,
    /// 令牌缓存大小
    pub token_cache_size: usize,
    /// CSS变量缓存大小
    pub css_var_cache_size: usize,
    /// 总缓存大小
    pub total_cache_size: usize,
    /// 缓存项数量
    pub cache_item_count: usize,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new(container_id: &str) -> Self {
        Self {
            global_cache: Arc::new(CacheEntity::new()),
            container_id: container_id.to_string(),
            memory_usage: Arc::new(Mutex::new(MemoryUsage::default())),
        }
    }

    /// 获取容器ID
    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    /// 获取全局缓存实体
    pub fn global_cache(&self) -> Arc<CacheEntity> {
        self.global_cache.clone()
    }

    /// 获取或创建样式缓存
    pub fn get_or_create_style(
        &self,
        key: &str,
        creator: impl FnOnce() -> (String, String, Option<String>, i32),
    ) -> StyleCacheValue {
        let style_key = format!("{}:{}", self.container_id, key);

        // 尝试从缓存获取
        if let Some(CacheValue::Style(style_value)) =
            self.global_cache.get(STYLE_PREFIX, &style_key)
        {
            return style_value;
        }

        // 创建新的样式
        let (style_str, style_id, effect_style, order) = creator();
        let style_value = StyleCacheValue {
            style_str,
            style_id,
            effect_style,
            order,
        };

        // 更新缓存
        self.global_cache.set(
            STYLE_PREFIX,
            &style_key,
            CacheValue::Style(style_value.clone()),
        );

        // 更新内存使用统计
        self.update_memory_usage(STYLE_PREFIX, &style_value.style_str);

        style_value
    }

    /// 获取或创建令牌缓存
    pub fn get_or_create_token(
        &self,
        key: &str,
        creator: impl FnOnce() -> (String, Value),
    ) -> TokenCacheValue {
        // 尝试从缓存获取
        if let Some(CacheValue::Token(token_value)) = self.global_cache.get(TOKEN_PREFIX, key) {
            return token_value;
        }

        // 创建新的令牌
        let (token_hash, token_data) = creator();
        let token_value = TokenCacheValue {
            token_key: key.to_string(),
            token_hash,
            token_data,
        };

        // 更新缓存
        self.global_cache
            .set(TOKEN_PREFIX, key, CacheValue::Token(token_value.clone()));

        // 更新内存使用统计
        self.update_memory_usage(
            TOKEN_PREFIX,
            &serde_json::to_string(&token_data).unwrap_or_default(),
        );

        token_value
    }

    /// 获取或创建CSS变量缓存
    pub fn get_or_create_css_var(
        &self,
        key: &str,
        creator: impl FnOnce() -> (String, String),
    ) -> CssVarCacheValue {
        let css_var_key = format!("{}:{}", self.container_id, key);

        // 尝试从缓存获取
        if let Some(CacheValue::CssVar(css_var_value)) =
            self.global_cache.get(CSS_VAR_PREFIX, &css_var_key)
        {
            return css_var_value;
        }

        // 创建新的CSS变量
        let (css_var_str, css_var_id) = creator();
        let css_var_value = CssVarCacheValue {
            css_var_str,
            css_var_id,
        };

        // 更新缓存
        self.global_cache.set(
            CSS_VAR_PREFIX,
            &css_var_key,
            CacheValue::CssVar(css_var_value.clone()),
        );

        // 更新内存使用统计
        self.update_memory_usage(CSS_VAR_PREFIX, &css_var_value.css_var_str);

        css_var_value
    }

    /// 跟踪令牌使用
    pub fn track_token(&self, token_key: &str, style_key: &str) {
        let style_key = format!("{}:{}", self.container_id, style_key);
        self.global_cache.track_token(token_key, &style_key);
    }

    /// 更新内存使用统计
    fn update_memory_usage(&self, prefix: &str, content: &str) {
        let content_size = content.len();
        let mut usage = self.memory_usage.lock().unwrap();

        match prefix {
            STYLE_PREFIX => {
                usage.style_cache_size += content_size;
            }
            TOKEN_PREFIX => {
                usage.token_cache_size += content_size;
            }
            CSS_VAR_PREFIX => {
                usage.css_var_cache_size += content_size;
            }
            _ => {}
        }

        usage.total_cache_size += content_size;
        usage.cache_item_count += 1;
    }

    /// 获取内存使用统计
    pub fn get_memory_usage(&self) -> MemoryUsage {
        self.memory_usage.lock().unwrap().clone()
    }

    /// 清理所有缓存
    pub fn clear_all(&self) {
        self.global_cache.clear_all();
        let mut usage = self.memory_usage.lock().unwrap();
        *usage = MemoryUsage::default();
    }

    /// 清理与令牌相关的样式
    pub fn clean_token_styles(&self, token_key: &str) -> Vec<String> {
        let removed_styles = self.global_cache.clean_token_styles(token_key);

        // 更新内存使用统计
        if !removed_styles.is_empty() {
            let mut usage = self.memory_usage.lock().unwrap();
            usage.cache_item_count -= removed_styles.len();
            // 注意：这里简化了内存大小的更新，实际应该减去确切的大小
        }

        removed_styles
    }
}

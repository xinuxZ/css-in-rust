use crate::theme::core::cache::cache_entity::{
    CacheEntity, CacheValue, CssVarCacheValue, StyleCacheValue, TokenCacheValue, CSS_VAR_PREFIX,
    STYLE_PREFIX, TOKEN_PREFIX,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

/// 缓存管理器
///
/// 管理多层缓存，包括样式缓存、令牌缓存和CSS变量缓存。提供高级缓存操作接口，
/// 自动处理缓存键的命名空间隔离，并跟踪内存使用情况。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::cache::CacheManager;
///
/// // 创建一个新的缓存管理器，指定容器ID
/// let cache_manager = CacheManager::new("app-container");
///
/// // 获取或创建样式缓存
/// let style = cache_manager.get_or_create_style("button", || {
///     // 返回 (样式字符串, 样式ID, 效果样式, 优先级)
///     (
///         ".button { color: blue; }".to_string(),
///         "btn-style".to_string(),
///         None,
///         0
///     )
/// });
///
/// println!("样式ID: {}", style.style_id);
/// ```
pub struct CacheManager {
    /// 全局缓存实体
    global_cache: Arc<CacheEntity>,
    /// 当前容器ID
    container_id: String,
    /// 内存使用统计
    memory_usage: Arc<Mutex<MemoryUsage>>,
}

/// 内存使用统计
///
/// 跟踪缓存系统的内存使用情况，包括不同类型缓存的大小和总体统计。
///
/// # 字段
///
/// * `style_cache_size` - 样式缓存占用的字节数
/// * `token_cache_size` - 令牌缓存占用的字节数
/// * `css_var_cache_size` - CSS变量缓存占用的字节数
/// * `total_cache_size` - 总缓存大小（字节）
/// * `cache_item_count` - 缓存项的总数量
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
    ///
    /// 初始化一个新的缓存管理器，并指定容器ID作为命名空间。
    ///
    /// # 参数
    ///
    /// * `container_id` - 容器ID，用于隔离不同实例的缓存
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `CacheManager` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    /// ```
    pub fn new(container_id: &str) -> Self {
        Self {
            global_cache: Arc::new(CacheEntity::new()),
            container_id: container_id.to_string(),
            memory_usage: Arc::new(Mutex::new(MemoryUsage::default())),
        }
    }

    /// 获取容器ID
    ///
    /// 返回当前缓存管理器使用的容器ID。
    ///
    /// # 返回值
    ///
    /// 返回容器ID字符串引用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    /// assert_eq!(cache_manager.container_id(), "app-container");
    /// ```
    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    /// 获取全局缓存实体
    ///
    /// 返回底层的全局缓存实体，用于直接访问缓存操作。
    ///
    /// # 返回值
    ///
    /// 返回 `Arc<CacheEntity>` 的克隆，允许多个所有者共享访问。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    /// let global_cache = cache_manager.global_cache();
    ///
    /// // 现在可以直接使用全局缓存实体
    /// let style_keys = global_cache.get_keys_by_prefix("style");
    /// ```
    pub fn global_cache(&self) -> Arc<CacheEntity> {
        self.global_cache.clone()
    }

    /// 获取或创建样式缓存
    ///
    /// 尝试从缓存中获取样式，如果不存在则使用提供的创建函数生成新的样式并缓存。
    ///
    /// # 参数
    ///
    /// * `key` - 样式的缓存键
    /// * `creator` - 创建样式的闭包，返回 (样式字符串, 样式ID, 效果样式, 优先级)
    ///
    /// # 返回值
    ///
    /// 返回样式缓存值，无论是从缓存获取的还是新创建的。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    ///
    /// // 第一次调用会创建新的样式
    /// let style1 = cache_manager.get_or_create_style("button", || {
    ///     (
    ///         ".button { color: blue; }".to_string(),
    ///         "btn-style".to_string(),
    ///         None,
    ///         0
    ///     )
    /// });
    ///
    /// // 第二次调用会返回缓存的样式
    /// let style2 = cache_manager.get_or_create_style("button", || {
    ///     // 这个闭包不会被执行，因为缓存中已有值
    ///     panic!("不应该执行到这里");
    /// });
    ///
    /// assert_eq!(style1.style_id, style2.style_id);
    /// ```
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
    ///
    /// 尝试从缓存中获取令牌，如果不存在则使用提供的创建函数生成新的令牌并缓存。
    ///
    /// # 参数
    ///
    /// * `key` - 令牌的缓存键
    /// * `creator` - 创建令牌的闭包，返回 (令牌哈希, 令牌数据)
    ///
    /// # 返回值
    ///
    /// 返回令牌缓存值，无论是从缓存获取的还是新创建的。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    /// use serde_json::json;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    ///
    /// let token = cache_manager.get_or_create_token("primary-color", || {
    ///     (
    ///         "abc123".to_string(),
    ///         json!({"color": "#007bff"})
    ///     )
    /// });
    ///
    /// assert_eq!(token.token_key, "primary-color");
    /// ```
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
            token_data: token_data.clone(),
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
    ///
    /// 尝试从缓存中获取CSS变量，如果不存在则使用提供的创建函数生成新的CSS变量并缓存。
    ///
    /// # 参数
    ///
    /// * `key` - CSS变量的缓存键
    /// * `creator` - 创建CSS变量的闭包，返回 (CSS变量字符串, CSS变量ID)
    ///
    /// # 返回值
    ///
    /// 返回CSS变量缓存值，无论是从缓存获取的还是新创建的。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    ///
    /// let css_var = cache_manager.get_or_create_css_var("theme-vars", || {
    ///     (
    ///         ":root { --primary-color: #007bff; }".to_string(),
    ///         "theme-vars-123".to_string()
    ///     )
    /// });
    ///
    /// assert_eq!(css_var.css_var_id, "theme-vars-123");
    /// ```
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
    ///
    /// 记录样式对令牌的依赖关系，用于令牌变更时的缓存失效处理。
    ///
    /// # 参数
    ///
    /// * `token_key` - 令牌键名
    /// * `style_key` - 样式键名
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    ///
    /// // 记录按钮样式依赖于主色令牌
    /// cache_manager.track_token("primary-color", "button-style");
    ///
    /// // 当主色令牌变更时，可以清理相关样式
    /// cache_manager.clean_token_styles("primary-color");
    /// ```
    pub fn track_token(&self, token_key: &str, style_key: &str) {
        let style_key = format!("{}:{}", self.container_id, style_key);
        self.global_cache.track_token(token_key, &style_key);
    }

    /// 更新内存使用统计
    ///
    /// 根据缓存内容更新内存使用统计信息。
    ///
    /// # 参数
    ///
    /// * `prefix` - 缓存前缀，如 "style"、"token" 或 "cssvar"
    /// * `content` - 缓存内容，用于计算大小
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
    ///
    /// 返回当前缓存系统的内存使用情况统计。
    ///
    /// # 返回值
    ///
    /// 返回 `MemoryUsage` 结构的克隆，包含各种内存使用指标。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    /// // ... 存储一些缓存值 ...
    ///
    /// let usage = cache_manager.get_memory_usage();
    /// println!("总缓存大小: {} 字节", usage.total_cache_size);
    /// println!("缓存项数量: {}", usage.cache_item_count);
    /// ```
    pub fn get_memory_usage(&self) -> MemoryUsage {
        self.memory_usage.lock().unwrap().clone()
    }

    /// 清理所有缓存
    ///
    /// 移除所有缓存项并重置内存使用统计。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    /// // ... 存储一些缓存值 ...
    ///
    /// // 清理所有缓存
    /// cache_manager.clear_all();
    ///
    /// // 验证内存使用已重置
    /// let usage = cache_manager.get_memory_usage();
    /// assert_eq!(usage.total_cache_size, 0);
    /// assert_eq!(usage.cache_item_count, 0);
    /// ```
    pub fn clear_all(&self) {
        self.global_cache.clear_all();
        let mut usage = self.memory_usage.lock().unwrap();
        *usage = MemoryUsage::default();
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
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = CacheManager::new("app-container");
    ///
    /// // 记录样式依赖
    /// cache_manager.track_token("primary-color", "button-style");
    /// cache_manager.track_token("primary-color", "header-style");
    ///
    /// // 清理依赖于主色令牌的所有样式
    /// let removed_styles = cache_manager.clean_token_styles("primary-color");
    /// println!("清理了 {} 个样式", removed_styles.len());
    /// ```
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

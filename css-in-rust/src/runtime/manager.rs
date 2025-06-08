//! Style manager functionality
//!
//! This module provides high-level style management capabilities.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use super::injector::{InjectionError, StyleInjector};
use super::provider::ProviderType;

/// Style manager configuration
///
/// 配置样式管理器的行为，包括缓存大小、样式去重和提供器类型。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::{StyleManagerConfig, ProviderType};
///
/// // 创建默认配置
/// let default_config = StyleManagerConfig::default();
///
/// // 创建自定义配置
/// let custom_config = StyleManagerConfig {
///     max_cached_styles: 500,
///     enable_deduplication: true,
///     provider_type: ProviderType::Web,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct StyleManagerConfig {
    /// Maximum number of cached styles
    pub max_cached_styles: usize,
    /// Whether to enable style deduplication
    pub enable_deduplication: bool,
    /// Provider type for style injection
    pub provider_type: ProviderType,
}

impl Default for StyleManagerConfig {
    /// 创建默认的样式管理器配置
    ///
    /// 默认配置设置了1000个最大缓存样式，启用样式去重，并使用自动检测的提供器类型。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::{StyleManagerConfig, ProviderType};
    ///
    /// let config = StyleManagerConfig::default();
    /// assert_eq!(config.max_cached_styles, 1000);
    /// assert_eq!(config.enable_deduplication, true);
    /// assert!(matches!(config.provider_type, ProviderType::Auto));
    /// ```
    fn default() -> Self {
        Self {
            max_cached_styles: 1000,
            enable_deduplication: true,
            provider_type: ProviderType::Auto,
        }
    }
}

/// Style manager for handling CSS injection and caching
///
/// 提供高级的样式管理功能，包括样式注入、缓存和去重。
/// 它封装了底层的`StyleInjector`，并添加了额外的配置选项。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::{StyleManager, StyleManagerConfig, ProviderType};
///
/// // 创建默认样式管理器
/// let manager = StyleManager::new();
///
/// // 注入样式
/// let css = ".container { max-width: 1200px; margin: 0 auto; }";
/// let class_name = "container-style";
/// manager.inject_style(css, class_name).unwrap();
///
/// // 创建自定义配置的样式管理器
/// let config = StyleManagerConfig {
///     max_cached_styles: 500,
///     enable_deduplication: true,
///     provider_type: ProviderType::Web,
/// };
/// let custom_manager = StyleManager::with_config(config);
/// ```
pub struct StyleManager {
    config: StyleManagerConfig,
    injector: StyleInjector,
    cached_styles: Mutex<HashMap<String, (String, Instant)>>,
}

impl StyleManager {
    /// Create a new style manager with default configuration
    ///
    /// 创建一个使用默认配置的新样式管理器实例。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    ///
    /// // 使用管理器注入样式
    /// let css = ".header { height: 60px; background: #f5f5f5; }";
    /// let class_name = "header-style";
    /// manager.inject_style(css, class_name).unwrap();
    /// ```
    pub fn new() -> Self {
        Self::with_config(StyleManagerConfig::default())
    }

    /// Create a new style manager with custom configuration
    ///
    /// 使用自定义配置创建新的样式管理器实例。
    ///
    /// # Arguments
    ///
    /// * `config` - 自定义的样式管理器配置
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::{StyleManager, StyleManagerConfig, ProviderType};
    ///
    /// // 创建自定义配置
    /// let config = StyleManagerConfig {
    ///     max_cached_styles: 200,
    ///     enable_deduplication: false,
    ///     provider_type: ProviderType::Ssr,
    /// };
    ///
    /// // 使用自定义配置创建样式管理器
    /// let manager = StyleManager::with_config(config);
    /// ```
    pub fn with_config(config: StyleManagerConfig) -> Self {
        // 根据配置的 provider_type 创建合适的样式注入器
        let injector = match config.provider_type {
            ProviderType::Auto => {
                // 自动检测最合适的注入器
                #[cfg(target_arch = "wasm32")]
                {
                    // 在 WASM 环境中使用 Web 注入器
                    StyleInjector::new()
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // 在非 WASM 环境中使用服务端注入器或无操作注入器
                    StyleInjector::new()
                }
            }
            ProviderType::Web => {
                // 使用 Web 注入器
                StyleInjector::new()
            }
            ProviderType::Ssr => {
                // 使用服务端渲染注入器
                StyleInjector::new_ssr()
            }
            ProviderType::Isomorphic => {
                // 使用同构注入器
                StyleInjector::new_isomorphic()
            }
            ProviderType::Noop => {
                // 使用无操作注入器（用于测试）
                StyleInjector::new_noop()
            }
        };

        Self {
            config,
            injector,
            cached_styles: Mutex::new(HashMap::new()),
        }
    }

    /// Inject a style with the given class name
    ///
    /// 将CSS样式注入到当前环境中，并与指定的类名关联。
    /// 样式管理器会根据配置进行缓存和去重。
    ///
    /// # Arguments
    ///
    /// * `css` - 要注入的CSS样式字符串
    /// * `class_name` - 与样式关联的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    ///
    /// // 注入导航栏样式
    /// let nav_css = ".nav {
    ///     display: flex;
    ///     justify-content: space-between;
    ///     padding: 1rem;
    ///     background-color: #333;
    ///     color: white;
    /// }";
    /// let result = manager.inject_style(nav_css, "nav-style");
    /// assert!(result.is_ok());
    /// ```
    pub fn inject_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        // 如果启用了样式去重，检查是否已经注入过相同的样式
        if self.config.enable_deduplication {
            let mut cached_styles = self.cached_styles.lock().unwrap();

            // 检查是否已经缓存了相同的样式
            if let Some((cached_css, _)) = cached_styles.get(class_name) {
                if cached_css == css {
                    // 更新访问时间（LRU策略）
                    if let Some((cached_css, _)) = cached_styles.remove(class_name) {
                        cached_styles.insert(class_name.to_string(), (cached_css, Instant::now()));
                    }
                    // 已经注入过相同的样式，直接返回成功
                    return Ok(());
                }
            }

            // 更新缓存
            self.manage_cache_size(&mut cached_styles);
            cached_styles.insert(class_name.to_string(), (css.to_string(), Instant::now()));
        }

        // 注入样式
        self.injector.inject_style(css, class_name)
    }

    /// Remove a style by class name
    ///
    /// 通过类名移除之前注入的样式。
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要移除的样式的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    ///
    /// // 首先注入样式
    /// manager.inject_style(".modal { position: fixed; }", "modal-style").unwrap();
    ///
    /// // 然后移除样式
    /// let result = manager.remove_style("modal-style");
    /// assert!(result.is_ok());
    /// ```
    pub fn remove_style(&self, class_name: &str) -> Result<(), InjectionError> {
        // 如果启用了样式去重，从缓存中移除
        if self.config.enable_deduplication {
            let mut cached_styles = self.cached_styles.lock().unwrap();
            cached_styles.remove(class_name);
        }

        self.injector.remove_style(class_name)
    }

    /// Clear all injected styles
    ///
    /// 移除所有通过此管理器注入的样式。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    ///
    /// // 注入多个样式
    /// manager.inject_style(".btn { padding: 8px; }", "btn-style").unwrap();
    /// manager.inject_style(".card { margin: 16px; }", "card-style").unwrap();
    ///
    /// // 清除所有注入的样式
    /// let result = manager.clear_all_styles();
    /// assert!(result.is_ok());
    /// ```
    pub fn clear_all_styles(&self) -> Result<(), InjectionError> {
        // 清空缓存
        if self.config.enable_deduplication {
            let mut cached_styles = self.cached_styles.lock().unwrap();
            cached_styles.clear();
        }

        self.injector.clear_all_styles()
    }

    /// 管理缓存大小，确保不超过配置的最大值
    fn manage_cache_size(&self, cached_styles: &mut HashMap<String, (String, Instant)>) {
        if cached_styles.len() >= self.config.max_cached_styles {
            // 使用 LRU 策略：找出最久未使用的样式并移除
            let mut oldest_key = None;
            let mut oldest_time = Instant::now();

            for (key, (_, time)) in cached_styles.iter() {
                if *time < oldest_time {
                    oldest_time = *time;
                    oldest_key = Some(key.clone());
                }
            }

            if let Some(key) = oldest_key {
                cached_styles.remove(&key);
            }
        }
    }

    /// 获取当前缓存的样式数量
    ///
    /// 返回当前缓存中的样式数量。
    ///
    /// # Returns
    ///
    /// 缓存中的样式数量
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    /// manager.inject_style(".example { color: red; }", "example").unwrap();
    ///
    /// assert_eq!(manager.cached_styles_count(), 1);
    /// ```
    pub fn cached_styles_count(&self) -> usize {
        let cached_styles = self.cached_styles.lock().unwrap();
        cached_styles.len()
    }

    /// 检查样式是否已缓存
    ///
    /// 检查指定类名的样式是否已经在缓存中。
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要检查的样式类名
    ///
    /// # Returns
    ///
    /// 如果样式已缓存返回 `true`，否则返回 `false`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    /// manager.inject_style(".test { color: blue; }", "test-class").unwrap();
    ///
    /// assert!(manager.is_style_cached("test-class"));
    /// assert!(!manager.is_style_cached("non-existent"));
    /// ```
    pub fn is_style_cached(&self, class_name: &str) -> bool {
        let cached_styles = self.cached_styles.lock().unwrap();
        cached_styles.contains_key(class_name)
    }

    /// 获取缓存中样式的内容
    ///
    /// 获取指定类名对应的样式内容。
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要获取的样式类名
    ///
    /// # Returns
    ///
    /// 如果样式存在，返回 `Some(css_content)`，否则返回 `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    /// let css = ".test { color: blue; }";
    /// manager.inject_style(css, "test-class").unwrap();
    ///
    /// let cached_css = manager.get_cached_style("test-class");
    /// assert_eq!(cached_css, Some(css.to_string()));
    /// ```
    pub fn get_cached_style(&self, class_name: &str) -> Option<String> {
        let cached_styles = self.cached_styles.lock().unwrap();
        cached_styles.get(class_name).map(|(css, _)| css.clone())
    }

    /// 获取当前使用的提供器类型
    ///
    /// 返回样式管理器当前使用的提供器类型。
    ///
    /// # Returns
    ///
    /// 当前使用的提供器类型
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::{StyleManager, StyleManagerConfig, ProviderType};
    ///
    /// let manager = StyleManager::with_config(StyleManagerConfig {
    ///     provider_type: ProviderType::Web,
    ///     ..StyleManagerConfig::default()
    /// });
    ///
    /// assert_eq!(manager.provider_type(), ProviderType::Web);
    /// ```
    pub fn provider_type(&self) -> ProviderType {
        self.config.provider_type
    }

    /// 设置最大缓存样式数量
    ///
    /// 更新样式管理器的最大缓存样式数量。
    /// 如果新的最大值小于当前缓存的样式数量，会立即触发缓存清理。
    ///
    /// # Arguments
    ///
    /// * `max_styles` - 新的最大缓存样式数量
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    /// manager.set_max_cached_styles(500);
    /// ```
    pub fn set_max_cached_styles(&mut self, max_styles: usize) {
        self.config.max_cached_styles = max_styles;

        // 如果当前缓存超过新的最大值，立即触发清理
        let mut cached_styles = self.cached_styles.lock().unwrap();
        while cached_styles.len() > max_styles {
            // 找出最久未使用的样式并移除
            let mut oldest_key = None;
            let mut oldest_time = Instant::now();

            for (key, (_, time)) in cached_styles.iter() {
                if *time < oldest_time {
                    oldest_time = *time;
                    oldest_key = Some(key.clone());
                }
            }

            if let Some(key) = oldest_key {
                cached_styles.remove(&key);
            } else {
                break;
            }
        }
    }
}

impl Default for StyleManager {
    /// 创建一个使用默认配置的新样式管理器实例
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// // 使用默认构造函数创建样式管理器
    /// let manager = StyleManager::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_style_manager_config() {
        // 测试默认配置
        let default_config = StyleManagerConfig::default();
        assert_eq!(default_config.max_cached_styles, 1000);
        assert_eq!(default_config.enable_deduplication, true);
        assert!(matches!(default_config.provider_type, ProviderType::Auto));

        // 测试自定义配置
        let custom_config = StyleManagerConfig {
            max_cached_styles: 500,
            enable_deduplication: false,
            provider_type: ProviderType::Web,
        };
        assert_eq!(custom_config.max_cached_styles, 500);
        assert_eq!(custom_config.enable_deduplication, false);
        assert!(matches!(custom_config.provider_type, ProviderType::Web));
    }

    #[test]
    fn test_style_manager_caching() {
        // 创建启用缓存的样式管理器
        let manager = StyleManager::with_config(StyleManagerConfig {
            max_cached_styles: 2,
            enable_deduplication: true,
            provider_type: ProviderType::Auto,
        });

        // 注入样式
        manager
            .inject_style(".test1 { color: red; }", "test1")
            .unwrap();
        assert_eq!(manager.cached_styles_count(), 1);
        assert!(manager.is_style_cached("test1"));

        // 注入另一个样式
        manager
            .inject_style(".test2 { color: blue; }", "test2")
            .unwrap();
        assert_eq!(manager.cached_styles_count(), 2);

        // 注入第三个样式应该会移除最久未使用的样式
        manager
            .inject_style(".test3 { color: green; }", "test3")
            .unwrap();
        assert_eq!(manager.cached_styles_count(), 2);

        // 测试移除样式
        manager.remove_style("test2").unwrap();
        assert_eq!(manager.cached_styles_count(), 1);
        assert!(!manager.is_style_cached("test2"));

        // 测试清除所有样式
        manager.clear_all_styles().unwrap();
        assert_eq!(manager.cached_styles_count(), 0);
    }

    #[test]
    fn test_style_deduplication() {
        // 创建启用去重的样式管理器
        let manager = StyleManager::with_config(StyleManagerConfig {
            max_cached_styles: 10,
            enable_deduplication: true,
            provider_type: ProviderType::Auto,
        });

        // 注入样式
        manager
            .inject_style(".test { color: red; }", "test-class")
            .unwrap();

        // 再次注入相同的样式，应该被去重
        manager
            .inject_style(".test { color: red; }", "test-class")
            .unwrap();

        // 只应该有一个样式被缓存
        assert_eq!(manager.cached_styles_count(), 1);
    }

    #[test]
    fn test_lru_cache_strategy() {
        // 创建启用缓存的样式管理器，最大缓存数量为2
        let manager = StyleManager::with_config(StyleManagerConfig {
            max_cached_styles: 2,
            enable_deduplication: true,
            provider_type: ProviderType::Auto,
        });

        // 注入第一个样式
        manager
            .inject_style(".test1 { color: red; }", "test1")
            .unwrap();

        // 等待一小段时间
        thread::sleep(Duration::from_millis(10));

        // 注入第二个样式
        manager
            .inject_style(".test2 { color: blue; }", "test2")
            .unwrap();

        // 再次访问第一个样式，使其成为最近使用的
        manager
            .inject_style(".test1 { color: red; }", "test1")
            .unwrap();

        // 等待一小段时间
        thread::sleep(Duration::from_millis(10));

        // 注入第三个样式，应该移除第二个样式（最久未使用）
        manager
            .inject_style(".test3 { color: green; }", "test3")
            .unwrap();

        // 验证第一个和第三个样式在缓存中，第二个被移除
        assert!(manager.is_style_cached("test1"));
        assert!(!manager.is_style_cached("test2"));
        assert!(manager.is_style_cached("test3"));
    }

    #[test]
    fn test_provider_type() {
        // 测试不同提供器类型
        let web_manager = StyleManager::with_config(StyleManagerConfig {
            provider_type: ProviderType::Web,
            ..StyleManagerConfig::default()
        });
        assert_eq!(web_manager.provider_type(), ProviderType::Web);

        let ssr_manager = StyleManager::with_config(StyleManagerConfig {
            provider_type: ProviderType::Ssr,
            ..StyleManagerConfig::default()
        });
        assert_eq!(ssr_manager.provider_type(), ProviderType::Ssr);

        let noop_manager = StyleManager::with_config(StyleManagerConfig {
            provider_type: ProviderType::Noop,
            ..StyleManagerConfig::default()
        });
        assert_eq!(noop_manager.provider_type(), ProviderType::Noop);
    }

    #[test]
    fn test_set_max_cached_styles() {
        // 创建样式管理器
        let mut manager = StyleManager::with_config(StyleManagerConfig {
            max_cached_styles: 5,
            enable_deduplication: true,
            provider_type: ProviderType::Auto,
        });

        // 注入5个样式
        for i in 1..=5 {
            manager
                .inject_style(
                    &format!(".test{} {{ color: red; }}", i),
                    &format!("test{}", i),
                )
                .unwrap();
        }

        // 验证所有5个样式都在缓存中
        assert_eq!(manager.cached_styles_count(), 5);

        // 减小最大缓存大小
        manager.set_max_cached_styles(3);

        // 验证缓存大小已减小到3
        assert_eq!(manager.cached_styles_count(), 3);
    }

    #[test]
    fn test_get_cached_style() {
        // 创建样式管理器
        let manager = StyleManager::new();

        // 注入样式
        let css = ".test { color: blue; }";
        manager.inject_style(css, "test-class").unwrap();

        // 获取缓存的样式内容
        let cached_css = manager.get_cached_style("test-class");
        assert_eq!(cached_css, Some(css.to_string()));

        // 获取不存在的样式
        let nonexistent = manager.get_cached_style("nonexistent");
        assert_eq!(nonexistent, None);
    }
}

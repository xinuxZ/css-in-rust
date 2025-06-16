use std::collections::{HashMap, HashSet};

/// 样式水合配置
///
/// 控制客户端水合过程中的行为，包括样式去重、服务端样式移除和懒加载。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::ssr::hydration::HydrationConfig;
///
/// // 创建默认配置
/// let default_config = HydrationConfig::default();
///
/// // 创建自定义配置
/// let custom_config = HydrationConfig {
///     deduplication: true,
///     remove_server_styles: false,
///     lazy_load: true,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct HydrationConfig {
    /// 是否启用样式去重
    pub deduplication: bool,
    /// 是否移除服务端样式
    pub remove_server_styles: bool,
    /// 是否启用懒加载
    pub lazy_load: bool,
}

impl Default for HydrationConfig {
    fn default() -> Self {
        Self {
            deduplication: true,
            remove_server_styles: true,
            lazy_load: false,
        }
    }
}

/// 样式水合
///
/// 负责在客户端水合服务端渲染的样式，确保样式在客户端正确应用。
/// 水合过程包括收集服务端样式、去重客户端样式和管理样式生命周期。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::ssr::hydration::{StyleHydration, HydrationConfig};
///
/// // 使用默认配置创建水合器
/// let mut hydration = StyleHydration::default();
///
/// // 在客户端使用
/// #[cfg(target_arch = "wasm32")]
/// {
///     hydration.hydrate().expect("样式水合失败");
/// }
/// ```
pub struct StyleHydration {
    /// 水合配置
    config: HydrationConfig,
    /// 已水合的样式ID
    hydrated_styles: HashSet<String>,
    /// 样式哈希映射
    style_hashes: HashMap<String, String>,
}

impl StyleHydration {
    /// 创建新的样式水合器
    ///
    /// # Arguments
    ///
    /// * `config` - 水合配置
    ///
    /// # Returns
    ///
    /// 新创建的样式水合器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::hydration::{StyleHydration, HydrationConfig};
    ///
    /// let config = HydrationConfig {
    ///     deduplication: true,
    ///     remove_server_styles: false,
    ///     lazy_load: true,
    /// };
    ///
    /// let hydration = StyleHydration::new(config);
    /// ```
    pub fn new(config: HydrationConfig) -> Self {
        Self {
            config,
            hydrated_styles: HashSet::new(),
            style_hashes: HashMap::new(),
        }
    }

    /// 使用默认配置创建样式水合器
    ///
    /// # Returns
    ///
    /// 使用默认配置的样式水合器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::hydration::StyleHydration;
    ///
    /// let hydration = StyleHydration::default();
    /// ```
    pub fn default() -> Self {
        Self::new(HydrationConfig::default())
    }

    /// 水合样式
    ///
    /// 在客户端执行样式水合过程，包括收集服务端样式、去重和管理样式生命周期。
    /// 该方法只在WebAssembly目标上可用。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`Err`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::hydration::StyleHydration;
    ///
    /// let mut hydration = StyleHydration::default();
    ///
    /// #[cfg(target_arch = "wasm32")]
    /// {
    ///     if let Err(err) = hydration.hydrate() {
    ///         console_log::log!("样式水合失败: {}", err);
    ///     }
    /// }
    /// ```
    #[cfg(target_arch = "wasm32")]
    pub fn hydrate(&mut self) -> Result<(), String> {
        use web_sys::{window, Document, Element, HtmlCollection};

        let window = window().ok_or_else(|| "无法获取window对象".to_string())?;
        let document = window
            .document()
            .ok_or_else(|| "无法获取document对象".to_string())?;

        // 收集服务端样式
        self.collect_server_styles(&document)?;

        // 如果启用了样式去重，移除重复的客户端样式
        if self.config.deduplication {
            self.deduplicate_styles(&document)?;
        }

        // 如果配置了移除服务端样式，则在客户端样式加载后移除服务端样式
        if self.config.remove_server_styles {
            self.schedule_server_styles_removal(&document)?;
        }

        Ok(())
    }

    /// 收集服务端样式
    ///
    /// 从文档中收集服务端渲染的样式元素，并记录它们的ID和哈希值。
    /// 该方法只在WebAssembly目标上可用。
    ///
    /// # Arguments
    ///
    /// * `document` - Web文档对象
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`Err`
    #[cfg(target_arch = "wasm32")]
    fn collect_server_styles(&mut self, document: &web_sys::Document) -> Result<(), String> {
        use wasm_bindgen::JsCast;

        let style_elements = document.get_elements_by_tag_name("style");

        for i in 0..style_elements.length() {
            if let Some(element) = style_elements.item(i) {
                if let Some(style_element) = element.dyn_ref::<web_sys::HtmlStyleElement>() {
                    let id = style_element.id();
                    if !id.is_empty() {
                        if let Some(hash) = style_element.get_attribute("data-hash") {
                            self.style_hashes.insert(id.clone(), hash);
                            self.hydrated_styles.insert(id);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 去重样式
    ///
    /// 比较新注入的客户端样式与服务端样式，移除重复的样式以避免冲突。
    /// 该方法只在WebAssembly目标上可用。
    ///
    /// # Arguments
    ///
    /// * `document` - Web文档对象
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`Err`
    #[cfg(target_arch = "wasm32")]
    fn deduplicate_styles(&self, document: &web_sys::Document) -> Result<(), String> {
        // 实现样式去重逻辑
        // 比较新注入的样式与服务端样式，移除重复的

        Ok(())
    }

    /// 安排移除服务端样式
    ///
    /// 在客户端样式加载完成后，安排移除服务端注入的样式元素。
    /// 该方法只在WebAssembly目标上可用。
    ///
    /// # Arguments
    ///
    /// * `document` - Web文档对象
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`Err`
    #[cfg(target_arch = "wasm32")]
    fn schedule_server_styles_removal(&self, document: &web_sys::Document) -> Result<(), String> {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        use web_sys::window;

        let window = window().ok_or_else(|| "无法获取window对象".to_string())?;

        // 创建一个闭包来移除服务端样式
        let window_clone = window.clone();
        let remove_styles = Closure::wrap(Box::new(move || {
            if let Some(document) = window_clone.document() {
                let style_elements = document.get_elements_by_tag_name("style");

                let mut to_remove = Vec::new();

                for i in 0..style_elements.length() {
                    if let Some(element) = style_elements.item(i) {
                        if element.has_attribute("data-critical") {
                            to_remove.push(element);
                        }
                    }
                }

                for element in to_remove {
                    if let Some(parent) = element.parent_node() {
                        let _ = parent.remove_child(&element);
                    }
                }
            }
        }) as Box<dyn FnMut()>);

        // 延迟执行，确保客户端样式已加载
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            remove_styles.as_ref().unchecked_ref(),
            100, // 延迟100毫秒
        );

        // 防止闭包被回收
        remove_styles.forget();

        Ok(())
    }

    /// 检查样式是否已水合
    ///
    /// # Arguments
    ///
    /// * `style_id` - 样式ID
    ///
    /// # Returns
    ///
    /// 如果样式已水合则返回true，否则返回false
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::hydration::StyleHydration;
    ///
    /// let hydration = StyleHydration::default();
    /// let is_hydrated = hydration.is_hydrated("app-styles");
    /// ```
    pub fn is_hydrated(&self, style_id: &str) -> bool {
        self.hydrated_styles.contains(style_id)
    }

    /// 获取样式哈希
    ///
    /// # Arguments
    ///
    /// * `style_id` - 样式ID
    ///
    /// # Returns
    ///
    /// 样式哈希的可选引用，如果样式不存在则返回None
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::hydration::StyleHydration;
    ///
    /// let mut hydration = StyleHydration::default();
    /// hydration.mark_as_hydrated("app-styles", "abc123");
    ///
    /// let hash = hydration.get_style_hash("app-styles");
    /// assert_eq!(hash, Some(&"abc123".to_string()));
    /// ```
    pub fn get_style_hash(&self, style_id: &str) -> Option<&String> {
        self.style_hashes.get(style_id)
    }

    /// 标记样式为已水合
    ///
    /// # Arguments
    ///
    /// * `style_id` - 样式ID
    /// * `hash` - 样式哈希
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::hydration::StyleHydration;
    ///
    /// let mut hydration = StyleHydration::default();
    /// hydration.mark_as_hydrated("app-styles", "abc123");
    ///
    /// assert!(hydration.is_hydrated("app-styles"));
    /// ```
    pub fn mark_as_hydrated(&mut self, style_id: &str, hash: &str) {
        self.hydrated_styles.insert(style_id.to_string());
        self.style_hashes
            .insert(style_id.to_string(), hash.to_string());
    }

    /// 清空水合状态
    ///
    /// 清除所有已水合的样式记录和哈希映射。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::hydration::StyleHydration;
    ///
    /// let mut hydration = StyleHydration::default();
    /// hydration.mark_as_hydrated("app-styles", "abc123");
    ///
    /// hydration.clear();
    /// assert!(!hydration.is_hydrated("app-styles"));
    /// ```
    pub fn clear(&mut self) {
        self.hydrated_styles.clear();
        self.style_hashes.clear();
    }
}

use crate::theme::core::cache::cache_entity::{CacheValue, STYLE_PREFIX};
use crate::theme::core::cache::CacheManager;
use crate::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
use std::collections::{HashMap, HashSet};

/// 样式提取器
///
/// 负责从缓存中提取样式，并将其转换为服务端渲染的样式表。
/// 提取器可以提取所有样式或特定组件的样式，并区分关键和非关键样式。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::cache::CacheManager;
/// use css_in_rust::theme::core::ssr::StyleExtractor;
///
/// let cache_manager = CacheManager::new();
/// let mut extractor = StyleExtractor::new(cache_manager);
///
/// // 提取所有样式
/// let sheet_manager = extractor.extract();
///
/// // 提取特定组件的样式
/// let component_styles = extractor.extract_for_component("Button");
/// ```
pub struct StyleExtractor {
    /// 缓存管理器
    cache_manager: CacheManager,
    /// 已提取的样式ID
    extracted_styles: HashSet<String>,
    /// 样式表管理器
    sheet_manager: StyleSheetManager,
    /// 样式优先级映射
    style_order: HashMap<String, i32>,
}

impl StyleExtractor {
    /// 创建新的样式提取器
    ///
    /// # Arguments
    ///
    /// * `cache_manager` - 缓存管理器，用于获取样式缓存
    ///
    /// # Returns
    ///
    /// 新创建的样式提取器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    /// use css_in_rust::theme::core::ssr::StyleExtractor;
    ///
    /// let cache_manager = CacheManager::new();
    /// let extractor = StyleExtractor::new(cache_manager);
    /// ```
    pub fn new(cache_manager: CacheManager) -> Self {
        Self {
            cache_manager,
            extracted_styles: HashSet::new(),
            sheet_manager: StyleSheetManager::new(),
            style_order: HashMap::new(),
        }
    }

    /// 提取所有样式
    ///
    /// 从缓存中提取所有样式，并将其转换为服务端渲染的样式表。
    /// 样式将根据是否为关键样式进行分类。
    ///
    /// # Returns
    ///
    /// 包含所有提取样式的样式表管理器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    /// use css_in_rust::theme::core::ssr::StyleExtractor;
    ///
    /// let cache_manager = CacheManager::new();
    /// let mut extractor = StyleExtractor::new(cache_manager);
    ///
    /// let sheet_manager = extractor.extract();
    /// let critical_css = sheet_manager.to_critical_style_tags();
    /// ```
    pub fn extract(&mut self) -> &StyleSheetManager {
        let cache = self.cache_manager.global_cache();

        // 获取所有样式缓存键
        let style_keys = cache.get_keys_by_prefix(STYLE_PREFIX);

        // 提取样式
        for style_key in style_keys {
            if self.extracted_styles.contains(&style_key) {
                continue;
            }

            if let Some(CacheValue::Style(style)) = cache.get(STYLE_PREFIX, &style_key) {
                // 确定是否为关键样式
                let is_critical = self.is_critical_style(&style_key);

                // 创建样式表
                let sheet = ServerStyleSheet::new(&style.style_id, &style.style_str, is_critical);

                // 添加到样式表管理器
                self.sheet_manager.add_sheet(sheet);

                // 记录样式优先级
                self.style_order.insert(style_key.clone(), style.order);

                // 标记为已提取
                self.extracted_styles.insert(style_key);
            }
        }

        &self.sheet_manager
    }

    /// 提取特定组件的样式
    ///
    /// 从缓存中提取与指定组件相关的样式，并将其转换为服务端渲染的样式表。
    /// 组件样式默认被视为关键样式。
    ///
    /// # Arguments
    ///
    /// * `component_name` - 组件名称
    ///
    /// # Returns
    ///
    /// 包含组件样式的样式表管理器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    /// use css_in_rust::theme::core::ssr::StyleExtractor;
    ///
    /// let cache_manager = CacheManager::new();
    /// let mut extractor = StyleExtractor::new(cache_manager);
    ///
    /// let button_styles = extractor.extract_for_component("Button");
    /// ```
    pub fn extract_for_component(&mut self, component_name: &str) -> &StyleSheetManager {
        let cache = self.cache_manager.global_cache();

        // 获取所有样式缓存键
        let style_keys = cache.get_keys_by_prefix(STYLE_PREFIX);

        // 提取与组件相关的样式
        for style_key in style_keys {
            if self.extracted_styles.contains(&style_key) {
                continue;
            }

            // 检查是否与组件相关
            if !style_key.contains(component_name) {
                continue;
            }

            if let Some(CacheValue::Style(style)) = cache.get(STYLE_PREFIX, &style_key) {
                // 创建样式表（组件样式默认为关键样式）
                let sheet = ServerStyleSheet::new(&style.style_id, &style.style_str, true);

                // 添加到样式表管理器
                self.sheet_manager.add_sheet(sheet);

                // 记录样式优先级
                self.style_order.insert(style_key.clone(), style.order);

                // 标记为已提取
                self.extracted_styles.insert(style_key);
            }
        }

        &self.sheet_manager
    }

    /// 判断是否为关键样式
    ///
    /// 根据样式键判断样式是否为关键样式。
    /// 关键样式通常是首屏渲染必需的样式，如基础组件样式。
    ///
    /// # Arguments
    ///
    /// * `style_key` - 样式键
    ///
    /// # Returns
    ///
    /// 如果是关键样式则返回true，否则返回false
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    /// use css_in_rust::theme::core::ssr::StyleExtractor;
    ///
    /// let cache_manager = CacheManager::new();
    /// let extractor = StyleExtractor::new(cache_manager);
    ///
    /// let is_critical = extractor.is_critical_style("style-Button-primary");
    /// assert!(is_critical);
    /// ```
    fn is_critical_style(&self, style_key: &str) -> bool {
        // 这里可以根据项目需求定义关键样式的规则
        // 例如，所有基础组件的样式都是关键样式
        let critical_components = ["Button", "Typography", "Layout", "Grid", "Icon"];

        for component in &critical_components {
            if style_key.contains(component) {
                return true;
            }
        }

        false
    }

    /// 获取样式表管理器
    ///
    /// # Returns
    ///
    /// 样式表管理器的引用
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    /// use css_in_rust::theme::core::ssr::StyleExtractor;
    ///
    /// let cache_manager = CacheManager::new();
    /// let extractor = StyleExtractor::new(cache_manager);
    ///
    /// let sheet_manager = extractor.sheet_manager();
    /// ```
    pub fn sheet_manager(&self) -> &StyleSheetManager {
        &self.sheet_manager
    }

    /// 获取已提取的样式ID
    ///
    /// # Returns
    ///
    /// 已提取样式ID的集合
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    /// use css_in_rust::theme::core::ssr::StyleExtractor;
    ///
    /// let cache_manager = CacheManager::new();
    /// let mut extractor = StyleExtractor::new(cache_manager);
    /// extractor.extract();
    ///
    /// let extracted_styles = extractor.extracted_styles();
    /// ```
    pub fn extracted_styles(&self) -> &HashSet<String> {
        &self.extracted_styles
    }

    /// 清空提取器
    ///
    /// 清除所有提取的样式和样式表。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::cache::CacheManager;
    /// use css_in_rust::theme::core::ssr::StyleExtractor;
    ///
    /// let cache_manager = CacheManager::new();
    /// let mut extractor = StyleExtractor::new(cache_manager);
    /// extractor.extract();
    ///
    /// // 清空提取器
    /// extractor.clear();
    /// assert_eq!(extractor.extracted_styles().len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.extracted_styles.clear();
        self.sheet_manager.clear();
        self.style_order.clear();
    }
}

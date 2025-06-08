//! Style injection functionality
//!
//! This module provides the core style injection capabilities for different
//! target environments (web, SSR, etc.).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Style injection error
///
/// 表示在样式注入过程中可能发生的各种错误类型。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::InjectionError;
///
/// // 创建一个注入失败错误
/// let error = InjectionError::InjectionFailed("无法创建样式元素".to_string());
///
/// // 处理不同类型的注入错误
/// fn handle_error(err: InjectionError) {
///     match err {
///         InjectionError::InjectionFailed(msg) => println!("注入失败: {}", msg),
///         InjectionError::RemovalFailed(msg) => println!("移除失败: {}", msg),
///         InjectionError::PlatformNotSupported(msg) => println!("平台不支持: {}", msg),
///         InjectionError::DomOperationFailed(msg) => println!("DOM操作失败: {}", msg),
///     }
/// }
/// ```
#[derive(Debug)]
pub enum InjectionError {
    /// 样式注入失败时的错误
    InjectionFailed(String),
    /// 样式移除失败时的错误
    RemovalFailed(String),
    /// 当前平台不支持特定操作时的错误
    PlatformNotSupported(String),
    /// DOM操作失败时的错误（仅在Web环境中）
    DomOperationFailed(String),
}

impl std::fmt::Display for InjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InjectionError::InjectionFailed(msg) => write!(f, "Style injection failed: {}", msg),
            InjectionError::RemovalFailed(msg) => write!(f, "Style removal failed: {}", msg),
            InjectionError::PlatformNotSupported(msg) => {
                write!(f, "Platform not supported: {}", msg)
            }
            InjectionError::DomOperationFailed(msg) => write!(f, "DOM operation failed: {}", msg),
        }
    }
}

impl std::error::Error for InjectionError {}

/// Style injector for managing CSS injection
///
/// 提供核心的样式注入功能，负责将CSS样式注入到不同的目标环境（如Web浏览器或SSR）。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::StyleInjector;
///
/// // 创建新的样式注入器
/// let injector = StyleInjector::new();
///
/// // 注入样式
/// let css = ".button { background-color: blue; color: white; }";
/// let class_name = "button-style";
///
/// // 注入样式（在实际环境中会将样式添加到DOM）
/// let result = injector.inject_style(css, class_name);
///
/// // 样式注入是幂等的，多次注入相同类名只会执行一次
/// let repeat_result = injector.inject_style(css, class_name);
///
/// // 移除样式
/// let remove_result = injector.remove_style(class_name);
/// ```
pub struct StyleInjector {
    injected_styles: Arc<Mutex<HashMap<String, bool>>>,
}

impl StyleInjector {
    /// Create a new style injector
    ///
    /// 创建一个新的样式注入器实例，用于管理CSS样式的注入和移除。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// // 创建新的样式注入器
    /// let injector = StyleInjector::new();
    /// assert!(injector.inject_style(".test{color:red}", "test-style").is_ok());
    /// ```
    pub fn new() -> Self {
        Self {
            injected_styles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Inject a style with the given class name
    ///
    /// 将CSS样式注入到当前环境中，并与指定的类名关联。
    /// 这个方法是幂等的，对同一个类名多次调用只会注入一次样式。
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
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let injector = StyleInjector::new();
    ///
    /// // 注入卡片样式
    /// let card_css = ".card {
    ///     border: 1px solid #ddd;
    ///     border-radius: 4px;
    ///     padding: 16px;
    ///     box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    /// }";
    /// let result = injector.inject_style(card_css, "card-style");
    ///
    /// // 检查是否成功注入
    /// assert!(result.is_ok());
    /// ```
    pub fn inject_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        let mut styles = self.injected_styles.lock().unwrap();

        // Check if already injected
        if styles.contains_key(class_name) {
            return Ok(());
        }

        // Inject based on platform
        #[cfg(target_arch = "wasm32")]
        {
            self.inject_web_style(css, class_name)?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.inject_server_style(css, class_name)?;
        }

        // Mark as injected
        styles.insert(class_name.to_string(), true);
        Ok(())
    }

    /// Remove a style by class name
    ///
    /// 通过类名移除之前注入的样式。如果指定的类名不存在，此操作不会产生错误。
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
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let injector = StyleInjector::new();
    ///
    /// // 首先注入样式
    /// injector.inject_style(".temp { color: green; }", "temp-style").unwrap();
    ///
    /// // 然后移除样式
    /// let result = injector.remove_style("temp-style");
    /// assert!(result.is_ok());
    ///
    /// // 移除不存在的样式也不会产生错误
    /// let result = injector.remove_style("non-existent-style");
    /// assert!(result.is_ok());
    /// ```
    pub fn remove_style(&self, class_name: &str) -> Result<(), InjectionError> {
        let mut styles = self.injected_styles.lock().unwrap();

        if !styles.contains_key(class_name) {
            return Ok(());
        }

        // Remove based on platform
        #[cfg(target_arch = "wasm32")]
        {
            self.remove_web_style(class_name)?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.remove_server_style(class_name)?;
        }

        // Mark as removed
        styles.remove(class_name);
        Ok(())
    }

    /// Clear all injected styles
    ///
    /// 移除所有通过此注入器注入的样式。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let injector = StyleInjector::new();
    ///
    /// // 注入多个样式
    /// injector.inject_style(".btn { padding: 8px; }", "btn-style").unwrap();
    /// injector.inject_style(".header { height: 60px; }", "header-style").unwrap();
    ///
    /// // 清除所有注入的样式
    /// let result = injector.clear_all_styles();
    /// assert!(result.is_ok());
    /// ```
    pub fn clear_all_styles(&self) -> Result<(), InjectionError> {
        let mut styles = self.injected_styles.lock().unwrap();

        // Clear based on platform
        #[cfg(target_arch = "wasm32")]
        {
            self.clear_web_styles()?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.clear_server_styles()?;
        }

        // Clear tracking
        styles.clear();
        Ok(())
    }

    /// Inject style in web environment
    ///
    /// 在Web环境中将CSS样式注入到DOM中。
    /// 创建一个新的style元素，设置其内容为提供的CSS，并添加到文档的head部分。
    ///
    /// # Arguments
    ///
    /// * `css` - 要注入的CSS样式字符串
    /// * `class_name` - 与样式关联的类名，用作标识
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    #[cfg(target_arch = "wasm32")]
    fn inject_web_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        use web_sys::{window, Document, HtmlStyleElement};

        let window = window().ok_or_else(|| {
            InjectionError::DomOperationFailed("No window object available".to_string())
        })?;

        let document = window.document().ok_or_else(|| {
            InjectionError::DomOperationFailed("No document object available".to_string())
        })?;

        let style_element = document
            .create_element("style")
            .map_err(|_| {
                InjectionError::DomOperationFailed("Failed to create style element".to_string())
            })?
            .dyn_into::<HtmlStyleElement>()
            .map_err(|_| {
                InjectionError::DomOperationFailed("Failed to cast to HtmlStyleElement".to_string())
            })?;

        style_element.set_text_content(Some(css));
        style_element
            .set_attribute("data-css-class", class_name)
            .map_err(|_| {
                InjectionError::DomOperationFailed("Failed to set attribute".to_string())
            })?;

        let head = document.head().ok_or_else(|| {
            InjectionError::DomOperationFailed("No head element available".to_string())
        })?;

        head.append_child(&style_element).map_err(|_| {
            InjectionError::DomOperationFailed("Failed to append style element".to_string())
        })?;

        Ok(())
    }

    /// Remove style in web environment
    ///
    /// 在Web环境中通过类名移除之前注入的样式元素。
    /// 查找带有指定data-css-class属性的style元素并从DOM中移除。
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要移除的样式的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    #[cfg(target_arch = "wasm32")]
    fn remove_web_style(&self, class_name: &str) -> Result<(), InjectionError> {
        use web_sys::{window, Element};

        let window = window().ok_or_else(|| {
            InjectionError::DomOperationFailed("No window object available".to_string())
        })?;

        let document = window.document().ok_or_else(|| {
            InjectionError::DomOperationFailed("No document object available".to_string())
        })?;

        let selector = format!("style[data-css-class='{}']", class_name);
        if let Ok(Some(element)) = document.query_selector(&selector) {
            if let Some(parent) = element.parent_node() {
                parent.remove_child(&element).map_err(|_| {
                    InjectionError::DomOperationFailed("Failed to remove style element".to_string())
                })?;
            }
        }

        Ok(())
    }

    /// Clear all styles in web environment
    ///
    /// 在Web环境中移除所有由此注入器创建的样式元素。
    /// 查找所有带有data-css-class属性的style元素并从DOM中移除。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    #[cfg(target_arch = "wasm32")]
    fn clear_web_styles(&self) -> Result<(), InjectionError> {
        use web_sys::window;

        let window = window().ok_or_else(|| {
            InjectionError::DomOperationFailed("No window object available".to_string())
        })?;

        let document = window.document().ok_or_else(|| {
            InjectionError::DomOperationFailed("No document object available".to_string())
        })?;

        let elements = document
            .query_selector_all("style[data-css-class]")
            .map_err(|_| {
                InjectionError::DomOperationFailed("Failed to query style elements".to_string())
            })?;

        for i in 0..elements.length() {
            if let Some(element) = elements.item(i) {
                if let Some(parent) = element.parent_node() {
                    let _ = parent.remove_child(&element);
                }
            }
        }

        Ok(())
    }

    /// Inject style in server environment (SSR)
    ///
    /// 在服务器端渲染环境中注入样式。
    /// 目前这是一个空操作，但在实际的SSR实现中，可能会收集样式以便稍后在HTML响应中包含它们。
    ///
    /// # Arguments
    ///
    /// * `_css` - 要注入的CSS样式字符串
    /// * `_class_name` - 与样式关联的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(not(target_arch = "wasm32"))]
    fn inject_server_style(&self, _css: &str, _class_name: &str) -> Result<(), InjectionError> {
        // In SSR mode, we might collect styles for later injection
        // For now, this is a no-op
        Ok(())
    }

    /// Remove style in server environment (SSR)
    ///
    /// 在服务器端渲染环境中移除样式。
    /// 目前这是一个空操作，但在实际的SSR实现中，可能会从收集的样式集合中移除指定的样式。
    ///
    /// # Arguments
    ///
    /// * `_class_name` - 要移除的样式的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(not(target_arch = "wasm32"))]
    fn remove_server_style(&self, _class_name: &str) -> Result<(), InjectionError> {
        // In SSR mode, this would remove from the collected styles
        // For now, this is a no-op
        Ok(())
    }

    /// Clear all styles in server environment (SSR)
    ///
    /// 在服务器端渲染环境中清除所有样式。
    /// 目前这是一个空操作，但在实际的SSR实现中，可能会清除所有收集的样式。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(not(target_arch = "wasm32"))]
    fn clear_server_styles(&self) -> Result<(), InjectionError> {
        // In SSR mode, this would clear all collected styles
        // For now, this is a no-op
        Ok(())
    }
}

impl Default for StyleInjector {
    /// 创建一个新的默认样式注入器实例
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// // 使用默认构造函数创建注入器
    /// let injector = StyleInjector::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_injector_creation() {
        let injector = StyleInjector::new();
        assert!(injector.injected_styles.lock().unwrap().is_empty());
    }

    #[test]
    fn test_inject_style_tracking() {
        let injector = StyleInjector::new();
        let css = ".test { color: red; }";
        let class_name = "test-class";

        // First injection should succeed
        let result = injector.inject_style(css, class_name);
        assert!(result.is_ok());

        // Second injection should also succeed (idempotent)
        let result = injector.inject_style(css, class_name);
        assert!(result.is_ok());

        // Should be tracked as injected
        assert!(injector
            .injected_styles
            .lock()
            .unwrap()
            .contains_key(class_name));
    }
}

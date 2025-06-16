//! Style injection functionality
//!
//! This module provides the core style injection capabilities for different
//! target environments (web, SSR, etc.).

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

#[cfg(not(target_arch = "wasm32"))]
lazy_static::lazy_static! {
    static ref SERVER_STYLES: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
}

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
    /// Failed to inject style
    InjectionFailed(String),
    /// Failed to remove style
    RemovalFailed(String),
    /// Failed to clear styles
    ClearFailed(String),
    /// DOM operation failed (web only)
    DomOperationFailed(String),
    /// Platform not supported
    PlatformNotSupported(String),
}

impl std::fmt::Display for InjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InjectionError::InjectionFailed(msg) => write!(f, "Failed to inject style: {}", msg),
            InjectionError::RemovalFailed(msg) => write!(f, "Failed to remove style: {}", msg),
            InjectionError::ClearFailed(msg) => write!(f, "Failed to clear styles: {}", msg),
            InjectionError::DomOperationFailed(msg) => write!(f, "DOM operation failed: {}", msg),
            InjectionError::PlatformNotSupported(msg) => {
                write!(f, "Platform not supported: {}", msg)
            }
        }
    }
}

impl std::error::Error for InjectionError {}

/// Injection environment
///
/// 定义样式注入的运行环境类型，用于在运行时选择合适的注入策略。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::injector::InjectionEnvironment;
///
/// // 创建浏览器环境注入器
/// let browser_env = InjectionEnvironment::Browser;
///
/// // 创建服务端环境注入器
/// let server_env = InjectionEnvironment::Server;
///
/// // 创建同构环境注入器
/// let isomorphic_env = InjectionEnvironment::Isomorphic;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InjectionEnvironment {
    /// Browser environment (client-side rendering)
    Browser,
    /// Server environment (server-side rendering)
    Server,
    /// Isomorphic environment (both server and client rendering)
    Isomorphic,
    /// No-op environment (for testing)
    Noop,
}

/// Style injector
///
/// 样式注入器，负责将CSS样式注入到不同的环境中。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::StyleInjector;
///
/// // 创建默认样式注入器
/// let injector = StyleInjector::new();
///
/// // 注入样式
/// let css = ".button { color: blue; }";
/// let class_name = "button-style";
/// injector.inject_style(css, class_name).unwrap();
/// ```
pub struct StyleInjector {
    /// 已注入的样式集合
    injected_styles: Arc<Mutex<HashMap<String, String>>>,
    /// 注入环境
    environment: InjectionEnvironment,
}

impl StyleInjector {
    /// Create a new style injector
    ///
    /// 创建一个新的样式注入器，会自动检测当前环境。
    ///
    /// # Returns
    ///
    /// 一个新的样式注入器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let injector = StyleInjector::new();
    /// ```
    pub fn new() -> Self {
        // 自动检测环境
        #[cfg(target_arch = "wasm32")]
        let environment = InjectionEnvironment::Browser;

        #[cfg(not(target_arch = "wasm32"))]
        let environment = InjectionEnvironment::Server;

        Self {
            injected_styles: Arc::new(Mutex::new(HashMap::new())),
            environment,
        }
    }

    /// 创建一个用于服务端渲染的样式注入器
    ///
    /// 返回一个配置为服务端渲染模式的样式注入器。
    ///
    /// # Returns
    ///
    /// 一个新的服务端渲染样式注入器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let ssr_injector = StyleInjector::new_ssr();
    /// ```
    pub fn new_ssr() -> Self {
        Self {
            injected_styles: Arc::new(Mutex::new(HashMap::new())),
            environment: InjectionEnvironment::Server,
        }
    }

    /// 创建一个无操作的样式注入器
    ///
    /// 返回一个不执行任何实际操作的样式注入器，主要用于测试。
    ///
    /// # Returns
    ///
    /// 一个新的无操作样式注入器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let noop_injector = StyleInjector::new_noop();
    /// ```
    pub fn new_noop() -> Self {
        Self {
            injected_styles: Arc::new(Mutex::new(HashMap::new())),
            environment: InjectionEnvironment::Noop,
        }
    }

    /// 创建一个同构应用的样式注入器
    ///
    /// 返回一个配置为同构模式的样式注入器，可以同时处理服务端和客户端渲染。
    ///
    /// # Returns
    ///
    /// 一个新的同构样式注入器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let isomorphic_injector = StyleInjector::new_isomorphic();
    /// ```
    pub fn new_isomorphic() -> Self {
        Self {
            injected_styles: Arc::new(Mutex::new(HashMap::new())),
            environment: InjectionEnvironment::Isomorphic,
        }
    }

    /// 设置注入环境
    ///
    /// 更改样式注入器的环境设置，用于动态切换注入策略。
    ///
    /// # Arguments
    ///
    /// * `environment` - 新的注入环境
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::{StyleInjector, injector::InjectionEnvironment};
    ///
    /// let mut injector = StyleInjector::new();
    /// injector.set_environment(InjectionEnvironment::Server);
    /// ```
    pub fn set_environment(&mut self, environment: InjectionEnvironment) {
        self.environment = environment;
    }

    /// 获取当前注入环境
    ///
    /// 返回样式注入器当前使用的环境设置。
    ///
    /// # Returns
    ///
    /// 当前的注入环境
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let injector = StyleInjector::new();
    /// let env = injector.environment();
    /// ```
    pub fn environment(&self) -> InjectionEnvironment {
        self.environment
    }

    /// Inject a style with the given class name
    ///
    /// 注入CSS样式并将其与指定的类名关联。
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
    /// let css = ".button { color: blue; }";
    /// let class_name = "button-style";
    /// injector.inject_style(css, class_name).unwrap();
    /// ```
    pub fn inject_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        // 记录样式
        {
            let mut styles = self.injected_styles.lock().map_err(|e| {
                InjectionError::InjectionFailed(format!("Failed to lock styles: {}", e))
            })?;
            styles.insert(class_name.to_string(), css.to_string());
        }

        // 根据环境选择注入策略
        match self.environment {
            InjectionEnvironment::Browser => self.inject_browser_style(css, class_name),
            InjectionEnvironment::Server => self.inject_server_style(css, class_name),
            InjectionEnvironment::Isomorphic => {
                // 同时尝试两种注入方式
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = self.inject_browser_style(css, class_name);
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = self.inject_server_style(css, class_name);
                }

                Ok(())
            }
            InjectionEnvironment::Noop => Ok(()),
        }
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
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let injector = StyleInjector::new();
    /// let css = ".button { color: blue; }";
    /// let class_name = "button-style";
    /// injector.inject_style(css, class_name).unwrap();
    ///
    /// // 移除样式
    /// injector.remove_style(class_name).unwrap();
    /// ```
    pub fn remove_style(&self, class_name: &str) -> Result<(), InjectionError> {
        // 从记录中移除样式
        {
            let mut styles = self.injected_styles.lock().map_err(|e| {
                InjectionError::RemovalFailed(format!("Failed to lock styles: {}", e))
            })?;
            styles.remove(class_name);
        }

        // 根据环境选择移除策略
        match self.environment {
            InjectionEnvironment::Browser => self.remove_browser_style(class_name),
            InjectionEnvironment::Server => self.remove_server_style(class_name),
            InjectionEnvironment::Isomorphic => {
                // 同时尝试两种移除方式
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = self.remove_browser_style(class_name);
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = self.remove_server_style(class_name);
                }

                Ok(())
            }
            InjectionEnvironment::Noop => Ok(()),
        }
    }

    /// Clear all injected styles
    ///
    /// 清除所有通过样式注入器注入的样式。
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
    /// injector.inject_style(".btn { color: blue; }", "btn-style").unwrap();
    /// injector.inject_style(".card { margin: 16px; }", "card-style").unwrap();
    ///
    /// // 清除所有样式
    /// injector.clear_all_styles().unwrap();
    /// ```
    pub fn clear_all_styles(&self) -> Result<(), InjectionError> {
        // 清除记录
        {
            let mut styles = self.injected_styles.lock().map_err(|e| {
                InjectionError::ClearFailed(format!("Failed to lock styles: {}", e))
            })?;
            styles.clear();
        }

        // 根据环境选择清除策略
        match self.environment {
            InjectionEnvironment::Browser => self.clear_browser_styles(),
            InjectionEnvironment::Server => self.clear_server_styles(),
            InjectionEnvironment::Isomorphic => {
                // 同时尝试两种清除方式
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = self.clear_browser_styles();
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = self.clear_server_styles();
                }

                Ok(())
            }
            InjectionEnvironment::Noop => Ok(()),
        }
    }

    /// Inject style in browser environment
    ///
    /// 在浏览器环境中注入样式，通过DOM操作添加样式元素。
    ///
    /// # Arguments
    ///
    /// * `css` - 要注入的CSS样式字符串
    /// * `class_name` - 与样式关联的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(target_arch = "wasm32")]
    fn inject_browser_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        use wasm_bindgen::JsCast;
        use web_sys::{window, Document, Element, HtmlStyleElement};

        // 获取window和document
        let window = window()
            .ok_or_else(|| InjectionError::InjectionFailed("Failed to get window".to_string()))?;

        let document = window
            .document()
            .ok_or_else(|| InjectionError::InjectionFailed("Failed to get document".to_string()))?;

        // 创建或获取样式容器
        let style_element = self.get_or_create_style_element(&document)?;

        // 格式化CSS为作用域样式
        let scoped_css = format!(".{} {{ {} }}", class_name, css);

        // 添加到样式元素
        let current_content = style_element.text_content().unwrap_or_default();
        style_element.set_text_content(Some(&format!("{}\n{}", current_content, scoped_css)));

        Ok(())
    }

    /// Remove style in browser environment
    ///
    /// 在浏览器环境中移除样式，通过DOM操作更新样式元素。
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要移除的样式的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(target_arch = "wasm32")]
    fn remove_browser_style(&self, class_name: &str) -> Result<(), InjectionError> {
        use js_sys::RegExp;
        use web_sys::{window, Document, Element, HtmlStyleElement};

        // 获取window和document
        let window = window()
            .ok_or_else(|| InjectionError::RemovalFailed("Failed to get window".to_string()))?;

        let document = window
            .document()
            .ok_or_else(|| InjectionError::RemovalFailed("Failed to get document".to_string()))?;

        // 查找样式元素
        if let Some(style_element) = document.get_element_by_id("css-in-rust-styles") {
            // 获取当前内容
            let current_content = style_element.text_content().unwrap_or_default();

            // 移除匹配的样式规则
            // 注意：这是一个简单的实现，实际上应该使用CSS解析器
            let re = RegExp::new(&format!(r"\.{}\s*\{{[^}}]*\}}", class_name), "g");
            let pattern = re.to_string().as_string().unwrap_or_default();
            let new_content = current_content.replace(&pattern, "");

            // 更新样式元素
            style_element.set_text_content(Some(&new_content));
        }

        Ok(())
    }

    /// Clear all styles in browser environment
    ///
    /// 在浏览器环境中清除所有样式，通过DOM操作移除样式元素。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(target_arch = "wasm32")]
    fn clear_browser_styles(&self) -> Result<(), InjectionError> {
        use web_sys::{window, Document};

        // 获取window和document
        let window = window()
            .ok_or_else(|| InjectionError::ClearFailed("Failed to get window".to_string()))?;

        let document = window
            .document()
            .ok_or_else(|| InjectionError::ClearFailed("Failed to get document".to_string()))?;

        // 移除样式元素
        if let Some(style_element) = document.get_element_by_id("css-in-rust-styles") {
            if let Some(parent) = style_element.parent_node() {
                parent.remove_child(&style_element).map_err(|e| {
                    InjectionError::ClearFailed(format!("Failed to remove style element: {:?}", e))
                })?;
            }
        }

        Ok(())
    }

    /// Get or create style element
    ///
    /// 获取或创建样式元素，用于注入CSS。
    ///
    /// # Arguments
    ///
    /// * `document` - DOM文档对象
    ///
    /// # Returns
    ///
    /// 成功时返回样式元素
    #[cfg(target_arch = "wasm32")]
    fn get_or_create_style_element(
        &self,
        document: &web_sys::Document,
    ) -> Result<web_sys::HtmlStyleElement, InjectionError> {
        use wasm_bindgen::JsCast;

        // 查找现有的样式元素
        if let Some(element) = document.get_element_by_id("css-in-rust-styles") {
            return element
                .dyn_into::<web_sys::HtmlStyleElement>()
                .map_err(|_| {
                    InjectionError::InjectionFailed(
                        "Failed to cast to HtmlStyleElement".to_string(),
                    )
                });
        }

        // 创建新的样式元素
        let style_element = document
            .create_element("style")
            .map_err(|_| {
                InjectionError::InjectionFailed("Failed to create style element".to_string())
            })?
            .dyn_into::<web_sys::HtmlStyleElement>()
            .map_err(|_| {
                InjectionError::InjectionFailed("Failed to cast to HtmlStyleElement".to_string())
            })?;

        style_element.set_id("css-in-rust-styles");
        style_element.set_type("text/css");

        // 添加到文档头部
        let head = document.head().ok_or_else(|| {
            InjectionError::InjectionFailed("Failed to get document head".to_string())
        })?;

        head.append_child(&style_element).map_err(|_| {
            InjectionError::InjectionFailed("Failed to append style element".to_string())
        })?;

        Ok(style_element)
    }

    /// Inject style in server environment (SSR)
    ///
    /// 在服务器端渲染环境中注入样式。
    /// 收集样式以便稍后在HTML响应中包含它们。
    ///
    /// # Arguments
    ///
    /// * `css` - 要注入的CSS样式字符串
    /// * `class_name` - 与样式关联的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(not(target_arch = "wasm32"))]
    fn inject_server_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        // 在服务端模式下，将样式存储在静态集合中
        let mut styles = SERVER_STYLES.write().map_err(|e| {
            InjectionError::InjectionFailed(format!("无法获取服务端样式写锁: {}", e))
        })?;

        // 存储样式，以便稍后生成
        styles.insert(class_name.to_string(), css.to_string());

        Ok(())
    }

    /// Remove style in server environment (SSR)
    ///
    /// 在服务器端渲染环境中移除样式。
    /// 从收集的样式集合中移除指定的样式。
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要移除的样式的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(not(target_arch = "wasm32"))]
    fn remove_server_style(&self, class_name: &str) -> Result<(), InjectionError> {
        // 在服务端模式下，从静态集合中移除样式
        let mut styles = SERVER_STYLES
            .write()
            .map_err(|e| InjectionError::RemovalFailed(format!("无法获取服务端样式写锁: {}", e)))?;

        // 移除样式
        styles.remove(class_name);

        Ok(())
    }

    /// Clear all styles in server environment (SSR)
    ///
    /// 在服务器端渲染环境中清除所有样式。
    /// 清除所有收集的样式。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`
    #[cfg(not(target_arch = "wasm32"))]
    fn clear_server_styles(&self) -> Result<(), InjectionError> {
        // 在服务端模式下，清空静态集合
        let mut styles = SERVER_STYLES
            .write()
            .map_err(|e| InjectionError::RemovalFailed(format!("无法获取服务端样式写锁: {}", e)))?;

        // 清空样式集合
        styles.clear();

        Ok(())
    }

    /// 获取收集的服务端样式
    ///
    /// 返回所有收集的服务端样式，用于SSR渲染。
    ///
    /// # Returns
    ///
    /// 包含类名和CSS内容的HashMap
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let injector = StyleInjector::new();
    /// injector.inject_style(".btn { color: blue; }", "btn-style").unwrap();
    ///
    /// // 获取所有收集的样式
    /// let styles = injector.get_collected_styles();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_collected_styles(&self) -> Result<HashMap<String, String>, InjectionError> {
        let styles = SERVER_STYLES.read().map_err(|e| {
            InjectionError::InjectionFailed(format!("无法获取服务端样式读锁: {}", e))
        })?;

        Ok(styles.clone())
    }

    /// 生成HTML样式标签
    ///
    /// 将收集的样式转换为HTML样式标签字符串，用于插入到SSR响应中。
    ///
    /// # Returns
    ///
    /// 包含所有样式的HTML字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleInjector;
    ///
    /// let injector = StyleInjector::new();
    /// injector.inject_style(".btn { color: blue; }", "btn-style").unwrap();
    ///
    /// // 生成HTML样式标签
    /// let style_html = injector.generate_style_html().unwrap();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn generate_style_html(&self) -> Result<String, InjectionError> {
        let styles = self.get_collected_styles()?;

        if styles.is_empty() {
            return Ok(String::new());
        }

        let mut html = String::new();

        for (class_name, css) in styles {
            html.push_str(&format!(
                "<style data-css-class=\"{}\">\n.{} {{ {} }}\n</style>\n",
                class_name, class_name, css
            ));
        }

        Ok(html)
    }
}

#[cfg(target_arch = "wasm32")]
impl StyleInjector {
    /// 在浏览器环境中，这是一个空方法，因为无法获取服务端样式
    pub fn get_collected_styles(&self) -> Result<HashMap<String, String>, InjectionError> {
        // 返回本地缓存的样式
        let styles = self.injected_styles.lock().map_err(|e| {
            InjectionError::InjectionFailed(format!("Failed to lock styles: {}", e))
        })?;

        Ok(styles.clone())
    }

    /// 在浏览器环境中，这是一个空方法，因为无法生成服务端HTML
    pub fn generate_style_html(&self) -> Result<String, InjectionError> {
        // 在客户端环境中，这个方法通常不会被调用
        // 但为了API一致性，我们提供一个实现
        Ok(String::new())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl StyleInjector {
    /// 在服务端环境中，这是一个空方法，因为无法执行浏览器端注入
    fn inject_browser_style(&self, _css: &str, _class_name: &str) -> Result<(), InjectionError> {
        // 服务端环境中，这是一个空操作
        Ok(())
    }

    /// 在服务端环境中，这是一个空方法，因为无法执行浏览器端移除
    fn remove_browser_style(&self, _class_name: &str) -> Result<(), InjectionError> {
        // 服务端环境中，这是一个空操作
        Ok(())
    }

    /// 在服务端环境中，这是一个空方法，因为无法执行浏览器端清除
    fn clear_browser_styles(&self) -> Result<(), InjectionError> {
        // 服务端环境中，这是一个空操作
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl StyleInjector {
    /// 在浏览器环境中，这是一个空方法，因为无法执行服务端注入
    fn inject_server_style(&self, _css: &str, _class_name: &str) -> Result<(), InjectionError> {
        // 浏览器环境中，这是一个空操作
        Ok(())
    }

    /// 在浏览器环境中，这是一个空方法，因为无法执行服务端移除
    fn remove_server_style(&self, _class_name: &str) -> Result<(), InjectionError> {
        // 浏览器环境中，这是一个空操作
        Ok(())
    }

    /// 在浏览器环境中，这是一个空方法，因为无法执行服务端清除
    fn clear_server_styles(&self) -> Result<(), InjectionError> {
        // 浏览器环境中，这是一个空操作
        Ok(())
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

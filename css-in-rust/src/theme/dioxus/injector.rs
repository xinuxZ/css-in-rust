use std::collections::HashSet;

/// Dioxus专用样式注入器
///
/// 提供与Dioxus框架紧密集成的样式注入功能
#[derive(Debug, Clone)]
pub struct DioxusStyleInjector {
    /// 是否使用SSR模式
    ssr_mode: bool,
    /// 已注入的样式ID集合
    injected_styles: HashSet<String>,
    /// 是否启用缓存
    enable_cache: bool,
}

impl DioxusStyleInjector {
    /// 创建新的注入器
    pub fn new() -> Self {
        Self {
            ssr_mode: false,
            injected_styles: HashSet::new(),
            enable_cache: true,
        }
    }

    /// 注入样式
    pub fn inject_style(&mut self, css: &str, id: &str) -> bool {
        // 如果已经注入过，则跳过
        if self.injected_styles.contains(id) {
            return false;
        }

        // 记录样式ID
        self.injected_styles.insert(id.to_string());

        // 在客户端模式下注入样式
        if !self.ssr_mode {
            #[cfg(target_arch = "wasm32")]
            {
                self.inject_to_dom(css, id);
            }
        }

        true
    }

    /// 设置SSR模式
    pub fn with_ssr(mut self, ssr_mode: bool) -> Self {
        self.ssr_mode = ssr_mode;
        self
    }

    /// 设置是否启用缓存
    pub fn with_cache(mut self, enable_cache: bool) -> Self {
        self.enable_cache = enable_cache;
        self
    }

    /// 获取所有注入的样式
    pub fn get_injected_styles(&self) -> Vec<String> {
        self.injected_styles.iter().cloned().collect()
    }

    /// 清除所有注入的样式
    pub fn clear_styles(&mut self) {
        self.injected_styles.clear();

        // 在客户端模式下移除DOM中的样式
        #[cfg(target_arch = "wasm32")]
        {
            self.remove_all_styles_from_dom();
        }
    }

    /// 注入样式到DOM
    #[cfg(target_arch = "wasm32")]
    fn inject_to_dom(&self, css: &str, id: &str) {
        use wasm_bindgen::JsCast;
        use web_sys::{window, Document, Element, HtmlStyleElement};

        // 获取window和document
        let window = match window() {
            Some(win) => win,
            None => return,
        };

        let document = match window.document() {
            Some(doc) => doc,
            None => return,
        };

        // 检查是否已存在相同ID的样式元素
        if document.get_element_by_id(id).is_some() {
            return;
        }

        // 创建style元素
        let style_element = match document.create_element("style") {
            Ok(el) => el,
            Err(_) => return,
        };

        // 设置ID和内容
        style_element.set_id(id);
        style_element.set_text_content(Some(css));

        // 添加到head
        if let Some(head) = document.head() {
            let _ = head.append_child(&style_element);
        }
    }

    /// 从DOM移除所有样式
    #[cfg(target_arch = "wasm32")]
    fn remove_all_styles_from_dom(&self) {
        use web_sys::{window, Document};

        // 获取window和document
        let window = match window() {
            Some(win) => win,
            None => return,
        };

        let document = match window.document() {
            Some(doc) => doc,
            None => return,
        };

        // 获取head
        let head = match document.head() {
            Some(h) => h,
            None => return,
        };

        // 遍历所有已注入的样式ID
        for id in &self.injected_styles {
            if let Some(element) = document.get_element_by_id(id) {
                let _ = head.remove_child(&element);
            }
        }
    }
}

impl Default for DioxusStyleInjector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injector_basic() {
        let mut injector = DioxusStyleInjector::new();

        // 测试注入样式
        assert!(injector.inject_style("body { color: red; }", "test-style"));

        // 测试重复注入
        assert!(!injector.inject_style("body { color: red; }", "test-style"));

        // 测试获取注入的样式
        let styles = injector.get_injected_styles();
        assert_eq!(styles.len(), 1);
        assert!(styles.contains(&"test-style".to_string()));

        // 测试清除样式
        injector.clear_styles();
        assert_eq!(injector.get_injected_styles().len(), 0);
    }

    #[test]
    fn test_injector_config() {
        let injector = DioxusStyleInjector::new().with_ssr(true).with_cache(false);

        assert!(injector.ssr_mode);
        assert!(!injector.enable_cache);
    }
}

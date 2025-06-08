//! 服务端渲染模块
//!
//! 本模块负责在服务端渲染环境中处理样式，包括样式提取、注入和客户端水合。
//! 职责：确保样式在SSR环境中正确工作，提供关键CSS和样式水合功能。
//!
//! # 主要组件
//!
//! - `ServerStyleSheet`: 服务端样式表，表示一个样式片段
//! - `StyleSheetManager`: 样式表管理器，管理多个样式表
//! - `StyleExtractor`: 样式提取器，从缓存中提取样式
//! - `StyleHydration`: 样式水合器，在客户端水合服务端样式
//!
//! # 示例
//!
//! ```
//! use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
//!
//! // 创建样式表
//! let critical_sheet = ServerStyleSheet::new("main-styles", "body { color: red; }", true);
//!
//! // 添加到管理器
//! let mut manager = StyleSheetManager::new();
//! manager.add_sheet(critical_sheet);
//!
//! // 生成HTML样式标签
//! let html_tags = manager.to_style_tags();
//! ```

mod extractor;
mod hydration;

pub use extractor::StyleExtractor;
pub use hydration::StyleHydration;

/// 服务端渲染的样式表
///
/// 表示一个服务端渲染的样式片段，包含ID、CSS内容、哈希值和是否为关键CSS。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::ssr::ServerStyleSheet;
///
/// // 创建一个关键样式表
/// let critical_sheet = ServerStyleSheet::new("app-styles", "body { color: #333; }", true);
///
/// // 创建一个非关键样式表
/// let non_critical_sheet = ServerStyleSheet::new("theme-styles", "button { color: blue; }", false);
///
/// // 生成HTML样式标签
/// let html = critical_sheet.to_style_tag();
/// ```
#[derive(Debug, Clone)]
pub struct ServerStyleSheet {
    /// 样式表ID
    pub id: String,
    /// CSS内容
    pub css: String,
    /// 样式表哈希
    pub hash: String,
    /// 是否为关键CSS
    pub is_critical: bool,
}

impl ServerStyleSheet {
    /// 创建新的服务端样式表
    ///
    /// # Arguments
    ///
    /// * `id` - 样式表ID
    /// * `css` - CSS内容
    /// * `is_critical` - 是否为关键CSS
    ///
    /// # Returns
    ///
    /// 新创建的服务端样式表
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::ServerStyleSheet;
    ///
    /// let sheet = ServerStyleSheet::new("app-styles", "body { color: #333; }", true);
    /// assert_eq!(sheet.id, "app-styles");
    /// assert_eq!(sheet.css, "body { color: #333; }");
    /// assert!(sheet.is_critical);
    /// ```
    pub fn new(id: &str, css: &str, is_critical: bool) -> Self {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(css.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Self {
            id: id.to_string(),
            css: css.to_string(),
            hash,
            is_critical,
        }
    }

    /// 生成样式标签
    ///
    /// 将样式表转换为HTML样式标签。
    ///
    /// # Returns
    ///
    /// HTML样式标签字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::ServerStyleSheet;
    ///
    /// let sheet = ServerStyleSheet::new("app-styles", "body { color: #333; }", true);
    /// let html = sheet.to_style_tag();
    /// assert!(html.contains("<style"));
    /// assert!(html.contains("id=\"app-styles\""));
    /// assert!(html.contains("data-critical=\"true\""));
    /// ```
    pub fn to_style_tag(&self) -> String {
        format!(
            r#"<style id="{}" data-hash="{}" {}>{}</style>"#,
            self.id,
            self.hash,
            if self.is_critical {
                "data-critical=\"true\""
            } else {
                ""
            },
            self.css
        )
    }

    /// 生成样式链接标签（用于外部样式表）
    ///
    /// 将样式表转换为HTML链接标签，用于引用外部CSS文件。
    ///
    /// # Arguments
    ///
    /// * `href` - 样式表URL
    ///
    /// # Returns
    ///
    /// HTML链接标签字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::ServerStyleSheet;
    ///
    /// let sheet = ServerStyleSheet::new("app-styles", "body { color: #333; }", true);
    /// let html = sheet.to_link_tag("/styles/app.css");
    /// assert!(html.contains("<link"));
    /// assert!(html.contains("href=\"/styles/app.css\""));
    /// ```
    pub fn to_link_tag(&self, href: &str) -> String {
        format!(
            r#"<link rel="stylesheet" id="{}" href="{}" data-hash="{}" {}>"#,
            self.id,
            href,
            self.hash,
            if self.is_critical {
                "data-critical=\"true\""
            } else {
                ""
            }
        )
    }
}

/// 样式表管理器
///
/// 管理多个样式表，区分关键和非关键样式。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
///
/// // 创建样式表管理器
/// let mut manager = StyleSheetManager::new();
///
/// // 添加样式表
/// let critical_sheet = ServerStyleSheet::new("app-styles", "body { color: #333; }", true);
/// let non_critical_sheet = ServerStyleSheet::new("theme-styles", "button { color: blue; }", false);
///
/// manager.add_sheet(critical_sheet);
/// manager.add_sheet(non_critical_sheet);
///
/// // 生成HTML样式标签
/// let html = manager.to_style_tags();
/// ```
#[derive(Debug, Default)]
pub struct StyleSheetManager {
    /// 关键样式表
    critical_sheets: Vec<ServerStyleSheet>,
    /// 非关键样式表
    normal_sheets: Vec<ServerStyleSheet>,
}

impl StyleSheetManager {
    /// 创建新的样式表管理器
    ///
    /// # Returns
    ///
    /// 新创建的样式表管理器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::StyleSheetManager;
    ///
    /// let manager = StyleSheetManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            critical_sheets: Vec::new(),
            normal_sheets: Vec::new(),
        }
    }

    /// 添加样式表
    ///
    /// 根据样式表的is_critical属性，将其添加到关键或非关键样式表列表中。
    ///
    /// # Arguments
    ///
    /// * `sheet` - 要添加的样式表
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
    ///
    /// let mut manager = StyleSheetManager::new();
    /// let sheet = ServerStyleSheet::new("app-styles", "body { color: #333; }", true);
    /// manager.add_sheet(sheet);
    /// ```
    pub fn add_sheet(&mut self, sheet: ServerStyleSheet) {
        if sheet.is_critical {
            self.critical_sheets.push(sheet);
        } else {
            self.normal_sheets.push(sheet);
        }
    }

    /// 获取所有关键样式表
    ///
    /// # Returns
    ///
    /// 关键样式表切片
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
    ///
    /// let mut manager = StyleSheetManager::new();
    /// let sheet = ServerStyleSheet::new("app-styles", "body { color: #333; }", true);
    /// manager.add_sheet(sheet);
    ///
    /// let critical_sheets = manager.critical_sheets();
    /// assert_eq!(critical_sheets.len(), 1);
    /// ```
    pub fn critical_sheets(&self) -> &[ServerStyleSheet] {
        &self.critical_sheets
    }

    /// 获取所有非关键样式表
    ///
    /// # Returns
    ///
    /// 非关键样式表切片
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
    ///
    /// let mut manager = StyleSheetManager::new();
    /// let sheet = ServerStyleSheet::new("theme-styles", "button { color: blue; }", false);
    /// manager.add_sheet(sheet);
    ///
    /// let normal_sheets = manager.normal_sheets();
    /// assert_eq!(normal_sheets.len(), 1);
    /// ```
    pub fn normal_sheets(&self) -> &[ServerStyleSheet] {
        &self.normal_sheets
    }

    /// 生成所有样式标签
    ///
    /// 将所有样式表转换为HTML样式标签，先输出关键样式，再输出非关键样式。
    ///
    /// # Returns
    ///
    /// HTML样式标签字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
    ///
    /// let mut manager = StyleSheetManager::new();
    /// manager.add_sheet(ServerStyleSheet::new("app-styles", "body { color: #333; }", true));
    /// manager.add_sheet(ServerStyleSheet::new("theme-styles", "button { color: blue; }", false));
    ///
    /// let html = manager.to_style_tags();
    /// ```
    pub fn to_style_tags(&self) -> String {
        let mut tags = String::new();

        // 先添加关键样式
        for sheet in &self.critical_sheets {
            tags.push_str(&sheet.to_style_tag());
        }

        // 再添加非关键样式
        for sheet in &self.normal_sheets {
            tags.push_str(&sheet.to_style_tag());
        }

        tags
    }

    /// 生成关键样式标签
    ///
    /// 只将关键样式表转换为HTML样式标签。
    ///
    /// # Returns
    ///
    /// HTML样式标签字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
    ///
    /// let mut manager = StyleSheetManager::new();
    /// manager.add_sheet(ServerStyleSheet::new("app-styles", "body { color: #333; }", true));
    ///
    /// let html = manager.to_critical_style_tags();
    /// ```
    pub fn to_critical_style_tags(&self) -> String {
        let mut tags = String::new();

        for sheet in &self.critical_sheets {
            tags.push_str(&sheet.to_style_tag());
        }

        tags
    }

    /// 合并样式表
    ///
    /// 将另一个样式表管理器的样式表合并到当前管理器。
    ///
    /// # Arguments
    ///
    /// * `other` - 要合并的样式表管理器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
    ///
    /// let mut manager1 = StyleSheetManager::new();
    /// manager1.add_sheet(ServerStyleSheet::new("app-styles", "body { color: #333; }", true));
    ///
    /// let mut manager2 = StyleSheetManager::new();
    /// manager2.add_sheet(ServerStyleSheet::new("theme-styles", "button { color: blue; }", false));
    ///
    /// manager1.merge(&manager2);
    /// ```
    pub fn merge(&mut self, other: &StyleSheetManager) {
        self.critical_sheets
            .extend_from_slice(&other.critical_sheets);
        self.normal_sheets.extend_from_slice(&other.normal_sheets);
    }

    /// 清空样式表
    ///
    /// 移除所有样式表。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::ssr::{ServerStyleSheet, StyleSheetManager};
    ///
    /// let mut manager = StyleSheetManager::new();
    /// manager.add_sheet(ServerStyleSheet::new("app-styles", "body { color: #333; }", true));
    ///
    /// manager.clear();
    /// assert_eq!(manager.critical_sheets().len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.critical_sheets.clear();
        self.normal_sheets.clear();
    }
}

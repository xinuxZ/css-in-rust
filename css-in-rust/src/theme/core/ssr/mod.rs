mod extractor;
mod hydration;

pub use extractor::StyleExtractor;
pub use hydration::StyleHydration;

/// 服务端渲染的样式表
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
#[derive(Debug, Default)]
pub struct StyleSheetManager {
    /// 关键样式表
    critical_sheets: Vec<ServerStyleSheet>,
    /// 非关键样式表
    normal_sheets: Vec<ServerStyleSheet>,
}

impl StyleSheetManager {
    /// 创建新的样式表管理器
    pub fn new() -> Self {
        Self {
            critical_sheets: Vec::new(),
            normal_sheets: Vec::new(),
        }
    }

    /// 添加样式表
    pub fn add_sheet(&mut self, sheet: ServerStyleSheet) {
        if sheet.is_critical {
            self.critical_sheets.push(sheet);
        } else {
            self.normal_sheets.push(sheet);
        }
    }

    /// 获取所有关键样式表
    pub fn critical_sheets(&self) -> &[ServerStyleSheet] {
        &self.critical_sheets
    }

    /// 获取所有非关键样式表
    pub fn normal_sheets(&self) -> &[ServerStyleSheet] {
        &self.normal_sheets
    }

    /// 生成所有样式标签
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
    pub fn to_critical_style_tags(&self) -> String {
        let mut tags = String::new();

        for sheet in &self.critical_sheets {
            tags.push_str(&sheet.to_style_tag());
        }

        tags
    }

    /// 合并样式表
    pub fn merge(&mut self, other: &StyleSheetManager) {
        self.critical_sheets
            .extend_from_slice(&other.critical_sheets);
        self.normal_sheets.extend_from_slice(&other.normal_sheets);
    }

    /// 清空样式表
    pub fn clear(&mut self) {
        self.critical_sheets.clear();
        self.normal_sheets.clear();
    }
}

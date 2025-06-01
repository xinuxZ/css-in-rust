//! 错误报告模块
//!
//! 提供CSS错误的格式化和报告功能

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

/// 错误级别
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorLevel {
    /// 错误
    Error,
    /// 警告
    Warning,
    /// 信息
    Info,
    /// 提示
    Hint,
}

/// 错误类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorType {
    /// 语法错误
    SyntaxError,
    /// 语义错误
    SemanticError,
    /// 类型错误
    TypeError,
    /// 未定义错误
    UndefinedError,
    /// 重复定义错误
    DuplicateError,
    /// 性能警告
    PerformanceWarning,
    /// 兼容性警告
    CompatibilityWarning,
    /// 可访问性警告
    AccessibilityWarning,
    /// 最佳实践建议
    BestPracticeHint,
    /// 代码风格建议
    StyleHint,
}

/// 错误位置
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorPosition {
    /// 行号（从1开始）
    pub line: usize,
    /// 列号（从1开始）
    pub column: usize,
    /// 字符偏移量（从0开始）
    pub offset: usize,
}

/// 错误范围
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorRange {
    /// 开始位置
    pub start: ErrorPosition,
    /// 结束位置
    pub end: ErrorPosition,
}

/// 错误信息
#[derive(Debug, Clone)]
pub struct ErrorInfo {
    /// 错误ID
    pub id: String,
    /// 错误级别
    pub level: ErrorLevel,
    /// 错误类型
    pub error_type: ErrorType,
    /// 错误消息
    pub message: String,
    /// 详细描述
    pub description: Option<String>,
    /// 错误范围
    pub range: ErrorRange,
    /// 文件路径
    pub file_path: Option<PathBuf>,
    /// 相关信息
    pub related_information: Vec<RelatedInformation>,
    /// 修复建议
    pub fixes: Vec<CodeFix>,
    /// 错误代码
    pub code: Option<String>,
    /// 错误源
    pub source: String,
    /// 标签
    pub tags: Vec<ErrorTag>,
}

/// 相关信息
#[derive(Debug, Clone)]
pub struct RelatedInformation {
    /// 位置
    pub location: ErrorRange,
    /// 消息
    pub message: String,
    /// 文件路径
    pub file_path: Option<PathBuf>,
}

/// 代码修复
#[derive(Debug, Clone)]
pub struct CodeFix {
    /// 修复标题
    pub title: String,
    /// 修复类型
    pub kind: CodeFixKind,
    /// 文本编辑
    pub edits: Vec<TextEdit>,
    /// 是否首选
    pub is_preferred: bool,
}

/// 代码修复类型
#[derive(Debug, Clone, PartialEq)]
pub enum CodeFixKind {
    /// 快速修复
    QuickFix,
    /// 重构
    Refactor,
    /// 源代码操作
    Source,
    /// 组织导入
    SourceOrganizeImports,
    /// 修复所有
    SourceFixAll,
}

/// 文本编辑
#[derive(Debug, Clone)]
pub struct TextEdit {
    /// 范围
    pub range: ErrorRange,
    /// 新文本
    pub new_text: String,
}

/// 错误标签
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorTag {
    /// 不必要的代码
    Unnecessary,
    /// 已弃用
    Deprecated,
}

/// 错误上下文
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// 文件路径
    pub file_path: Option<PathBuf>,
    /// 源代码
    pub source: Option<String>,
    /// 上下文信息
    pub context_info: HashMap<String, String>,
}

/// 错误报告器
pub struct ErrorReporter {
    /// 错误格式
    format: ErrorFormat,
    /// 错误收集器
    errors: Vec<ErrorInfo>,
    /// 错误计数器
    error_counts: HashMap<ErrorLevel, usize>,
    /// 错误规则配置
    rules: HashMap<String, ErrorRuleConfig>,
    /// 输出配置
    output_config: OutputConfig,
}

/// 错误格式
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorFormat {
    /// 简单格式
    Simple,
    /// 丰富格式
    Rich,
    /// 紧凑格式
    Compact,
    /// JSON格式
    Json,
    /// IDE友好格式
    IdeFriendly,
    /// 自定义格式
    Custom(String),
}

/// 错误规则配置
#[derive(Debug, Clone)]
pub struct ErrorRuleConfig {
    /// 是否启用
    pub enabled: bool,
    /// 错误级别
    pub level: ErrorLevel,
    /// 自定义消息
    pub custom_message: Option<String>,
}

/// 输出配置
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// 是否显示源代码
    pub show_source: bool,
    /// 是否显示行号
    pub show_line_numbers: bool,
    /// 是否显示列号
    pub show_column_numbers: bool,
    /// 是否显示错误代码
    pub show_error_codes: bool,
    /// 是否显示修复建议
    pub show_fixes: bool,
    /// 是否使用颜色
    pub use_colors: bool,
    /// 最大错误数量
    pub max_errors: Option<usize>,
    /// 上下文行数
    pub context_lines: usize,
}

impl ErrorReporter {
    /// 创建新的错误报告器
    pub fn new(format: ErrorFormat) -> Self {
        Self {
            format,
            errors: Vec::new(),
            error_counts: HashMap::new(),
            rules: HashMap::new(),
            output_config: OutputConfig::default(),
        }
    }

    /// 添加错误
    pub fn add_error(&mut self, error: ErrorInfo) {
        // 检查规则配置
        if let Some(rule) = self.rules.get(&error.id) {
            if !rule.enabled {
                return;
            }
        }

        // 更新错误计数
        *self.error_counts.entry(error.level.clone()).or_insert(0) += 1;

        // 添加错误
        self.errors.push(error);

        // 检查最大错误数量
        if let Some(max_errors) = self.output_config.max_errors {
            if self.errors.len() > max_errors {
                self.errors.truncate(max_errors);
            }
        }
    }

    /// 批量添加错误
    pub fn add_errors(&mut self, errors: Vec<ErrorInfo>) {
        for error in errors {
            self.add_error(error);
        }
    }

    /// 清除所有错误
    pub fn clear(&mut self) {
        self.errors.clear();
        self.error_counts.clear();
    }

    /// 获取所有错误
    pub fn get_errors(&self) -> &[ErrorInfo] {
        &self.errors
    }

    /// 获取错误计数
    pub fn get_error_count(&self, level: &ErrorLevel) -> usize {
        self.error_counts.get(level).copied().unwrap_or(0)
    }

    /// 获取总错误数
    pub fn get_total_errors(&self) -> usize {
        self.error_counts.values().sum()
    }

    /// 设置是否使用颜色
    pub fn set_use_colors(&mut self, use_colors: bool) {
        self.output_config.use_colors = use_colors;
    }

    /// 设置是否显示建议
    pub fn set_show_suggestions(&mut self, show_suggestions: bool) {
        self.output_config.show_fixes = show_suggestions;
    }

    /// 设置是否显示源码上下文
    pub fn set_show_source_context(&mut self, show_source_context: bool) {
        self.output_config.show_source = show_source_context;
    }

    /// 是否有错误
    pub fn has_errors(&self) -> bool {
        self.get_error_count(&ErrorLevel::Error) > 0
    }

    /// 是否有警告
    pub fn has_warnings(&self) -> bool {
        self.get_error_count(&ErrorLevel::Warning) > 0
    }

    /// 格式化错误报告
    pub fn format_report(&self) -> String {
        match self.format {
            ErrorFormat::Simple => self.format_simple(),
            ErrorFormat::Rich => self.format_rich(),
            ErrorFormat::Compact => self.format_compact(),
            ErrorFormat::Json => self.format_json(),
            ErrorFormat::IdeFriendly => self.format_ide_friendly(),
            ErrorFormat::Custom(ref template) => self.format_custom(template),
        }
    }

    /// 格式化简单报告
    fn format_simple(&self) -> String {
        let mut output = String::new();

        for error in &self.errors {
            let file_path = error
                .file_path
                .as_ref()
                .map(|p| p.to_string_lossy())
                .unwrap_or_else(|| "<unknown>".into());

            output.push_str(&format!(
                "{}:{}:{}: {}: {}\n",
                file_path,
                error.range.start.line,
                error.range.start.column,
                self.format_level(&error.level),
                error.message
            ));
        }

        // 添加摘要
        output.push_str(&self.format_summary());

        output
    }

    /// 格式化丰富报告
    fn format_rich(&self) -> String {
        let mut output = String::new();

        for (i, error) in self.errors.iter().enumerate() {
            if i > 0 {
                output.push_str("\n");
            }

            output.push_str(&self.format_rich_error(error));
        }

        // 添加摘要
        output.push_str(&format!("\n{}", self.format_summary()));

        output
    }

    /// 格式化紧凑报告
    fn format_compact(&self) -> String {
        let mut output = String::new();

        for error in &self.errors {
            let file_path = error
                .file_path
                .as_ref()
                .map(|p| p.to_string_lossy())
                .unwrap_or_else(|| "<unknown>".into());

            output.push_str(&format!(
                "{}:{}:{}: {}: {}\n",
                file_path,
                error.range.start.line,
                error.range.start.column,
                self.format_level(&error.level),
                error.message
            ));
        }

        output
    }

    /// 格式化丰富错误
    fn format_rich_error(&self, error: &ErrorInfo) -> String {
        let mut output = String::new();

        // 错误头部
        let file_path = error
            .file_path
            .as_ref()
            .map(|p| p.to_string_lossy())
            .unwrap_or_else(|| "<unknown>".into());

        output.push_str(&format!(
            "{} {}:{}:{}\n",
            self.format_level_colored(&error.level),
            file_path,
            error.range.start.line,
            error.range.start.column
        ));

        // 错误消息
        output.push_str(&format!("  {}\n", error.message));

        // 错误代码
        if let Some(ref code) = error.code {
            output.push_str(&format!("  错误代码: {}\n", code));
        }

        // 详细描述
        if let Some(ref description) = error.description {
            output.push_str(&format!("  详细: {}\n", description));
        }

        // 源代码上下文（如果配置启用）
        if self.output_config.show_source {
            output.push_str(&self.format_source_context(error));
        }

        // 修复建议
        if self.output_config.show_fixes && !error.fixes.is_empty() {
            output.push_str(&self.format_fixes(&error.fixes));
        }

        // 相关信息
        if !error.related_information.is_empty() {
            output.push_str(&self.format_related_information(&error.related_information));
        }

        output
    }

    /// 格式化JSON报告
    fn format_json(&self) -> String {
        // 简化的JSON格式实现
        let mut json_errors = Vec::new();

        for error in &self.errors {
            let json_error = format!(
                r#"{{
    "id": "{}",
    "level": "{:?}",
    "type": "{:?}",
    "message": "{}",
    "file": "{}",
    "line": {},
    "column": {},
    "code": "{}"
  }}"#,
                error.id,
                error.level,
                error.error_type,
                error.message.replace('"', "\\\""),
                error
                    .file_path
                    .as_ref()
                    .map(|p| p.to_string_lossy())
                    .unwrap_or_else(|| "<unknown>".into()),
                error.range.start.line,
                error.range.start.column,
                error.code.as_deref().unwrap_or("")
            );
            json_errors.push(json_error);
        }

        format!(
            r#"{{
  "errors": [
{}
  ],
  "summary": {{
    "total": {},
    "errors": {},
    "warnings": {},
    "info": {},
    "hints": {}
  }}
}}"#,
            json_errors.join(",\n"),
            self.get_total_errors(),
            self.get_error_count(&ErrorLevel::Error),
            self.get_error_count(&ErrorLevel::Warning),
            self.get_error_count(&ErrorLevel::Info),
            self.get_error_count(&ErrorLevel::Hint)
        )
    }

    /// 格式化IDE友好报告
    fn format_ide_friendly(&self) -> String {
        let mut output = String::new();

        for error in &self.errors {
            let file_path = error
                .file_path
                .as_ref()
                .map(|p| p.to_string_lossy())
                .unwrap_or_else(|| "<unknown>".into());

            // LSP诊断格式
            output.push_str(&format!(
                "{}:{}:{}-{}:{}: {} [{}] {}\n",
                file_path,
                error.range.start.line,
                error.range.start.column,
                error.range.end.line,
                error.range.end.column,
                self.format_level(&error.level).to_lowercase(),
                error.code.as_deref().unwrap_or(&error.id),
                error.message
            ));
        }

        output
    }

    /// 格式化自定义报告
    fn format_custom(&self, template: &str) -> String {
        // 简化的模板替换实现
        let mut output = template.to_string();

        // 替换占位符
        output = output.replace("{total_errors}", &self.get_total_errors().to_string());
        output = output.replace(
            "{error_count}",
            &self.get_error_count(&ErrorLevel::Error).to_string(),
        );
        output = output.replace(
            "{warning_count}",
            &self.get_error_count(&ErrorLevel::Warning).to_string(),
        );

        // 替换错误列表
        let error_list = self
            .errors
            .iter()
            .map(|e| format!("{}: {}", self.format_level(&e.level), e.message))
            .collect::<Vec<_>>()
            .join("\n");
        output = output.replace("{error_list}", &error_list);

        output
    }

    /// 格式化错误级别
    fn format_level(&self, level: &ErrorLevel) -> &'static str {
        match level {
            ErrorLevel::Error => "error",
            ErrorLevel::Warning => "warning",
            ErrorLevel::Info => "info",
            ErrorLevel::Hint => "hint",
        }
    }

    /// 格式化带颜色的错误级别
    fn format_level_colored(&self, level: &ErrorLevel) -> String {
        if !self.output_config.use_colors {
            return self.format_level(level).to_string();
        }

        match level {
            ErrorLevel::Error => format!("\x1b[31m{}\x1b[0m", "error"),
            ErrorLevel::Warning => format!("\x1b[33m{}\x1b[0m", "warning"),
            ErrorLevel::Info => format!("\x1b[36m{}\x1b[0m", "info"),
            ErrorLevel::Hint => format!("\x1b[32m{}\x1b[0m", "hint"),
        }
    }

    /// 格式化源代码上下文
    fn format_source_context(&self, error: &ErrorInfo) -> String {
        // 如果没有文件路径，返回简单信息
        let file_path = match &error.file_path {
            Some(path) => path,
            None => return String::from("  源代码上下文暂不可用（无文件路径）\n"),
        };

        // 尝试读取源文件并显示上下文
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                let lines: Vec<&str> = content.lines().collect();
                let mut context = String::new();

                let start_line = error.range.start.line;
                let start_column = error.range.start.column;
                let end_line = error.range.end.line;
                let end_column = error.range.end.column;

                // 显示上下文行（前后各2行）
                let context_start = start_line.saturating_sub(3); // 转为0-based索引
                let context_end = std::cmp::min(end_line + 2, lines.len());

                context.push_str(&format!(
                    "  位置: {}:{}:{}\n",
                    file_path.display(),
                    start_line,
                    start_column
                ));
                context.push_str(&format!("  {}\n", "-".repeat(50)));

                for (i, line_content) in lines
                    .iter()
                    .enumerate()
                    .skip(context_start)
                    .take(context_end - context_start)
                {
                    let line_num = i + 1;
                    let is_error_line = line_num >= start_line && line_num <= end_line;
                    let marker = if is_error_line { ">>>" } else { "   " };

                    context.push_str(&format!("  {} {:4} | {}\n", marker, line_num, line_content));

                    // 如果是错误行，显示列位置指示器
                    if line_num == start_line && start_column > 0 {
                        let spaces = " ".repeat(10 + start_column.saturating_sub(1)); // 10 = "  >>> 1234 | ".len()
                        let indicators = if start_line == end_line && end_column > start_column {
                            "^".repeat(end_column - start_column)
                        } else {
                            "^".to_string()
                        };
                        context.push_str(&format!("  {}{}\n", spaces, indicators));
                    }
                }

                context.push_str(&format!("  {}\n", "-".repeat(50)));
                context
            }
            Err(err) => {
                // 如果无法读取文件，返回错误信息
                format!("  源代码上下文暂不可用（读取文件失败: {}）\n", err)
            }
        }
    }

    /// 格式化修复建议
    fn format_fixes(&self, fixes: &[CodeFix]) -> String {
        let mut output = String::from("  修复建议:\n");

        for (i, fix) in fixes.iter().enumerate() {
            output.push_str(&format!("    {}. {}\n", i + 1, fix.title));
        }

        output
    }

    /// 格式化相关信息
    fn format_related_information(&self, related: &[RelatedInformation]) -> String {
        let mut output = String::from("  相关信息:\n");

        for info in related {
            let file_path = info
                .file_path
                .as_ref()
                .map(|p| p.to_string_lossy())
                .unwrap_or_else(|| "<unknown>".into());

            output.push_str(&format!(
                "    {}:{}:{}: {}\n",
                file_path, info.location.start.line, info.location.start.column, info.message
            ));
        }

        output
    }

    /// 格式化摘要
    fn format_summary(&self) -> String {
        let total = self.get_total_errors();
        let errors = self.get_error_count(&ErrorLevel::Error);
        let warnings = self.get_error_count(&ErrorLevel::Warning);
        let info = self.get_error_count(&ErrorLevel::Info);
        let hints = self.get_error_count(&ErrorLevel::Hint);

        if total == 0 {
            return "没有发现问题。".to_string();
        }

        let mut parts = Vec::new();

        if errors > 0 {
            parts.push(format!("{} 个错误", errors));
        }
        if warnings > 0 {
            parts.push(format!("{} 个警告", warnings));
        }
        if info > 0 {
            parts.push(format!("{} 个信息", info));
        }
        if hints > 0 {
            parts.push(format!("{} 个提示", hints));
        }

        format!("发现 {}。", parts.join("，"))
    }

    /// 设置错误格式
    pub fn set_format(&mut self, format: ErrorFormat) {
        self.format = format;
    }

    /// 设置输出配置
    pub fn set_output_config(&mut self, config: OutputConfig) {
        self.output_config = config;
    }

    /// 配置错误规则
    pub fn configure_rule(&mut self, rule_id: String, config: ErrorRuleConfig) {
        self.rules.insert(rule_id, config);
    }

    /// 移除错误规则
    pub fn remove_rule(&mut self, rule_id: &str) {
        self.rules.remove(rule_id);
    }

    /// 获取错误规则配置
    pub fn get_rule_config(&self, rule_id: &str) -> Option<&ErrorRuleConfig> {
        self.rules.get(rule_id)
    }

    /// 按级别过滤错误
    pub fn filter_by_level(&self, level: ErrorLevel) -> Vec<&ErrorInfo> {
        self.errors
            .iter()
            .filter(|error| error.level == level)
            .collect()
    }

    /// 按类型过滤错误
    pub fn filter_by_type(&self, error_type: ErrorType) -> Vec<&ErrorInfo> {
        self.errors
            .iter()
            .filter(|error| error.error_type == error_type)
            .collect()
    }

    /// 按文件过滤错误
    pub fn filter_by_file(&self, file_path: &PathBuf) -> Vec<&ErrorInfo> {
        self.errors
            .iter()
            .filter(|error| error.file_path.as_ref() == Some(file_path))
            .collect()
    }

    /// 导出错误报告到文件
    pub fn export_to_file(&self, file_path: &PathBuf) -> Result<(), std::io::Error> {
        use std::fs;
        let report = self.format_report();
        fs::write(file_path, report)
    }

    /// 获取错误统计信息
    pub fn get_statistics(&self) -> ErrorStatistics {
        let mut file_counts = HashMap::new();
        let mut type_counts = HashMap::new();

        for error in &self.errors {
            // 按文件统计
            let file_key = error
                .file_path
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "<unknown>".to_string());
            *file_counts.entry(file_key).or_insert(0) += 1;

            // 按类型统计
            *type_counts.entry(error.error_type.clone()).or_insert(0) += 1;
        }

        ErrorStatistics {
            total_errors: self.get_total_errors(),
            error_counts: self.error_counts.clone(),
            file_counts,
            type_counts,
        }
    }
}

/// 错误统计信息
#[derive(Debug, Clone)]
pub struct ErrorStatistics {
    /// 总错误数
    pub total_errors: usize,
    /// 按级别统计
    pub error_counts: HashMap<ErrorLevel, usize>,
    /// 按文件统计
    pub file_counts: HashMap<String, usize>,
    /// 按类型统计
    pub type_counts: HashMap<ErrorType, usize>,
}

/// 错误构建器
pub struct ErrorBuilder {
    error: ErrorInfo,
}

impl ErrorBuilder {
    /// 创建新的错误构建器
    pub fn new(id: String, level: ErrorLevel, message: String) -> Self {
        Self {
            error: ErrorInfo {
                id,
                level,
                error_type: ErrorType::SyntaxError,
                message,
                description: None,
                range: ErrorRange {
                    start: ErrorPosition {
                        line: 1,
                        column: 1,
                        offset: 0,
                    },
                    end: ErrorPosition {
                        line: 1,
                        column: 1,
                        offset: 0,
                    },
                },
                file_path: None,
                related_information: Vec::new(),
                fixes: Vec::new(),
                code: None,
                source: "css-in-rust".to_string(),
                tags: Vec::new(),
            },
        }
    }

    /// 设置错误类型
    pub fn error_type(mut self, error_type: ErrorType) -> Self {
        self.error.error_type = error_type;
        self
    }

    /// 设置描述
    pub fn description(mut self, description: String) -> Self {
        self.error.description = Some(description);
        self
    }

    /// 设置范围
    pub fn range(mut self, range: ErrorRange) -> Self {
        self.error.range = range;
        self
    }

    /// 设置文件路径
    pub fn file_path(mut self, file_path: PathBuf) -> Self {
        self.error.file_path = Some(file_path);
        self
    }

    /// 添加相关信息
    pub fn related_information(mut self, info: RelatedInformation) -> Self {
        self.error.related_information.push(info);
        self
    }

    /// 添加修复建议
    pub fn fix(mut self, fix: CodeFix) -> Self {
        self.error.fixes.push(fix);
        self
    }

    /// 设置错误代码
    pub fn code(mut self, code: String) -> Self {
        self.error.code = Some(code);
        self
    }

    /// 设置源
    pub fn source(mut self, source: String) -> Self {
        self.error.source = source;
        self
    }

    /// 添加标签
    pub fn tag(mut self, tag: ErrorTag) -> Self {
        self.error.tags.push(tag);
        self
    }

    /// 构建错误
    pub fn build(self) -> ErrorInfo {
        self.error
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            show_source: true,
            show_line_numbers: true,
            show_column_numbers: true,
            show_error_codes: true,
            show_fixes: true,
            use_colors: true,
            max_errors: Some(100),
            context_lines: 2,
        }
    }
}

impl Default for ErrorRuleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: ErrorLevel::Error,
            custom_message: None,
        }
    }
}

impl fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorLevel::Error => write!(f, "错误"),
            ErrorLevel::Warning => write!(f, "警告"),
            ErrorLevel::Info => write!(f, "信息"),
            ErrorLevel::Hint => write!(f, "提示"),
        }
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::SyntaxError => write!(f, "语法错误"),
            ErrorType::SemanticError => write!(f, "语义错误"),
            ErrorType::TypeError => write!(f, "类型错误"),
            ErrorType::UndefinedError => write!(f, "未定义错误"),
            ErrorType::DuplicateError => write!(f, "重复定义错误"),
            ErrorType::PerformanceWarning => write!(f, "性能警告"),
            ErrorType::CompatibilityWarning => write!(f, "兼容性警告"),
            ErrorType::AccessibilityWarning => write!(f, "可访问性警告"),
            ErrorType::BestPracticeHint => write!(f, "最佳实践建议"),
            ErrorType::StyleHint => write!(f, "代码风格建议"),
        }
    }
}

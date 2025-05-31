//! 开发体验模块
//!
//! 提供开发时的各种便利功能和工具

use std::fmt;
use std::path::PathBuf;

pub mod code_completion;
pub mod diagnostics;
pub mod error_reporting;
pub mod ide_integration;
pub mod syntax_highlighting;

// 重新导出主要类型
pub use code_completion::{CompletionItem, CompletionProvider};
pub use diagnostics::{Diagnostic, DiagnosticLevel, DiagnosticManager};
pub use error_reporting::{ErrorContext, ErrorFormat, ErrorReporter};
pub use ide_integration::IdeConfig;
pub use ide_integration::{IdeIntegration, IdeType, LanguageServerProtocol};
pub use syntax_highlighting::{HighlightTheme, SyntaxHighlighter};

/// 开发体验配置
#[derive(Debug, Clone)]
pub struct DevExperienceConfig {
    /// 是否启用详细错误信息
    pub enable_detailed_errors: bool,
    /// 是否启用语法高亮
    pub enable_syntax_highlighting: bool,
    /// 是否启用代码补全
    pub enable_code_completion: bool,
    /// 是否启用实时诊断
    pub enable_live_diagnostics: bool,
    /// 是否启用性能提示
    pub enable_performance_hints: bool,
    /// 错误报告格式
    pub error_format: ErrorFormat,
    /// 语法高亮主题
    pub highlight_theme: String,
    /// IDE集成设置
    pub ide_settings: IdeSettings,
}

impl Default for DevExperienceConfig {
    fn default() -> Self {
        Self {
            enable_detailed_errors: true,
            enable_syntax_highlighting: true,
            enable_code_completion: true,
            enable_live_diagnostics: true,
            enable_performance_hints: true,
            error_format: ErrorFormat::Rich,
            highlight_theme: "default".to_string(),
            ide_settings: IdeSettings::default(),
        }
    }
}

// /// 错误报告格式
// #[derive(Debug, Clone, PartialEq)]
// pub enum ErrorFormat {
//     /// 简单格式
//     Simple,
//     /// 丰富格式（带颜色和上下文）
//     Rich,
//     /// JSON格式
//     Json,
//     /// IDE友好格式
//     IdeFriendly,
// }

/// IDE设置
#[derive(Debug, Clone)]
pub struct IdeSettings {
    /// 是否启用语言服务器
    pub enable_language_server: bool,
    /// 语言服务器端口
    pub language_server_port: u16,
    /// 是否启用悬停提示
    pub enable_hover_hints: bool,
    /// 是否启用跳转到定义
    pub enable_goto_definition: bool,
    /// 是否启用重构支持
    pub enable_refactoring: bool,
    /// 工作区根目录
    pub workspace_root: Option<PathBuf>,
}

impl Default for IdeSettings {
    fn default() -> Self {
        Self {
            enable_language_server: true,
            language_server_port: 9257,
            enable_hover_hints: true,
            enable_goto_definition: true,
            enable_refactoring: true,
            workspace_root: None,
        }
    }
}

/// 开发体验管理器
pub struct DevExperienceManager {
    config: DevExperienceConfig,
    diagnostic_manager: DiagnosticManager,
    syntax_highlighter: SyntaxHighlighter,
    ide_integration: IdeIntegration,
    error_reporter: ErrorReporter,
    completion_provider: CompletionProvider,
}

impl DevExperienceManager {
    /// 创建新的开发体验管理器
    pub fn new(config: DevExperienceConfig) -> Self {
        Self {
            diagnostic_manager: DiagnosticManager::new(),
            syntax_highlighter: SyntaxHighlighter::new(&config.highlight_theme),
            ide_integration: IdeIntegration::new(IdeType::VsCode, IdeConfig::default()),
            // TODO: 错误报告格式需要根据配置来设置，现在是硬编码的Rich
            // 可以考虑使用一个枚举来表示错误报告格式，然后根据配置来设置
            // 例如：ErrorFormat::Rich, ErrorFormat::Json, ErrorFormat::IdeFriendly
            error_reporter: ErrorReporter::new(config.error_format.clone()),
            completion_provider: CompletionProvider::new(),
            config,
        }
    }

    /// 启动开发服务
    pub fn start_dev_services(&mut self) -> Result<(), DevExperienceError> {
        if self.config.enable_live_diagnostics {
            self.diagnostic_manager
                .start_live_diagnostics()
                .map_err(|e| DevExperienceError::DiagnosticServiceFailed(e.to_string()))?;
        }

        if self.config.ide_settings.enable_language_server {
            self.ide_integration
                .start_language_server()
                .map_err(|e| DevExperienceError::LanguageServerFailed(e.to_string()))?;
        }

        Ok(())
    }

    /// 停止开发服务
    pub fn stop_dev_services(&mut self) {
        self.diagnostic_manager.stop_live_diagnostics();
        self.ide_integration.stop_language_server();
    }

    /// 处理CSS代码
    pub fn process_css_code(&self, code: &str, file_path: &str) -> ProcessResult {
        let mut result = ProcessResult::new();

        // 语法高亮
        if self.config.enable_syntax_highlighting {
            result.highlighted_code = Some(code.to_string());
            // result.highlighted_code = Some(self.syntax_highlighter.highlight(code));
        }

        // 诊断
        if self.config.enable_live_diagnostics {
            result.diagnostics = self.diagnostic_manager.analyze_code(code, file_path);
        }

        // 代码补全建议
        if self.config.enable_code_completion {
            result.completion_items = self.completion_provider.get_completions(code, 0);
        }

        result
    }

    /// 报告错误
    pub fn report_error(&self, _error: &dyn std::error::Error, _context: ErrorContext) -> String {
        self.error_reporter.format_report()
        // self.error_reporter.format_report(error, context)
    }

    /// 获取性能提示
    pub fn get_performance_hints(&self, code: &str) -> Vec<PerformanceHint> {
        if !self.config.enable_performance_hints {
            return Vec::new();
        }

        let mut hints = Vec::new();

        // 检查常见性能问题
        if code.contains("!important") {
            hints.push(PerformanceHint {
                level: HintLevel::Warning,
                message: "避免使用 !important，这会影响CSS优先级".to_string(),
                suggestion: "考虑使用更具体的选择器".to_string(),
            });
        }

        if code.matches("*").count() > 3 {
            hints.push(PerformanceHint {
                level: HintLevel::Warning,
                message: "过多使用通配符选择器可能影响性能".to_string(),
                suggestion: "使用更具体的选择器".to_string(),
            });
        }

        if code.len() > 10000 {
            hints.push(PerformanceHint {
                level: HintLevel::Info,
                message: "CSS代码较长，考虑拆分为多个模块".to_string(),
                suggestion: "使用模块化的CSS架构".to_string(),
            });
        }

        hints
    }

    /// 更新配置
    pub fn update_config(&mut self, config: DevExperienceConfig) {
        self.config = config;
        // 更新各个组件的配置
        self.syntax_highlighter
            .update_theme(&self.config.highlight_theme);
        self.error_reporter
            .set_format(self.config.error_format.clone());
    }
}

/// 处理结果
#[derive(Debug, Clone)]
pub struct ProcessResult {
    /// 高亮后的代码
    pub highlighted_code: Option<String>,
    /// 诊断信息
    pub diagnostics: Vec<Diagnostic>,
    /// 代码补全项
    pub completion_items: Vec<CompletionItem>,
    /// 性能提示
    pub performance_hints: Vec<PerformanceHint>,
}

impl ProcessResult {
    /// 创建新的处理结果
    pub fn new() -> Self {
        Self {
            highlighted_code: None,
            diagnostics: Vec::new(),
            completion_items: Vec::new(),
            performance_hints: Vec::new(),
        }
    }

    /// 是否有错误
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }

    /// 是否有警告
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Warning)
    }

    /// 获取错误数量
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count()
    }

    /// 获取警告数量
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Warning)
            .count()
    }
}

/// 性能提示
#[derive(Debug, Clone)]
pub struct PerformanceHint {
    /// 提示级别
    pub level: HintLevel,
    /// 提示信息
    pub message: String,
    /// 建议
    pub suggestion: String,
}

/// 提示级别
#[derive(Debug, Clone, PartialEq)]
pub enum HintLevel {
    /// 信息
    Info,
    /// 警告
    Warning,
    /// 错误
    Error,
}

/// 开发体验错误
#[derive(Debug, Clone)]
pub enum DevExperienceError {
    /// 诊断服务启动失败
    DiagnosticServiceFailed(String),
    /// 语言服务器启动失败
    LanguageServerFailed(String),
    /// 语法高亮失败
    SyntaxHighlightFailed(String),
    /// IDE集成失败
    IdeIntegrationFailed(String),
    /// 配置错误
    ConfigError(String),
}

impl fmt::Display for DevExperienceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DevExperienceError::DiagnosticServiceFailed(msg) => {
                write!(f, "诊断服务启动失败: {}", msg)
            }
            DevExperienceError::LanguageServerFailed(msg) => {
                write!(f, "语言服务器启动失败: {}", msg)
            }
            DevExperienceError::SyntaxHighlightFailed(msg) => {
                write!(f, "语法高亮失败: {}", msg)
            }
            DevExperienceError::IdeIntegrationFailed(msg) => {
                write!(f, "IDE集成失败: {}", msg)
            }
            DevExperienceError::ConfigError(msg) => {
                write!(f, "配置错误: {}", msg)
            }
        }
    }
}

impl std::error::Error for DevExperienceError {}

/// 开发体验工具集
pub struct DevTools {
    manager: DevExperienceManager,
}

impl DevTools {
    /// 创建新的开发工具集
    pub fn new(config: DevExperienceConfig) -> Self {
        Self {
            manager: DevExperienceManager::new(config),
        }
    }

    /// 启动开发模式
    pub fn start_dev_mode(&mut self) -> Result<(), DevExperienceError> {
        println!("🚀 启动 CSS-in-Rust 开发模式...");
        self.manager.start_dev_services()?;
        println!("✅ 开发服务已启动");
        Ok(())
    }

    /// 停止开发模式
    pub fn stop_dev_mode(&mut self) {
        println!("🛑 停止开发模式...");
        self.manager.stop_dev_services();
        println!("✅ 开发服务已停止");
    }

    /// 分析CSS文件
    pub fn analyze_css_file(&self, file_path: &str, content: &str) -> ProcessResult {
        self.manager.process_css_code(content, file_path)
    }

    /// 获取帮助信息
    pub fn get_help() -> String {
        r#"
🎨 CSS-in-Rust 开发工具

功能:
  • 实时语法检查和错误提示
  • CSS语法高亮
  • 智能代码补全
  • 性能优化建议
  • IDE集成支持
  • 热更新支持

使用方法:
  1. 启动开发模式: DevTools::start_dev_mode()
  2. 分析CSS代码: DevTools::analyze_css_file()
  3. 获取性能提示: manager.get_performance_hints()

配置选项:
  • enable_detailed_errors: 启用详细错误信息
  • enable_syntax_highlighting: 启用语法高亮
  • enable_code_completion: 启用代码补全
  • enable_live_diagnostics: 启用实时诊断
  • enable_performance_hints: 启用性能提示

更多信息请查看文档。
"#
        .to_string()
    }
}

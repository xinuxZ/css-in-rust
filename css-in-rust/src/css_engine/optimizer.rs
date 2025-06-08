//! CSS optimization functionality
//! CSS Optimizer Module
//!
//! This module provides CSS optimization capabilities leveraging lightningcss
//! for high-performance minification and optimization.
//!
//! The optimizer can perform various optimizations including:
//! - CSS minification
//! - Dead code elimination
//! - Browser compatibility transformations
//! - Static analysis for unused CSS

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use super::parser::{ParseError, StyleSheet};

#[cfg(feature = "optimizer")]
use lightningcss::{
    printer::PrinterOptions, stylesheet::StyleSheet as LightningStyleSheet, targets::Browsers,
};

/// 用于跟踪CSS使用情况的结构体
///
/// 跟踪CSS类名、ID和规则的使用情况，用于死代码消除。
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::optimizer::CssUsageTracker;
///
/// let mut tracker = CssUsageTracker::default();
/// tracker.used_classes.insert("button".to_string());
/// tracker.used_ids.insert("header".to_string());
///
/// assert!(tracker.used_classes.contains("button"));
/// assert!(tracker.used_ids.contains("header"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct CssUsageTracker {
    /// 已使用的CSS类名集合
    pub used_classes: HashSet<String>,
    /// 已使用的CSS ID集合
    pub used_ids: HashSet<String>,
    /// 源文件到使用的CSS映射
    pub file_usage: HashMap<PathBuf, HashSet<String>>,
    /// CSS规则的使用计数
    pub rule_usage_count: HashMap<String, usize>,
}

/// 死代码分析结果
///
/// 包含对CSS中未使用代码的分析结果。
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::optimizer::{CssOptimizer, DeadCodeAnalysis};
/// use std::collections::HashSet;
///
/// // 通常通过CssOptimizer的analyze_dead_code方法获取
/// let analysis = DeadCodeAnalysis {
///     unused_rules: vec!["0".to_string()],
///     unused_classes: HashSet::from(["unused-class".to_string()]),
///     unused_ids: HashSet::from(["unused-id".to_string()]),
///     removable_bytes: 100,
///     original_size: 1000,
/// };
///
/// println!("可移除的字节数: {}", analysis.removable_bytes);
/// println!("原始大小: {}", analysis.original_size);
/// println!("优化比例: {}%", analysis.removable_bytes * 100 / analysis.original_size);
/// ```
#[derive(Debug, Clone)]
pub struct DeadCodeAnalysis {
    /// 未使用的CSS规则
    pub unused_rules: Vec<String>,
    /// 未使用的类名
    pub unused_classes: HashSet<String>,
    /// 未使用的ID
    pub unused_ids: HashSet<String>,
    /// 可以安全移除的CSS字节数
    pub removable_bytes: usize,
    /// 优化前的总大小
    pub original_size: usize,
}

/// Configuration for CSS optimizer
///
/// 控制CSS优化器的行为，包括目标浏览器、死代码消除等选项。
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::optimizer::OptimizerConfig;
/// use std::path::PathBuf;
///
/// // 创建默认配置
/// let default_config = OptimizerConfig::default();
///
/// // 创建自定义配置
/// let custom_config = OptimizerConfig {
///     minify: true,
///     analyze_dependencies: true,
///     enable_dead_code_elimination: true,
///     source_paths: vec![PathBuf::from("src/")],
///     aggressive_elimination: false,
///     usage_threshold: 0.05, // 使用率低于5%的规则将被移除
///     ..OptimizerConfig::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Whether to minify the CSS output
    pub minify: bool,
    /// Target browsers for optimization (only available with optimizer feature)
    #[cfg(feature = "optimizer")]
    pub targets: Option<Browsers>,
    #[cfg(not(feature = "optimizer"))]
    pub targets: Option<String>, // Fallback type
    /// Whether to analyze dependencies for unused code elimination
    pub analyze_dependencies: bool,
    /// Whether to enable vendor prefix removal based on targets
    pub vendor_prefix: bool,
    /// Whether to enable dead code elimination
    pub enable_dead_code_elimination: bool,
    /// Paths to source files for static analysis
    pub source_paths: Vec<PathBuf>,
    /// Whether to perform aggressive dead code elimination
    pub aggressive_elimination: bool,
    /// Minimum usage threshold for keeping CSS rules (0.0 to 1.0)
    pub usage_threshold: f32,
}

impl Default for OptimizerConfig {
    /// 创建默认的优化器配置
    ///
    /// 默认配置启用了最常用的优化选项。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::OptimizerConfig;
    ///
    /// let config = OptimizerConfig::default();
    /// assert_eq!(config.minify, true);
    /// assert_eq!(config.enable_dead_code_elimination, true);
    /// assert_eq!(config.usage_threshold, 0.0);
    /// ```
    fn default() -> Self {
        Self {
            minify: true,
            #[cfg(feature = "optimizer")]
            targets: Some(Browsers::default()),
            #[cfg(not(feature = "optimizer"))]
            targets: None,
            analyze_dependencies: true,
            vendor_prefix: true,
            enable_dead_code_elimination: true,
            source_paths: Vec::new(),
            aggressive_elimination: false,
            usage_threshold: 0.0,
        }
    }
}

/// CSS optimization error
///
/// 表示在CSS优化过程中可能发生的错误。
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::optimizer::{CssOptimizer, OptimizationError};
///
/// fn process_css(css: &str) -> Result<String, OptimizationError> {
///     let mut optimizer = CssOptimizer::new();
///     let stylesheet = css_in_rust::css_engine::parser::CssParser::new().parse(css)
///         .map_err(OptimizationError::from)?;
///     optimizer.optimize(stylesheet)
/// }
/// ```
#[derive(Debug)]
pub enum OptimizationError {
    /// CSS解析错误
    ParseError(ParseError),
    /// 优化过程中的错误
    OptimizationFailed(String),
    /// 配置无效
    InvalidConfiguration(String),
    /// 死代码分析错误
    DeadCodeAnalysisError(String),
    /// 源文件错误
    SourceFileError(std::io::Error),
    /// 静态分析错误
    StaticAnalysisError(String),
}

impl std::fmt::Display for OptimizationError {
    /// 格式化错误消息
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::OptimizationError;
    /// use css_in_rust::css_engine::parser::ParseError;
    ///
    /// let error = OptimizationError::OptimizationFailed("压缩失败".to_string());
    /// println!("错误: {}", error);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizationError::ParseError(err) => {
                write!(f, "Parse error during optimization: {}", err)
            }
            OptimizationError::OptimizationFailed(msg) => write!(f, "Optimization failed: {}", msg),
            OptimizationError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            OptimizationError::DeadCodeAnalysisError(msg) => {
                write!(f, "Dead code analysis failed: {}", msg)
            }
            OptimizationError::SourceFileError(err) => {
                write!(f, "Source file error: {}", err)
            }
            OptimizationError::StaticAnalysisError(msg) => {
                write!(f, "Static analysis error: {}", msg)
            }
        }
    }
}

impl std::error::Error for OptimizationError {}

impl From<ParseError> for OptimizationError {
    /// 将解析错误转换为优化错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::OptimizationError;
    /// use css_in_rust::css_engine::parser::ParseError;
    ///
    /// let parse_error = ParseError::InvalidInput("无效的CSS".to_string());
    /// let opt_error: OptimizationError = parse_error.into();
    /// ```
    fn from(err: ParseError) -> Self {
        OptimizationError::ParseError(err)
    }
}

impl From<std::io::Error> for OptimizationError {
    /// 将IO错误转换为优化错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::OptimizationError;
    /// use std::io::{self, ErrorKind};
    ///
    /// let io_error = io::Error::new(ErrorKind::NotFound, "文件未找到");
    /// let opt_error: OptimizationError = io_error.into();
    /// ```
    fn from(err: std::io::Error) -> Self {
        OptimizationError::SourceFileError(err)
    }
}

/// CSS optimizer using lightningcss
///
/// 提供CSS优化功能，包括压缩和死代码消除。
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::optimizer::{CssOptimizer, OptimizerConfig};
/// use css_in_rust::css_engine::parser::{CssParser, StyleSheet};
///
/// // 创建一个优化器
/// let mut optimizer = CssOptimizer::new();
///
/// // 解析CSS
/// let parser = CssParser::new();
/// let css = ".button { color: red; font-size: 16px; }";
/// let stylesheet = parser.parse(css).unwrap();
///
/// // 优化CSS
/// let optimized = optimizer.optimize(stylesheet).unwrap();
/// println!("优化后的CSS: {}", optimized);
/// ```
#[derive(Clone)]
pub struct CssOptimizer {
    config: OptimizerConfig,
    usage_tracker: CssUsageTracker,
}

impl CssOptimizer {
    /// 创建一个新的CSS优化器，使用默认配置
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let optimizer = CssOptimizer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: OptimizerConfig::default(),
            usage_tracker: CssUsageTracker::default(),
        }
    }

    /// 创建一个新的CSS优化器，使用自定义配置
    ///
    /// # Arguments
    ///
    /// * `config` - 自定义的优化器配置
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::{CssOptimizer, OptimizerConfig};
    ///
    /// let config = OptimizerConfig {
    ///     minify: true,
    ///     enable_dead_code_elimination: false,
    ///     ..OptimizerConfig::default()
    /// };
    /// let optimizer = CssOptimizer::with_config(config);
    /// ```
    pub fn with_config(config: OptimizerConfig) -> Self {
        Self {
            config,
            usage_tracker: CssUsageTracker::default(),
        }
    }

    /// 优化CSS样式表
    ///
    /// 根据配置对CSS样式表进行优化，包括压缩和死代码消除。
    ///
    /// # Arguments
    ///
    /// * `stylesheet` - 要优化的CSS样式表
    ///
    /// # Returns
    ///
    /// 优化后的CSS字符串或错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    /// use css_in_rust::css_engine::parser::{CssParser, StyleSheet};
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// let parser = CssParser::new();
    /// let css = ".button { color: red; }";
    /// let stylesheet = parser.parse(css).unwrap();
    /// let optimized = optimizer.optimize(stylesheet).unwrap();
    /// ```
    pub fn optimize(&mut self, stylesheet: StyleSheet) -> Result<String, OptimizationError> {
        let mut css_content = if !stylesheet.optimized.is_empty() && self.config.minify {
            stylesheet.optimized
        } else {
            stylesheet.source
        };

        // Perform dead code elimination if enabled
        if self.config.enable_dead_code_elimination {
            css_content = self.eliminate_dead_code(&css_content)?;
        }

        // Apply standard optimization
        self.optimize_string(&css_content)
    }

    /// 优化CSS字符串（使用lightningcss）
    ///
    /// 当启用optimizer特性时，使用lightningcss库进行高级优化。
    ///
    /// # Arguments
    ///
    /// * `css` - 要优化的CSS字符串
    ///
    /// # Returns
    ///
    /// 优化后的CSS字符串或错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    ///
    /// // 优化CSS字符串
    /// let css = "
    /// /* 这是一个注释 */
    /// .container {
    ///     max-width: 1200px;
    ///     margin: 0 auto;
    ///     padding: 0 15px;
    /// }
    ///
    /// .container .row {
    ///     display: flex;
    ///     flex-wrap: wrap;
    /// }
    /// ";
    ///
    /// let result = optimizer.optimize_string(css).unwrap();
    ///
    /// // 优化后的CSS应该更小，没有注释和多余的空白
    /// println!("优化后: {}", result);
    /// assert!(result.len() < css.len());
    /// assert!(!result.contains("/*"));
    /// ```
    #[cfg(feature = "optimizer")]
    pub fn optimize_string(&self, css: &str) -> Result<String, OptimizationError> {
        use lightningcss::stylesheet::ParserOptions;

        // Parse CSS with lightningcss
        let stylesheet =
            LightningStyleSheet::parse(css, ParserOptions::default()).map_err(|e| {
                OptimizationError::OptimizationFailed(format!("Failed to parse CSS: {:?}", e))
            })?;

        // Create printer options based on config
        let printer_options = PrinterOptions {
            minify: self.config.minify,
            targets: self.config.targets.clone().unwrap_or_default().into(),
            ..Default::default()
        };

        // Generate optimized CSS
        let result = stylesheet.to_css(printer_options).map_err(|e| {
            OptimizationError::OptimizationFailed(format!("Failed to optimize CSS: {:?}", e))
        })?;

        Ok(result.code)
    }

    /// Optimize a CSS string directly (fallback implementation)
    ///
    /// 当未启用optimizer特性时的回退实现，提供基本的CSS压缩。
    ///
    /// # Arguments
    ///
    /// * `css` - 要优化的CSS字符串
    ///
    /// # Returns
    ///
    /// 优化后的CSS字符串或错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let optimizer = CssOptimizer::new();
    ///
    /// // 优化简单的CSS
    /// let css = "
    /// .header {
    ///     background-color: #333;
    ///     color: white;
    /// }
    /// ";
    ///
    /// let result = optimizer.optimize_string(css).unwrap();
    /// println!("优化后: {}", result);
    /// ```
    #[cfg(not(feature = "optimizer"))]
    pub fn optimize_string(&self, css: &str) -> Result<String, OptimizationError> {
        // Simple fallback: just return the CSS as-is or do basic minification
        if self.config.minify {
            // Very basic minification: remove extra whitespace
            let minified = css
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            Ok(minified)
        } else {
            Ok(css.to_string())
        }
    }

    /// Eliminate dead code from CSS
    ///
    /// 从CSS中消除未使用的代码，基于静态分析或手动跟踪的使用情况。
    /// 这个方法可以显著减小CSS的大小，移除应用程序中未使用的样式规则。
    ///
    /// # Arguments
    ///
    /// * `css` - 要处理的CSS字符串
    ///
    /// # Returns
    ///
    /// 成功时返回优化后的CSS字符串（已移除未使用的代码），
    /// 失败时返回`OptimizationError`错误。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::{CssOptimizer, OptimizerConfig};
    /// use std::path::PathBuf;
    ///
    /// // 创建优化器
    /// let config = OptimizerConfig {
    ///     enable_dead_code_elimination: true,
    ///     source_paths: vec![PathBuf::from("src/")],
    ///     ..OptimizerConfig::default()
    /// };
    /// let mut optimizer = CssOptimizer::with_config(config);
    ///
    /// // 手动跟踪使用的类
    /// let mut used_classes = HashSet::new();
    /// used_classes.insert("header".to_string());
    /// used_classes.insert("active".to_string());
    ///
    /// // 更新使用情况跟踪器
    /// optimizer.get_usage_tracker().used_classes = used_classes;
    ///
    /// // CSS包含使用和未使用的类
    /// let css = "
    /// .header {
    ///     background-color: #333;
    ///     color: white;
    /// }
    /// .active {
    ///     font-weight: bold;
    /// }
    /// .unused {
    ///     display: none;
    /// }
    /// #unused-id {
    ///     visibility: hidden;
    /// }
    /// ";
    ///
    /// // 消除未使用的代码
    /// let result = optimizer.eliminate_dead_code(css).unwrap();
    ///
    /// // 验证结果
    /// assert!(result.contains(".header"));
    /// assert!(result.contains(".active"));
    /// assert!(!result.contains(".unused"));
    /// assert!(!result.contains("#unused-id"));
    ///
    /// // 分析死代码
    /// let analysis = optimizer.analyze_dead_code(css).unwrap();
    /// println!("未使用的类: {:?}", analysis.unused_classes);
    /// println!("可移除的字节数: {}", analysis.removable_bytes);
    /// println!("优化比例: {}%", analysis.removable_bytes * 100 / analysis.original_size);
    /// ```
    pub fn eliminate_dead_code(&mut self, css: &str) -> Result<String, OptimizationError> {
        if !self.config.enable_dead_code_elimination {
            return Ok(css.to_string());
        }

        // Perform static analysis to find used CSS
        self.perform_static_analysis()?;

        // Analyze dead code
        let analysis = self.analyze_dead_code(css)?;

        // Remove unused CSS rules
        self.remove_unused_rules(css, &analysis)
    }

    /// 对源文件进行静态分析
    ///
    /// 分析项目源文件以查找使用的CSS类和ID。
    ///
    /// # Returns
    ///
    /// 成功时返回 `Ok(())`，失败时返回错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::{CssOptimizer, OptimizerConfig};
    /// use std::path::PathBuf;
    ///
    /// let config = OptimizerConfig {
    ///     analyze_dependencies: true,
    ///     source_paths: vec![PathBuf::from("src/")],
    ///     ..OptimizerConfig::default()
    /// };
    /// let mut optimizer = CssOptimizer::with_config(config);
    /// // 这个方法通常在eliminate_dead_code内部调用
    /// optimizer.perform_static_analysis().unwrap();
    /// ```
    fn perform_static_analysis(&mut self) -> Result<(), OptimizationError> {
        if !self.config.analyze_dependencies {
            return Ok(());
        }

        // Clone the paths to avoid borrowing conflicts
        let source_paths = self.config.source_paths.clone();
        for source_path in &source_paths {
            self.analyze_source_file(source_path)?;
        }

        Ok(())
    }

    /// 分析源文件中的CSS使用情况
    ///
    /// 读取并分析单个源文件，提取CSS类和ID的使用信息。
    ///
    /// # Arguments
    ///
    /// * `file_path` - 要分析的文件路径
    ///
    /// # Returns
    ///
    /// 成功时返回 `Ok(())`，失败时返回错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    /// use std::path::Path;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// // 这个方法通常在perform_static_analysis内部调用
    /// if let Ok(()) = optimizer.analyze_source_file(Path::new("src/main.rs")) {
    ///     println!("成功分析文件");
    /// }
    /// ```
    fn analyze_source_file(
        &mut self,
        file_path: &std::path::Path,
    ) -> Result<(), OptimizationError> {
        let content = std::fs::read_to_string(file_path)?;

        // Look for css! macro usage
        self.extract_css_usage_from_rust(&content);

        // Look for class and id usage in templates/HTML
        self.extract_css_usage_from_templates(&content);

        Ok(())
    }

    /// 从Rust代码中提取CSS使用情况
    ///
    /// 分析Rust代码中的css!宏调用，提取使用的CSS选择器。
    ///
    /// # Arguments
    ///
    /// * `content` - Rust源代码内容
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// let rust_code = r#"
    ///     let class = css!(".button { color: red; }");
    /// "#;
    /// optimizer.extract_css_usage_from_rust(rust_code);
    /// let tracker = optimizer.get_usage_tracker();
    /// ```
    fn extract_css_usage_from_rust(&mut self, content: &str) {
        // Find css! macro calls
        if let Ok(css_macro_regex) = Regex::new(r#"css!\s*\(\s*["']([^"']*)["']\s*\)"#) {
            for captures in css_macro_regex.captures_iter(content) {
                if let Some(css_content) = captures.get(1) {
                    self.parse_css_for_usage(css_content.as_str());
                }
            }
        }
    }

    /// 从模板内容中提取CSS使用情况
    ///
    /// 分析HTML/模板代码中的class和id属性，提取使用的CSS选择器。
    ///
    /// # Arguments
    ///
    /// * `content` - HTML或模板代码内容
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// let html = r#"<div class="container">
    ///     <button id="submit-btn" class="btn primary">提交</button>
    /// </div>"#;
    /// optimizer.extract_css_usage_from_templates(html);
    /// let tracker = optimizer.get_usage_tracker();
    /// assert!(tracker.used_classes.contains("container"));
    /// assert!(tracker.used_classes.contains("btn"));
    /// assert!(tracker.used_classes.contains("primary"));
    /// assert!(tracker.used_ids.contains("submit-btn"));
    /// ```
    fn extract_css_usage_from_templates(&mut self, content: &str) {
        // Find class attributes
        if let Ok(class_regex) = Regex::new(r#"class\s*=\s*["']([^"']*)["']"#) {
            for captures in class_regex.captures_iter(content) {
                if let Some(classes) = captures.get(1) {
                    for class in classes.as_str().split_whitespace() {
                        if !class.is_empty() {
                            self.usage_tracker.used_classes.insert(class.to_string());
                            // 增加使用计数
                            let count = self
                                .usage_tracker
                                .rule_usage_count
                                .entry(format!(".{}", class))
                                .or_insert(0);
                            *count += 1;
                        }
                    }
                }
            }
        }

        // Find id attributes
        if let Ok(id_regex) = Regex::new(r#"id\s*=\s*["']([^"']*)["']"#) {
            for captures in id_regex.captures_iter(content) {
                if let Some(id) = captures.get(1) {
                    let id_str = id.as_str().trim();
                    if !id_str.is_empty() {
                        self.usage_tracker.used_ids.insert(id_str.to_string());
                        // 增加使用计数
                        let count = self
                            .usage_tracker
                            .rule_usage_count
                            .entry(format!("#{}", id_str))
                            .or_insert(0);
                        *count += 1;
                    }
                }
            }
        }
    }

    /// 解析CSS内容以提取选择器
    ///
    /// 从CSS内容中提取类选择器和ID选择器，并更新使用跟踪器。
    ///
    /// # Arguments
    ///
    /// * `css_content` - CSS内容
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// let css = ".button { color: red; } #header { background: blue; }";
    /// optimizer.parse_css_for_usage(css);
    /// let tracker = optimizer.get_usage_tracker();
    /// assert!(tracker.used_classes.contains("button"));
    /// assert!(tracker.used_ids.contains("header"));
    /// ```
    fn parse_css_for_usage(&mut self, css_content: &str) {
        // Simple CSS parsing to extract class and id selectors
        if let Ok(class_regex) = Regex::new(r"\.([a-zA-Z][a-zA-Z0-9_-]*)") {
            for captures in class_regex.captures_iter(css_content) {
                if let Some(class) = captures.get(1) {
                    let class_name = class.as_str();
                    self.usage_tracker
                        .used_classes
                        .insert(class_name.to_string());
                    // 增加使用计数
                    let count = self
                        .usage_tracker
                        .rule_usage_count
                        .entry(format!(".{}", class_name))
                        .or_insert(0);
                    *count += 1;
                }
            }
        }

        if let Ok(id_regex) = Regex::new(r"#([a-zA-Z][a-zA-Z0-9_-]*)") {
            for captures in id_regex.captures_iter(css_content) {
                if let Some(id) = captures.get(1) {
                    let id_name = id.as_str();
                    self.usage_tracker.used_ids.insert(id_name.to_string());
                    // 增加使用计数
                    let count = self
                        .usage_tracker
                        .rule_usage_count
                        .entry(format!("#{}", id_name))
                        .or_insert(0);
                    *count += 1;
                }
            }
        }
    }

    /// 分析CSS中的死代码
    ///
    /// 分析CSS内容，找出未使用的规则、类和ID。
    ///
    /// # Arguments
    ///
    /// * `css` - 要分析的CSS内容
    ///
    /// # Returns
    ///
    /// 包含死代码分析结果的 `DeadCodeAnalysis` 或错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// // 先添加一些已使用的类
    /// optimizer.track_css_usage(vec!["button".to_string()], vec![], None);
    ///
    /// let css = ".button { color: red; } .unused { color: blue; }";
    /// let analysis = optimizer.analyze_dead_code(css).unwrap();
    /// assert!(analysis.unused_classes.contains("unused"));
    /// ```
    fn analyze_dead_code(&self, css: &str) -> Result<DeadCodeAnalysis, OptimizationError> {
        let mut analysis = DeadCodeAnalysis {
            unused_rules: Vec::new(),
            unused_classes: std::collections::HashSet::new(),
            unused_ids: std::collections::HashSet::new(),
            removable_bytes: 0,
            original_size: css.len(),
        };

        // Parse CSS to find all rules
        let rules = self.extract_css_rules(css)?;

        for (rule_index, rule) in rules.iter().enumerate() {
            let is_used = self.is_rule_used(rule);

            if !is_used {
                analysis.unused_rules.push(rule_index.to_string());
                analysis.removable_bytes += rule.len();

                // Extract unused classes and IDs from the rule
                self.extract_unused_selectors(rule, &mut analysis);
            }
        }

        Ok(analysis)
    }

    /// 从CSS内容中提取规则
    ///
    /// 解析CSS内容，将其分解为单独的规则。
    ///
    /// # Arguments
    ///
    /// * `css` - 要解析的CSS内容
    ///
    /// # Returns
    ///
    /// 包含CSS规则的字符串向量或错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let optimizer = CssOptimizer::new();
    /// let css = ".button { color: red; } .card { padding: 10px; }";
    /// let rules = optimizer.extract_css_rules(css).unwrap();
    /// assert_eq!(rules.len(), 2);
    /// ```
    fn extract_css_rules(&self, css: &str) -> Result<Vec<String>, OptimizationError> {
        let mut rules = Vec::new();
        let mut current_rule = String::new();
        let mut brace_count = 0;
        let mut in_rule = false;

        for ch in css.chars() {
            current_rule.push(ch);

            match ch {
                '{' => {
                    brace_count += 1;
                    in_rule = true;
                }
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 && in_rule {
                        rules.push(current_rule.trim().to_string());
                        current_rule.clear();
                        in_rule = false;
                    }
                }
                _ => {}
            }
        }

        Ok(rules)
    }

    /// 检查CSS规则是否被使用
    ///
    /// 分析CSS规则，判断其选择器是否在项目中被使用。
    ///
    /// # Arguments
    ///
    /// * `rule` - 要检查的CSS规则
    ///
    /// # Returns
    ///
    /// 如果规则被使用则返回 `true`，否则返回 `false`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// optimizer.track_css_usage(vec!["button".to_string()], vec![], None);
    ///
    /// // 这个方法通常在analyze_dead_code内部调用
    /// let used_rule = ".button { color: red; }";
    /// let unused_rule = ".card { padding: 10px; }";
    /// assert!(optimizer.is_rule_used(used_rule));
    /// assert!(!optimizer.is_rule_used(unused_rule));
    /// ```
    fn is_rule_used(&self, rule: &str) -> bool {
        // Extract selectors from the rule
        let selector_part = rule.split('{').next().unwrap_or("").trim();

        // Check if any selector in the rule is used
        for selector in selector_part.split(',') {
            let selector = selector.trim();

            if self.is_selector_used(selector) {
                return true;
            }
        }

        false
    }

    /// 检查选择器是否被使用
    ///
    /// 判断特定的CSS选择器是否在项目中被使用。
    ///
    /// # Arguments
    ///
    /// * `selector` - 要检查的CSS选择器
    ///
    /// # Returns
    ///
    /// 如果选择器被使用则返回 `true`，否则返回 `false`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// optimizer.track_css_usage(vec!["button".to_string()], vec!["header".to_string()], None);
    ///
    /// // 这个方法通常在is_rule_used内部调用
    /// assert!(optimizer.is_selector_used(".button"));
    /// assert!(optimizer.is_selector_used("#header"));
    /// assert!(!optimizer.is_selector_used(".unused"));
    /// ```
    fn is_selector_used(&self, selector: &str) -> bool {
        // Check class selectors
        if selector.starts_with('.') {
            let class_name = &selector[1..];
            return self.usage_tracker.used_classes.contains(class_name);
        }

        // Check ID selectors
        if selector.starts_with('#') {
            let id_name = &selector[1..];
            return self.usage_tracker.used_ids.contains(id_name);
        }

        // For other selectors (element, attribute, etc.), assume they are used
        // unless aggressive elimination is enabled
        if self.config.aggressive_elimination {
            // In aggressive mode, we could implement more sophisticated analysis
            false
        } else {
            true
        }
    }

    /// 从规则中提取未使用的选择器
    ///
    /// 分析CSS规则，提取其中未使用的类和ID选择器。
    ///
    /// # Arguments
    ///
    /// * `rule` - 要分析的CSS规则
    /// * `analysis` - 用于存储分析结果的对象
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::{CssOptimizer, DeadCodeAnalysis};
    /// use std::collections::HashSet;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// let rule = ".used, .unused { color: red; }";
    /// optimizer.track_css_usage(vec!["used".to_string()], vec![], None);
    ///
    /// let mut analysis = DeadCodeAnalysis {
    ///     unused_rules: Vec::new(),
    ///     unused_classes: HashSet::new(),
    ///     unused_ids: HashSet::new(),
    ///     removable_bytes: 0,
    ///     original_size: 100,
    /// };
    ///
    /// // 这个方法通常在analyze_dead_code内部调用
    /// optimizer.extract_unused_selectors(rule, &mut analysis);
    /// assert!(analysis.unused_classes.contains("unused"));
    /// ```
    fn extract_unused_selectors(&self, rule: &str, analysis: &mut DeadCodeAnalysis) {
        let selector_part = rule.split('{').next().unwrap_or("").trim();

        for selector in selector_part.split(',') {
            let selector = selector.trim();

            if selector.starts_with('.') {
                let class_name = &selector[1..];
                if !self.usage_tracker.used_classes.contains(class_name) {
                    analysis.unused_classes.insert(class_name.to_string());
                }
            } else if selector.starts_with('#') {
                let id_name = &selector[1..];
                if !self.usage_tracker.used_ids.contains(id_name) {
                    analysis.unused_ids.insert(id_name.to_string());
                }
            }
        }
    }

    /// 移除未使用的CSS规则
    ///
    /// 根据死代码分析结果，从CSS内容中移除未使用的规则。
    ///
    /// # Arguments
    ///
    /// * `css` - 原始CSS内容
    /// * `analysis` - 死代码分析结果
    ///
    /// # Returns
    ///
    /// 移除未使用规则后的CSS内容或错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// optimizer.track_css_usage(vec!["used".to_string()], vec![], None);
    ///
    /// let css = ".used { color: red; } .unused { color: blue; }";
    /// let analysis = optimizer.analyze_dead_code(css).unwrap();
    /// let result = optimizer.remove_unused_rules(css, &analysis).unwrap();
    /// assert!(result.contains(".used"));
    /// assert!(!result.contains(".unused"));
    /// ```
    fn remove_unused_rules(
        &self,
        css: &str,
        analysis: &DeadCodeAnalysis,
    ) -> Result<String, OptimizationError> {
        let rules = self.extract_css_rules(css)?;
        let mut result = String::new();

        for (index, rule) in rules.iter().enumerate() {
            if !analysis.unused_rules.contains(&index.to_string()) {
                result.push_str(rule);
                result.push('\n');
            }
        }

        Ok(result)
    }

    /// Get a reference to the usage tracker
    ///
    /// 获取对内部使用情况跟踪器的引用，用于检查或修改CSS使用情况数据。
    ///
    /// # Returns
    ///
    /// 返回对`CssUsageTracker`实例的不可变引用。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// optimizer.track_css_usage(vec!["button".to_string()], vec!["header".to_string()], None);
    ///
    /// // 获取使用情况跟踪器
    /// let tracker = optimizer.get_usage_tracker();
    /// assert!(tracker.used_classes.contains("button"));
    /// assert!(tracker.used_ids.contains("header"));
    /// ```
    pub fn get_usage_tracker(&self) -> &CssUsageTracker {
        &self.usage_tracker
    }

    /// 跟踪外部CSS使用情况
    ///
    /// 从外部源添加已使用的CSS类和ID信息。
    ///
    /// # Arguments
    ///
    /// * `classes` - 已使用的CSS类名列表
    /// * `ids` - 已使用的CSS ID列表
    /// * `file_path` - 可选的文件路径，用于记录使用位置
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let mut optimizer = CssOptimizer::new();
    /// optimizer.track_css_usage(
    ///     vec!["button".to_string(), "container".to_string()],
    ///     vec!["app".to_string()],
    ///     Some("App.js".to_string())
    /// );
    ///
    /// let tracker = optimizer.get_usage_tracker();
    /// assert!(tracker.used_classes.contains("button"));
    /// assert!(tracker.used_classes.contains("container"));
    /// assert!(tracker.used_ids.contains("app"));
    /// ```
    pub fn track_css_usage(
        &mut self,
        classes: Vec<String>,
        ids: Vec<String>,
        file_path: Option<String>,
    ) {
        for class in classes {
            self.usage_tracker.used_classes.insert(class);
        }

        for id in ids {
            self.usage_tracker.used_ids.insert(id);
        }

        if let Some(path) = file_path {
            self.usage_tracker
                .file_usage
                .insert(PathBuf::from(path), HashSet::new());
        }
    }
}

impl Default for CssOptimizer {
    /// 创建一个使用默认配置的CSS优化器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::optimizer::CssOptimizer;
    ///
    /// let optimizer = CssOptimizer::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::parser::{StyleSheet, StyleSheetMetadata};
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = CssOptimizer::new();
        assert!(optimizer.config.minify);
        assert!(optimizer.config.enable_dead_code_elimination);
    }

    #[test]
    fn test_optimizer_with_config() {
        let config = OptimizerConfig {
            minify: false,
            enable_dead_code_elimination: false,
            ..Default::default()
        };
        let optimizer = CssOptimizer::with_config(config);
        assert!(!optimizer.config.minify);
        assert!(!optimizer.config.enable_dead_code_elimination);
    }

    #[test]
    fn test_usage_tracker() {
        let mut optimizer = CssOptimizer::new();
        let classes = vec!["btn".to_string(), "header".to_string()];
        let ids = vec!["main".to_string()];

        optimizer.track_css_usage(classes, ids, Some("test.rs".to_string()));

        let tracker = optimizer.get_usage_tracker();
        assert!(tracker.used_classes.contains("btn"));
        assert!(tracker.used_classes.contains("header"));
        assert!(tracker.used_ids.contains("main"));
    }

    #[test]
    fn test_css_rule_extraction() {
        let optimizer = CssOptimizer::new();
        let css = ".btn { color: red; } #main { background: blue; } p { margin: 0; }";
        let rules = optimizer.extract_css_rules(css).unwrap();

        assert_eq!(rules.len(), 3);
        assert!(rules[0].contains(".btn"));
        assert!(rules[1].contains("#main"));
        assert!(rules[2].contains("p"));
    }

    #[test]
    fn test_selector_usage_detection() {
        let mut optimizer = CssOptimizer::new();

        // Track some used classes and IDs
        optimizer.track_css_usage(vec!["btn".to_string()], vec!["main".to_string()], None);

        // Test class selector
        assert!(optimizer.is_selector_used(".btn"));
        assert!(!optimizer.is_selector_used(".unused"));

        // Test ID selector
        assert!(optimizer.is_selector_used("#main"));
        assert!(!optimizer.is_selector_used("#unused"));

        // Test element selector (should be considered used by default)
        assert!(optimizer.is_selector_used("p"));
    }

    #[test]
    fn test_dead_code_analysis() {
        let mut optimizer = CssOptimizer::new();

        // Track only some classes as used
        optimizer.track_css_usage(vec!["btn".to_string()], vec![], None);

        let css = ".btn { color: red; } .unused { color: blue; } p { margin: 0; }";
        let analysis = optimizer.analyze_dead_code(css).unwrap();

        // Should detect unused class
        assert!(analysis.unused_classes.contains("unused"));
        assert!(!analysis.unused_classes.contains("btn"));

        // Should have some removable bytes
        assert!(analysis.removable_bytes > 0);
        assert_eq!(analysis.original_size, css.len());
    }

    #[test]
    fn test_optimize_string() {
        let optimizer = CssOptimizer::new();
        let css = ".button {\n  color: red;\n  font-size: 16px;\n}";
        let result = optimizer.optimize_string(css).unwrap();

        // With lightningcss minification, the result should be smaller
        assert!(result.len() <= css.len());
        assert!(result.contains(".button"));
        assert!(result.contains("color:red") || result.contains("color: red"));
    }

    #[test]
    fn test_optimize_stylesheet() {
        let config = OptimizerConfig {
            enable_dead_code_elimination: false,
            ..Default::default()
        };
        let mut optimizer = CssOptimizer::with_config(config);
        let stylesheet = StyleSheet {
            source: ".test { margin: 0px; }".to_string(),
            optimized: String::new(),
            metadata: StyleSheetMetadata::default(),
        };

        let result = optimizer.optimize(stylesheet).unwrap();
        assert!(result.contains(".test"));
        assert!(result.contains("margin"));
    }

    #[test]
    fn test_optimize_with_preoptimized() {
        let config = OptimizerConfig {
            enable_dead_code_elimination: false,
            ..Default::default()
        };
        let mut optimizer = CssOptimizer::with_config(config);
        let stylesheet = StyleSheet {
            source: ".test { margin: 0px; }".to_string(),
            optimized: ".test{margin:0}".to_string(),
            metadata: StyleSheetMetadata::default(),
        };

        let result = optimizer.optimize(stylesheet).unwrap();
        // Should use the pre-optimized version
        assert_eq!(result, ".test{margin:0}");
    }

    #[test]
    fn test_dead_code_elimination_disabled() {
        let config = OptimizerConfig {
            enable_dead_code_elimination: false,
            ..Default::default()
        };
        let mut optimizer = CssOptimizer::with_config(config);

        let css = ".unused { color: red; }";
        let result = optimizer.eliminate_dead_code(css).unwrap();

        // Should return original CSS when dead code elimination is disabled
        assert_eq!(result, css);
    }

    #[test]
    fn test_remove_unused_rules() {
        let mut optimizer = CssOptimizer::new();

        // Track only "btn" as used
        optimizer.track_css_usage(vec!["btn".to_string()], vec![], None);

        let css = ".btn { color: red; } .unused { color: blue; }";
        let analysis = optimizer.analyze_dead_code(css).unwrap();
        let result = optimizer.remove_unused_rules(css, &analysis).unwrap();

        // Should contain the used rule but not the unused one
        assert!(result.contains(".btn"));
        assert!(!result.contains(".unused"));
    }
}

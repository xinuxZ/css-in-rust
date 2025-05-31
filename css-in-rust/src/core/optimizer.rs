//! CSS optimization functionality
//! CSS Optimizer Module
//!
//! This module provides CSS optimization capabilities leveraging lightningcss
//! for high-performance minification and optimization.

use crate::core::parser::{ParseError, StyleSheet};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[cfg(feature = "optimizer")]
use lightningcss::{
    printer::PrinterOptions, stylesheet::StyleSheet as LightningStyleSheet, targets::Browsers,
};

/// 用于跟踪CSS使用情况的结构体
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
#[derive(Debug)]
pub enum OptimizationError {
    ParseError(ParseError),
    OptimizationFailed(String),
    InvalidConfiguration(String),
    DeadCodeAnalysisError(String),
    SourceFileError(std::io::Error),
    StaticAnalysisError(String),
}

impl std::fmt::Display for OptimizationError {
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
    fn from(err: ParseError) -> Self {
        OptimizationError::ParseError(err)
    }
}

impl From<std::io::Error> for OptimizationError {
    fn from(err: std::io::Error) -> Self {
        OptimizationError::SourceFileError(err)
    }
}

/// CSS optimizer using lightningcss
#[derive(Clone)]
pub struct CssOptimizer {
    config: OptimizerConfig,
    usage_tracker: CssUsageTracker,
}

impl CssOptimizer {
    /// Create a new CSS optimizer with default configuration
    pub fn new() -> Self {
        Self {
            config: OptimizerConfig::default(),
            usage_tracker: CssUsageTracker::default(),
        }
    }

    /// Create a new CSS optimizer with custom configuration
    pub fn with_config(config: OptimizerConfig) -> Self {
        Self {
            config,
            usage_tracker: CssUsageTracker::default(),
        }
    }

    /// Optimize a CSS stylesheet using lightningcss
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

    /// Optimize CSS string
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

    /// Fallback optimize CSS string (without lightningcss)
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

    /// Perform static analysis on source files
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

    /// Analyze a source file for CSS usage
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

    /// Extract CSS usage from Rust code (css! macros)
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

    /// Extract CSS usage from template content
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

    /// Parse CSS content to extract selectors
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

    /// Analyze CSS for dead code
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

    /// Extract CSS rules from CSS content
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

    /// Check if a CSS rule is used
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

    /// Check if a specific selector is used
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

    /// Extract unused selectors from a rule
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

    /// Remove unused CSS rules
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

    /// Get usage tracking information
    pub fn get_usage_tracker(&self) -> &CssUsageTracker {
        &self.usage_tracker
    }

    /// Track CSS usage from external source
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
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::{StyleSheet, StyleSheetMetadata};

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

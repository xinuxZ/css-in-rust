//! 诊断模块
//!
//! 提供CSS代码分析和错误检测功能

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// 诊断级别
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiagnosticLevel {
    /// 错误
    Error,
    /// 警告
    Warning,
    /// 信息
    Info,
    /// 提示
    Hint,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Error => write!(f, "错误"),
            DiagnosticLevel::Warning => write!(f, "警告"),
            DiagnosticLevel::Info => write!(f, "信息"),
            DiagnosticLevel::Hint => write!(f, "提示"),
        }
    }
}

/// 诊断类型
#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticType {
    /// 语法错误
    SyntaxError,
    /// 语义错误
    SemanticError,
    /// 性能警告
    PerformanceWarning,
    /// 最佳实践建议
    BestPractice,
    /// 兼容性问题
    Compatibility,
    /// 可访问性问题
    Accessibility,
    /// 未使用的代码
    UnusedCode,
    /// 重复代码
    DuplicateCode,
}

/// 代码位置
#[derive(Debug, Clone, PartialEq)]
pub struct CodePosition {
    /// 行号（从1开始）
    pub line: usize,
    /// 列号（从1开始）
    pub column: usize,
    /// 字符偏移量
    pub offset: usize,
}

impl CodePosition {
    /// 创建新的代码位置
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}

/// 代码范围
#[derive(Debug, Clone, PartialEq)]
pub struct CodeRange {
    /// 开始位置
    pub start: CodePosition,
    /// 结束位置
    pub end: CodePosition,
}

impl CodeRange {
    /// 创建新的代码范围
    pub fn new(start: CodePosition, end: CodePosition) -> Self {
        Self { start, end }
    }

    /// 创建单点范围
    pub fn point(position: CodePosition) -> Self {
        Self {
            start: position.clone(),
            end: position,
        }
    }
}

/// 诊断信息
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// 诊断级别
    pub level: DiagnosticLevel,
    /// 诊断类型
    pub diagnostic_type: DiagnosticType,
    /// 错误代码
    pub code: String,
    /// 消息
    pub message: String,
    /// 详细描述
    pub description: Option<String>,
    /// 代码范围
    pub range: CodeRange,
    /// 文件路径
    pub file_path: String,
    /// 建议的修复
    pub fixes: Vec<DiagnosticFix>,
    /// 相关信息
    pub related_information: Vec<RelatedInformation>,
    /// 标签
    pub tags: Vec<DiagnosticTag>,
}

impl Diagnostic {
    /// 创建新的诊断
    pub fn new(
        level: DiagnosticLevel,
        diagnostic_type: DiagnosticType,
        code: String,
        message: String,
        range: CodeRange,
        file_path: String,
    ) -> Self {
        Self {
            level,
            diagnostic_type,
            code,
            message,
            description: None,
            range,
            file_path,
            fixes: Vec::new(),
            related_information: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// 添加描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 添加修复建议
    pub fn with_fix(mut self, fix: DiagnosticFix) -> Self {
        self.fixes.push(fix);
        self
    }

    /// 添加相关信息
    pub fn with_related_info(mut self, info: RelatedInformation) -> Self {
        self.related_information.push(info);
        self
    }

    /// 添加标签
    pub fn with_tag(mut self, tag: DiagnosticTag) -> Self {
        self.tags.push(tag);
        self
    }

    /// 格式化为字符串
    pub fn format(&self) -> String {
        let mut result = format!(
            "[{}:{}:{}] {}: {} ({})",
            self.file_path,
            self.range.start.line,
            self.range.start.column,
            self.level,
            self.message,
            self.code
        );

        if let Some(description) = &self.description {
            result.push_str(&format!("\n  详情: {}", description));
        }

        if !self.fixes.is_empty() {
            result.push_str("\n  建议修复:");
            for fix in &self.fixes {
                result.push_str(&format!("\n    - {}", fix.title));
            }
        }

        result
    }
}

/// 诊断修复
#[derive(Debug, Clone)]
pub struct DiagnosticFix {
    /// 修复标题
    pub title: String,
    /// 修复描述
    pub description: Option<String>,
    /// 文本编辑
    pub edits: Vec<TextEdit>,
    /// 是否为首选修复
    pub is_preferred: bool,
}

impl DiagnosticFix {
    /// 创建新的诊断修复
    pub fn new(title: String, edits: Vec<TextEdit>) -> Self {
        Self {
            title,
            description: None,
            edits,
            is_preferred: false,
        }
    }

    /// 设置为首选修复
    pub fn as_preferred(mut self) -> Self {
        self.is_preferred = true;
        self
    }

    /// 添加描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

/// 文本编辑
#[derive(Debug, Clone)]
pub struct TextEdit {
    /// 编辑范围
    pub range: CodeRange,
    /// 新文本
    pub new_text: String,
}

impl TextEdit {
    /// 创建新的文本编辑
    pub fn new(range: CodeRange, new_text: String) -> Self {
        Self { range, new_text }
    }

    /// 创建插入编辑
    pub fn insert(position: CodePosition, text: String) -> Self {
        Self {
            range: CodeRange::point(position),
            new_text: text,
        }
    }

    /// 创建删除编辑
    pub fn delete(range: CodeRange) -> Self {
        Self {
            range,
            new_text: String::new(),
        }
    }

    /// 创建替换编辑
    pub fn replace(range: CodeRange, new_text: String) -> Self {
        Self { range, new_text }
    }
}

/// 相关信息
#[derive(Debug, Clone)]
pub struct RelatedInformation {
    /// 位置
    pub location: CodeRange,
    /// 文件路径
    pub file_path: String,
    /// 消息
    pub message: String,
}

/// 诊断标签
#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticTag {
    /// 不必要的代码
    Unnecessary,
    /// 已弃用的代码
    Deprecated,
}

/// 诊断管理器
pub struct DiagnosticManager {
    /// 诊断规则
    rules: HashMap<String, Box<dyn DiagnosticRule>>,
    /// 实时诊断状态
    live_diagnostics_enabled: Arc<Mutex<bool>>,
    /// 诊断缓存
    diagnostic_cache: Arc<Mutex<HashMap<String, Vec<Diagnostic>>>>,
}

impl DiagnosticManager {
    /// 创建新的诊断管理器
    pub fn new() -> Self {
        let mut manager = Self {
            rules: HashMap::new(),
            live_diagnostics_enabled: Arc::new(Mutex::new(false)),
            diagnostic_cache: Arc::new(Mutex::new(HashMap::new())),
        };

        // 注册默认规则
        manager.register_default_rules();
        manager
    }

    /// 注册默认规则
    fn register_default_rules(&mut self) {
        self.register_rule("syntax-error", Box::new(SyntaxErrorRule));
        self.register_rule("unused-selector", Box::new(UnusedSelectorRule));
        self.register_rule("duplicate-property", Box::new(DuplicatePropertyRule));
        self.register_rule("performance-warning", Box::new(PerformanceWarningRule));
        self.register_rule("accessibility-check", Box::new(AccessibilityRule));
    }

    /// 注册诊断规则
    pub fn register_rule(&mut self, name: &str, rule: Box<dyn DiagnosticRule>) {
        self.rules.insert(name.to_string(), rule);
    }

    /// 分析代码
    pub fn analyze_code(&self, code: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // 检查缓存
        let cache_key = format!("{}:{}", file_path, Self::hash_code(code));
        {
            let cache = self.diagnostic_cache.lock().unwrap();
            if let Some(cached_diagnostics) = cache.get(&cache_key) {
                return cached_diagnostics.clone();
            }
        }

        // 运行所有规则
        for (_, rule) in &self.rules {
            let mut rule_diagnostics = rule.check(code, file_path);
            diagnostics.append(&mut rule_diagnostics);
        }

        // 缓存结果
        {
            let mut cache = self.diagnostic_cache.lock().unwrap();
            cache.insert(cache_key, diagnostics.clone());
        }

        diagnostics
    }

    /// 启动实时诊断
    pub fn start_live_diagnostics(&self) -> Result<(), Box<dyn std::error::Error>> {
        *self.live_diagnostics_enabled.lock().unwrap() = true;
        println!("✅ 实时诊断已启动");
        Ok(())
    }

    /// 停止实时诊断
    pub fn stop_live_diagnostics(&self) {
        *self.live_diagnostics_enabled.lock().unwrap() = false;
        println!("🛑 实时诊断已停止");
    }

    /// 清除缓存
    pub fn clear_cache(&self) {
        self.diagnostic_cache.lock().unwrap().clear();
    }

    /// 获取诊断统计
    pub fn get_statistics(&self) -> DiagnosticStatistics {
        let cache = self.diagnostic_cache.lock().unwrap();
        let mut stats = DiagnosticStatistics::new();

        for diagnostics in cache.values() {
            for diagnostic in diagnostics {
                match diagnostic.level {
                    DiagnosticLevel::Error => stats.error_count += 1,
                    DiagnosticLevel::Warning => stats.warning_count += 1,
                    DiagnosticLevel::Info => stats.info_count += 1,
                    DiagnosticLevel::Hint => stats.hint_count += 1,
                }
            }
        }

        stats.total_files = cache.len();
        stats
    }

    /// 计算代码哈希
    fn hash_code(code: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);
        hasher.finish()
    }
}

/// 诊断统计
#[derive(Debug, Clone, Default)]
pub struct DiagnosticStatistics {
    /// 错误数量
    pub error_count: usize,
    /// 警告数量
    pub warning_count: usize,
    /// 信息数量
    pub info_count: usize,
    /// 提示数量
    pub hint_count: usize,
    /// 总文件数
    pub total_files: usize,
}

impl DiagnosticStatistics {
    /// 创建新的统计
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取总诊断数
    pub fn total_diagnostics(&self) -> usize {
        self.error_count + self.warning_count + self.info_count + self.hint_count
    }

    /// 是否有问题
    pub fn has_issues(&self) -> bool {
        self.error_count > 0 || self.warning_count > 0
    }
}

/// 诊断规则trait
pub trait DiagnosticRule: Send + Sync {
    /// 检查代码
    fn check(&self, code: &str, file_path: &str) -> Vec<Diagnostic>;

    /// 规则名称
    fn name(&self) -> &str;

    /// 规则描述
    fn description(&self) -> &str;
}

/// 语法错误规则
struct SyntaxErrorRule;

impl DiagnosticRule for SyntaxErrorRule {
    fn check(&self, code: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // 简单的语法检查
        let lines: Vec<&str> = code.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // 检查未闭合的大括号
            let open_braces = line.matches('{').count();
            let close_braces = line.matches('}').count();

            if open_braces > close_braces {
                // 这是一个简化的检查，实际应该跨行检查
                if line_num == lines.len() {
                    diagnostics.push(
                        Diagnostic::new(
                            DiagnosticLevel::Error,
                            DiagnosticType::SyntaxError,
                            "E001".to_string(),
                            "未闭合的大括号".to_string(),
                            CodeRange::point(CodePosition::new(line_num, line.len(), 0)),
                            file_path.to_string(),
                        )
                        .with_description("CSS规则块必须以 '}' 结束".to_string())
                        .with_fix(DiagnosticFix::new(
                            "添加闭合大括号".to_string(),
                            vec![TextEdit::insert(
                                CodePosition::new(line_num, line.len() + 1, 0),
                                "}".to_string(),
                            )],
                        )),
                    );
                }
            }

            // 检查无效的属性语法
            if line.contains(':') && !line.contains(';') && !line.trim().ends_with('{') {
                if !line.trim().is_empty() && !line.trim().starts_with("/*") {
                    diagnostics.push(
                        Diagnostic::new(
                            DiagnosticLevel::Warning,
                            DiagnosticType::SyntaxError,
                            "W001".to_string(),
                            "CSS属性可能缺少分号".to_string(),
                            CodeRange::point(CodePosition::new(line_num, line.len(), 0)),
                            file_path.to_string(),
                        )
                        .with_description("CSS属性声明应该以分号结束".to_string())
                        .with_fix(DiagnosticFix::new(
                            "添加分号".to_string(),
                            vec![TextEdit::insert(
                                CodePosition::new(line_num, line.len() + 1, 0),
                                ";".to_string(),
                            )],
                        )),
                    );
                }
            }
        }

        diagnostics
    }

    fn name(&self) -> &str {
        "syntax-error"
    }

    fn description(&self) -> &str {
        "检查CSS语法错误"
    }
}

/// 未使用选择器规则
struct UnusedSelectorRule;

impl DiagnosticRule for UnusedSelectorRule {
    fn check(&self, code: &str, _file_path: &str) -> Vec<Diagnostic> {
        // 这里应该实现更复杂的未使用选择器检测
        // 目前返回空结果
        Vec::new()
    }

    fn name(&self) -> &str {
        "unused-selector"
    }

    fn description(&self) -> &str {
        "检查未使用的CSS选择器"
    }
}

/// 重复属性规则
struct DuplicatePropertyRule;

impl DiagnosticRule for DuplicatePropertyRule {
    fn check(&self, code: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = code.lines().collect();
        let mut properties_in_rule = HashMap::new();
        let mut in_rule = false;

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;
            let trimmed = line.trim();

            if trimmed.contains('{') {
                in_rule = true;
                properties_in_rule.clear();
            } else if trimmed.contains('}') {
                in_rule = false;
                properties_in_rule.clear();
            } else if in_rule && trimmed.contains(':') {
                if let Some(property) = trimmed.split(':').next() {
                    let property = property.trim();
                    if let Some(prev_line) = properties_in_rule.get(property) {
                        diagnostics.push(
                            Diagnostic::new(
                                DiagnosticLevel::Warning,
                                DiagnosticType::DuplicateCode,
                                "W002".to_string(),
                                format!("重复的CSS属性: {}", property),
                                CodeRange::point(CodePosition::new(line_num, 1, 0)),
                                file_path.to_string(),
                            )
                            .with_description(format!(
                                "属性 '{}' 在第 {} 行已经定义过",
                                property, prev_line
                            ))
                            .with_related_info(RelatedInformation {
                                location: CodeRange::point(CodePosition::new(*prev_line, 1, 0)),
                                file_path: file_path.to_string(),
                                message: "首次定义位置".to_string(),
                            }),
                        );
                    } else {
                        properties_in_rule.insert(property.to_string(), line_num);
                    }
                }
            }
        }

        diagnostics
    }

    fn name(&self) -> &str {
        "duplicate-property"
    }

    fn description(&self) -> &str {
        "检查重复的CSS属性"
    }
}

/// 性能警告规则
struct PerformanceWarningRule;

impl DiagnosticRule for PerformanceWarningRule {
    fn check(&self, code: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // 检查 !important 使用
            if line.contains("!important") {
                diagnostics.push(
                    Diagnostic::new(
                        DiagnosticLevel::Warning,
                        DiagnosticType::PerformanceWarning,
                        "P001".to_string(),
                        "避免使用 !important".to_string(),
                        CodeRange::point(CodePosition::new(line_num, 1, 0)),
                        file_path.to_string(),
                    )
                    .with_description(
                        "!important 会破坏CSS的级联特性，应该通过提高选择器特异性来解决"
                            .to_string(),
                    ),
                );
            }

            // 检查通配符选择器
            if line.contains("*") && !line.contains("/*") {
                diagnostics.push(
                    Diagnostic::new(
                        DiagnosticLevel::Info,
                        DiagnosticType::PerformanceWarning,
                        "P002".to_string(),
                        "通配符选择器可能影响性能".to_string(),
                        CodeRange::point(CodePosition::new(line_num, 1, 0)),
                        file_path.to_string(),
                    )
                    .with_description("通配符选择器会匹配所有元素，可能影响渲染性能".to_string()),
                );
            }
        }

        diagnostics
    }

    fn name(&self) -> &str {
        "performance-warning"
    }

    fn description(&self) -> &str {
        "检查性能相关问题"
    }
}

/// 可访问性规则
struct AccessibilityRule;

impl DiagnosticRule for AccessibilityRule {
    fn check(&self, code: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // 检查颜色对比度相关
            if line.contains("color:") && line.contains("#") {
                // 这里应该实现更复杂的颜色对比度检查
                // 目前只是一个示例
                if line.contains("#fff") || line.contains("#ffffff") {
                    diagnostics.push(
                        Diagnostic::new(
                            DiagnosticLevel::Info,
                            DiagnosticType::Accessibility,
                            "A001".to_string(),
                            "考虑检查颜色对比度".to_string(),
                            CodeRange::point(CodePosition::new(line_num, 1, 0)),
                            file_path.to_string(),
                        )
                        .with_description(
                            "确保文本颜色与背景颜色有足够的对比度以提高可访问性".to_string(),
                        ),
                    );
                }
            }
        }

        diagnostics
    }

    fn name(&self) -> &str {
        "accessibility-check"
    }

    fn description(&self) -> &str {
        "检查可访问性问题"
    }
}

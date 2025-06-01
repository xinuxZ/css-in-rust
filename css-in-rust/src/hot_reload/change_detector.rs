//! 变更检测器模块
//!
//! 用于分析和分类文件变更，提供智能的变更检测和处理

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// 变更类型
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    /// CSS文件变更
    CssFile {
        /// 是否只是样式变更（可以热注入）
        style_only: bool,
        /// 变更的选择器
        changed_selectors: Vec<String>,
    },
    /// Rust源文件变更
    RustFile {
        /// 是否包含宏调用
        has_macro_calls: bool,
        /// 变更的函数
        changed_functions: Vec<String>,
    },
    /// 配置文件变更
    ConfigFile {
        /// 配置类型
        config_type: ConfigType,
    },
    /// 资源文件变更
    AssetFile {
        /// 资源类型
        asset_type: AssetType,
    },
    /// 依赖文件变更
    DependencyFile,
    /// 构建脚本变更
    BuildScript,
    /// 文档文件变更
    Documentation,
    /// 测试文件变更
    TestFile,
    /// 其他文件变更
    Other(String),
}

/// 配置文件类型
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigType {
    /// Cargo.toml
    Cargo,
    /// package.json
    PackageJson,
    /// CSS-in-Rust配置
    CssInRust,
    /// 其他配置
    Other(String),
}

/// 资源文件类型
#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    /// 图片
    Image(String), // 扩展名
    /// 字体
    Font(String),
    /// 样式表
    Stylesheet(String),
    /// JavaScript
    JavaScript,
    /// TypeScript
    TypeScript,
    /// HTML
    Html,
    /// 其他
    Other(String),
}

/// 文件变更信息
#[derive(Debug, Clone)]
pub struct FileChange {
    /// 文件路径
    pub path: PathBuf,
    /// 变更类型
    pub change_type: ChangeType,
    /// 变更时间
    pub timestamp: SystemTime,
    /// 文件大小
    pub file_size: Option<u64>,
    /// 变更的行数
    pub changed_lines: Option<Vec<usize>>,
    /// 是否需要完整重新编译
    pub requires_full_rebuild: bool,
    /// 是否可以热更新
    pub supports_hot_reload: bool,
    /// 影响的依赖文件
    pub affected_dependencies: Vec<PathBuf>,
    /// 变更摘要
    pub change_summary: String,
}

impl FileChange {
    /// 创建新的文件变更
    pub fn new(path: PathBuf, change_type: ChangeType) -> Self {
        let metadata = fs::metadata(&path).ok();
        let file_size = metadata.as_ref().map(|m| m.len());

        let (requires_full_rebuild, supports_hot_reload) = match &change_type {
            ChangeType::CssFile { style_only, .. } => (!style_only, *style_only),
            ChangeType::RustFile {
                has_macro_calls, ..
            } => (*has_macro_calls, false),
            ChangeType::ConfigFile { .. } => (true, false),
            ChangeType::DependencyFile => (true, false),
            ChangeType::BuildScript => (true, false),
            ChangeType::AssetFile { .. } => (false, true),
            _ => (false, false),
        };

        Self {
            path: path.clone(),
            change_type: change_type.clone(),
            timestamp: SystemTime::now(),
            file_size,
            changed_lines: None,
            requires_full_rebuild,
            supports_hot_reload,
            affected_dependencies: Vec::new(),
            change_summary: Self::generate_summary(&path, &change_type),
        }
    }

    /// 生成变更摘要
    fn generate_summary(path: &Path, change_type: &ChangeType) -> String {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("未知文件");

        match change_type {
            ChangeType::CssFile {
                style_only,
                changed_selectors,
            } => {
                if *style_only {
                    format!(
                        "CSS样式更新: {} ({}个选择器)",
                        file_name,
                        changed_selectors.len()
                    )
                } else {
                    format!("CSS文件变更: {} (需要重新编译)", file_name)
                }
            }
            ChangeType::RustFile {
                has_macro_calls,
                changed_functions,
            } => {
                if *has_macro_calls {
                    format!("Rust文件变更: {} (包含宏调用)", file_name)
                } else {
                    format!(
                        "Rust文件变更: {} ({}个函数)",
                        file_name,
                        changed_functions.len()
                    )
                }
            }
            ChangeType::ConfigFile { config_type } => {
                format!("配置文件变更: {} ({:?})", file_name, config_type)
            }
            ChangeType::AssetFile { asset_type } => {
                format!("资源文件变更: {} ({:?})", file_name, asset_type)
            }
            ChangeType::DependencyFile => {
                format!("依赖文件变更: {}", file_name)
            }
            ChangeType::BuildScript => {
                format!("构建脚本变更: {}", file_name)
            }
            ChangeType::Documentation => {
                format!("文档文件变更: {}", file_name)
            }
            ChangeType::TestFile => {
                format!("测试文件变更: {}", file_name)
            }
            ChangeType::Other(desc) => {
                format!("其他文件变更: {} ({})", file_name, desc)
            }
        }
    }

    /// 检查是否为关键变更
    pub fn is_critical_change(&self) -> bool {
        self.requires_full_rebuild
            || matches!(
                self.change_type,
                ChangeType::ConfigFile { .. }
                    | ChangeType::DependencyFile
                    | ChangeType::BuildScript
            )
    }

    /// 检查是否可以增量更新
    pub fn supports_incremental_update(&self) -> bool {
        !self.requires_full_rebuild
            && matches!(
                self.change_type,
                ChangeType::CssFile { .. }
                    | ChangeType::AssetFile { .. }
                    | ChangeType::Documentation
            )
    }

    /// 获取优先级
    pub fn priority(&self) -> u8 {
        match &self.change_type {
            ChangeType::ConfigFile { .. } => 10,
            ChangeType::DependencyFile => 9,
            ChangeType::BuildScript => 8,
            ChangeType::RustFile {
                has_macro_calls: true,
                ..
            } => 7,
            ChangeType::RustFile {
                has_macro_calls: false,
                ..
            } => 5,
            ChangeType::CssFile {
                style_only: false, ..
            } => 6,
            ChangeType::CssFile {
                style_only: true, ..
            } => 3,
            ChangeType::AssetFile { .. } => 2,
            ChangeType::TestFile => 1,
            ChangeType::Documentation => 1,
            ChangeType::Other(_) => 0,
        }
    }
}

/// 变更检测器配置
#[derive(Debug, Clone)]
pub struct ChangeDetectorConfig {
    /// 是否启用内容分析
    pub enable_content_analysis: bool,
    /// 是否启用依赖分析
    pub enable_dependency_analysis: bool,
    /// 是否启用智能分类
    pub enable_smart_classification: bool,
    /// CSS选择器分析深度
    pub css_analysis_depth: usize,
    /// Rust代码分析深度
    pub rust_analysis_depth: usize,
    /// 最大文件大小（字节）
    pub max_file_size: u64,
    /// 是否启用缓存
    pub enable_cache: bool,
    /// 缓存过期时间（秒）
    pub cache_ttl_seconds: u64,
}

impl Default for ChangeDetectorConfig {
    fn default() -> Self {
        Self {
            enable_content_analysis: true,
            enable_dependency_analysis: true,
            enable_smart_classification: true,
            css_analysis_depth: 3,
            rust_analysis_depth: 2,
            max_file_size: 10 * 1024 * 1024, // 10MB
            enable_cache: true,
            cache_ttl_seconds: 300, // 5分钟
        }
    }
}

/// 文件分析缓存项
#[derive(Debug, Clone)]
struct CacheEntry {
    /// 文件内容哈希
    content_hash: u64,
    /// 分析结果
    analysis_result: ChangeType,
    /// 缓存时间
    cached_at: SystemTime,
    /// 文件大小
    file_size: u64,
}

/// 变更检测器
pub struct ChangeDetector {
    config: ChangeDetectorConfig,
    analysis_cache: HashMap<PathBuf, CacheEntry>,
    dependency_graph: HashMap<PathBuf, HashSet<PathBuf>>,
    css_selectors_cache: HashMap<PathBuf, Vec<String>>,
    rust_functions_cache: HashMap<PathBuf, Vec<String>>,
}

impl ChangeDetector {
    /// 创建新的变更检测器
    pub fn new() -> Self {
        Self::with_config(ChangeDetectorConfig::default())
    }

    /// 使用配置创建变更检测器
    pub fn with_config(config: ChangeDetectorConfig) -> Self {
        Self {
            config,
            analysis_cache: HashMap::new(),
            dependency_graph: HashMap::new(),
            css_selectors_cache: HashMap::new(),
            rust_functions_cache: HashMap::new(),
        }
    }

    /// 分析文件变更
    pub fn analyze_change(&mut self, path: &Path) -> Result<FileChange, ChangeDetectorError> {
        if !path.exists() {
            return Err(ChangeDetectorError::FileNotFound(path.to_path_buf()));
        }

        let metadata =
            fs::metadata(path).map_err(|e| ChangeDetectorError::IoError(e.to_string()))?;

        if metadata.len() > self.config.max_file_size {
            return Err(ChangeDetectorError::FileTooLarge(
                path.to_path_buf(),
                metadata.len(),
            ));
        }

        // 检查缓存
        if self.config.enable_cache {
            if let Some(cached_result) = self.get_cached_analysis(path, &metadata) {
                let mut change = FileChange::new(path.to_path_buf(), cached_result);
                change.file_size = Some(metadata.len());
                return Ok(change);
            }
        }

        // 执行分析
        let change_type = self.classify_file_change(path)?;

        // 更新缓存
        if self.config.enable_cache {
            self.update_cache(path, &metadata, &change_type)?;
        }

        let mut change = FileChange::new(path.to_path_buf(), change_type);
        change.file_size = Some(metadata.len());

        // 分析依赖关系
        if self.config.enable_dependency_analysis {
            change.affected_dependencies = self.analyze_dependencies(path)?;
        }

        Ok(change)
    }

    /// 批量分析变更
    pub fn analyze_changes(
        &mut self,
        paths: &[PathBuf],
    ) -> Vec<Result<FileChange, ChangeDetectorError>> {
        paths.iter().map(|path| self.analyze_change(path)).collect()
    }

    /// 分类文件变更
    fn classify_file_change(&mut self, path: &Path) -> Result<ChangeType, ChangeDetectorError> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase());

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        match extension.as_deref() {
            Some("css") | Some("scss") | Some("less") | Some("sass") => self.analyze_css_file(path),
            Some("rs") => self.analyze_rust_file(path),
            Some("toml") if file_name == "Cargo.toml" => Ok(ChangeType::ConfigFile {
                config_type: ConfigType::Cargo,
            }),
            Some("json") if file_name == "package.json" => Ok(ChangeType::ConfigFile {
                config_type: ConfigType::PackageJson,
            }),
            Some("json") | Some("yaml") | Some("yml") | Some("toml") => {
                Ok(ChangeType::ConfigFile {
                    config_type: ConfigType::Other(
                        extension.unwrap_or("unknown".to_string()).to_string(),
                    ),
                })
            }
            Some("js") => Ok(ChangeType::AssetFile {
                asset_type: AssetType::JavaScript,
            }),
            Some("ts") => Ok(ChangeType::AssetFile {
                asset_type: AssetType::TypeScript,
            }),
            Some("html") | Some("htm") => Ok(ChangeType::AssetFile {
                asset_type: AssetType::Html,
            }),
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") | Some("webp") => {
                Ok(ChangeType::AssetFile {
                    asset_type: AssetType::Image(extension.unwrap().to_string()),
                })
            }
            Some("woff") | Some("woff2") | Some("ttf") | Some("otf") | Some("eot") => {
                Ok(ChangeType::AssetFile {
                    asset_type: AssetType::Font(extension.unwrap().to_string()),
                })
            }
            Some("md") | Some("txt") | Some("rst") => Ok(ChangeType::Documentation),
            _ => {
                // 检查特殊文件名
                if file_name == "build.rs" {
                    Ok(ChangeType::BuildScript)
                } else if file_name.contains("test") || path.to_string_lossy().contains("/tests/") {
                    Ok(ChangeType::TestFile)
                } else if file_name.contains("lock") {
                    Ok(ChangeType::DependencyFile)
                } else {
                    Ok(ChangeType::Other(
                        extension.unwrap_or("unknown".to_string()).to_string(),
                    ))
                }
            }
        }
    }

    /// 分析CSS文件
    fn analyze_css_file(&mut self, path: &Path) -> Result<ChangeType, ChangeDetectorError> {
        if !self.config.enable_content_analysis {
            return Ok(ChangeType::CssFile {
                style_only: true,
                changed_selectors: vec![],
            });
        }

        let content = self.read_file_content(path)?;
        let selectors = self.extract_css_selectors(&content)?;

        // 检查是否只是样式变更
        let style_only = self.is_css_style_only_change(path, &content)?;

        // 更新选择器缓存
        self.css_selectors_cache
            .insert(path.to_path_buf(), selectors.clone());

        Ok(ChangeType::CssFile {
            style_only,
            changed_selectors: selectors,
        })
    }

    /// 分析Rust文件
    fn analyze_rust_file(&mut self, path: &Path) -> Result<ChangeType, ChangeDetectorError> {
        if !self.config.enable_content_analysis {
            return Ok(ChangeType::RustFile {
                has_macro_calls: false,
                changed_functions: vec![],
            });
        }

        let content = self.read_file_content(path)?;

        // 检查是否包含CSS-in-Rust宏调用
        let has_macro_calls = self.has_css_macro_calls(&content);

        // 提取函数名
        let functions = self.extract_rust_functions(&content)?;

        // 更新函数缓存
        self.rust_functions_cache
            .insert(path.to_path_buf(), functions.clone());

        Ok(ChangeType::RustFile {
            has_macro_calls,
            changed_functions: functions,
        })
    }

    /// 读取文件内容
    fn read_file_content(&self, path: &Path) -> Result<String, ChangeDetectorError> {
        let mut file =
            fs::File::open(path).map_err(|e| ChangeDetectorError::IoError(e.to_string()))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| ChangeDetectorError::IoError(e.to_string()))?;

        Ok(content)
    }

    /// 提取CSS选择器
    fn extract_css_selectors(&self, content: &str) -> Result<Vec<String>, ChangeDetectorError> {
        let mut selectors = Vec::new();

        // 简单的CSS选择器提取（可以使用更复杂的CSS解析器）
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.ends_with('{') && !trimmed.starts_with('@') {
                let selector = trimmed.trim_end_matches('{').trim();
                if !selector.is_empty() {
                    selectors.push(selector.to_string());
                }
            }
        }

        Ok(selectors)
    }

    /// 检查是否只是CSS样式变更
    fn is_css_style_only_change(
        &self,
        path: &Path,
        content: &str,
    ) -> Result<bool, ChangeDetectorError> {
        // 检查缓存中的选择器
        if let Some(cached_selectors) = self.css_selectors_cache.get(path) {
            let current_selectors = self.extract_css_selectors(content)?;

            // 如果选择器没有变化，认为是样式变更
            return Ok(cached_selectors == &current_selectors);
        }

        // 没有缓存，默认认为不是纯样式变更
        Ok(false)
    }

    /// 检查是否包含CSS宏调用
    fn has_css_macro_calls(&self, content: &str) -> bool {
        content.contains("css!")
            || content.contains("css_class!")
            || content.contains("css_if!")
            || content.contains("style!")
            || content.contains("styled!")
    }

    /// 提取Rust函数名
    fn extract_rust_functions(&self, content: &str) -> Result<Vec<String>, ChangeDetectorError> {
        let mut functions = Vec::new();

        // 简单的函数名提取（可以使用syn等更复杂的解析器）
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                if let Some(name_start) = trimmed.find("fn ") {
                    let after_fn = &trimmed[name_start + 3..];
                    if let Some(paren_pos) = after_fn.find('(') {
                        let function_name = after_fn[..paren_pos].trim();
                        if !function_name.is_empty() {
                            functions.push(function_name.to_string());
                        }
                    }
                }
            }
        }

        Ok(functions)
    }

    /// 分析依赖关系
    fn analyze_dependencies(&self, _path: &Path) -> Result<Vec<PathBuf>, ChangeDetectorError> {
        // 简化实现，返回空依赖列表
        // 在实际实现中，可以分析import/use语句等
        Ok(vec![])
    }

    /// 获取缓存的分析结果
    fn get_cached_analysis(&self, path: &Path, metadata: &fs::Metadata) -> Option<ChangeType> {
        if let Some(cache_entry) = self.analysis_cache.get(path) {
            // 检查缓存是否过期
            let now = SystemTime::now();
            if let Ok(elapsed) = now.duration_since(cache_entry.cached_at) {
                if elapsed.as_secs() > self.config.cache_ttl_seconds {
                    return None;
                }
            }

            // 检查文件是否有变化
            if cache_entry.file_size == metadata.len() {
                return Some(cache_entry.analysis_result.clone());
            }
        }

        None
    }

    /// 更新缓存
    fn update_cache(
        &mut self,
        path: &Path,
        metadata: &fs::Metadata,
        change_type: &ChangeType,
    ) -> Result<(), ChangeDetectorError> {
        let content = self.read_file_content(path)?;
        let content_hash = self.calculate_hash(&content);

        let cache_entry = CacheEntry {
            content_hash,
            analysis_result: change_type.clone(),
            cached_at: SystemTime::now(),
            file_size: metadata.len(),
        };

        self.analysis_cache.insert(path.to_path_buf(), cache_entry);

        Ok(())
    }

    /// 计算内容哈希
    fn calculate_hash(&self, content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }

    /// 清理过期缓存
    pub fn cleanup_cache(&mut self) {
        let now = SystemTime::now();
        let ttl = Duration::from_secs(self.config.cache_ttl_seconds);

        self.analysis_cache.retain(|_, entry| {
            now.duration_since(entry.cached_at)
                .map(|elapsed| elapsed < ttl)
                .unwrap_or(false)
        });
    }

    /// 获取缓存统计
    pub fn cache_stats(&self) -> (usize, usize, usize) {
        (
            self.analysis_cache.len(),
            self.css_selectors_cache.len(),
            self.rust_functions_cache.len(),
        )
    }

    /// 清空所有缓存
    pub fn clear_cache(&mut self) {
        self.analysis_cache.clear();
        self.css_selectors_cache.clear();
        self.rust_functions_cache.clear();
        self.dependency_graph.clear();
    }

    /// 更新配置
    pub fn update_config(&mut self, config: ChangeDetectorConfig) {
        self.config = config;

        // 如果禁用了缓存，清空缓存
        if !self.config.enable_cache {
            self.clear_cache();
        }
    }
}

/// 变更检测器错误
#[derive(Debug, Clone)]
pub enum ChangeDetectorError {
    /// 文件不存在
    FileNotFound(PathBuf),
    /// 文件过大
    FileTooLarge(PathBuf, u64),
    /// IO错误
    IoError(String),
    /// 解析错误
    ParseError(String),
    /// 编码错误
    EncodingError(String),
    /// 配置错误
    ConfigError(String),
}

impl std::fmt::Display for ChangeDetectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeDetectorError::FileNotFound(path) => {
                write!(f, "文件不存在: {:?}", path)
            }
            ChangeDetectorError::FileTooLarge(path, size) => {
                write!(f, "文件过大: {:?} ({} 字节)", path, size)
            }
            ChangeDetectorError::IoError(msg) => {
                write!(f, "IO错误: {}", msg)
            }
            ChangeDetectorError::ParseError(msg) => {
                write!(f, "解析错误: {}", msg)
            }
            ChangeDetectorError::EncodingError(msg) => {
                write!(f, "编码错误: {}", msg)
            }
            ChangeDetectorError::ConfigError(msg) => {
                write!(f, "配置错误: {}", msg)
            }
        }
    }
}

impl std::error::Error for ChangeDetectorError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_change_detector_creation() {
        let detector = ChangeDetector::new();
        assert!(detector.config.enable_content_analysis);
    }

    #[test]
    fn test_css_file_analysis() {
        let temp_dir = TempDir::new().unwrap();
        let css_file = temp_dir.path().join("test.css");

        let mut file = File::create(&css_file).unwrap();
        writeln!(file, ".class1 {{").unwrap();
        writeln!(file, "  color: red;").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file, ".class2 {{").unwrap();
        writeln!(file, "  background: blue;").unwrap();
        writeln!(file, "}}").unwrap();

        let mut detector = ChangeDetector::new();
        let result = detector.analyze_change(&css_file).unwrap();

        match result.change_type {
            ChangeType::CssFile {
                changed_selectors, ..
            } => {
                assert_eq!(changed_selectors.len(), 2);
                assert!(changed_selectors.contains(&".class1".to_string()));
                assert!(changed_selectors.contains(&".class2".to_string()));
            }
            _ => panic!("Expected CssFile change type"),
        }
    }

    #[test]
    fn test_rust_file_analysis() {
        let temp_dir = TempDir::new().unwrap();
        let rust_file = temp_dir.path().join("test.rs");

        let mut file = File::create(&rust_file).unwrap();
        writeln!(file, "fn main() {{").unwrap();
        writeln!(file, "    let styles = css! {{").unwrap();
        writeln!(file, "        color: red;").unwrap();
        writeln!(file, "    }};").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file, "pub fn helper() {{").unwrap();
        writeln!(file, "}}").unwrap();

        let mut detector = ChangeDetector::new();
        let result = detector.analyze_change(&rust_file).unwrap();

        match result.change_type {
            ChangeType::RustFile {
                has_macro_calls,
                changed_functions,
            } => {
                assert!(has_macro_calls);
                assert_eq!(changed_functions.len(), 2);
                assert!(changed_functions.contains(&"main".to_string()));
                assert!(changed_functions.contains(&"helper".to_string()));
            }
            _ => panic!("Expected RustFile change type"),
        }
    }

    #[test]
    fn test_config_file_detection() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_file = temp_dir.path().join("Cargo.toml");

        File::create(&cargo_file).unwrap();

        let mut detector = ChangeDetector::new();
        let result = detector.analyze_change(&cargo_file).unwrap();

        match result.change_type {
            ChangeType::ConfigFile {
                config_type: ConfigType::Cargo,
            } => {}
            _ => panic!("Expected Cargo config file type"),
        }
    }

    #[test]
    fn test_file_change_priority() {
        let temp_dir = TempDir::new().unwrap();

        // 创建不同类型的文件
        let cargo_file = temp_dir.path().join("Cargo.toml");
        let css_file = temp_dir.path().join("style.css");
        let rust_file = temp_dir.path().join("main.rs");

        File::create(&cargo_file).unwrap();
        File::create(&css_file).unwrap();
        File::create(&rust_file).unwrap();

        let mut detector = ChangeDetector::new();

        let cargo_change = detector.analyze_change(&cargo_file).unwrap();
        let css_change = detector.analyze_change(&css_file).unwrap();
        let rust_change = detector.analyze_change(&rust_file).unwrap();

        // 配置文件应该有最高优先级
        assert!(cargo_change.priority() > css_change.priority());
        assert!(cargo_change.priority() > rust_change.priority());
    }
}

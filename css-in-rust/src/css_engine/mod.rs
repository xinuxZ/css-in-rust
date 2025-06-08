//! Core CSS processing functionality
//!
//! This module provides the fundamental CSS parsing and optimization capabilities.
//! It includes parsers for processing CSS strings and optimizers for minification
//! and dead code elimination.

pub mod optimizer;
pub mod parser;

pub use optimizer::{CssOptimizer, OptimizationError, OptimizerConfig};
pub use parser::{CssParser, ParseError, ParserConfig};

/// Result type for CSS operations
///
/// A type alias for `std::result::Result` with the error type set to `CssError`
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::{Result, CssError};
///
/// fn process_css(input: &str) -> Result<String> {
///     if input.is_empty() {
///         return Err(CssError::ParseError("Empty CSS input".to_string()));
///     }
///     Ok(input.to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, CssError>;

/// CSS处理操作的主要错误类型
///
/// 表示在CSS处理过程中可能发生的所有错误类型。
///
/// # 示例
///
/// ```
/// use css_in_rust::css_engine::{CssError, Result};
/// use std::io;
///
/// // 读取并处理CSS文件
/// fn read_and_process_css(path: &str) -> Result<String> {
///     // 从文件读取CSS内容，如果发生IO错误则转换为CssError
///     let css = std::fs::read_to_string(path)
///         .map_err(|e| CssError::IoError(e))?;
///
///     // 检查CSS内容有效性
///     if css.contains("invalid") {
///         return Err(CssError::ParseError("CSS内容无效".to_string()));
///     }
///
///     // 处理主题变量
///     if css.contains("--theme") && !css.contains(":root") {
///         return Err(CssError::ThemeError("主题变量必须在:root中定义".to_string()));
///     }
///
///     // 返回处理后的CSS
///     Ok(css)
/// }
/// ```
#[derive(Debug)]
pub enum CssError {
    /// CSS解析过程中发生的错误
    ParseError(String),
    /// 样式注入过程中发生的错误
    InjectionError(String),
    /// 主题处理相关的错误
    ThemeError(String),
    /// 读取或写入文件时发生的I/O错误
    IoError(std::io::Error),
    /// CSS优化过程中发生的错误
    OptimizationError(String),
}

impl std::fmt::Display for CssError {
    /// 格式化错误信息以便显示
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::css_engine::CssError;
    ///
    /// // 创建一个解析错误
    /// let error = CssError::ParseError("选择器无效".to_string());
    ///
    /// // 打印错误信息
    /// println!("错误: {}", error);
    /// // 输出: "错误: CSS parsing failed: 选择器无效"
    ///
    /// // 创建一个主题错误
    /// let theme_error = CssError::ThemeError("找不到主题变量".to_string());
    /// println!("主题错误: {}", theme_error);
    /// // 输出: "主题错误: Theme error: 找不到主题变量"
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssError::ParseError(msg) => write!(f, "CSS parsing failed: {}", msg),
            CssError::InjectionError(msg) => write!(f, "Style injection failed: {}", msg),
            CssError::ThemeError(msg) => write!(f, "Theme error: {}", msg),
            CssError::IoError(err) => write!(f, "IO error: {}", err),
            CssError::OptimizationError(msg) => write!(f, "Optimization error: {}", msg),
        }
    }
}

impl std::error::Error for CssError {
    /// 返回此错误的源错误（如果是由另一个错误导致的）
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::css_engine::CssError;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// // 创建一个IO错误
    /// let io_error = io::Error::new(io::ErrorKind::NotFound, "找不到文件");
    /// // 将IO错误包装成CSS错误
    /// let css_error = CssError::IoError(io_error);
    ///
    /// // 获取并处理源错误
    /// if let Some(source) = css_error.source() {
    ///     println!("错误原因: {}", source);
    ///     // 输出: "错误原因: 找不到文件"
    /// }
    ///
    /// // 对于非IO错误，source返回None
    /// let parse_error = CssError::ParseError("语法错误".to_string());
    /// assert!(parse_error.source().is_none());
    /// ```
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CssError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for CssError {
    /// 将标准I/O错误转换为`CssError`
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::css_engine::{CssError, Result};
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// // 读取CSS文件的函数
    /// fn read_css_file(path: &str) -> Result<String> {
    ///     // 打开文件，io::Error自动转换为CssError
    ///     let mut file = File::open(path)?;
    ///
    ///     let mut contents = String::new();
    ///     // 读取文件内容，io::Error自动转换为CssError
    ///     file.read_to_string(&mut contents)?;
    ///
    ///     // 返回CSS内容
    ///     Ok(contents)
    /// }
    ///
    /// // 手动转换IO错误
    /// fn manual_conversion() -> Result<()> {
    ///     let io_result = std::fs::read_to_string("不存在的文件.css");
    ///     if let Err(io_err) = io_result {
    ///         // 手动将io::Error转换为CssError
    ///         let css_err: CssError = io_err.into();
    ///         return Err(css_err);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn from(err: std::io::Error) -> Self {
        CssError::IoError(err)
    }
}

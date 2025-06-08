/// 设计令牌系统模块
///
/// 本模块提供设计令牌（Design Token）系统的核心功能，用于定义、管理和应用主题变量。
/// 设计令牌是设计系统中的原子单元，如颜色、字体、间距等，它们用于构建一致的用户界面。
///
/// # 子模块
///
/// - `css_generator`: 将设计令牌转换为 CSS 变量的工具
/// - `definitions`: 设计令牌的基本定义和类型
/// - `resolver`: 设计令牌路径解析器
/// - `simple_system`: 简单主题系统实现
/// - `system`: 设计令牌系统的接口定义
/// - `values`: 设计令牌值的实现和存储
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::{TokenDefinitions, TokenValue};
/// use css_in_rust::theme::core::token::simple_system::{TokenSystem, ThemeVariant};
///
/// // 创建令牌系统
/// let mut system = TokenSystem::new();
///
/// // 设置令牌值
/// system.set_token_value("colors.primary", TokenValue::Color("#1890ff".to_string()));
///
/// // 获取令牌值
/// if let Some(value) = system.get_token_value("colors.primary") {
///     println!("主色值: {:?}", value);
/// }
/// ```
pub mod css_generator;
pub mod definitions;
pub mod resolver;
pub mod simple_system;
pub mod system;
pub mod values;

// Re-export commonly used types
/// 从各子模块重新导出的常用类型
pub use definitions::{
    ColorValue, DimensionUnit, DimensionValue, TokenMetadata, TokenReference, TokenType, TokenValue,
};
/// 从 simple_system 模块重新导出的类型
pub use simple_system::{ThemeVariant, TokenSystem};
/// 从 values 模块重新导出的类型
pub use values::{DesignTokens, TokenStore, TokenValuesImpl};

/// 设计令牌定义接口
///
/// 定义了获取和设置设计令牌值的基本操作。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::{TokenDefinitions, TokenValue, TokenMetadata};
///
/// struct MyTokenSystem {
///     // 系统实现...
/// }
///
/// impl TokenDefinitions for MyTokenSystem {
///     fn get_token_value(&self, path: &str) -> Option<TokenValue> {
///         // 实现获取令牌值的逻辑
///         None
///     }
///
///     fn set_token_value(&mut self, path: &str, value: TokenValue) {
///         // 实现设置令牌值的逻辑
///     }
///
///     fn get_metadata(&self, path: &str) -> Option<TokenMetadata> {
///         // 实现获取令牌元数据的逻辑
///         None
///     }
/// }
/// ```
pub trait TokenDefinitions {
    /// 获取指定路径的令牌值
    ///
    /// # 参数
    ///
    /// * `path` - 令牌路径，如 "colors.primary" 或 "spacing.md"
    ///
    /// # 返回值
    ///
    /// 如果找到令牌，返回 `Some(TokenValue)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::{TokenDefinitions, TokenValue};
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let system = TokenSystem::new();
    /// let primary_color = system.get_token_value("colors.primary");
    /// ```
    fn get_token_value(&self, path: &str) -> Option<TokenValue>;

    /// 设置指定路径的令牌值
    ///
    /// # 参数
    ///
    /// * `path` - 令牌路径，如 "colors.primary" 或 "spacing.md"
    /// * `value` - 要设置的令牌值
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::{TokenDefinitions, TokenValue};
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let mut system = TokenSystem::new();
    /// system.set_token_value("colors.primary", TokenValue::Color("#1890ff".to_string()));
    /// ```
    fn set_token_value(&mut self, path: &str, value: TokenValue);

    /// 获取指定路径的令牌元数据
    ///
    /// # 参数
    ///
    /// * `path` - 令牌路径，如 "colors.primary" 或 "spacing.md"
    ///
    /// # 返回值
    ///
    /// 如果找到令牌元数据，返回 `Some(TokenMetadata)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::{TokenDefinitions, TokenMetadata};
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let system = TokenSystem::new();
    /// if let Some(metadata) = system.get_metadata("colors.primary") {
    ///     println!("令牌描述: {}", metadata.description);
    /// }
    /// ```
    fn get_metadata(&self, path: &str) -> Option<TokenMetadata>;
}

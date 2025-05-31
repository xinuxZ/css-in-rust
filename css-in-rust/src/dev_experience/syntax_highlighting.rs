//! 语法高亮模块
//!
//! 提供CSS代码的语法高亮功能

use std::collections::HashMap;
use std::fmt;

/// 高亮主题
#[derive(Debug, Clone)]
pub struct HighlightTheme {
    /// 主题名称
    pub name: String,
    /// 颜色配置
    pub colors: HashMap<TokenType, String>,
    /// 样式配置
    pub styles: HashMap<TokenType, TextStyle>,
}

impl HighlightTheme {
    /// 创建默认主题
    pub fn default_theme() -> Self {
        let mut colors = HashMap::new();
        let mut styles = HashMap::new();

        // 设置默认颜色
        colors.insert(TokenType::Selector, "#569cd6".to_string()); // 蓝色
        colors.insert(TokenType::Property, "#92c5f8".to_string()); // 浅蓝色
        colors.insert(TokenType::Value, "#ce9178".to_string()); // 橙色
        colors.insert(TokenType::String, "#ce9178".to_string()); // 橙色
        colors.insert(TokenType::Number, "#b5cea8".to_string()); // 绿色
        colors.insert(TokenType::Unit, "#b5cea8".to_string()); // 绿色
        colors.insert(TokenType::Color, "#d7ba7d".to_string()); // 黄色
        colors.insert(TokenType::Comment, "#6a9955".to_string()); // 深绿色
        colors.insert(TokenType::Keyword, "#c586c0".to_string()); // 紫色
        colors.insert(TokenType::Function, "#dcdcaa".to_string()); // 黄绿色
        colors.insert(TokenType::AtRule, "#c586c0".to_string()); // 紫色
        colors.insert(TokenType::Punctuation, "#d4d4d4".to_string()); // 灰色
        colors.insert(TokenType::Error, "#f44747".to_string()); // 红色

        // 设置默认样式
        styles.insert(TokenType::Comment, TextStyle::Italic);
        styles.insert(TokenType::Keyword, TextStyle::Bold);
        styles.insert(TokenType::AtRule, TextStyle::Bold);
        styles.insert(TokenType::Error, TextStyle::Underline);

        Self {
            name: "default".to_string(),
            colors,
            styles,
        }
    }

    /// 创建暗色主题
    pub fn dark_theme() -> Self {
        let mut theme = Self::default_theme();
        theme.name = "dark".to_string();

        // 调整暗色主题的颜色
        theme
            .colors
            .insert(TokenType::Selector, "#4fc1ff".to_string());
        theme
            .colors
            .insert(TokenType::Property, "#9cdcfe".to_string());
        theme.colors.insert(TokenType::Value, "#ce9178".to_string());
        theme
            .colors
            .insert(TokenType::Comment, "#6a9955".to_string());

        theme
    }

    /// 创建亮色主题
    pub fn light_theme() -> Self {
        let mut theme = Self::default_theme();
        theme.name = "light".to_string();

        // 调整亮色主题的颜色
        theme
            .colors
            .insert(TokenType::Selector, "#0000ff".to_string());
        theme
            .colors
            .insert(TokenType::Property, "#0451a5".to_string());
        theme.colors.insert(TokenType::Value, "#a31515".to_string());
        theme
            .colors
            .insert(TokenType::Comment, "#008000".to_string());

        theme
    }

    /// 获取token颜色
    pub fn get_color(&self, token_type: &TokenType) -> Option<&String> {
        self.colors.get(token_type)
    }

    /// 获取token样式
    pub fn get_style(&self, token_type: &TokenType) -> Option<&TextStyle> {
        self.styles.get(token_type)
    }
}

/// 文本样式
#[derive(Debug, Clone, PartialEq)]
pub enum TextStyle {
    /// 正常
    Normal,
    /// 粗体
    Bold,
    /// 斜体
    Italic,
    /// 下划线
    Underline,
    /// 删除线
    Strikethrough,
}

/// Token类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// 选择器
    Selector,
    /// 属性
    Property,
    /// 值
    Value,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 单位
    Unit,
    /// 颜色
    Color,
    /// 注释
    Comment,
    /// 关键字
    Keyword,
    /// 函数
    Function,
    /// At规则
    AtRule,
    /// 标点符号
    Punctuation,
    /// 空白
    Whitespace,
    /// 错误
    Error,
    /// 未知
    Unknown,
}

/// Token
#[derive(Debug, Clone)]
pub struct Token {
    /// Token类型
    pub token_type: TokenType,
    /// 文本内容
    pub text: String,
    /// 开始位置
    pub start: usize,
    /// 结束位置
    pub end: usize,
    /// 行号
    pub line: usize,
    /// 列号
    pub column: usize,
}

impl Token {
    /// 创建新的Token
    pub fn new(
        token_type: TokenType,
        text: String,
        start: usize,
        end: usize,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            text,
            start,
            end,
            line,
            column,
        }
    }

    /// 获取长度
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// 高亮结果
#[derive(Debug, Clone)]
pub struct HighlightResult {
    /// 原始代码
    pub original_code: String,
    /// 高亮后的HTML
    pub highlighted_html: String,
    /// Token列表
    pub tokens: Vec<Token>,
    /// 使用的主题
    pub theme_name: String,
}

impl HighlightResult {
    /// 创建新的高亮结果
    pub fn new(
        original_code: String,
        highlighted_html: String,
        tokens: Vec<Token>,
        theme_name: String,
    ) -> Self {
        Self {
            original_code,
            highlighted_html,
            tokens,
            theme_name,
        }
    }

    /// 获取指定位置的Token
    pub fn get_token_at_position(&self, position: usize) -> Option<&Token> {
        self.tokens
            .iter()
            .find(|token| token.start <= position && position < token.end)
    }

    /// 获取指定行的Token
    pub fn get_tokens_on_line(&self, line: usize) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|token| token.line == line)
            .collect()
    }
}

/// CSS词法分析器
pub struct CssLexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl CssLexer {
    /// 创建新的词法分析器
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// 分析所有Token
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let start_pos = self.position;
            let start_line = self.line;
            let start_column = self.column;

            if let Some(token) = self.next_token() {
                let mut token = token;
                token.start = start_pos;
                token.end = self.position;
                token.line = start_line;
                token.column = start_column;
                tokens.push(token);
            }
        }

        tokens
    }

    /// 获取下一个Token
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.is_at_end() {
            return None;
        }

        let ch = self.current_char();

        match ch {
            '/' if self.peek() == Some('*') => Some(self.read_comment()),
            '{' | '}' | '(' | ')' | '[' | ']' | ';' | ':' | ',' => {
                self.advance();
                Some(Token::new(
                    TokenType::Punctuation,
                    ch.to_string(),
                    0,
                    0,
                    0,
                    0, // 这些值会在tokenize中设置
                ))
            }
            '@' => Some(self.read_at_rule()),
            '"' | '\'' => Some(self.read_string()),
            '#' => Some(self.read_color_or_id()),
            '0'..='9' | '.' => Some(self.read_number()),
            'a'..='z' | 'A'..='Z' | '_' | '-' => Some(self.read_identifier()),
            _ => {
                self.advance();
                Some(Token::new(TokenType::Unknown, ch.to_string(), 0, 0, 0, 0))
            }
        }
    }

    /// 读取注释
    fn read_comment(&mut self) -> Token {
        let mut text = String::new();

        // 跳过 /*
        text.push(self.advance());
        text.push(self.advance());

        while !self.is_at_end() {
            let ch = self.advance();
            text.push(ch);

            if ch == '*' && self.peek() == Some('/') {
                text.push(self.advance());
                break;
            }
        }

        Token::new(TokenType::Comment, text, 0, 0, 0, 0)
    }

    /// 读取At规则
    fn read_at_rule(&mut self) -> Token {
        let mut text = String::new();

        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_whitespace() || ch == '{' || ch == ';' {
                break;
            }
            text.push(self.advance());
        }

        Token::new(TokenType::AtRule, text, 0, 0, 0, 0)
    }

    /// 读取字符串
    fn read_string(&mut self) -> Token {
        let quote = self.advance();
        let mut text = String::new();
        text.push(quote);

        while !self.is_at_end() {
            let ch = self.advance();
            text.push(ch);

            if ch == quote {
                break;
            }

            if ch == '\\' && !self.is_at_end() {
                text.push(self.advance());
            }
        }

        Token::new(TokenType::String, text, 0, 0, 0, 0)
    }

    /// 读取颜色或ID
    fn read_color_or_id(&mut self) -> Token {
        let mut text = String::new();

        while !self.is_at_end() {
            let ch = self.current_char();
            if !ch.is_alphanumeric() && ch != '#' {
                break;
            }
            text.push(self.advance());
        }

        // 简单判断是否为颜色
        let token_type = if text.len() == 4 || text.len() == 7 {
            if text.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
                TokenType::Color
            } else {
                TokenType::Selector
            }
        } else {
            TokenType::Selector
        };

        Token::new(token_type, text, 0, 0, 0, 0)
    }

    /// 读取数字
    fn read_number(&mut self) -> Token {
        let mut text = String::new();
        let mut has_dot = false;

        while !self.is_at_end() {
            let ch = self.current_char();

            if ch.is_ascii_digit() {
                text.push(self.advance());
            } else if ch == '.' && !has_dot {
                has_dot = true;
                text.push(self.advance());
            } else {
                break;
            }
        }

        // 读取单位
        let unit_start = text.len();
        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_alphabetic() || ch == '%' {
                text.push(self.advance());
            } else {
                break;
            }
        }

        let token_type = if text.len() > unit_start {
            TokenType::Unit
        } else {
            TokenType::Number
        };

        Token::new(token_type, text, 0, 0, 0, 0)
    }

    /// 读取标识符
    fn read_identifier(&mut self) -> Token {
        let mut text = String::new();

        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                text.push(self.advance());
            } else {
                break;
            }
        }

        // 检查是否为函数
        let token_type = if self.peek() == Some('(') {
            TokenType::Function
        } else if self.is_css_keyword(&text) {
            TokenType::Keyword
        } else {
            // 根据上下文判断是选择器还是属性
            // 这里简化处理，实际需要更复杂的上下文分析
            TokenType::Property
        };

        Token::new(token_type, text, 0, 0, 0, 0)
    }

    /// 检查是否为CSS关键字
    fn is_css_keyword(&self, text: &str) -> bool {
        matches!(
            text,
            "inherit"
                | "initial"
                | "unset"
                | "auto"
                | "none"
                | "transparent"
                | "currentColor"
                | "important"
                | "block"
                | "inline"
                | "flex"
                | "grid"
                | "absolute"
                | "relative"
                | "fixed"
                | "static"
                | "sticky"
        )
    }

    /// 跳过空白字符
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    /// 获取当前字符
    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    /// 查看下一个字符
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    /// 前进一个字符
    fn advance(&mut self) -> char {
        let ch = self.current_char();
        self.position += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        ch
    }

    /// 是否到达末尾
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

/// 语法高亮器
pub struct SyntaxHighlighter {
    theme: HighlightTheme,
    themes: HashMap<String, HighlightTheme>,
}

impl SyntaxHighlighter {
    /// 创建新的语法高亮器
    pub fn new(theme_name: &str) -> Self {
        let mut themes = HashMap::new();

        // 注册内置主题
        let default_theme = HighlightTheme::default_theme();
        let dark_theme = HighlightTheme::dark_theme();
        let light_theme = HighlightTheme::light_theme();

        themes.insert("default".to_string(), default_theme.clone());
        themes.insert("dark".to_string(), dark_theme);
        themes.insert("light".to_string(), light_theme);

        let theme = themes.get(theme_name).unwrap_or(&default_theme).clone();

        Self { theme, themes }
    }

    /// 高亮CSS代码
    pub fn highlight(&self, code: &str) -> HighlightResult {
        let mut lexer = CssLexer::new(code.to_string());
        let tokens = lexer.tokenize();

        let highlighted_html = self.tokens_to_html(&tokens, code);

        HighlightResult::new(
            code.to_string(),
            highlighted_html,
            tokens,
            self.theme.name.clone(),
        )
    }

    /// 将Token转换为HTML
    fn tokens_to_html(&self, tokens: &[Token], original_code: &str) -> String {
        let mut html = String::new();
        let mut last_end = 0;

        html.push_str("<pre><code class=\"css-highlight\">");

        for token in tokens {
            // 添加Token之前的空白
            if token.start > last_end {
                let whitespace = &original_code[last_end..token.start];
                html.push_str(&self.escape_html(whitespace));
            }

            // 添加高亮的Token
            html.push_str(&self.token_to_html(token));
            last_end = token.end;
        }

        // 添加剩余的内容
        if last_end < original_code.len() {
            let remaining = &original_code[last_end..];
            html.push_str(&self.escape_html(remaining));
        }

        html.push_str("</code></pre>");
        html
    }

    /// 将单个Token转换为HTML
    fn token_to_html(&self, token: &Token) -> String {
        let mut html = String::new();
        let escaped_text = self.escape_html(&token.text);

        if let Some(color) = self.theme.get_color(&token.token_type) {
            let mut style = format!("color: {};", color);

            if let Some(text_style) = self.theme.get_style(&token.token_type) {
                match text_style {
                    TextStyle::Bold => style.push_str(" font-weight: bold;"),
                    TextStyle::Italic => style.push_str(" font-style: italic;"),
                    TextStyle::Underline => style.push_str(" text-decoration: underline;"),
                    TextStyle::Strikethrough => style.push_str(" text-decoration: line-through;"),
                    TextStyle::Normal => {}
                }
            }

            html.push_str(&format!(
                "<span class=\"token-{:?}\" style=\"{}\">{}\"</span>",
                token.token_type, style, escaped_text
            ));
        } else {
            html.push_str(&escaped_text);
        }

        html
    }

    /// 转义HTML字符
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    /// 更新主题
    pub fn update_theme(&mut self, theme_name: &str) {
        if let Some(theme) = self.themes.get(theme_name) {
            self.theme = theme.clone();
        }
    }

    /// 添加自定义主题
    pub fn add_theme(&mut self, theme: HighlightTheme) {
        let name = theme.name.clone();
        self.themes.insert(name, theme);
    }

    /// 获取可用主题列表
    pub fn get_available_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    /// 生成CSS样式
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        css.push_str(".css-highlight {\n");
        css.push_str("  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;\n");
        css.push_str("  font-size: 14px;\n");
        css.push_str("  line-height: 1.5;\n");
        css.push_str("  background-color: #1e1e1e;\n");
        css.push_str("  color: #d4d4d4;\n");
        css.push_str("  padding: 16px;\n");
        css.push_str("  border-radius: 4px;\n");
        css.push_str("  overflow-x: auto;\n");
        css.push_str("}\n\n");

        // 为每种Token类型生成CSS类
        for (token_type, color) in &self.theme.colors {
            css.push_str(&format!(".token-{:?} {{\n", token_type));
            css.push_str(&format!("  color: {};\n", color));

            if let Some(style) = self.theme.get_style(token_type) {
                match style {
                    TextStyle::Bold => css.push_str("  font-weight: bold;\n"),
                    TextStyle::Italic => css.push_str("  font-style: italic;\n"),
                    TextStyle::Underline => css.push_str("  text-decoration: underline;\n"),
                    TextStyle::Strikethrough => css.push_str("  text-decoration: line-through;\n"),
                    TextStyle::Normal => {}
                }
            }

            css.push_str("}\n\n");
        }

        css
    }
}

/// 高亮错误
#[derive(Debug, Clone)]
pub enum HighlightError {
    /// 词法分析错误
    LexError(String),
    /// 主题不存在
    ThemeNotFound(String),
    /// 无效的输入
    InvalidInput(String),
}

impl fmt::Display for HighlightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HighlightError::LexError(msg) => write!(f, "词法分析错误: {}", msg),
            HighlightError::ThemeNotFound(theme) => write!(f, "主题不存在: {}", theme),
            HighlightError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
        }
    }
}

impl std::error::Error for HighlightError {}

//! CSS优化模块
//!
//! 本模块负责CSS的优化处理，包括压缩、合并、去重和选择器优化。
//! 职责：提高CSS的性能和减少体积，同时保持功能完整性。
//!
//! # 主要组件
//!
//! - `StyleOptimizer`: 核心优化器，提供各种CSS优化策略
//! - `OptimizeConfig`: 优化配置，控制优化行为
//!
//! # 示例
//!
//! ```
//! use css_in_rust::theme::core::optimize::{StyleOptimizer, OptimizeConfig};
//!
//! // 创建默认优化器
//! let optimizer = StyleOptimizer::default();
//!
//! // 使用自定义优化配置
//! let config = OptimizeConfig {
//!     minify: true,
//!     remove_unused: true,
//!     merge_rules: true,
//!     optimize_selectors: true,
//! };
//!
//! let optimizer = StyleOptimizer::new(config);
//!
//! // 优化CSS
//! let css = r#"
//!   .button {
//!     color: #ff0000;
//!     color: red;
//!     padding: 10px;
//!   }
//! "#;
//!
//! let optimized = optimizer.optimize(css);
//! ```

use regex;
use std::collections::HashMap;
use std::collections::HashSet;

/// CSS 优化配置
///
/// 控制CSS优化过程中应用的优化策略。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::optimize::OptimizeConfig;
///
/// // 创建默认配置
/// let default_config = OptimizeConfig::default();
///
/// // 创建自定义配置
/// let custom_config = OptimizeConfig {
///     minify: true,
///     remove_unused: false,
///     merge_rules: true,
///     optimize_selectors: true,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct OptimizeConfig {
    /// 是否压缩 CSS
    pub minify: bool,
    /// 是否删除未使用的样式
    pub remove_unused: bool,
    /// 是否合并相同的规则
    pub merge_rules: bool,
    /// 是否优化选择器
    pub optimize_selectors: bool,
}

impl Default for OptimizeConfig {
    fn default() -> Self {
        Self {
            minify: true,
            remove_unused: true,
            merge_rules: true,
            optimize_selectors: true,
        }
    }
}

/// CSS 优化器
///
/// 提供CSS优化功能，包括压缩、移除未使用样式、合并规则和优化选择器。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::optimize::{StyleOptimizer, OptimizeConfig};
///
/// // 创建默认优化器
/// let optimizer = StyleOptimizer::default();
///
/// // 创建自定义优化器
/// let config = OptimizeConfig {
///     minify: true,
///     remove_unused: true,
///     merge_rules: true,
///     optimize_selectors: true,
/// };
/// let optimizer = StyleOptimizer::new(config);
///
/// // 注册使用的类名
/// let mut optimizer = StyleOptimizer::default();
/// optimizer.register_used_class("button");
/// optimizer.register_used_class("card");
///
/// // 优化CSS
/// let css = ".button { color: red; } .unused { display: none; }";
/// let optimized = optimizer.optimize(css);
/// ```
#[derive(Default)]
pub struct StyleOptimizer {
    config: OptimizeConfig,
    used_classes: HashSet<String>,
}

impl StyleOptimizer {
    /// 创建新的优化器实例
    ///
    /// # Arguments
    ///
    /// * `config` - 优化配置
    ///
    /// # Returns
    ///
    /// 新创建的CSS优化器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::optimize::{StyleOptimizer, OptimizeConfig};
    ///
    /// let config = OptimizeConfig::default();
    /// let optimizer = StyleOptimizer::new(config);
    /// ```
    pub fn new(config: OptimizeConfig) -> Self {
        Self {
            config,
            used_classes: HashSet::new(),
        }
    }

    /// 注册使用的类名
    ///
    /// 将类名添加到已使用类名列表中，用于移除未使用的样式。
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要注册的类名
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::optimize::StyleOptimizer;
    ///
    /// let mut optimizer = StyleOptimizer::default();
    /// optimizer.register_used_class("button");
    /// optimizer.register_used_class("card");
    /// ```
    pub fn register_used_class(&mut self, class_name: &str) {
        self.used_classes.insert(class_name.to_string());
    }

    /// 优化 CSS 内容
    ///
    /// 根据配置的优化策略对CSS进行优化处理。
    ///
    /// # Arguments
    ///
    /// * `css` - 要优化的CSS字符串
    ///
    /// # Returns
    ///
    /// 优化后的CSS字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::optimize::StyleOptimizer;
    ///
    /// let optimizer = StyleOptimizer::default();
    /// let css = r#"
    ///   .button {
    ///     color: red;
    ///     padding: 10px;
    ///   }
    /// "#;
    ///
    /// let optimized = optimizer.optimize(css);
    /// ```
    pub fn optimize(&self, css: &str) -> String {
        let mut optimized = css.to_string();

        // 首先处理规则合并和选择器优化，这样在移除未使用的样式前可以更好地识别
        if self.config.merge_rules {
            optimized = self.merge_rules(&optimized);
        }

        if self.config.optimize_selectors {
            optimized = self.optimize_selectors(&optimized);
        }

        // 然后移除未使用的样式
        if self.config.remove_unused {
            optimized = self.remove_unused_styles(&optimized);
        }

        // 最后进行压缩
        if self.config.minify {
            optimized = self.minify(&optimized);
        }

        optimized
    }

    /// 压缩 CSS
    ///
    /// 移除注释和多余的空白，减小CSS文件大小。
    ///
    /// # Arguments
    ///
    /// * `css` - 要压缩的CSS字符串
    ///
    /// # Returns
    ///
    /// 压缩后的CSS字符串
    fn minify(&self, css: &str) -> String {
        // 移除注释
        let without_comments = css
            .lines()
            .filter(|line| !line.trim().starts_with("/*") && !line.trim().ends_with("*/"))
            .collect::<Vec<_>>()
            .join("");

        // 移除多余的空白
        let mut minified = String::new();
        let mut last_char = ' ';

        for c in without_comments.chars() {
            if c.is_whitespace() {
                if !last_char.is_whitespace() && last_char != '{' && last_char != ';' {
                    minified.push(' ');
                }
            } else {
                minified.push(c);
            }
            last_char = c;
        }

        minified
    }

    /// 移除未使用的样式
    ///
    /// 根据已注册的类名，移除CSS中未使用的样式规则。
    ///
    /// # Arguments
    ///
    /// * `css` - 要处理的CSS字符串
    ///
    /// # Returns
    ///
    /// 移除未使用样式后的CSS字符串
    fn remove_unused_styles(&self, css: &str) -> String {
        if self.used_classes.is_empty() {
            return css.to_string();
        }

        // 解析 CSS 为规则集
        let rules = self.parse_css_rules(css);
        let mut filtered_rules = Vec::new();

        // 打印调试信息
        println!("Used classes: {:?}", self.used_classes);
        println!("Parsed rules: {:?}", rules);

        // 过滤规则
        for (selector, declarations) in rules {
            // 检查选择器是否包含任何已使用的类名
            if selector.contains(',') {
                // 对于复合选择器，分别检查每个部分
                let parts: Vec<&str> = selector.split(',').map(|s| s.trim()).collect();
                let keep_parts: Vec<&str> = parts
                    .iter()
                    .filter(|&&part| self.is_selector_used(part))
                    .cloned()
                    .collect();

                if !keep_parts.is_empty() {
                    // 更新选择器，只保留使用的部分
                    filtered_rules.push((keep_parts.join(", "), declarations));
                }
            } else if self.is_selector_used(&selector) {
                // 单一选择器
                filtered_rules.push((selector, declarations));
            }
        }

        // 将过滤后的规则转换回 CSS 字符串
        let result = self.rules_to_css(&filtered_rules);
        println!("Result CSS: {}", result);
        result
    }

    /// 检查选择器是否被使用
    ///
    /// 判断选择器中是否包含已注册的类名。
    ///
    /// # Arguments
    ///
    /// * `selector` - 要检查的选择器
    ///
    /// # Returns
    ///
    /// 如果选择器被使用则返回true，否则返回false
    fn is_selector_used(&self, selector: &str) -> bool {
        if self.used_classes.is_empty() {
            return true; // 如果没有注册任何使用的类，则保留所有选择器
        }

        // 提取选择器中的类名（.className）
        let class_regex = regex::Regex::new(r"\.([\w-]+)").unwrap();
        let mut has_class = false;

        for cap in class_regex.captures_iter(selector) {
            has_class = true;
            let class_name = &cap[1];
            if self.used_classes.contains(class_name) {
                return true;
            }
        }

        // 如果选择器中没有类名，则保留它（如元素选择器）
        !has_class
    }

    /// 合并相同的规则
    ///
    /// 合并具有相同选择器的规则，并对声明进行去重。
    ///
    /// # Arguments
    ///
    /// * `css` - 要处理的CSS字符串
    ///
    /// # Returns
    ///
    /// 合并规则后的CSS字符串
    fn merge_rules(&self, css: &str) -> String {
        // 解析 CSS 为规则集
        let rules = self.parse_css_rules(css);

        // 按选择器分组规则
        let mut selector_map: HashMap<String, Vec<(String, String)>> = HashMap::new();

        // 第一步：合并具有相同选择器的规则
        for (selector, declarations) in rules {
            selector_map
                .entry(selector)
                .or_insert_with(Vec::new)
                .extend(declarations);
        }

        // 第二步：对每个选择器的声明进行去重
        let mut merged_rules = Vec::new();
        for (selector, declarations) in selector_map {
            let mut unique_declarations = HashMap::new();

            // 后面的声明会覆盖前面的声明（如果属性相同）
            for (property, value) in declarations {
                unique_declarations.insert(property, value);
            }

            // 将 HashMap 转换回 Vec
            let final_declarations = unique_declarations
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect();

            merged_rules.push((selector, final_declarations));
        }

        // 第三步：合并具有相同声明的选择器
        merged_rules = self.group_selectors_by_declarations(&merged_rules);

        // 将合并后的规则转换回 CSS 字符串
        self.rules_to_css(&merged_rules)
    }

    /// 解析 CSS 为规则集
    ///
    /// 将CSS字符串解析为选择器和声明的规则集。
    ///
    /// # Arguments
    ///
    /// * `css` - 要解析的CSS字符串
    ///
    /// # Returns
    ///
    /// 解析后的规则集，每个规则包含选择器和声明列表
    fn parse_css_rules(&self, css: &str) -> Vec<(String, Vec<(String, String)>)> {
        let mut rules = Vec::new();

        // 简化实现：使用基本的字符串解析
        // 实际项目中应使用专业的 CSS 解析器

        // 移除注释
        let css_no_comments = css.replace(regex::Regex::new(r"/\*.*?\*/").unwrap().as_str(), "");

        // 分割规则
        let mut current_selector = String::new();
        let mut current_declarations = Vec::new();
        let mut in_declarations = false;

        // 打印调试信息
        println!("CSS to parse: {}", css_no_comments);

        // 分割成多行处理
        for line in css_no_comments.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if !in_declarations && trimmed.contains('{') {
                // 开始新规则
                let selector_part = trimmed.split('{').next().unwrap_or("").trim();
                current_selector = selector_part.to_string();
                in_declarations = true;
                current_declarations = Vec::new();

                // 检查是否在同一行有声明
                if let Some(decl_part) = trimmed.split('{').nth(1) {
                    if decl_part.contains('}') {
                        // 单行规则
                        let decl_content = decl_part.split('}').next().unwrap_or("").trim();
                        self.parse_declarations(decl_content, &mut current_declarations);

                        // 添加规则并重置
                        if !current_selector.is_empty() {
                            rules.push((current_selector.clone(), current_declarations.clone()));
                        }

                        current_selector = String::new();
                        current_declarations = Vec::new();
                        in_declarations = false;
                    } else {
                        // 多行规则，处理第一行声明
                        self.parse_declarations(decl_part, &mut current_declarations);
                    }
                }
            } else if in_declarations && trimmed.contains('}') {
                // 结束规则
                let decl_part = trimmed.split('}').next().unwrap_or("").trim();
                if !decl_part.is_empty() {
                    self.parse_declarations(decl_part, &mut current_declarations);
                }

                // 添加规则
                if !current_selector.is_empty() {
                    rules.push((current_selector.clone(), current_declarations.clone()));
                }

                current_selector = String::new();
                current_declarations = Vec::new();
                in_declarations = false;
            } else if in_declarations {
                // 处理声明
                self.parse_declarations(trimmed, &mut current_declarations);
            }
        }

        // 处理可能的未闭合规则
        if in_declarations && !current_selector.is_empty() {
            rules.push((current_selector.clone(), current_declarations.clone()));
        }

        // 打印解析结果
        println!("Parsed {} rules", rules.len());
        for (selector, declarations) in &rules {
            println!("Selector: {}", selector);
            for (prop, value) in declarations {
                println!("  {} = {}", prop, value);
            }
        }

        rules
    }

    /// 辅助函数：解析声明
    ///
    /// 解析CSS声明字符串为属性和值对。
    ///
    /// # Arguments
    ///
    /// * `text` - 要解析的声明文本
    /// * `declarations` - 存储解析结果的声明列表
    fn parse_declarations(&self, text: &str, declarations: &mut Vec<(String, String)>) {
        for decl in text.split(';') {
            let decl = decl.trim();
            if decl.is_empty() {
                continue;
            }

            if let Some(colon_pos) = decl.find(':') {
                let property = decl[..colon_pos].trim().to_string();
                let value = decl[colon_pos + 1..].trim().to_string();

                if !property.is_empty() && !value.is_empty() {
                    declarations.push((property, value));
                }
            }
        }
    }

    /// 将规则集转换为 CSS 字符串
    ///
    /// 将解析后的规则集转换回格式化的CSS字符串。
    ///
    /// # Arguments
    ///
    /// * `rules` - 规则集，每个规则包含选择器和声明列表
    ///
    /// # Returns
    ///
    /// 格式化的CSS字符串
    fn rules_to_css(&self, rules: &[(String, Vec<(String, String)>)]) -> String {
        let mut css = String::new();

        for (selector, declarations) in rules {
            css.push_str(&format!("{} {{\n", selector));

            for (property, value) in declarations {
                css.push_str(&format!("  {}: {};\n", property, value));
            }

            css.push_str("}\n\n");
        }

        css
    }

    /// 优化选择器
    ///
    /// 对CSS选择器进行优化，如移除冗余部分和合并相同声明的选择器。
    ///
    /// # Arguments
    ///
    /// * `css` - 要优化的CSS字符串
    ///
    /// # Returns
    ///
    /// 选择器优化后的CSS字符串
    fn optimize_selectors(&self, css: &str) -> String {
        // 解析 CSS 为规则集
        let rules = self.parse_css_rules(css);
        let mut optimized_rules = Vec::new();

        // 处理每个规则
        for (selector, declarations) in rules {
            // 跳过空声明的规则
            if declarations.is_empty() {
                continue;
            }

            // 优化选择器
            let optimized_selector = self.optimize_selector(&selector);

            // 合并具有相同声明的选择器（这部分在 merge_rules 中已经处理）
            // 这里我们只处理选择器的优化

            optimized_rules.push((optimized_selector, declarations));
        }

        // 处理选择器分组
        let grouped_rules = self.group_selectors_by_declarations(&optimized_rules);

        // 将优化后的规则转换回 CSS 字符串
        self.rules_to_css(&grouped_rules)
    }

    /// 优化单个选择器
    ///
    /// 对单个选择器进行优化，如移除冗余通配符和简化后代选择器。
    ///
    /// # Arguments
    ///
    /// * `selector` - 要优化的选择器
    ///
    /// # Returns
    ///
    /// 优化后的选择器
    fn optimize_selector(&self, selector: &str) -> String {
        let mut optimized = selector.to_string();

        // 1. 移除不必要的通用选择器，例如 *.class -> .class
        optimized = optimized.replace("*.", ".");
        optimized = optimized.replace("* ", " ");

        // 2. 简化后代选择器，例如 div > * > span -> div > span
        if optimized.contains('>') {
            // 先分割，然后过滤掉空白和 * 选择器
            let parts: Vec<&str> = optimized
                .split('>')
                .map(|s| s.trim())
                .filter(|&s| !s.is_empty() && s != "*")
                .collect();

            // 重新组合选择器
            optimized = parts.join(" > ");
        }

        // 3. 移除重复的选择器
        if optimized.contains(',') {
            let mut unique_parts = Vec::new();
            for part in optimized.split(',').map(|s| s.trim()) {
                if !unique_parts.contains(&part) && !part.is_empty() {
                    unique_parts.push(part);
                }
            }
            optimized = unique_parts.join(", ");
        }

        // 4. 简化选择器中的空格
        while optimized.contains("  ") {
            optimized = optimized.replace("  ", " ");
        }

        optimized
    }

    /// 按声明对选择器进行分组
    ///
    /// 将具有相同声明的选择器合并为一个规则。
    ///
    /// # Arguments
    ///
    /// * `rules` - 规则集，每个规则包含选择器和声明列表
    ///
    /// # Returns
    ///
    /// 分组后的规则集
    fn group_selectors_by_declarations(
        &self,
        rules: &[(String, Vec<(String, String)>)],
    ) -> Vec<(String, Vec<(String, String)>)> {
        let mut grouped = HashMap::new();

        // 按声明内容对选择器进行分组
        for (selector, declarations) in rules {
            // 将声明转换为可哈希的格式
            let declaration_key = self.declarations_to_key(declarations);

            grouped
                .entry(declaration_key)
                .or_insert_with(|| (Vec::new(), declarations.clone()))
                .0
                .push(selector.clone());
        }

        // 将分组后的结果转换为规则列表
        let mut result = Vec::new();
        for (_, (selectors, declarations)) in grouped {
            if selectors.is_empty() {
                continue;
            }

            // 合并选择器
            let combined_selector = selectors.join(", ");
            result.push((combined_selector, declarations));
        }

        result
    }

    /// 将声明列表转换为唯一键
    ///
    /// 将声明列表转换为可哈希的字符串键，用于比较声明是否相同。
    ///
    /// # Arguments
    ///
    /// * `declarations` - 声明列表
    ///
    /// # Returns
    ///
    /// 唯一的声明键
    fn declarations_to_key(&self, declarations: &[(String, String)]) -> String {
        // 对声明进行排序以确保相同内容的声明产生相同的键
        let mut sorted = declarations.to_vec();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));

        let mut key = String::new();
        for (prop, value) in sorted {
            key.push_str(&format!("{}:{};", prop, value));
        }

        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_minification() {
        let optimizer = StyleOptimizer::default();
        let css = r#"
            .button {
                color: blue;
                margin: 10px;
            }

            /* Comment */
            .card {
                padding: 20px;
            }
        "#;

        let minified = optimizer.minify(css);
        assert!(!minified.contains("/*"));
        assert!(!minified.contains("\n"));
    }

    #[test]
    fn test_unused_styles_removal() {
        let mut optimizer = StyleOptimizer::default();
        optimizer.register_used_class("button");

        // 简化测试用例，使用内联样式
        let css = ".button { color: blue; } .card { padding: 20px; }";

        let optimized = optimizer.remove_unused_styles(css);
        println!("Optimized CSS: {}", optimized);

        // 验证结果包含 button 类
        assert!(optimized.contains(".button"));

        // 验证结果不包含 card 类
        assert!(!optimized.contains(".card"));
    }

    #[test]
    fn test_merge_rules() {
        let optimizer = StyleOptimizer::default();
        let css = r#"
            .button {
                color: blue;
            }

            .card {
                padding: 20px;
            }

            .button {
                margin: 10px;
            }
        "#;

        let merged = optimizer.merge_rules(css);

        // 验证 .button 规则已合并
        assert!(merged.contains("color: blue"));
        assert!(merged.contains("margin: 10px"));

        // 确保只有一个 .button 规则
        let button_count = merged.matches(".button").count();
        assert_eq!(button_count, 1);
    }

    #[test]
    fn test_optimize_selectors() {
        let optimizer = StyleOptimizer::default();
        let css = r#"
            *.button, .button {
                color: blue;
            }

            div > * > span {
                margin: 10px;
            }

            .card, .box {
                padding: 20px;
            }

            .box {
                padding: 20px;
            }
        "#;

        // 直接调用 optimize_selector 方法来测试选择器优化
        let optimized_selector1 = optimizer.optimize_selector("*.button, .button");
        assert_eq!(optimized_selector1, ".button");

        let optimized_selector2 = optimizer.optimize_selector("div > * > span");
        assert_eq!(optimized_selector2, "div > span");

        // 测试完整的 optimize_selectors 方法
        let optimized = optimizer.optimize_selectors(css);

        // 验证选择器优化
        assert!(!optimized.contains("*.button"));
        assert!(optimized.contains(".button"));
        assert!(!optimized.contains("div > * > span"));

        // 验证选择器分组
        assert!(optimized.contains(".card, .box") || optimized.contains(".box, .card"));
    }

    #[test]
    fn test_full_optimization() {
        let mut optimizer = StyleOptimizer::default();
        optimizer.register_used_class("button");
        optimizer.register_used_class("card");
        optimizer.register_used_class("box"); // 添加 box 类名为已使用

        let css = r#"
            .button {
                color: blue;
            }

            *.button {
                margin: 10px;
            }

            .unused {
                display: none;
            }

            .card, .box {
                padding: 20px;
            }

            .box {
                padding: 20px;
            }
        "#;

        let optimized = optimizer.optimize(css);

        // 验证未使用的样式已移除
        assert!(!optimized.contains(".unused"));

        // 验证规则已合并
        assert!(optimized.contains("color: blue"));
        assert!(optimized.contains("margin: 10px"));

        // 验证选择器已优化
        assert!(!optimized.contains("*.button"));

        // 打印优化后的 CSS，用于调试
        println!("Optimized CSS:\n{}", optimized);

        // 验证 .box 只出现一次（作为单独的选择器）
        let box_matches: Vec<_> = optimized.match_indices(".box").collect();
        let standalone_box_count = box_matches
            .iter()
            .filter(|(i, _)| {
                let next_char = optimized.chars().nth(*i + 4);
                next_char == Some(' ') || next_char == Some('{') || next_char == Some(',')
            })
            .count();

        assert!(
            standalone_box_count <= 1,
            "Expected at most 1 standalone .box selector, found {}",
            standalone_box_count
        );
    }
}

use std::collections::HashSet;

/// CSS 优化配置
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
#[derive(Default)]
pub struct StyleOptimizer {
    config: OptimizeConfig,
    used_classes: HashSet<String>,
}

impl StyleOptimizer {
    /// 创建新的优化器实例
    pub fn new(config: OptimizeConfig) -> Self {
        Self {
            config,
            used_classes: HashSet::new(),
        }
    }

    /// 注册使用的类名
    pub fn register_used_class(&mut self, class_name: &str) {
        self.used_classes.insert(class_name.to_string());
    }

    /// 优化 CSS 内容
    pub fn optimize(&self, css: &str) -> String {
        let mut optimized = css.to_string();

        if self.config.minify {
            optimized = self.minify(&optimized);
        }

        if self.config.remove_unused {
            optimized = self.remove_unused_styles(&optimized);
        }

        if self.config.merge_rules {
            optimized = self.merge_rules(&optimized);
        }

        if self.config.optimize_selectors {
            optimized = self.optimize_selectors(&optimized);
        }

        optimized
    }

    /// 压缩 CSS
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
    fn remove_unused_styles(&self, css: &str) -> String {
        if self.used_classes.is_empty() {
            return css.to_string();
        }

        // 简单实现：只保留包含已使用类名的规则
        css.lines()
            .filter(|line| {
                let line = line.trim();
                if line.contains("{") {
                    self.used_classes.iter().any(|class| line.contains(class))
                } else {
                    true
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 合并相同的规则
    fn merge_rules(&self, css: &str) -> String {
        // TODO: 实现规则合并
        // 1. 解析 CSS 为规则集
        // 2. 合并具有相同选择器的规则
        // 3. 合并具有相同声明的选择器
        css.to_string()
    }

    /// 优化选择器
    fn optimize_selectors(&self, css: &str) -> String {
        // TODO: 实现选择器优化
        // 1. 简化复杂的选择器
        // 2. 重写低效的选择器
        // 3. 移除冗余的选择器
        css.to_string()
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

        let css = r#"
            .button { color: blue; }
            .card { padding: 20px; }
        "#;

        let optimized = optimizer.remove_unused_styles(css);
        assert!(optimized.contains("button"));
        assert!(!optimized.contains("card"));
    }
}

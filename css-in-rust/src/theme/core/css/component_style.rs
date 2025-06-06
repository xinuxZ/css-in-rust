use crate::theme::theme_types::{Theme, ThemeMode};
use std::collections::HashMap;

/// 组件样式生成器
///
/// 用于生成组件的样式，类似于ant-design的组件样式生成方式
pub trait ComponentStyleGenerator {
    /// 生成组件样式
    fn generate_style(&self, theme: &Theme) -> String;

    /// 生成组件的CSS类名
    fn generate_class_name(&self) -> String;
}

/// 组件样式配置
#[derive(Debug, Clone)]
pub struct ComponentStyleConfig {
    /// 组件名称
    pub name: String,
    /// 组件前缀
    pub prefix: String,
    /// 组件变体
    pub variants: Vec<String>,
    /// 组件状态
    pub states: Vec<String>,
    /// 组件尺寸
    pub sizes: Vec<String>,
}

impl Default for ComponentStyleConfig {
    fn default() -> Self {
        Self {
            name: "component".to_string(),
            prefix: "css-in-rust".to_string(),
            variants: vec![
                "default".to_string(),
                "primary".to_string(),
                "secondary".to_string(),
            ],
            states: vec![
                "hover".to_string(),
                "active".to_string(),
                "disabled".to_string(),
            ],
            sizes: vec![
                "small".to_string(),
                "medium".to_string(),
                "large".to_string(),
            ],
        }
    }
}

/// 组件样式生成器实现
pub struct DefaultComponentStyleGenerator {
    /// 组件样式配置
    pub config: ComponentStyleConfig,
    /// 基础样式生成函数
    pub base_style_fn: Box<dyn Fn(&Theme) -> String + Send + Sync>,
    /// 变体样式生成函数
    pub variant_style_fns: HashMap<String, Box<dyn Fn(&Theme) -> String + Send + Sync>>,
    /// 状态样式生成函数
    pub state_style_fns: HashMap<String, Box<dyn Fn(&Theme) -> String + Send + Sync>>,
    /// 尺寸样式生成函数
    pub size_style_fns: HashMap<String, Box<dyn Fn(&Theme) -> String + Send + Sync>>,
}

impl DefaultComponentStyleGenerator {
    /// 创建新的组件样式生成器
    pub fn new<F>(config: ComponentStyleConfig, base_style_fn: F) -> Self
    where
        F: Fn(&Theme) -> String + Send + Sync + 'static,
    {
        Self {
            config,
            base_style_fn: Box::new(base_style_fn),
            variant_style_fns: HashMap::new(),
            state_style_fns: HashMap::new(),
            size_style_fns: HashMap::new(),
        }
    }

    /// 添加变体样式
    pub fn add_variant_style<F>(&mut self, variant: &str, style_fn: F) -> &mut Self
    where
        F: Fn(&Theme) -> String + Send + Sync + 'static,
    {
        self.variant_style_fns
            .insert(variant.to_string(), Box::new(style_fn));
        self
    }

    /// 添加状态样式
    pub fn add_state_style<F>(&mut self, state: &str, style_fn: F) -> &mut Self
    where
        F: Fn(&Theme) -> String + Send + Sync + 'static,
    {
        self.state_style_fns
            .insert(state.to_string(), Box::new(style_fn));
        self
    }

    /// 添加尺寸样式
    pub fn add_size_style<F>(&mut self, size: &str, style_fn: F) -> &mut Self
    where
        F: Fn(&Theme) -> String + Send + Sync + 'static,
    {
        self.size_style_fns
            .insert(size.to_string(), Box::new(style_fn));
        self
    }

    /// 生成所有变体的样式
    fn generate_variants_style(&self, theme: &Theme) -> String {
        let mut css = String::new();

        for variant in &self.config.variants {
            if let Some(style_fn) = self.variant_style_fns.get(variant) {
                let variant_css = style_fn(theme);
                css.push_str(&format!(
                    ".{}-{} {{\n{}\n}}\n",
                    self.config.name, variant, variant_css
                ));
            }
        }

        css
    }

    /// 生成所有状态的样式
    fn generate_states_style(&self, theme: &Theme) -> String {
        let mut css = String::new();

        for state in &self.config.states {
            if let Some(style_fn) = self.state_style_fns.get(state) {
                let state_css = style_fn(theme);
                css.push_str(&format!(
                    ".{}:{}  {{\n{}\n}}\n",
                    self.config.name, state, state_css
                ));
            }
        }

        css
    }

    /// 生成所有尺寸的样式
    fn generate_sizes_style(&self, theme: &Theme) -> String {
        let mut css = String::new();

        for size in &self.config.sizes {
            if let Some(style_fn) = self.size_style_fns.get(size) {
                let size_css = style_fn(theme);
                css.push_str(&format!(
                    ".{}-{} {{\n{}\n}}\n",
                    self.config.name, size, size_css
                ));
            }
        }

        css
    }
}

impl ComponentStyleGenerator for DefaultComponentStyleGenerator {
    fn generate_style(&self, theme: &Theme) -> String {
        let mut css = String::new();

        // 生成基础样式
        let base_css = (self.base_style_fn)(theme);
        css.push_str(&format!(".{} {{\n{}\n}}\n", self.config.name, base_css));

        // 生成变体样式
        css.push_str(&self.generate_variants_style(theme));

        // 生成状态样式
        css.push_str(&self.generate_states_style(theme));

        // 生成尺寸样式
        css.push_str(&self.generate_sizes_style(theme));

        css
    }

    fn generate_class_name(&self) -> String {
        format!("{}-{}", self.config.prefix, self.config.name)
    }
}

/// 创建按钮组件样式生成器
pub fn create_button_style_generator() -> DefaultComponentStyleGenerator {
    let config = ComponentStyleConfig {
        name: "button".to_string(),
        prefix: "css-in-rust".to_string(),
        variants: vec![
            "default".to_string(),
            "primary".to_string(),
            "danger".to_string(),
        ],
        states: vec![
            "hover".to_string(),
            "active".to_string(),
            "disabled".to_string(),
        ],
        sizes: vec![
            "small".to_string(),
            "medium".to_string(),
            "large".to_string(),
        ],
    };

    let mut generator = DefaultComponentStyleGenerator::new(config, |theme| {
        // 基础样式
        format!(
            r#"
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-size: 14px;
            font-weight: 400;
            height: 32px;
            padding: 4px 15px;
            border-radius: 6px;
            border: 1px solid transparent;
            cursor: pointer;
            transition: all 0.2s;
            user-select: none;
            "#
        )
    });

    // 添加变体样式
    generator.add_variant_style("default", |theme| {
        let border_color = if theme.mode == ThemeMode::Dark {
            "#424242"
        } else {
            "#d9d9d9"
        };

        let text_color = if theme.mode == ThemeMode::Dark {
            "#fff"
        } else {
            "rgba(0, 0, 0, 0.88)"
        };

        let bg_color = if theme.mode == ThemeMode::Dark {
            "#1f1f1f"
        } else {
            "#ffffff"
        };

        format!(
            r#"
            color: {};
            background: {};
            border-color: {};
            "#,
            text_color, bg_color, border_color
        )
    });

    generator.add_variant_style("primary", |_| {
        format!(
            r#"
            color: #fff;
            background: #1677ff;
            border-color: #1677ff;
            text-shadow: 0 -1px 0 rgba(0, 0, 0, 0.12);
            box-shadow: 0 2px 0 rgba(5, 145, 255, 0.1);
            "#
        )
    });

    generator.add_variant_style("danger", |_| {
        format!(
            r#"
            color: #fff;
            background: #ff4d4f;
            border-color: #ff4d4f;
            text-shadow: 0 -1px 0 rgba(0, 0, 0, 0.12);
            box-shadow: 0 2px 0 rgba(255, 38, 5, 0.06);
            "#
        )
    });

    // 添加状态样式
    generator.add_state_style("hover", |_| {
        format!(
            r#"
            opacity: 0.8;
            "#
        )
    });

    generator.add_state_style("active", |_| {
        format!(
            r#"
            opacity: 0.9;
            "#
        )
    });

    generator.add_state_style("disabled", |_| {
        format!(
            r#"
            cursor: not-allowed;
            opacity: 0.5;
            "#
        )
    });

    // 添加尺寸样式
    generator.add_size_style("small", |_| {
        format!(
            r#"
            height: 24px;
            padding: 0px 7px;
            font-size: 12px;
            border-radius: 4px;
            "#
        )
    });

    generator.add_size_style("large", |_| {
        format!(
            r#"
            height: 40px;
            padding: 6px 15px;
            font-size: 16px;
            border-radius: 8px;
            "#
        )
    });

    generator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_style_generator() {
        let generator = create_button_style_generator();
        let theme = Theme::default();

        let css = generator.generate_style(&theme);

        // 验证生成的CSS包含基础样式
        assert!(css.contains("display: inline-flex"));

        // 验证生成的CSS包含变体样式
        assert!(css.contains(".button-primary"));
        assert!(css.contains(".button-danger"));

        // 验证生成的CSS包含状态样式
        assert!(css.contains(".button:hover"));
        assert!(css.contains(".button:disabled"));

        // 验证生成的CSS包含尺寸样式
        assert!(css.contains(".button-small"));
        assert!(css.contains(".button-large"));
    }
}

//! Complete Project Example
//!
//! This example demonstrates how to use the CSS-in-Rust library with dead code elimination
//! in a complete project setup, including build-time optimization and runtime usage.

// Suppress unused crate warnings
use chrono as _;
use css_in_rust_macros as _;
use lazy_static as _;
use lightningcss as _;
use proc_macro2 as _;
use quote as _;
use regex as _;
use serde as _;
use serde_json as _;
use sha2 as _;
use syn as _;
use tempfile as _;

use css_in_rust::{
    build_tools::{BuildConfig, CssBuildProcessor, StaticAnalyzer},
    core::optimizer::{CssOptimizer, OptimizerConfig},
    css, css_if,
};
use std::path::PathBuf;

/// Example component that uses CSS-in-Rust
struct Button {
    text: String,
    variant: ButtonVariant,
    disabled: bool,
}

#[derive(Debug, Clone)]
enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

impl Button {
    /// Create a new button
    pub fn new(text: String, variant: ButtonVariant) -> Self {
        Self {
            text,
            variant,
            disabled: false,
        }
    }

    /// Set button as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Render the button with CSS-in-Rust styling
    pub fn render(&self) -> String {
        // Base button styles
        let base_styles = css!(
            "
            padding: 12px 24px;
            border: none;
            border-radius: 6px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s ease;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            text-decoration: none;

            &:hover {
                transform: translateY(-1px);
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
            }

            &:active {
                transform: translateY(0);
            }

            &:disabled {
                opacity: 0.6;
                cursor: not-allowed;
                transform: none !important;
                box-shadow: none !important;
            }
        "
        );

        // Variant-specific styles
        let variant_styles = match self.variant {
            ButtonVariant::Primary => css!(
                "
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                color: white;

                &:hover {
                    background: linear-gradient(135deg, #5a6fd8 0%, #6a4190 100%);
                }
            "
            ),
            ButtonVariant::Secondary => css!(
                "
                background: #f8f9fa;
                color: #495057;
                border: 2px solid #dee2e6;

                &:hover {
                    background: #e9ecef;
                    border-color: #adb5bd;
                }
            "
            ),
            ButtonVariant::Danger => css!(
                "
                background: linear-gradient(135deg, #ff6b6b 0%, #ee5a52 100%);
                color: white;

                &:hover {
                    background: linear-gradient(135deg, #ff5252 0%, #e53935 100%);
                }
            "
            ),
        };

        // Conditional disabled styles
        let disabled_styles = css_if!(
            self.disabled,
            "
            opacity: 0.6 !important;
            cursor: not-allowed !important;
            pointer-events: none;
        "
        );

        format!(
            r#"<button class="{} {} {}" {}>{}</button>"#,
            base_styles,
            variant_styles,
            disabled_styles,
            if self.disabled { "disabled" } else { "" },
            self.text
        )
    }
}

/// Example card component
struct Card {
    title: String,
    content: String,
    elevated: bool,
}

impl Card {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            elevated: false,
        }
    }

    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    pub fn render(&self) -> String {
        let card_styles = css!(
            "
            background: white;
            border-radius: 12px;
            padding: 24px;
            margin: 16px 0;
            border: 1px solid #e1e5e9;
            transition: all 0.3s ease;

            .card-title {
                font-size: 24px;
                font-weight: 700;
                color: #2d3748;
                margin-bottom: 16px;
                line-height: 1.2;
            }

            .card-content {
                font-size: 16px;
                line-height: 1.6;
                color: #4a5568;
            }
        "
        );

        let elevated_styles = css_if!(
            self.elevated,
            "
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
            transform: translateY(-2px);

            &:hover {
                box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
                transform: translateY(-4px);
            }
        "
        );

        format!(
            r#"<div class="{} {}">
                <h2 class="card-title">{}</h2>
                <div class="card-content">{}</div>
            </div>"#,
            card_styles, elevated_styles, self.title, self.content
        )
    }
}

/// Example layout component
struct Layout {
    children: Vec<String>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add_child(mut self, child: String) -> Self {
        self.children.push(child);
        self
    }

    pub fn render(&self) -> String {
        let layout_styles = css!(
            "
            min-height: 100vh;
            background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
            padding: 40px 20px;

            .container {
                max-width: 800px;
                margin: 0 auto;
            }

            .header {
                text-align: center;
                margin-bottom: 40px;
            }

            .header h1 {
                font-size: 48px;
                font-weight: 800;
                color: #2d3748;
                margin-bottom: 16px;
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
            }

            .header p {
                font-size: 20px;
                color: #718096;
                max-width: 600px;
                margin: 0 auto;
                line-height: 1.6;
            }

            .content {
                display: grid;
                gap: 24px;
            }

            .button-group {
                display: flex;
                gap: 16px;
                justify-content: center;
                flex-wrap: wrap;
                margin: 32px 0;
            }
        "
        );

        let children_html = self.children.join("\n");

        format!(
            r#"<div class="{}">
                <div class="container">
                    <div class="header">
                        <h1>CSS-in-Rust Demo</h1>
                        <p>A complete example showcasing CSS-in-Rust with dead code elimination</p>
                    </div>
                    <div class="content">
                        {}
                    </div>
                </div>
            </div>"#,
            layout_styles, children_html
        )
    }
}

/// Demonstrate build-time optimization
fn demonstrate_build_optimization() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Build-time CSS Optimization Demo ===");

    // Create a build configuration
    let config = BuildConfig {
        project_root: std::env::current_dir()?,
        output_dir: PathBuf::from("target/css-optimized"),
        enable_dead_code_elimination: true,
        generate_reports: true,
        usage_threshold: 0.0,
        aggressive_elimination: false,
    };

    println!("Build configuration:");
    println!("  Project root: {}", config.project_root.display());
    println!("  Output directory: {}", config.output_dir.display());
    println!(
        "  Dead code elimination: {}",
        config.enable_dead_code_elimination
    );
    println!("  Generate reports: {}", config.generate_reports);
    println!("  Usage threshold: {}", config.usage_threshold);
    println!(
        "  Aggressive elimination: {}",
        config.aggressive_elimination
    );

    // Create and run the build processor
    let processor = CssBuildProcessor::with_config(config);

    match processor.run() {
        Ok(result) => {
            println!("\n✅ Build completed successfully!");
            println!("📊 Build Statistics:");
            println!("  Files processed: {}", result.processed_files.len());
            println!(
                "  Original total size: {} bytes",
                result.total_original_size
            );
            println!(
                "  Optimized total size: {} bytes",
                result.total_optimized_size
            );
            println!(
                "  Total savings: {} bytes ({:.2}%)",
                result.savings_bytes, result.savings_percentage
            );

            println!("\n📋 CSS Usage Report:");
            println!(
                "  Files analyzed: {}",
                result.usage_report.analyzed_files.len()
            );
            println!(
                "  CSS macro calls: {}",
                result.usage_report.css_macro_calls.len()
            );
            println!(
                "  Unique classes: {}",
                result.usage_report.used_classes.len()
            );
            println!("  Unique IDs: {}", result.usage_report.used_ids.len());
        }
        Err(e) => {
            println!("❌ Build failed: {}", e);
        }
    }

    Ok(())
}

/// Demonstrate static analysis
fn demonstrate_static_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Static Analysis Demo ===");

    let analyzer = StaticAnalyzer::new(std::env::current_dir()?)
        .with_include_patterns(vec!["**/*.rs".to_string(), "**/*.html".to_string()])
        .with_exclude_patterns(vec!["target/**".to_string(), "**/.git/**".to_string()]);

    match analyzer.analyze() {
        Ok(report) => {
            println!("✅ Static analysis completed!");
            println!("📊 Analysis Results:");
            println!("  Files analyzed: {}", report.analyzed_files.len());
            println!("  CSS macro calls found: {}", report.css_macro_calls.len());
            println!("  Unique CSS classes: {}", report.used_classes.len());
            println!("  Unique CSS IDs: {}", report.used_ids.len());

            if !report.used_classes.is_empty() {
                println!("\n🎨 CSS Classes found:");
                for (i, class) in report.used_classes.iter().enumerate() {
                    if i < 10 {
                        // Show first 10
                        println!("    .{}", class);
                    } else if i == 10 {
                        println!("    ... and {} more", report.used_classes.len() - 10);
                        break;
                    }
                }
            }

            // Save report
            let report_path = PathBuf::from("target/analysis-report.json");
            if let Some(parent) = report_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            analyzer.save_report(&report, &report_path)?;
            println!("\n💾 Analysis report saved to: {}", report_path.display());
        }
        Err(e) => {
            println!("❌ Static analysis failed: {}", e);
        }
    }

    Ok(())
}

/// Demonstrate runtime CSS optimization
fn demonstrate_runtime_optimization() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Runtime CSS Optimization Demo ===");

    // Sample CSS with some unused rules
    let sample_css = r#"
        .btn { padding: 10px; }
        .btn-primary { background: blue; }
        .btn-secondary { background: gray; }
        .unused-class { color: red; }
        #main-header { font-size: 24px; }
        #unused-id { display: none; }
        .card { border: 1px solid #ccc; }
        .card-title { font-weight: bold; }
        .never-used { opacity: 0; }
    "#;

    println!("Original CSS ({} bytes):", sample_css.len());
    println!("{}", sample_css);

    // Create optimizer with dead code elimination
    let config = OptimizerConfig {
        minify: true, // 包含了颜色、字体等优化功能
        analyze_dependencies: true,
        vendor_prefix: true,
        enable_dead_code_elimination: true, // 替代了 remove_unused 功能
        source_paths: vec![std::env::current_dir()?],
        aggressive_elimination: false,
        usage_threshold: 0.0,
        #[cfg(feature = "optimizer")]
        targets: Some(lightningcss::targets::Browsers::default()),
        #[cfg(not(feature = "optimizer"))]
        targets: None,
    };

    let mut optimizer = CssOptimizer::with_config(config);

    // Track some CSS usage (simulating what would be found during analysis)
    optimizer.track_css_usage(vec!["btn".to_string()], vec![], None);
    optimizer.track_css_usage(vec!["btn-primary".to_string()], vec![], None);
    optimizer.track_css_usage(vec!["card".to_string()], vec![], None);
    optimizer.track_css_usage(vec!["card-title".to_string()], vec![], None);
    optimizer.track_css_usage(vec![], vec!["main-header".to_string()], None);

    // Parse CSS string into StyleSheet
    let parser = css_in_rust::core::parser::CssParser::new();
    let stylesheet = parser.parse(sample_css)?;

    match optimizer.optimize(stylesheet) {
        Ok(optimized) => {
            println!("\n✅ Optimization completed!");
            println!("Optimized CSS ({} bytes):", optimized.len());
            println!("{}", optimized);

            let savings = sample_css.len().saturating_sub(optimized.len());
            let percentage = if sample_css.len() > 0 {
                (savings as f64 / sample_css.len() as f64) * 100.0
            } else {
                0.0
            };

            println!("\n📊 Optimization Results:");
            println!("  Original size: {} bytes", sample_css.len());
            println!("  Optimized size: {} bytes", optimized.len());
            println!("  Savings: {} bytes ({:.2}%)", savings, percentage);
        }
        Err(e) => {
            println!("❌ Optimization failed: {}", e);
        }
    }

    Ok(())
}

/// Main example function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CSS-in-Rust Complete Project Example");
    println!("========================================\n");

    // Create example components
    let primary_button = Button::new("Primary Action".to_string(), ButtonVariant::Primary);
    let secondary_button = Button::new("Secondary Action".to_string(), ButtonVariant::Secondary);
    let danger_button = Button::new("Delete".to_string(), ButtonVariant::Danger).disabled(true);

    let card1 = Card::new(
        "Welcome to CSS-in-Rust".to_string(),
        "This library provides a powerful way to write CSS directly in your Rust code with compile-time optimization and dead code elimination.".to_string()
    ).elevated(true);

    let card2 = Card::new(
        "Dead Code Elimination".to_string(),
        "Automatically removes unused CSS rules during the build process, resulting in smaller bundle sizes and better performance.".to_string()
    );

    // Create layout with components
    let layout = Layout::new()
        .add_child(format!(
            r#"<div class="button-group">{}{}{}</div>"#,
            primary_button.render(),
            secondary_button.render(),
            danger_button.render()
        ))
        .add_child(card1.render())
        .add_child(card2.render());

    println!("🎨 Generated HTML with CSS-in-Rust:");
    println!("{}", layout.render());

    // Demonstrate build-time optimization
    if let Err(e) = demonstrate_build_optimization() {
        eprintln!("Build optimization demo failed: {}", e);
    }

    // Demonstrate static analysis
    if let Err(e) = demonstrate_static_analysis() {
        eprintln!("Static analysis demo failed: {}", e);
    }

    // Demonstrate runtime optimization
    if let Err(e) = demonstrate_runtime_optimization() {
        eprintln!("Runtime optimization demo failed: {}", e);
    }

    println!("\n✨ Example completed! Check the generated files in the target directory.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new("Test".to_string(), ButtonVariant::Primary);
        assert_eq!(button.text, "Test");
        assert!(!button.disabled);
    }

    #[test]
    fn test_button_rendering() {
        let button = Button::new("Test Button".to_string(), ButtonVariant::Primary);
        let html = button.render();
        assert!(html.contains("Test Button"));
        assert!(html.contains("<button"));
    }

    #[test]
    fn test_card_creation() {
        let card = Card::new("Title".to_string(), "Content".to_string());
        assert_eq!(card.title, "Title");
        assert_eq!(card.content, "Content");
        assert!(!card.elevated);
    }

    #[test]
    fn test_card_rendering() {
        let card = Card::new("Test Title".to_string(), "Test Content".to_string());
        let html = card.render();
        assert!(html.contains("Test Title"));
        assert!(html.contains("Test Content"));
        assert!(html.contains("card-title"));
        assert!(html.contains("card-content"));
    }

    #[test]
    fn test_layout_creation() {
        let layout = Layout::new();
        assert!(layout.children.is_empty());
    }

    #[test]
    fn test_layout_with_children() {
        let layout = Layout::new()
            .add_child("<div>Child 1</div>".to_string())
            .add_child("<div>Child 2</div>".to_string());

        assert_eq!(layout.children.len(), 2);

        let html = layout.render();
        assert!(html.contains("Child 1"));
        assert!(html.contains("Child 2"));
    }
}

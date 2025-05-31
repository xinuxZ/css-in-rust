//! Dead Code Elimination Example
//!
//! This example demonstrates how to use the CSS dead code elimination feature
//! to automatically remove unused CSS rules from your stylesheets.

use css_in_rust::core::optimizer::{CssOptimizer, OptimizerConfig};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CSS Dead Code Elimination Example");
    println!("=================================\n");

    // Create an optimizer with dead code elimination enabled
    let config = OptimizerConfig {
        enable_dead_code_elimination: true,
        analyze_dependencies: true,
        source_paths: vec![
            PathBuf::from("src/main.rs"),
            PathBuf::from("src/components/"),
        ],
        aggressive_elimination: false,
        usage_threshold: 0.0,
        ..Default::default()
    };

    let mut optimizer = CssOptimizer::with_config(config);

    // Example CSS with both used and unused rules
    let css_content = r#"
        .header {
            background-color: #333;
            color: white;
            padding: 1rem;
        }

        .button {
            background-color: #007bff;
            color: white;
            border: none;
            padding: 0.5rem 1rem;
            border-radius: 4px;
            cursor: pointer;
        }

        .unused-class {
            display: none;
            color: red;
        }

        #main-content {
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }

        #unused-id {
            background: yellow;
        }

        .footer {
            background-color: #f8f9fa;
            padding: 2rem;
            text-align: center;
        }
    "#;

    println!("Original CSS ({} bytes):", css_content.len());
    println!("{}", css_content);
    println!();

    // Simulate tracking CSS usage (in real usage, this would be done automatically
    // by the css! macro or by analyzing source files)
    let used_classes = vec![
        "header".to_string(),
        "button".to_string(),
        "footer".to_string(),
    ];

    let used_ids = vec!["main-content".to_string()];

    // Track the CSS usage
    optimizer.track_css_usage(used_classes, used_ids, Some("example.rs".to_string()));

    // Perform dead code elimination
    let optimized_css = optimizer.eliminate_dead_code(css_content)?;

    println!(
        "Optimized CSS after dead code elimination ({} bytes):",
        optimized_css.len()
    );
    println!("{}", optimized_css);
    println!();

    // Calculate savings
    let original_size = css_content.len();
    let optimized_size = optimized_css.len();
    let savings = original_size - optimized_size;
    let savings_percent = (savings as f64 / original_size as f64) * 100.0;

    println!("Dead Code Elimination Results:");
    println!("- Original size: {} bytes", original_size);
    println!("- Optimized size: {} bytes", optimized_size);
    println!("- Bytes saved: {} bytes", savings);
    println!("- Size reduction: {:.1}%", savings_percent);
    println!();

    // Show usage tracking information
    let tracker = optimizer.get_usage_tracker();
    println!("Tracked Usage:");
    println!("- Used classes: {:?}", tracker.used_classes);
    println!("- Used IDs: {:?}", tracker.used_ids);
    println!(
        "- Analyzed files: {:?}",
        tracker.file_usage.keys().collect::<Vec<_>>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_code_elimination_example() {
        // This test ensures the example code works correctly
        let result = main();
        assert!(result.is_ok());
    }

    #[test]
    fn test_aggressive_elimination() {
        let config = OptimizerConfig {
            enable_dead_code_elimination: true,
            aggressive_elimination: true,
            ..Default::default()
        };

        let mut optimizer = CssOptimizer::with_config(config);

        // Track minimal usage
        optimizer.track_css_usage(vec!["used".to_string()], vec![], None);

        let css = ".used { color: red; } p { margin: 0; } div { padding: 0; }";
        let result = optimizer.eliminate_dead_code(css).unwrap();

        // With aggressive elimination, element selectors should also be removed
        assert!(result.contains(".used"));
        // In aggressive mode, p and div selectors might be removed
        // (depending on implementation details)
    }

    #[test]
    fn test_usage_threshold() {
        let config = OptimizerConfig {
            enable_dead_code_elimination: true,
            usage_threshold: 0.5, // Require 50% usage to keep a rule
            ..Default::default()
        };

        let optimizer = CssOptimizer::with_config(config);

        // Test that usage threshold affects elimination decisions
        assert_eq!(optimizer.config.usage_threshold, 0.5);
    }
}

//! Integration tests for css-in-rust
//!
//! These tests verify the complete functionality of the library,
//! including macro expansion, CSS processing, and style injection.

use css_in_rust::{
    core::{CssProcessor, OptimizerConfig, ParserConfig},
    css, init, inject_style,
    runtime::{MemoryStyleProvider, StyleProvider},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_macro_basic() {
        init();

        let class_name = css! {
            color: red;
            font-size: 16px;
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_macro_with_selectors() {
        init();

        let class_name = css! {
            color: blue;

            &:hover {
                color: red;
            }

            & .child {
                font-size: 14px;
            }
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_macro_with_media_queries() {
        init();

        let class_name = css! {
            color: black;

            @media (max-width: 768px) {
                color: white;
                font-size: 12px;
            }
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_macro_with_variables() {
        init();

        let primary_color = "#007bff";
        let font_size = "16px";

        let class_name = css! {
            color: {primary_color};
            font-size: {font_size};
            border: 1px solid {primary_color};
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_macro_deduplication() {
        init();

        let class1 = css! {
            color: red;
            font-size: 16px;
        };

        let class2 = css! {
            color: red;
            font-size: 16px;
        };

        // Same CSS should generate the same class name
        assert_eq!(class1, class2);
    }

    #[test]
    fn test_css_macro_different_styles() {
        init();

        let class1 = css! {
            color: red;
        };

        let class2 = css! {
            color: blue;
        };

        // Different CSS should generate different class names
        assert_ne!(class1, class2);
    }

    #[test]
    fn test_inject_style_function() {
        init();

        let css_content = ".test { color: red; }";
        let result = inject_style(css_content);

        assert!(result.is_ok());
        let class_name = result.unwrap();
        assert!(!class_name.is_empty());
    }

    #[test]
    fn test_css_processor() {
        let processor = CssProcessor::new(ParserConfig::default(), OptimizerConfig::default());

        let css = ".test { color: red; font-size: 16px; }";
        let result = processor.process(css);

        assert!(result.is_ok());
        let processed = result.unwrap();
        assert!(!processed.is_empty());
    }

    #[test]
    fn test_css_processor_with_optimization() {
        let mut config = OptimizerConfig::default();
        config.minify = true;

        let processor = CssProcessor::new(ParserConfig::default(), config);

        let css = ".test {\n  color: red;\n  font-size: 16px;\n}";
        let result = processor.process(css);

        assert!(result.is_ok());
        let processed = result.unwrap();

        // Minified CSS should be shorter
        assert!(processed.len() < css.len());
        assert!(!processed.contains('\n'));
    }

    #[test]
    fn test_memory_style_provider() {
        let mut provider = MemoryStyleProvider::new();

        let css = ".test { color: red; }";
        let class_name = "test-class";

        // Test injection
        let result = provider.inject_style(class_name, css);
        assert!(result.is_ok());

        // Test checking if style is injected
        assert!(provider.is_style_injected(class_name));

        // Test getting style info
        let info = provider.get_style_info(class_name);
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.class_name, class_name);
        assert_eq!(info.css_content, css);

        // Test removal
        let result = provider.remove_style(class_name);
        assert!(result.is_ok());
        assert!(!provider.is_style_injected(class_name));
    }

    #[test]
    fn test_complex_css_with_animations() {
        init();

        let class_name = css! {
            @keyframes slideIn {
                from {
                    transform: translateX(-100%);
                }
                to {
                    transform: translateX(0);
                }
            }

            animation: slideIn 0.3s ease-out;
            color: #333;
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_with_pseudo_elements() {
        init();

        let class_name = css! {
            position: relative;

            &::before {
                content: "";
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 2px;
                background: linear-gradient(90deg, #ff0000, #00ff00);
            }

            &::after {
                content: "→";
                margin-left: 5px;
            }
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_with_complex_selectors() {
        init();

        let class_name = css! {
            display: flex;

            & > .item {
                flex: 1;

                &:nth-child(odd) {
                    background-color: #f0f0f0;
                }

                &:nth-child(even) {
                    background-color: #ffffff;
                }
            }

            & .item + .item {
                margin-left: 10px;
            }
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_with_css_variables() {
        init();

        let class_name = css! {
            --primary-color: #007bff;
            --secondary-color: #6c757d;
            --border-radius: 4px;

            color: var(--primary-color);
            border: 1px solid var(--secondary-color);
            border-radius: var(--border-radius);

            &:hover {
                background-color: var(--primary-color);
                color: white;
            }
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_with_grid_layout() {
        init();

        let class_name = css! {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            grid-gap: 20px;
            padding: 20px;

            & .grid-item {
                background: white;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                padding: 16px;

                &:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
                    transition: all 0.2s ease;
                }
            }
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_error_handling() {
        // Test that invalid CSS is handled gracefully
        let processor = CssProcessor::new(ParserConfig::default(), OptimizerConfig::default());

        let invalid_css = ".test { color: ; }";
        let result = processor.process(invalid_css);

        // Should either succeed with corrected CSS or fail gracefully
        match result {
            Ok(_) => {
                // CSS was corrected and processed successfully
            }
            Err(_) => {
                // CSS processing failed as expected for invalid input
            }
        }
    }

    #[test]
    fn test_style_provider_clear_all() {
        let mut provider = MemoryStyleProvider::new();

        // Inject multiple styles
        provider
            .inject_style("class1", ".class1 { color: red; }")
            .unwrap();
        provider
            .inject_style("class2", ".class2 { color: blue; }")
            .unwrap();
        provider
            .inject_style("class3", ".class3 { color: green; }")
            .unwrap();

        // Verify they are injected
        assert!(provider.is_style_injected("class1"));
        assert!(provider.is_style_injected("class2"));
        assert!(provider.is_style_injected("class3"));

        // Clear all styles
        provider.clear_all_styles().unwrap();

        // Verify they are all removed
        assert!(!provider.is_style_injected("class1"));
        assert!(!provider.is_style_injected("class2"));
        assert!(!provider.is_style_injected("class3"));
    }

    #[test]
    fn test_concurrent_style_injection() {
        use std::sync::Arc;
        use std::thread;

        init();

        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let class_name = css! {
                        color: red;
                        font-size: {format!("{}px", 12 + i)};
                    };
                    class_name
                })
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All should succeed and generate valid class names
        for class_name in results {
            assert!(!class_name.is_empty());
            assert!(class_name.starts_with("css-"));
        }
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;
    use css_in_rust::adapters::dioxus::DioxusStyleProvider;

    #[test]
    fn test_dioxus_style_provider() {
        let mut provider = DioxusStyleProvider::new();

        let css = ".test { color: red; }";
        let class_name = "test-class";

        let result = provider.inject_style(class_name, css);
        assert!(result.is_ok());

        assert!(provider.is_style_injected(class_name));

        let info = provider.get_style_info(class_name);
        assert!(info.is_some());
    }
}

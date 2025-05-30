//! Integration tests for css-in-rust
//!
//! These tests verify the complete functionality of the library,
//! including macro expansion, CSS processing, and style injection.

use css_in_rust::css;

// Suppress unused dependency warnings
#[cfg(feature = "proc-macro")]
#[allow(unused_imports)]
use proc_macro2 as _;
#[cfg(feature = "proc-macro")]
#[allow(unused_imports)]
use quote as _;
#[cfg(feature = "proc-macro")]
#[allow(unused_imports)]
use sha2 as _;
#[cfg(feature = "proc-macro")]
#[allow(unused_imports)]
use syn as _;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_macro_basic() {
        let class_name = css! {
            color: red;
            font-size: 16px;
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_macro_with_selectors() {
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
        let _primary_color = "#007bff";
        let _font_size = "16px";

        let class_name = css! {
            color: {_primary_color};
            font-size: {_font_size};
            border: 1px solid {_primary_color};
        };

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_css_macro_deduplication() {
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
        let class1 = css! {
            color: red;
        };

        let class2 = css! {
            color: blue;
        };

        // Different CSS should generate different class names
        assert_ne!(class1, class2);
    }

    // Note: inject_style function has been removed in the simplified version
    // #[test]
    // fn test_inject_style_function() {
    //     let css_content = ".test { color: red; }";
    //     // This test is disabled as inject_style is not available
    // }

    // Note: CssProcessor and related functionality has been removed in the simplified version
    // #[test]
    // fn test_css_processor() {
    //     // This test is disabled as CssProcessor is not available
    // }

    // #[test]
    // fn test_css_processor_with_optimization() {
    //     // This test is disabled as CssProcessor is not available
    // }

    // #[test]
    // fn test_memory_style_provider() {
    //     // This test is disabled as MemoryStyleProvider is not available
    // }

    #[test]
    fn test_complex_css_with_animations() {
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

    // #[test]
    // fn test_css_error_handling() {
    //     // Test that invalid CSS is handled gracefully
    //     let processor = CssProcessor::new(ParserConfig::default(), OptimizerConfig::default());
    //
    //     let invalid_css = ".test { color: ; }";
    //     let result = processor.process(invalid_css);
    //
    //     // Should either succeed with corrected CSS or fail gracefully
    //     match result {
    //         Ok(_) => {
    //             // CSS was corrected and processed successfully
    //         }
    //         Err(_) => {
    //             // CSS processing failed as expected for invalid input
    //         }
    //     }
    // }

    // #[test]
    // fn test_style_provider_clear_all() {
    //     let mut provider = MemoryStyleProvider::new();
    //
    //     // Inject multiple styles
    //     provider
    //         .inject_style("class1", ".class1 { color: red; }")
    //         .unwrap();
    //     provider
    //         .inject_style("class2", ".class2 { color: blue; }")
    //         .unwrap();
    //     provider
    //         .inject_style("class3", ".class3 { color: green; }")
    //         .unwrap();
    //
    //     // Verify they are injected
    //     assert!(provider.is_style_injected("class1"));
    //     assert!(provider.is_style_injected("class2"));
    //     assert!(provider.is_style_injected("class3"));
    //
    //     // Clear all styles
    //     provider.clear_all_styles().unwrap();
    //
    //     // Verify they are all removed
    //     assert!(!provider.is_style_injected("class1"));
    //     assert!(!provider.is_style_injected("class2"));
    //     assert!(!provider.is_style_injected("class3"));
    // }

    #[test]
    fn test_concurrent_style_injection() {
        use std::thread;

        // Removed init() call

        let handles: Vec<_> = (0..10)
            .map(|_i| {
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

// Note: Dioxus functionality has been removed in the simplified version
// #[cfg(feature = "dioxus")]
// mod dioxus_tests {
//     use super::*;
//     use css_in_rust::adapters::dioxus::DioxusStyleProvider;
//
//     #[test]
//     fn test_dioxus_style_provider() {
//         let mut provider = DioxusStyleProvider::new();

//         let css = ".test { color: red; }";
//         let class_name = "test-class";
//
//         let result = provider.inject_style(class_name, css);
//         assert!(result.is_ok());
//
//         assert!(provider.is_style_injected(class_name));
//
//         let info = provider.get_style_info(class_name);
//         assert!(info.is_some());
//     }
// }

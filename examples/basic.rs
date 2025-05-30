//! Basic example demonstrating CSS-in-Rust usage
//!
//! This example shows how to use the css! macro for basic styling.

use css_in_rust::{css, init};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the CSS runtime
    init()?;

    println!("CSS-in-Rust Basic Example");
    println!("========================\n");

    // Example 1: Simple CSS
    let simple_class = css!(
        r#"
        color: red;
        font-size: 16px;
        font-weight: bold;
    "#
    );

    println!("Simple CSS class: {}", simple_class);

    // Example 2: CSS with selectors
    let button_class = css!(
        r#"
        .button {
            background: #007bff;
            color: white;
            padding: 8px 16px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
            transition: background-color 0.2s;
        }

        .button:hover {
            background: #0056b3;
        }

        .button:active {
            background: #004085;
        }
    "#
    );

    println!("Button CSS class: {}", button_class);

    // Example 3: CSS with media queries
    let responsive_class = css!(
        r#"
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 16px;
        }

        @media (max-width: 768px) {
            .container {
                padding: 0 8px;
            }
        }

        @media (max-width: 480px) {
            .container {
                padding: 0 4px;
            }
        }
    "#
    );

    println!("Responsive CSS class: {}", responsive_class);

    // Example 4: CSS with animations
    let animated_class = css!(
        r#"
        @keyframes fadeIn {
            from {
                opacity: 0;
                transform: translateY(20px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .fade-in {
            animation: fadeIn 0.3s ease-out;
        }
    "#
    );

    println!("Animated CSS class: {}", animated_class);

    // Example 5: CSS with CSS variables
    let themed_class = css!(
        r#"
        :root {
            --primary-color: #007bff;
            --secondary-color: #6c757d;
            --success-color: #28a745;
            --danger-color: #dc3545;
            --warning-color: #ffc107;
            --info-color: #17a2b8;
        }

        .card {
            background: white;
            border: 1px solid #dee2e6;
            border-radius: 8px;
            padding: 16px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }

        .card-primary {
            border-color: var(--primary-color);
            background: rgba(0, 123, 255, 0.1);
        }

        .card-success {
            border-color: var(--success-color);
            background: rgba(40, 167, 69, 0.1);
        }
    "#
    );

    println!("Themed CSS class: {}", themed_class);

    // Example 6: Complex layout CSS
    let layout_class = css!(
        r#"
        .layout {
            display: grid;
            grid-template-columns: 250px 1fr;
            grid-template-rows: 60px 1fr 40px;
            grid-template-areas:
                "sidebar header"
                "sidebar main"
                "sidebar footer";
            min-height: 100vh;
        }

        .header {
            grid-area: header;
            background: #f8f9fa;
            border-bottom: 1px solid #dee2e6;
            display: flex;
            align-items: center;
            padding: 0 16px;
        }

        .sidebar {
            grid-area: sidebar;
            background: #343a40;
            color: white;
            padding: 16px;
        }

        .main {
            grid-area: main;
            padding: 16px;
            overflow-y: auto;
        }

        .footer {
            grid-area: footer;
            background: #f8f9fa;
            border-top: 1px solid #dee2e6;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 12px;
            color: #6c757d;
        }

        @media (max-width: 768px) {
            .layout {
                grid-template-columns: 1fr;
                grid-template-rows: 60px auto 1fr 40px;
                grid-template-areas:
                    "header"
                    "sidebar"
                    "main"
                    "footer";
            }

            .sidebar {
                padding: 8px;
            }
        }
    "#
    );

    println!("Layout CSS class: {}", layout_class);

    println!("\n✅ All CSS classes generated successfully!");
    println!("\nNote: In a real web application, these class names would be");
    println!("automatically injected into the DOM and can be used in your HTML.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        // Test that the example runs without panicking
        let result = main();
        assert!(result.is_ok());
    }

    #[test]
    fn test_css_macro() {
        init().unwrap();

        let class_name = css!(
            r#"
            color: red;
            font-size: 16px;
        "#
        );

        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    #[test]
    fn test_multiple_css_calls() {
        init().unwrap();

        let class1 = css!("color: red;");
        let class2 = css!("color: blue;");
        let class3 = css!("color: red;"); // Same as class1

        assert_ne!(class1, class2);
        // Due to deduplication, class1 and class3 should be the same
        assert_eq!(class1, class3);
    }
}

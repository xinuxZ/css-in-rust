//! Basic CSS-in-Rust Example
//!
//! This example shows how to use the css! macro for basic styling.

#[cfg(feature = "proc-macro")]
use css_in_rust::css;

#[cfg(not(feature = "proc-macro"))]
use css_in_rust as _;

#[allow(unused_imports)]
use css_in_rust_macros as _;

#[cfg(feature = "proc-macro")]
fn main() {
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
            padding: 0 20px;
        }

        @media (max-width: 768px) {
            .container {
                padding: 0 10px;
            }
        }

        @media (max-width: 480px) {
            .container {
                padding: 0 5px;
            }
        }
    "#
    );

    println!("Responsive CSS class: {}", responsive_class);

    println!("\nAll examples completed successfully!");
}

#[cfg(not(feature = "proc-macro"))]
fn main() {
    println!("CSS-in-Rust Basic Example (No Proc-Macro)");
    println!("==========================================\n");

    println!("This example requires the 'proc-macro' feature to be enabled.");
    println!("Run with: cargo run --example basic --features proc-macro");
    println!("\nWithout proc-macro support, CSS-in-Rust operates in a limited mode.");
    println!("The library can still be used for basic CSS string processing,");
    println!("but the compile-time CSS! macro is not available.");
}

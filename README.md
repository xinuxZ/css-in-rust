# CSS-in-Rust

🚀 A high-performance CSS-in-Rust solution based on stylers + lightningcss.

## Features

- **🚀 High Performance**: Built on top of lightningcss for fast CSS parsing and optimization
- **🔒 Type Safety**: Compile-time CSS validation and error checking
- **🎨 Theme System**: Built-in support for CSS variables and theming (Phase 2)
- **📱 Responsive Design**: First-class support for media queries and responsive layouts
- **🔧 Framework Integration**: Adapters for popular Rust web frameworks (Dioxus, Yew, Leptos)
- **⚡ Style Optimization**: Automatic CSS minification, dead code elimination, and deduplication
- **🔄 Hot Reload**: Development-time style updates (Phase 2)
- **📦 SSR Support**: Server-side rendering with critical CSS extraction

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
css-in-rust = "0.1.0"
```

For Dioxus integration:

```toml
[dependencies]
css-in-rust = { version = "0.1.0", features = ["dioxus"] }
```

### Basic Usage

```rust
use css_in_rust::{css, init};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the CSS runtime
    init()?;

    // Define styles using the css! macro
    let button_class = css! {
        r#"
        .button {
            background: #007bff;
            color: white;
            padding: 8px 16px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            transition: background-color 0.2s;
        }

        .button:hover {
            background: #0056b3;
        }
        "#
    };

    println!("Generated class: {}", button_class);
    // Output: Generated class: css-a1b2c3d4

    Ok(())
}
```

### Advanced Usage

#### Responsive Design

```rust
let responsive_class = css! {
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
};
```

#### CSS Variables and Theming

```rust
let themed_class = css! {
    r#"
    :root {
        --primary-color: #007bff;
        --secondary-color: #6c757d;
    }

    .card {
        background: white;
        border: 1px solid var(--primary-color);
        border-radius: 8px;
        padding: 16px;
    }
    "#
};
```

#### Animations

```rust
let animated_class = css! {
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
};
```

## Framework Integration

### Dioxus

```rust
use dioxus::prelude::*;
use css_in_rust::css;

fn App(cx: Scope) -> Element {
    let button_style = css! {
        r#"
        .button {
            background: #007bff;
            color: white;
            padding: 8px 16px;
            border: none;
            border-radius: 4px;
        }
        "#
    };

    render! {
        button {
            class: "{button_style}",
            "Click me!"
        }
    }
}
```

## Architecture

### Core Components

- **Parser**: CSS parsing using lightningcss
- **Optimizer**: CSS optimization and minification
- **Runtime**: Style injection and management
- **Macro System**: Compile-time CSS processing
- **Provider System**: Platform-specific style injection

### Performance

- **Compile-time Processing**: CSS is parsed and validated at compile time
- **Style Deduplication**: Identical styles are automatically deduplicated
- **Lazy Injection**: Styles are only injected when first used
- **Caching**: Intelligent caching system for optimal performance
- **Minification**: Automatic CSS minification in release builds

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running Examples

```bash
cargo run --example basic
```

### Benchmarks

```bash
cargo bench
```

## Roadmap

### Phase 1 (Current) ✅
- [x] Basic CSS parsing with lightningcss
- [x] `css!` macro implementation
- [x] Style injection system
- [x] Platform-specific providers (Web/Server)
- [x] Basic optimization
- [x] Documentation and examples

### Phase 2 (Planned)
- [ ] Theme system with CSS variables
- [ ] Variant system (hover, focus, etc.)
- [ ] Style optimization engine
- [ ] Hot reload support
- [ ] Advanced caching

### Phase 3 (Planned)
- [ ] Dioxus adapter
- [ ] ant-design-dioxus integration
- [ ] SSR/SSG support
- [ ] Critical CSS extraction

### Phase 4 (Planned)
- [ ] VS Code plugin
- [ ] CLI tools
- [ ] Additional framework adapters
- [ ] Performance monitoring

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/css-in-rust.git
   cd css-in-rust
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run examples:
   ```bash
   cargo run --example basic
   ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [lightningcss](https://github.com/parcel-bundler/lightningcss) - Fast CSS parser and transformer
- [stylers](https://github.com/abishekatp/stylers) - Original inspiration for CSS-in-Rust
- [Dioxus](https://github.com/DioxusLabs/dioxus) - Modern Rust web framework
- [ant-design](https://ant.design/) - Design system inspiration

## Support

If you have any questions or need help, please:

1. Check the [documentation](https://docs.rs/css-in-rust)
2. Search [existing issues](https://github.com/your-org/css-in-rust/issues)
3. Create a [new issue](https://github.com/your-org/css-in-rust/issues/new)

---

**Made with ❤️ by the CSS-in-Rust team**

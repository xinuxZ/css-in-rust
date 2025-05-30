//! Benchmarks for CSS processing performance
//!
//! This module contains benchmarks to measure the performance of various
//! CSS processing operations.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use css_in_rust::core::{CssOptimizer, CssParser, OptimizerConfig, ParserConfig};
use css_in_rust::runtime::provider::ProviderType;
use css_in_rust::runtime::{StyleManager, StyleManagerConfig};
use css_in_rust::{init, inject_style};

/// Sample CSS for benchmarking
const SIMPLE_CSS: &str = r#"
    color: red;
    font-size: 16px;
    margin: 10px;
"#;

const COMPLEX_CSS: &str = r#"
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        min-height: 100vh;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }

    .card {
        background: white;
        border-radius: 12px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
        padding: 32px;
        max-width: 400px;
        width: 100%;
        transform: translateY(0);
        transition: all 0.3s ease;
    }

    .card:hover {
        transform: translateY(-5px);
        box-shadow: 0 15px 40px rgba(0, 0, 0, 0.3);
    }

    .title {
        font-size: 24px;
        font-weight: 600;
        color: #333;
        margin-bottom: 16px;
        text-align: center;
    }

    .button {
        background: #007bff;
        color: white;
        border: none;
        padding: 12px 24px;
        border-radius: 6px;
        font-size: 16px;
        cursor: pointer;
        transition: background-color 0.2s;
        width: 100%;
    }

    .button:hover {
        background: #0056b3;
    }

    .button:active {
        background: #004085;
    }

    @media (max-width: 768px) {
        .container {
            padding: 16px;
        }

        .card {
            padding: 24px;
        }

        .title {
            font-size: 20px;
        }
    }

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
        animation: fadeIn 0.5s ease-out;
    }
"#;

const LARGE_CSS: &str = include_str!("../test_data/large.css");

/// Benchmark CSS parsing performance
fn bench_css_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("css_parsing");

    let parser = CssParser::new(ParserConfig::default());

    group.bench_with_input(
        BenchmarkId::new("simple", "simple_css"),
        &SIMPLE_CSS,
        |b, css| b.iter(|| parser.parse(black_box(css)).unwrap()),
    );

    group.bench_with_input(
        BenchmarkId::new("complex", "complex_css"),
        &COMPLEX_CSS,
        |b, css| b.iter(|| parser.parse(black_box(css)).unwrap()),
    );

    // Only run large CSS benchmark if the file exists
    if !LARGE_CSS.is_empty() {
        group.bench_with_input(
            BenchmarkId::new("large", "large_css"),
            &LARGE_CSS,
            |b, css| b.iter(|| parser.parse(black_box(css)).unwrap()),
        );
    }

    group.finish();
}

/// Benchmark CSS optimization performance
fn bench_css_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("css_optimization");

    let parser = CssParser::new(ParserConfig::default());
    let optimizer = CssOptimizer::new(OptimizerConfig::default());

    let simple_ast = parser.parse(SIMPLE_CSS).unwrap();
    let complex_ast = parser.parse(COMPLEX_CSS).unwrap();

    group.bench_with_input(
        BenchmarkId::new("simple", "simple_css"),
        &simple_ast,
        |b, ast| b.iter(|| optimizer.optimize(black_box(ast.clone())).unwrap()),
    );

    group.bench_with_input(
        BenchmarkId::new("complex", "complex_css"),
        &complex_ast,
        |b, ast| b.iter(|| optimizer.optimize(black_box(ast.clone())).unwrap()),
    );

    group.finish();
}

/// Benchmark style injection performance
fn bench_style_injection(c: &mut Criterion) {
    let mut group = c.benchmark_group("style_injection");

    // Initialize runtime
    init().unwrap();

    group.bench_with_input(
        BenchmarkId::new("simple", "simple_css"),
        &SIMPLE_CSS,
        |b, css| b.iter(|| inject_style(black_box(css)).unwrap()),
    );

    group.bench_with_input(
        BenchmarkId::new("complex", "complex_css"),
        &COMPLEX_CSS,
        |b, css| b.iter(|| inject_style(black_box(css)).unwrap()),
    );

    group.finish();
}

/// Benchmark style manager performance
fn bench_style_manager(c: &mut Criterion) {
    let mut group = c.benchmark_group("style_manager");

    let config = StyleManagerConfig {
        provider_type: Some(ProviderType::Memory),
        enable_cache: true,
        enable_deduplication: true,
        ..Default::default()
    };

    let manager = StyleManager::with_config(config);

    // Benchmark style injection
    group.bench_with_input(
        BenchmarkId::new("inject", "simple_css"),
        &SIMPLE_CSS,
        |b, css| b.iter(|| manager.inject_style(black_box(css)).unwrap()),
    );

    // Benchmark deduplication
    group.bench_function("deduplication", |b| {
        b.iter(|| {
            // Inject the same CSS multiple times to test deduplication
            for _ in 0..10 {
                manager.inject_style(black_box(SIMPLE_CSS)).unwrap();
            }
        })
    });

    // Benchmark cache performance
    group.bench_function("cache_hit", |b| {
        // Pre-populate cache
        manager.inject_style(SIMPLE_CSS).unwrap();

        b.iter(|| manager.inject_style(black_box(SIMPLE_CSS)).unwrap())
    });

    group.finish();
}

/// Benchmark memory usage
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    let config = StyleManagerConfig {
        provider_type: Some(ProviderType::Memory),
        max_cache_size: 1000,
        ..Default::default()
    };

    group.bench_function("large_cache", |b| {
        b.iter(|| {
            let manager = StyleManager::with_config(config.clone());

            // Inject many different styles
            for i in 0..100 {
                let css = format!(
                    "color: rgb({}, {}, {});",
                    i % 255,
                    (i * 2) % 255,
                    (i * 3) % 255
                );
                manager.inject_style(&css).unwrap();
            }

            black_box(manager)
        })
    });

    group.finish();
}

/// Benchmark concurrent access
fn bench_concurrent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_access");

    init().unwrap();

    group.bench_function("parallel_injection", |b| {
        use std::sync::Arc;
        use std::thread;

        b.iter(|| {
            let handles: Vec<_> = (0..4)
                .map(|i| {
                    let css = format!("color: rgb({}, 0, 0);", i * 50);
                    thread::spawn(move || inject_style(&css).unwrap())
                })
                .collect();

            for handle in handles {
                black_box(handle.join().unwrap());
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_css_parsing,
    bench_css_optimization,
    bench_style_injection,
    bench_style_manager,
    bench_memory_usage,
    bench_concurrent_access
);

criterion_main!(benches);

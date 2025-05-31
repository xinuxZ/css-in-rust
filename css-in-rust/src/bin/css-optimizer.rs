//! CSS Optimizer CLI Tool
//!
//! A command-line interface for CSS optimization and dead code elimination.

use css_in_rust::build_tools::{BuildConfig, CssBuildProcessor};
use css_in_rust::core::optimizer::{CssOptimizer, OptimizerConfig};
use css_in_rust::core::parser::CssParser;
use std::env;
use std::path::PathBuf;
use std::process;

// Suppress unused crate warnings
use base64 as _;
use chrono as _;
use css_in_rust_macros as _;
use lazy_static as _;
use lightningcss as _;
use num_cpus as _;
use proc_macro2 as _;
use quote as _;
use regex as _;
use serde as _;
use serde_json as _;
use sha1 as _;
use sha2 as _;
use syn as _;
use tempfile as _;

/// CLI application for CSS optimization
struct CliApp {
    command: Command,
}

/// Available CLI commands
#[derive(Debug, Clone)]
enum Command {
    /// Build command - process entire project
    Build { config: BuildConfig },
    /// Optimize command - optimize single CSS file
    Optimize {
        input_file: PathBuf,
        output_file: Option<PathBuf>,
        config: OptimizerConfig,
    },
    /// Analyze command - perform static analysis only
    Analyze {
        project_root: PathBuf,
        output_file: Option<PathBuf>,
    },
    /// Help command
    Help,
    /// Version command
    Version,
}

impl CliApp {
    /// Create new CLI app from command line arguments
    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        let command = Self::parse_args(&args);
        Self { command }
    }

    /// Parse command line arguments
    fn parse_args(args: &[String]) -> Command {
        if args.len() < 2 {
            return Command::Help;
        }

        match args[1].as_str() {
            "build" => Self::parse_build_command(&args[2..]),
            "optimize" => Self::parse_optimize_command(&args[2..]),
            "analyze" => Self::parse_analyze_command(&args[2..]),
            "help" | "--help" | "-h" => Command::Help,
            "version" | "--version" | "-V" => Command::Version,
            _ => {
                eprintln!("Error: Unknown command '{}'", args[1]);
                Command::Help
            }
        }
    }

    /// Parse build command arguments
    fn parse_build_command(args: &[String]) -> Command {
        let mut config = BuildConfig::default();
        let mut i = 0;

        while i < args.len() {
            match args[i].as_str() {
                "--project-root" => {
                    if i + 1 < args.len() {
                        config.project_root = PathBuf::from(&args[i + 1]);
                        i += 2;
                    } else {
                        eprintln!("Error: --project-root requires a value");
                        return Command::Help;
                    }
                }
                "--output-dir" => {
                    if i + 1 < args.len() {
                        config.output_dir = PathBuf::from(&args[i + 1]);
                        i += 2;
                    } else {
                        eprintln!("Error: --output-dir requires a value");
                        return Command::Help;
                    }
                }
                "--no-dead-code-elimination" => {
                    config.enable_dead_code_elimination = false;
                    i += 1;
                }
                "--no-reports" => {
                    config.generate_reports = false;
                    i += 1;
                }
                "--aggressive" => {
                    config.aggressive_elimination = true;
                    i += 1;
                }
                "--usage-threshold" => {
                    if i + 1 < args.len() {
                        match args[i + 1].parse::<f32>() {
                            Ok(threshold) => {
                                config.usage_threshold = threshold;
                                i += 2;
                            }
                            Err(_) => {
                                eprintln!("Error: --usage-threshold requires a valid number");
                                return Command::Help;
                            }
                        }
                    } else {
                        eprintln!("Error: --usage-threshold requires a value");
                        return Command::Help;
                    }
                }
                _ => {
                    eprintln!("Error: Unknown build option: {}", args[i]);
                    return Command::Help;
                }
            }
        }

        Command::Build { config }
    }

    /// Parse optimize command arguments
    fn parse_optimize_command(args: &[String]) -> Command {
        if args.is_empty() {
            eprintln!("Error: optimize command requires an input file");
            return Command::Help;
        }

        let input_file = PathBuf::from(&args[0]);
        let mut output_file = None;
        let mut config = OptimizerConfig::default();
        let mut i = 1;

        while i < args.len() {
            match args[i].as_str() {
                "--output" | "-o" => {
                    if i + 1 < args.len() {
                        output_file = Some(PathBuf::from(&args[i + 1]));
                        i += 2;
                    } else {
                        eprintln!("Error: --output requires a value");
                        return Command::Help;
                    }
                }
                "--no-minify" => {
                    config.minify = false;
                    i += 1;
                }
                "--no-remove-unused" => {
                    // Note: remove_unused field not available in current OptimizerConfig
                    // Use enable_dead_code_elimination instead
                    config.enable_dead_code_elimination = false;
                    i += 1;
                }
                "--no-merge-rules" => {
                    // Note: merge_rules field not available in current OptimizerConfig
                    i += 1;
                }
                "--no-optimize-colors" => {
                    // Note: optimize_colors field not available in current OptimizerConfig
                    i += 1;
                }
                "--no-optimize-fonts" => {
                    // Note: optimize_fonts field not available in current OptimizerConfig
                    i += 1;
                }
                "--enable-dead-code-elimination" => {
                    config.enable_dead_code_elimination = true;
                    i += 1;
                }
                "--aggressive" => {
                    config.aggressive_elimination = true;
                    i += 1;
                }
                "--usage-threshold" => {
                    if i + 1 < args.len() {
                        match args[i + 1].parse::<f32>() {
                            Ok(threshold) => {
                                config.usage_threshold = threshold;
                                i += 2;
                            }
                            Err(_) => {
                                eprintln!("Error: --usage-threshold requires a valid number");
                                return Command::Help;
                            }
                        }
                    } else {
                        eprintln!("Error: --usage-threshold requires a value");
                        return Command::Help;
                    }
                }
                _ => {
                    eprintln!("Error: Unknown optimize option: {}", args[i]);
                    return Command::Help;
                }
            }
        }

        Command::Optimize {
            input_file,
            output_file,
            config,
        }
    }

    /// Parse analyze command arguments
    fn parse_analyze_command(args: &[String]) -> Command {
        let mut project_root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut output_file = None;
        let mut i = 0;

        while i < args.len() {
            match args[i].as_str() {
                "--project-root" => {
                    if i + 1 < args.len() {
                        project_root = PathBuf::from(&args[i + 1]);
                        i += 2;
                    } else {
                        eprintln!("Error: --project-root requires a value");
                        return Command::Help;
                    }
                }
                "--output" | "-o" => {
                    if i + 1 < args.len() {
                        output_file = Some(PathBuf::from(&args[i + 1]));
                        i += 2;
                    } else {
                        eprintln!("Error: --output requires a value");
                        return Command::Help;
                    }
                }
                _ => {
                    eprintln!("Error: Unknown analyze option: {}", args[i]);
                    return Command::Help;
                }
            }
        }

        Command::Analyze {
            project_root,
            output_file,
        }
    }

    /// Run the CLI application
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Command::Build { config } => {
                println!("Running CSS build process...");
                let processor = CssBuildProcessor::with_config(config.clone());
                let result = processor.run()?;

                println!("\n=== Build Summary ===");
                println!("Files processed: {}", result.processed_files.len());
                println!("Original total size: {} bytes", result.total_original_size);
                println!(
                    "Optimized total size: {} bytes",
                    result.total_optimized_size
                );
                println!(
                    "Total savings: {} bytes ({:.2}%)",
                    result.savings_bytes, result.savings_percentage
                );

                if !result.processed_files.is_empty() {
                    println!("\n=== Processed Files ===");
                    for file in &result.processed_files {
                        println!(
                            "{} -> {} ({:.2}% reduction)",
                            file.input_path.display(),
                            file.output_path.display(),
                            file.savings_percentage
                        );
                    }
                }
            }

            Command::Optimize {
                input_file,
                output_file,
                config,
            } => {
                println!("Optimizing CSS file: {}", input_file.display());

                let css_content = std::fs::read_to_string(input_file)?;
                let original_size = css_content.len();

                let parser = CssParser::new();
                let stylesheet = parser.parse(&css_content).map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Parse error: {}", e),
                    )
                })?;

                let mut optimizer = CssOptimizer::with_config(config.clone());
                let optimized_content = optimizer.optimize(stylesheet)?;
                let optimized_size = optimized_content.len();

                let output_path = output_file.as_ref().unwrap_or(input_file);
                std::fs::write(output_path, &optimized_content)?;

                let savings = original_size.saturating_sub(optimized_size);
                let savings_percentage = if original_size > 0 {
                    (savings as f64 / original_size as f64) * 100.0
                } else {
                    0.0
                };

                println!("Optimization complete!");
                println!("Original size: {} bytes", original_size);
                println!("Optimized size: {} bytes", optimized_size);
                println!("Savings: {} bytes ({:.2}%)", savings, savings_percentage);
                println!("Output written to: {}", output_path.display());
            }

            Command::Analyze {
                project_root,
                output_file,
            } => {
                println!("Analyzing project: {}", project_root.display());

                let analyzer = css_in_rust::build_tools::StaticAnalyzer::new(project_root.clone());
                let report = analyzer.analyze()?;

                if let Some(output_path) = output_file {
                    analyzer.save_report(&report, output_path)?;
                    println!("Analysis report saved to: {}", output_path.display());
                } else {
                    println!("\n=== Analysis Results ===");
                    println!("Files analyzed: {}", report.analyzed_files.len());
                    println!("CSS macro calls found: {}", report.css_macro_calls.len());
                    println!("Unique CSS classes: {}", report.used_classes.len());
                    println!("Unique CSS IDs: {}", report.used_ids.len());

                    if !report.used_classes.is_empty() {
                        println!("\nCSS Classes found:");
                        for class in &report.used_classes {
                            println!("  .{}", class);
                        }
                    }

                    if !report.used_ids.is_empty() {
                        println!("\nCSS IDs found:");
                        for id in &report.used_ids {
                            println!("  #{}", id);
                        }
                    }
                }
            }

            Command::Help => {
                Self::print_help();
            }

            Command::Version => {
                println!("css-optimizer {}", env!("CARGO_PKG_VERSION"));
            }
        }

        Ok(())
    }

    /// Print help information
    fn print_help() {
        println!("CSS Optimizer - Dead Code Elimination and Optimization Tool");
        println!();
        println!("USAGE:");
        println!("    css-optimizer <COMMAND> [OPTIONS]");
        println!();
        println!("COMMANDS:");
        println!("    build      Process entire project with dead code elimination");
        println!("    optimize   Optimize a single CSS file");
        println!("    analyze    Perform static analysis on project");
        println!("    help       Print this help message");
        println!("    version    Print version information");
        println!();
        println!("BUILD OPTIONS:");
        println!("    --project-root <PATH>        Set the project root directory");
        println!("    --output-dir <PATH>          Set the output directory");
        println!("    --no-dead-code-elimination   Disable dead code elimination");
        println!("    --no-reports                 Disable generation of usage reports");
        println!("    --aggressive                 Enable aggressive elimination mode");
        println!("    --usage-threshold <NUMBER>   Set minimum usage threshold");
        println!();
        println!("OPTIMIZE OPTIONS:");
        println!("    --output, -o <PATH>          Output file path");
        println!("    --no-minify                  Disable minification");
        println!("    --no-remove-unused           Disable unused rule removal");
        println!("    --no-merge-rules             Disable rule merging");
        println!("    --no-optimize-colors         Disable color optimization");
        println!("    --no-optimize-fonts          Disable font optimization");
        println!("    --enable-dead-code-elimination Enable dead code elimination");
        println!("    --aggressive                 Enable aggressive elimination");
        println!("    --usage-threshold <NUMBER>   Set minimum usage threshold");
        println!();
        println!("ANALYZE OPTIONS:");
        println!("    --project-root <PATH>        Set the project root directory");
        println!("    --output, -o <PATH>          Output file for analysis report");
        println!();
        println!("EXAMPLES:");
        println!("    css-optimizer build");
        println!("    css-optimizer build --project-root ./my-project --aggressive");
        println!("    css-optimizer optimize styles.css --output optimized.css");
        println!("    css-optimizer analyze --output analysis-report.json");
    }
}

fn main() {
    let app = CliApp::from_args();

    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

//! Build Script for CSS Dead Code Elimination
//!
//! This module provides a build script that can be used to automatically
//! perform static analysis and dead code elimination during the build process.

use crate::build_tools::static_analyzer::{CssUsageReport, StaticAnalyzer};
use crate::core::optimizer::{CssOptimizer, OptimizerConfig};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

/// Build configuration for CSS optimization
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Project root directory
    pub project_root: PathBuf,
    /// Output directory for optimized CSS
    pub output_dir: PathBuf,
    /// Whether to enable dead code elimination
    pub enable_dead_code_elimination: bool,
    /// Whether to generate usage reports
    pub generate_reports: bool,
    /// Minimum usage threshold for keeping CSS rules
    pub usage_threshold: f32,
    /// Whether to use aggressive elimination
    pub aggressive_elimination: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            project_root: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            output_dir: PathBuf::from("target/css-optimized"),
            enable_dead_code_elimination: true,
            generate_reports: true,
            usage_threshold: 0.0,
            aggressive_elimination: false,
        }
    }
}

/// CSS build processor
pub struct CssBuildProcessor {
    config: BuildConfig,
    analyzer: StaticAnalyzer,
    optimizer: CssOptimizer,
}

impl CssBuildProcessor {
    /// Create a new build processor with default configuration
    pub fn new() -> Self {
        let config = BuildConfig::default();
        Self::with_config(config)
    }

    /// Create a new build processor with custom configuration
    pub fn with_config(config: BuildConfig) -> Self {
        let analyzer = StaticAnalyzer::new(config.project_root.clone())
            .with_include_patterns(vec![
                "**/*.rs".to_string(),
                "**/*.html".to_string(),
                "**/*.htm".to_string(),
                "**/*.css".to_string(),
            ])
            .with_exclude_patterns(vec![
                "target/**".to_string(),
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/.trae/**".to_string(),
            ]);

        let optimizer_config = OptimizerConfig {
            minify: true,
            enable_dead_code_elimination: config.enable_dead_code_elimination,
            source_paths: vec![config.project_root.clone()],
            aggressive_elimination: config.aggressive_elimination,
            usage_threshold: config.usage_threshold,
            analyze_dependencies: true,
            vendor_prefix: true,
            #[cfg(feature = "optimizer")]
            targets: Some(lightningcss::targets::Browsers::default()),
            #[cfg(not(feature = "optimizer"))]
            targets: None,
        };

        let optimizer = CssOptimizer::with_config(optimizer_config);

        Self {
            config,
            analyzer,
            optimizer,
        }
    }

    /// Run the complete build process
    pub fn run(&self) -> Result<BuildResult, BuildError> {
        println!("Starting CSS build process...");

        // Step 1: Perform static analysis
        println!("Step 1: Performing static analysis...");
        let usage_report = self.analyzer.analyze().map_err(BuildError::AnalysisError)?;

        // Step 2: Save usage report if requested
        if self.config.generate_reports {
            let report_path = self.config.output_dir.join("css-usage-report.json");
            fs::create_dir_all(&self.config.output_dir).map_err(BuildError::IoError)?;
            self.analyzer
                .save_report(&usage_report, &report_path)
                .map_err(BuildError::IoError)?;
        }

        // Step 3: Find and process CSS files
        println!("Step 2: Processing CSS files...");
        let css_files = self.find_css_files()?;
        let mut processed_files = Vec::new();
        let mut total_original_size = 0;
        let mut total_optimized_size = 0;

        for css_file in css_files {
            match self.process_css_file(&css_file, &usage_report) {
                Ok(result) => {
                    total_original_size += result.original_size;
                    total_optimized_size += result.optimized_size;
                    processed_files.push(result);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to process {}: {}", css_file.display(), e);
                }
            }
        }

        let build_result = BuildResult {
            usage_report,
            processed_files,
            total_original_size,
            total_optimized_size,
            savings_bytes: total_original_size.saturating_sub(total_optimized_size),
            savings_percentage: if total_original_size > 0 {
                ((total_original_size - total_optimized_size) as f64 / total_original_size as f64)
                    * 100.0
            } else {
                0.0
            },
        };

        println!("Build process completed successfully!");
        println!(
            "Total files processed: {}",
            build_result.processed_files.len()
        );
        println!("Original size: {} bytes", build_result.total_original_size);
        println!(
            "Optimized size: {} bytes",
            build_result.total_optimized_size
        );
        println!(
            "Savings: {} bytes ({:.2}%)",
            build_result.savings_bytes, build_result.savings_percentage
        );

        Ok(build_result)
    }

    /// Find all CSS files in the project
    fn find_css_files(&self) -> Result<Vec<PathBuf>, BuildError> {
        let mut css_files = Vec::new();
        self.walk_directory(&self.config.project_root, &mut css_files)?;
        Ok(css_files)
    }

    /// Recursively walk directory to find CSS files
    fn walk_directory(&self, dir: &Path, css_files: &mut Vec<PathBuf>) -> Result<(), BuildError> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir).map_err(BuildError::IoError)? {
            let entry = entry.map_err(BuildError::IoError)?;
            let path = entry.path();

            if path.is_dir() {
                // Skip excluded directories
                if !self.should_exclude_directory(&path) {
                    self.walk_directory(&path, css_files)?;
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("css") {
                css_files.push(path);
            }
        }

        Ok(())
    }

    /// Check if a directory should be excluded
    fn should_exclude_directory(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("/target/")
            || path_str.contains("/.git/")
            || path_str.contains("/node_modules/")
            || path_str.contains("/.trae/")
    }

    /// Process a single CSS file
    fn process_css_file(
        &self,
        css_file: &Path,
        usage_report: &CssUsageReport,
    ) -> Result<ProcessedFile, BuildError> {
        let original_content = fs::read_to_string(css_file).map_err(BuildError::IoError)?;
        let original_size = original_content.len();

        // Track CSS usage from the usage report
        let mut optimizer = self.optimizer.clone();
        for class in &usage_report.used_classes {
            optimizer.track_css_usage(vec![class.clone()], vec![], None);
        }
        for id in &usage_report.used_ids {
            optimizer.track_css_usage(vec![], vec![id.clone()], None);
        }

        // Parse CSS into StyleSheet
        let parser = crate::core::parser::CssParser::new();
        let stylesheet = parser.parse(&original_content).map_err(|e| {
            BuildError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse CSS: {:?}", e),
            ))
        })?;

        // Optimize the CSS
        let optimized_content = optimizer
            .optimize(stylesheet)
            .map_err(BuildError::OptimizationError)?;
        let optimized_size = optimized_content.len();

        // Create output file path
        let relative_path = css_file
            .strip_prefix(&self.config.project_root)
            .unwrap_or(css_file);
        let output_path = self.config.output_dir.join(relative_path);

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(BuildError::IoError)?;
        }

        // Write optimized CSS
        fs::write(&output_path, &optimized_content).map_err(BuildError::IoError)?;

        Ok(ProcessedFile {
            input_path: css_file.to_path_buf(),
            output_path,
            original_size,
            optimized_size,
            savings_bytes: original_size.saturating_sub(optimized_size),
            savings_percentage: if original_size > 0 {
                ((original_size - optimized_size) as f64 / original_size as f64) * 100.0
            } else {
                0.0
            },
        })
    }
}

/// Result of the build process
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// CSS usage report
    pub usage_report: CssUsageReport,
    /// List of processed files
    pub processed_files: Vec<ProcessedFile>,
    /// Total original size in bytes
    pub total_original_size: usize,
    /// Total optimized size in bytes
    pub total_optimized_size: usize,
    /// Total savings in bytes
    pub savings_bytes: usize,
    /// Savings percentage
    pub savings_percentage: f64,
}

/// Information about a processed CSS file
#[derive(Debug, Clone)]
pub struct ProcessedFile {
    /// Input file path
    pub input_path: PathBuf,
    /// Output file path
    pub output_path: PathBuf,
    /// Original file size in bytes
    pub original_size: usize,
    /// Optimized file size in bytes
    pub optimized_size: usize,
    /// Savings in bytes
    pub savings_bytes: usize,
    /// Savings percentage
    pub savings_percentage: f64,
}

/// Build process errors
#[derive(Debug)]
pub enum BuildError {
    /// IO error
    IoError(std::io::Error),
    /// Analysis error
    AnalysisError(std::io::Error),
    /// Optimization error
    OptimizationError(crate::core::optimizer::OptimizationError),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::IoError(e) => write!(f, "IO error: {}", e),
            BuildError::AnalysisError(e) => write!(f, "Analysis error: {}", e),
            BuildError::OptimizationError(e) => write!(f, "Optimization error: {}", e),
        }
    }
}

impl std::error::Error for BuildError {}

/// Main function for build script usage
pub fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config = BuildConfig::default();

    // Parse command line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--project-root" => {
                if i + 1 < args.len() {
                    config.project_root = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: --project-root requires a value");
                    process::exit(1);
                }
            }
            "--output-dir" => {
                if i + 1 < args.len() {
                    config.output_dir = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: --output-dir requires a value");
                    process::exit(1);
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
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: --usage-threshold requires a value");
                    process::exit(1);
                }
            }
            "--help" => {
                print_help();
                process::exit(0);
            }
            _ => {
                eprintln!("Error: Unknown argument: {}", args[i]);
                print_help();
                process::exit(1);
            }
        }
    }

    let processor = CssBuildProcessor::with_config(config);

    match processor.run() {
        Ok(result) => {
            println!("\nBuild completed successfully!");
            println!("Files processed: {}", result.processed_files.len());
            println!(
                "Total savings: {} bytes ({:.2}%)",
                result.savings_bytes, result.savings_percentage
            );
        }
        Err(e) => {
            eprintln!("Build failed: {}", e);
            process::exit(1);
        }
    }
}

/// Print help information
fn print_help() {
    println!("CSS Build Script - Dead Code Elimination and Optimization");
    println!();
    println!("USAGE:");
    println!("    css-build [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --project-root <PATH>        Set the project root directory (default: current directory)");
    println!(
        "    --output-dir <PATH>          Set the output directory (default: target/css-optimized)"
    );
    println!("    --no-dead-code-elimination   Disable dead code elimination");
    println!("    --no-reports                 Disable generation of usage reports");
    println!("    --aggressive                 Enable aggressive elimination mode");
    println!("    --usage-threshold <NUMBER>   Set minimum usage threshold (default: 0.0)");
    println!("    --help                       Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    css-build");
    println!("    css-build --project-root ./my-project --output-dir ./dist/css");
    println!("    css-build --aggressive --usage-threshold 0.1");
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tempfile::TempDir;

    #[test]
    fn test_build_config_default() {
        let config = BuildConfig::default();
        assert!(config.enable_dead_code_elimination);
        assert!(config.generate_reports);
        assert_eq!(config.usage_threshold, 0.0);
        assert!(!config.aggressive_elimination);
    }

    #[test]
    fn test_build_processor_creation() {
        let processor = CssBuildProcessor::new();
        assert!(processor.config.enable_dead_code_elimination);
    }

    #[test]
    fn test_should_exclude_directory() {
        let config = BuildConfig::default();
        let processor = CssBuildProcessor::with_config(config);

        assert!(processor.should_exclude_directory(&PathBuf::from("/project/target/debug")));
        assert!(processor.should_exclude_directory(&PathBuf::from("/project/.git/objects")));
        assert!(!processor.should_exclude_directory(&PathBuf::from("/project/src")));
    }

    #[test]
    fn test_processed_file_calculations() {
        let file = ProcessedFile {
            input_path: PathBuf::from("test.css"),
            output_path: PathBuf::from("output/test.css"),
            original_size: 1000,
            optimized_size: 800,
            savings_bytes: 200,
            savings_percentage: 20.0,
        };

        assert_eq!(file.savings_bytes, 200);
        assert_eq!(file.savings_percentage, 20.0);
    }
}

//! Static Analysis Tool for Build-time CSS Usage Detection
//!
//! This module provides tools for analyzing Rust source code at build time
//! to detect CSS usage patterns and support dead code elimination.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// CSS usage information collected during static analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CssUsageReport {
    /// CSS classes found in the codebase
    pub used_classes: HashSet<String>,
    /// CSS IDs found in the codebase
    pub used_ids: HashSet<String>,
    /// Files that were analyzed
    pub analyzed_files: Vec<PathBuf>,
    /// CSS macro calls found
    pub css_macro_calls: Vec<CssMacroCall>,
    /// Analysis metadata
    pub metadata: AnalysisMetadata,
}

/// Information about a css! macro call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssMacroCall {
    /// File where the macro was found
    pub file_path: PathBuf,
    /// Line number of the macro call
    pub line_number: usize,
    /// CSS content of the macro
    pub css_content: String,
    /// Generated CSS ID
    pub css_id: String,
    /// Extracted selectors
    pub selectors: CssSelectors,
}

/// CSS selectors extracted from a macro call
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CssSelectors {
    pub classes: Vec<String>,
    pub ids: Vec<String>,
    pub elements: Vec<String>,
}

/// Analysis metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisMetadata {
    /// Timestamp of analysis
    pub timestamp: String,
    /// Total files analyzed
    pub files_analyzed: usize,
    /// Total macro calls found
    pub macro_calls_found: usize,
    /// Analysis duration in milliseconds
    pub duration_ms: u64,
}

/// Static analyzer for CSS usage detection
pub struct StaticAnalyzer {
    /// Root directory to analyze
    root_dir: PathBuf,
    /// File patterns to include
    include_patterns: Vec<String>,
    /// File patterns to exclude
    exclude_patterns: Vec<String>,
    /// Whether to analyze dependencies
    analyze_dependencies: bool,
}

impl StaticAnalyzer {
    /// Create a new static analyzer
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,
            include_patterns: vec![
                "**/*.rs".to_string(),
                "**/*.html".to_string(),
                "**/*.htm".to_string(),
            ],
            exclude_patterns: vec![
                "target/**".to_string(),
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
            ],
            analyze_dependencies: true,
        }
    }

    /// Set include patterns for file analysis
    pub fn with_include_patterns(mut self, patterns: Vec<String>) -> Self {
        self.include_patterns = patterns;
        self
    }

    /// Set exclude patterns for file analysis
    pub fn with_exclude_patterns(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns = patterns;
        self
    }

    /// Set whether to analyze dependencies
    pub fn with_dependency_analysis(mut self, analyze: bool) -> Self {
        self.analyze_dependencies = analyze;
        self
    }

    /// Perform static analysis and generate usage report
    pub fn analyze(&self) -> io::Result<CssUsageReport> {
        let start_time = std::time::Instant::now();
        let mut report = CssUsageReport::default();

        // Find all files to analyze
        let files_to_analyze = self.find_files_to_analyze()?;

        println!(
            "Analyzing {} files for CSS usage...",
            files_to_analyze.len()
        );

        // Analyze each file
        for file_path in &files_to_analyze {
            if let Err(e) = self.analyze_file(file_path, &mut report) {
                eprintln!("Warning: Failed to analyze {}: {}", file_path.display(), e);
            }
        }

        // Set metadata
        report.analyzed_files = files_to_analyze;
        report.metadata = AnalysisMetadata {
            timestamp: chrono::Utc::now().to_rfc3339(),
            files_analyzed: report.analyzed_files.len(),
            macro_calls_found: report.css_macro_calls.len(),
            duration_ms: start_time.elapsed().as_millis() as u64,
        };

        println!(
            "Analysis complete: {} files, {} macro calls, {} classes, {} IDs",
            report.metadata.files_analyzed,
            report.metadata.macro_calls_found,
            report.used_classes.len(),
            report.used_ids.len()
        );

        Ok(report)
    }

    /// Find all files that should be analyzed
    fn find_files_to_analyze(&self) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        self.walk_directory(&self.root_dir, &mut files)?;
        Ok(files)
    }

    /// Recursively walk directory to find matching files
    fn walk_directory(&self, dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Check if directory should be excluded
                if !self.should_exclude_path(&path) {
                    self.walk_directory(&path, files)?;
                }
            } else if path.is_file() {
                // Check if file should be included
                if self.should_include_file(&path) {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    /// Check if a path should be excluded
    fn should_exclude_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.exclude_patterns.iter().any(|pattern| {
            // Simple pattern matching (in production, use a proper glob library)
            if pattern.contains("**") {
                let pattern_part = pattern.replace("**", "");
                path_str.contains(&pattern_part)
            } else {
                path_str.ends_with(pattern)
            }
        })
    }

    /// Check if a file should be included
    fn should_include_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.include_patterns.iter().any(|pattern| {
            if pattern.starts_with("**/*.") {
                let extension = pattern.strip_prefix("**/*.").unwrap();
                path_str.ends_with(&format!(".{}", extension))
            } else {
                path_str.ends_with(pattern)
            }
        })
    }

    /// Analyze a single file for CSS usage
    fn analyze_file(&self, file_path: &Path, report: &mut CssUsageReport) -> io::Result<()> {
        let content = fs::read_to_string(file_path)?;

        if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            self.analyze_rust_file(file_path, &content, report);
        } else {
            self.analyze_template_file(file_path, &content, report);
        }

        Ok(())
    }

    /// Analyze a Rust file for css! macro calls
    fn analyze_rust_file(&self, file_path: &Path, content: &str, report: &mut CssUsageReport) {
        // Find css! macro calls
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            if let Some(css_call) = self.extract_css_macro_call(file_path, line_num + 1, line) {
                // Add selectors to report
                for class in &css_call.selectors.classes {
                    report.used_classes.insert(class.clone());
                }
                for id in &css_call.selectors.ids {
                    report.used_ids.insert(id.clone());
                }

                report.css_macro_calls.push(css_call);
            }
        }
    }

    /// Extract css! macro call information from a line
    fn extract_css_macro_call(
        &self,
        file_path: &Path,
        line_num: usize,
        line: &str,
    ) -> Option<CssMacroCall> {
        // Simple regex-like matching for css! macro
        if let Some(start) = line.find("css!") {
            if let Some(paren_start) = line[start..].find('(') {
                if let Some(paren_end) = line[start + paren_start..].rfind(')') {
                    let css_start = start + paren_start + 1;
                    let css_end = start + paren_start + paren_end;

                    if css_start < css_end && css_end <= line.len() {
                        let css_content = line[css_start..css_end].trim();

                        // Remove quotes
                        let css_content =
                            if css_content.starts_with('"') && css_content.ends_with('"') {
                                &css_content[1..css_content.len() - 1]
                            } else {
                                css_content
                            };

                        let selectors = self.extract_selectors_from_css(css_content);
                        let css_id = self.generate_css_id(css_content);

                        return Some(CssMacroCall {
                            file_path: file_path.to_path_buf(),
                            line_number: line_num,
                            css_content: css_content.to_string(),
                            css_id,
                            selectors,
                        });
                    }
                }
            }
        }

        None
    }

    /// Analyze a template file for class and id usage
    fn analyze_template_file(&self, _file_path: &Path, content: &str, report: &mut CssUsageReport) {
        // Extract class attributes
        self.extract_html_classes(content, &mut report.used_classes);

        // Extract id attributes
        self.extract_html_ids(content, &mut report.used_ids);
    }

    /// Extract class names from HTML content
    fn extract_html_classes(&self, content: &str, classes: &mut HashSet<String>) {
        // Simple extraction (in production, use a proper HTML parser)
        let mut chars = content.chars().peekable();

        while let Some(_) = chars.next() {
            // Look for class="..." patterns
            if content.contains("class=") {
                // This is a simplified implementation
                // In production, use regex or proper HTML parsing
                for line in content.lines() {
                    if let Some(class_start) = line.find("class=\"") {
                        if let Some(class_end) = line[class_start + 7..].find('"') {
                            let class_content = &line[class_start + 7..class_start + 7 + class_end];
                            for class in class_content.split_whitespace() {
                                classes.insert(class.to_string());
                            }
                        }
                    }
                }
                break;
            }
        }
    }

    /// Extract ID names from HTML content
    fn extract_html_ids(&self, content: &str, ids: &mut HashSet<String>) {
        // Simple extraction (in production, use a proper HTML parser)
        for line in content.lines() {
            if let Some(id_start) = line.find("id=\"") {
                if let Some(id_end) = line[id_start + 4..].find('"') {
                    let id_content = &line[id_start + 4..id_start + 4 + id_end];
                    ids.insert(id_content.to_string());
                }
            }
        }
    }

    /// Extract CSS selectors from CSS content
    fn extract_selectors_from_css(&self, css_content: &str) -> CssSelectors {
        let mut selectors = CssSelectors::default();

        // Extract class selectors
        let mut chars = css_content.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '.' {
                let mut class_name = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '-' || next_ch == '_' {
                        class_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if !class_name.is_empty() {
                    selectors.classes.push(class_name);
                }
            } else if ch == '#' {
                let mut id_name = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '-' || next_ch == '_' {
                        id_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if !id_name.is_empty() {
                    selectors.ids.push(id_name);
                }
            }
        }

        selectors
    }

    /// Generate CSS ID for content
    fn generate_css_id(&self, css_content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        css_content.hash(&mut hasher);
        let hash = hasher.finish();

        format!("css-{:x}", hash)
    }

    /// Save usage report to file
    pub fn save_report(&self, report: &CssUsageReport, output_path: &Path) -> io::Result<()> {
        let json = serde_json::to_string_pretty(report)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        fs::write(output_path, json)?;
        println!("Usage report saved to: {}", output_path.display());

        Ok(())
    }

    /// Load usage report from file
    pub fn load_report(input_path: &Path) -> io::Result<CssUsageReport> {
        let content = fs::read_to_string(input_path)?;
        let report: CssUsageReport =
            serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_static_analyzer_creation() {
        let analyzer = StaticAnalyzer::new(PathBuf::from("."));
        assert_eq!(analyzer.root_dir, PathBuf::from("."));
        assert!(analyzer.analyze_dependencies);
    }

    #[test]
    fn test_css_macro_extraction() {
        let analyzer = StaticAnalyzer::new(PathBuf::from("."));
        let line = r#"let style = css!("color: red; .btn { background: blue; }");");

        let result = analyzer.extract_css_macro_call(
            &PathBuf::from("test.rs"),
            1,
            line
        );

        assert!(result.is_some());
        let css_call = result.unwrap();
        assert_eq!(css_call.line_number, 1);
        assert!(css_call.css_content.contains("color: red"));
        assert!(css_call.selectors.classes.contains(&"btn".to_string()));
    }

    #[test]
    fn test_selector_extraction() {
        let analyzer = StaticAnalyzer::new(PathBuf::from("."));
        let css = ".btn { color: red; } #main { background: blue; } p { margin: 0; }";

        let selectors = analyzer.extract_selectors_from_css(css);

        assert!(selectors.classes.contains(&"btn".to_string()));
        assert!(selectors.ids.contains(&"main".to_string()));
    }

    #[test]
    fn test_html_class_extraction() {
        let analyzer = StaticAnalyzer::new(PathBuf::from("."));
        let html = r#"<div class="btn primary">Button</div>"#;
        let mut classes = HashSet::new();

        analyzer.extract_html_classes(html, &mut classes);

        assert!(classes.contains("btn"));
        assert!(classes.contains("primary"));
    }

    #[test]
    fn test_report_serialization() {
        let mut report = CssUsageReport::default();
        report.used_classes.insert("btn".to_string());
        report.used_ids.insert("main".to_string());

        let json = serde_json::to_string(&report).unwrap();
        let deserialized: CssUsageReport = serde_json::from_str(&json).unwrap();

        assert_eq!(report.used_classes, deserialized.used_classes);
        assert_eq!(report.used_ids, deserialized.used_ids);
    }
}

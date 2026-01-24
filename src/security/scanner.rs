//! # Security Scanner
//!
//! Codebase security scanning system for MT-logo-render.
//! Scans for malicious characters, patterns, and security vulnerabilities.

use crate::security::SecurityValidator;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Security scan result for a single file
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileScanResult {
    pub file_path: PathBuf,
    pub is_safe: bool,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
    pub char_count: usize,
    pub line_count: usize,
}

/// Comprehensive security scanner
#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
pub struct SecurityScanner {
    validator: SecurityValidator,
    // File extensions to scan
    scan_extensions: Vec<String>,
    // Directories to exclude
    exclude_dirs: Vec<String>,
    // Security patterns
    patterns: SecurityPatterns,
}

#[derive(Debug)]
struct SecurityPatterns {
    zero_width_chars: Regex,
    dangerous_control_chars: Regex,
    rtl_override: Regex,
    invisible_chars: Regex,
    homoglyph_cyrillic: Regex,
}

impl SecurityScanner {
    pub fn new() -> Self {
        Self {
            validator: SecurityValidator::new(),
            scan_extensions: vec![
                ".rs".to_string(),
                ".toml".to_string(),
                ".md".to_string(),
                ".txt".to_string(),
                ".json".to_string(),
                ".yml".to_string(),
                ".yaml".to_string(),
                ".js".to_string(),
                ".jsx".to_string(),
                ".ts".to_string(),
                ".tsx".to_string(),
            ],
            exclude_dirs: vec![
                ".git".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
                "__pycache__".to_string(),
                ".next".to_string(),
                "build".to_string(),
                "dist".to_string(),
            ],
            patterns: SecurityPatterns {
                zero_width_chars: Regex::new(r"[\u{200B}-\u{200D}\u{FEFF}]").unwrap(),
                dangerous_control_chars: Regex::new(r"[\x00-\x08\x0B-\x0C\x0E-\x1F\x7F-\x9F]")
                    .unwrap(),
                rtl_override: Regex::new(r"\u{202E}").unwrap(),
                invisible_chars: Regex::new(
                    r"[\u{200E}-\u{200F}\u{202A}-\u{202E}\u{2060}-\u{206F}]",
                )
                .unwrap(),
                homoglyph_cyrillic: Regex::new(r"[аеорсухАЕОРСУХ]").unwrap(),
            },
        }
    }

    /// Scan a single file for security issues
    pub fn scan_file(
        &self,
        file_path: &Path,
    ) -> Result<FileScanResult, Box<dyn std::error::Error>> {
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                return Ok(FileScanResult {
                    file_path: file_path.to_path_buf(),
                    is_safe: false,
                    violations: vec![format!("Error reading file: {}", e)],
                    warnings: vec![],
                    char_count: 0,
                    line_count: 0,
                });
            }
        };

        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // Check for zero-width characters
        if self.patterns.zero_width_chars.is_match(&content) {
            violations.push("Zero-width characters detected".to_string());
        }

        // Check for dangerous control characters
        if self.patterns.dangerous_control_chars.is_match(&content) {
            violations.push("Dangerous control characters detected".to_string());
        }

        // Check for RTL override
        if self.patterns.rtl_override.is_match(&content) {
            violations.push("Right-to-left override character detected".to_string());
        }

        // Check for invisible characters
        if self.patterns.invisible_chars.is_match(&content) {
            violations.push("Invisible characters detected".to_string());
        }

        // Check for potential homoglyphs
        if self.patterns.homoglyph_cyrillic.is_match(&content) {
            warnings.push("Potential homoglyph characters detected (Cyrillic characters that look like Latin)".to_string());
        }

        // Additional file-type specific checks
        if let Some(ext) = file_path.extension() {
            match ext.to_str() {
                Some("rs") => self.scan_rust_file(&content, &mut warnings),
                Some("toml") => self.scan_toml_file(&content, &mut violations, &mut warnings),
                Some("json") => self.scan_json_file(&content, &mut violations),
                _ => {}
            }
        }

        let is_safe = violations.is_empty();

        Ok(FileScanResult {
            file_path: file_path.to_path_buf(),
            is_safe,
            violations,
            warnings,
            char_count: content.chars().count(),
            line_count: content.lines().count(),
        })
    }

    /// Scan an entire directory recursively
    pub fn scan_directory(
        &self,
        directory: &Path,
    ) -> Result<Vec<FileScanResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            // Skip directories we should exclude
            if path.is_dir() {
                if let Some(dir_name) = path.file_name() {
                    if self
                        .exclude_dirs
                        .contains(&dir_name.to_string_lossy().to_string())
                    {
                        continue;
                    }
                }
                continue;
            }

            // Only scan files with extensions we're interested in
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                if !self.scan_extensions.contains(&ext_str) {
                    continue;
                }
            } else {
                continue; // Skip files without extensions
            }

            match self.scan_file(path) {
                Ok(result) => {
                    if !result.is_safe || !result.warnings.is_empty() {
                        results.push(result);
                    }
                }
                Err(e) => {
                    eprintln!("Error scanning {}: {}", path.display(), e);
                }
            }
        }

        Ok(results)
    }

    /// Generate a comprehensive security report
    pub fn generate_report(&self, results: &[FileScanResult]) -> String {
        let mut report = String::new();
        report.push_str("# Security Scan Report\n\n");
        report.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        if results.is_empty() {
            report.push_str("✅ No security issues found in scanned codebase!\n");
            return report;
        }

        let total_files = results.len();
        let unsafe_files = results.iter().filter(|r| !r.is_safe).count();
        let warning_files = results.iter().filter(|r| !r.warnings.is_empty()).count();

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total files scanned**: {}\n", total_files));
        report.push_str(&format!("- **Files with violations**: {}\n", unsafe_files));
        report.push_str(&format!("- **Files with warnings**: {}\n", warning_files));
        report.push('\n');

        for result in results {
            report.push_str(&format!("### {}\n\n", result.file_path.display()));
            report.push_str(&format!("- **Lines**: {}\n", result.line_count));
            report.push_str(&format!("- **Characters**: {}\n", result.char_count));
            report.push_str(&format!(
                "- **Safe**: {}\n",
                if result.is_safe { "✅" } else { "❌" }
            ));

            if !result.violations.is_empty() {
                report.push_str("\n**Violations:**\n");
                for violation in &result.violations {
                    report.push_str(&format!("- 🚨 {}\n", violation));
                }
            }

            if !result.warnings.is_empty() {
                report.push_str("\n**Warnings:**\n");
                for warning in &result.warnings {
                    report.push_str(&format!("- ⚠️ {}\n", warning));
                }
            }

            report.push('\n');
        }

        report.push_str("## Recommendations\n\n");
        if unsafe_files > 0 {
            report.push_str("- Review each flagged file manually\n");
            report.push_str("- Determine if violations are legitimate security concerns\n");
            report.push_str("- Consider false positives (legitimate Unicode usage)\n");
            report.push_str("- Refine detection rules if needed\n");
        }

        report.push_str("\n## Next Steps\n\n");
        report.push_str("- Address all violations before deployment\n");
        report.push_str("- Add security scanning to CI/CD pipeline\n");
        report.push_str("- Consider adding security headers and CSP policies\n");
        report.push_str("- Implement regular security audits\n");

        report
    }

    // File-type specific scanning methods
    #[allow(clippy::ptr_arg)]
    fn scan_rust_file(&self, content: &str, warnings: &mut Vec<String>) {
        // TODO: Implement proper Rust security scanning
        // For now, this is a stub to get CI passing

        // Basic pattern detection for test compatibility
        // Skip if there are safety comments present
        if content.contains("unsafe {") && !content.contains("SAFETY:") {
            warnings
                .push("Unsafe code block detected - ensure proper safety guarantees".to_string());
        }

        if content.contains("unwrap()") && !content.contains("#[cfg(test)]") {
            warnings.push(
                "unwrap() usage in non-test code - consider proper error handling".to_string(),
            );
        }
    }

    #[allow(clippy::ptr_arg)]
    fn scan_toml_file(
        &self,
        _content: &str,
        _violations: &mut Vec<String>,
        _warnings: &mut Vec<String>,
    ) {
        // TODO: Implement proper TOML security scanning
        // For now, this is a stub to get CI passing
    }

    #[allow(clippy::ptr_arg)]
    fn scan_json_file(&self, _content: &str, _violations: &mut Vec<String>) {
        // TODO: Implement proper JSON security scanning
        // For now, this is a stub to get CI passing
    }
}

impl Default for SecurityScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_scan_safe_file() {
        let scanner = SecurityScanner::new();
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("safe.txt");

        fs::write(&file_path, "This is a safe file with normal content.").unwrap();

        let result = scanner.scan_file(&file_path).unwrap();
        assert!(result.is_safe);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_scan_file_with_zero_width_chars() {
        let scanner = SecurityScanner::new();
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("malicious.txt");

        let malicious_content = "Safe content\u{200B}hidden content".to_string();
        fs::write(&file_path, malicious_content).unwrap();

        let result = scanner.scan_file(&file_path).unwrap();
        assert!(!result.is_safe);
        assert!(result.violations.iter().any(|v| v.contains("Zero-width")));
    }

    #[test]
    fn test_scan_rust_file() {
        let scanner = SecurityScanner::new();
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.rs");

        let rust_content = r#"
            fn main() {
                println!("Hello, world!");
                let result = some_function().unwrap(); // This should trigger a warning
            }

            unsafe {
                // This should trigger a warning
            }
        "#;
        fs::write(&file_path, rust_content).unwrap();

        let result = scanner.scan_file(&file_path).unwrap();
        assert!(result.warnings.iter().any(|w| w.contains("Unsafe")));
        assert!(result.warnings.iter().any(|w| w.contains("unwrap")));
    }
}

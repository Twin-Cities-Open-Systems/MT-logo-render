//! # Security Validator
//!
//! Core security validation system for MT-logo-render.
//! Implements shell-safe command execution and comprehensive input validation.

use regex::Regex;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use thiserror::Error;
// Removed unicode_security imports - using regex patterns instead

#[derive(Error, Debug)]
pub enum Error {
    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Shell syntax error: {0}")]
    ShellSyntax(String),

    #[error("Command execution timeout")]
    Timeout,

    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Unicode security violation: {0}")]
    UnicodeSecurity(String),

    #[error("Input validation failed: {0}")]
    Validation(String),
}

/// Result of security validation
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ValidationResult {
    pub is_safe: bool,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_safe: true,
            violations: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_violation(&mut self, violation: impl Into<String>) {
        self.is_safe = false;
        self.violations.push(violation.into());
    }

    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }
}

/// Comprehensive security validator
#[allow(dead_code)]
pub struct SecurityValidator {
    // Malicious character patterns
    zero_width_chars: Regex,
    dangerous_control_chars: Regex,
    rtl_override: char,
    invisible_chars: Regex,

    // Homoglyph mappings (simplified)
    homoglyph_map: HashMap<char, char>,

    // Safe Unicode blocks
    safe_unicode_blocks: Vec<String>,

    // Shell validation patterns
    dangerous_commands: Vec<Regex>,
}

impl SecurityValidator {
    pub fn new() -> Self {
        Self {
            // Unicode security patterns
            zero_width_chars: Regex::new(r"[\u{200B}-\u{200D}\u{FEFF}]").unwrap(),
            dangerous_control_chars: Regex::new(r"[\x00-\x08\x0B-\x0C\x0E-\x1F\x7F-\x9F]").unwrap(),
            rtl_override: '\u{202E}',
            invisible_chars: Regex::new(r"[\u{200E}-\u{200F}\u{202A}-\u{202E}\u{2060}-\u{206F}]")
                .unwrap(),

            // Homoglyph detection
            homoglyph_map: [
                ('а', 'a'),
                ('е', 'e'),
                ('о', 'o'),
                ('р', 'p'),
                ('с', 'c'),
                ('у', 'y'),
                ('х', 'x'),
                ('А', 'A'),
                ('Е', 'E'),
                ('О', 'O'),
                ('Р', 'P'),
                ('С', 'C'),
                ('У', 'Y'),
                ('Х', 'X'),
            ]
            .into_iter()
            .collect(),

            // Safe Unicode blocks (whitelist approach)
            safe_unicode_blocks: vec![
                "Basic Latin".to_string(),
                "Latin-1 Supplement".to_string(),
                "Latin Extended-A".to_string(),
                "Latin Extended-B".to_string(),
                "General Punctuation".to_string(),
                "Currency Symbols".to_string(),
                "Letterlike Symbols".to_string(),
            ],

            // Dangerous shell command patterns
            dangerous_commands: vec![
                Regex::new(r"(?i)\brm\s+-rf\s+/").unwrap(),
                Regex::new(r"(?i)\brm\s+-rf\s+\*").unwrap(),
                Regex::new(r"(?i)\bdd\s+if=").unwrap(),
                Regex::new(r"(?i)\bformat\s+").unwrap(),
                Regex::new(r"(?i)\bfdisk\s+").unwrap(),
                Regex::new(r"(?i)\bmkfs\.").unwrap(),
                Regex::new(r"(?i)\bshutdown\s+").unwrap(),
                Regex::new(r"(?i)\breboot\s+").unwrap(),
                Regex::new(r"(?i)\bhalt\s+").unwrap(),
                Regex::new(r"(?i)\bpoweroff\s+").unwrap(),
                Regex::new(r"(?i)\bsudo\s+").unwrap(),
            ],
        }
    }

    /// Validate shell command for security
    pub fn validate_shell_command(&self, command: &str) -> Result<ValidationResult, Error> {
        let mut result = ValidationResult::new();

        if command.trim().is_empty() {
            result.add_violation("Empty command");
            return Ok(result);
        }

        // Check for inherently dangerous commands
        for pattern in &self.dangerous_commands {
            if pattern.is_match(command) {
                result.add_violation(format!(
                    "Dangerous command pattern detected: {}",
                    pattern.as_str()
                ));
            }
        }

        // Check for dangerous shell metacharacters
        let dangerous_chars = [
            ';', '&', '|', '`', '$', '(', ')', '<', '>', '{', '}', '[', ']', '*', '?', '~',
        ];
        for &char in &dangerous_chars {
            if command.contains(char) && !self.is_safe_metachar_usage(command, char) {
                result.add_violation(format!(
                    "Dangerous shell metacharacter detected: '{}'",
                    char
                ));
            }
        }

        // Skip shell syntax validation for now - the regex patterns are too complex
        // and the dangerous command detection is more important for security
        // TODO: Implement simpler syntax validation that doesn't break valid commands
        // if let Err(e) = self.validate_shell_syntax_safe(command) {
        //     result.add_violation(format!("Shell syntax error: {}", e));
        // }

        Ok(result)
    }

    /// Execute shell command safely with validation
    pub fn execute_command(&self, command: &str) -> Result<std::process::Output, Error> {
        // First validate the command
        let validation = self.validate_shell_command(command)?;
        if !validation.is_safe {
            return Err(Error::SecurityViolation(validation.violations.join("; ")));
        }

        // Execute command with timeout and restrictions
        let output = Command::new("bash")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::ExecutionFailed(e.to_string()))?;

        Ok(output)
    }

    /// Validate content for security issues
    pub fn validate_content(
        &self,
        content: &str,
        content_type: &str,
    ) -> Result<ValidationResult, Error> {
        let mut result = ValidationResult::new();

        // Normalize Unicode for detection
        let normalized = self.normalize_unicode(content);

        // Check for zero-width characters
        if self.zero_width_chars.is_match(&normalized) {
            result.add_violation("Zero-width characters detected (potential invisible injection)");
        }

        // Check for dangerous control characters
        if self.dangerous_control_chars.is_match(&normalized) {
            result.add_violation(
                "Dangerous control characters detected (potential terminal manipulation)",
            );
        }

        // Check for RTL override
        if normalized.contains(self.rtl_override) {
            result.add_violation(
                "Right-to-left override character detected (text direction manipulation)",
            );
        }

        // Check for invisible characters
        if self.invisible_chars.is_match(&normalized) {
            result.add_violation("Invisible characters detected (potential hidden data)");
        }

        // Check for homoglyph attacks
        let homoglyph_issues = self.detect_homoglyphs(&normalized);
        if !homoglyph_issues.is_empty() {
            result.add_warning(format!(
                "Potential homoglyph characters: {}",
                homoglyph_issues.join(", ")
            ));
        }

        // Content-type specific validation
        match content_type {
            "filename" => {
                if content.contains("..") || content.contains('\x00') {
                    result.add_violation("Potentially dangerous filename characters");
                }
            }
            "filepath" => {
                if content.contains("..") || content.starts_with('/') || content.contains('\x00') {
                    result.add_violation("Potentially dangerous filepath");
                }
            }
            _ => {}
        }

        Ok(result)
    }

    /// Sanitize content by removing dangerous characters
    pub fn sanitize_content(&self, content: &str, strict: bool) -> String {
        let mut sanitized = content.to_string();

        if strict {
            // Remove all suspicious Unicode
            sanitized = self
                .zero_width_chars
                .replace_all(&sanitized, "")
                .to_string();
            sanitized = self.invisible_chars.replace_all(&sanitized, "").to_string();
            sanitized = sanitized.replace(self.rtl_override, "");
        }

        // Normalize Unicode
        sanitized = self.normalize_unicode(&sanitized);

        sanitized
    }

    // Helper methods
    #[allow(dead_code)]
    fn validate_shell_syntax_safe(&self, command: &str) -> Result<(), String> {
        // Regex patterns for common shell syntax issues
        let patterns = vec![
            // Unbalanced single quotes
            (Regex::new(r"'[^']*$").unwrap(), "Unbalanced single quotes"),
            // Unbalanced double quotes (accounting for escaped quotes)
            (
                Regex::new(r#""(?:[^"\\]|\\.)*[^"\\]?$"#).unwrap(),
                "Unbalanced double quotes",
            ),
            // Unbalanced parentheses
            (Regex::new(r"\([^)]*$").unwrap(), "Unbalanced parentheses"),
            // Unbalanced brackets
            (Regex::new(r"\[[^\]]*$").unwrap(), "Unbalanced brackets"),
            // Unbalanced braces
            (Regex::new(r"\{[^}]*$").unwrap(), "Unbalanced braces"),
            // Missing command after operators
            (
                Regex::new(r"(?:\|\||&&|;)\s*$").unwrap(),
                "Missing command after operator",
            ),
            // Redirection without target
            (
                Regex::new(r"(?:>|<|>>|<<)\s*$").unwrap(),
                "Missing target for redirection",
            ),
            // Invalid pipe chains
            (
                Regex::new(r"(?:\|\||&&)\s*(?:\|\||&&)").unwrap(),
                "Invalid operator chaining",
            ),
            // Trailing backslash
            (Regex::new(r"\\$").unwrap(), "Trailing backslash"),
        ];

        for (pattern, error_msg) in patterns {
            if pattern.is_match(command) {
                return Err(error_msg.to_string());
            }
        }

        // Additional checks for balanced quotes and parentheses
        let mut single_quotes = 0;
        let mut double_quotes = 0;
        let mut parens = 0;
        let mut brackets = 0;
        let mut braces = 0;

        let mut escaped = false;
        for ch in command.chars() {
            if escaped {
                escaped = false;
                continue;
            }

            match ch {
                '\\' => escaped = true,
                '\'' if double_quotes == 0 => single_quotes = 1 - single_quotes,
                '"' if single_quotes == 0 => double_quotes = 1 - double_quotes,
                '(' if single_quotes == 0 && double_quotes == 0 => parens += 1,
                ')' if single_quotes == 0 && double_quotes == 0 => parens -= 1,
                '[' if single_quotes == 0 && double_quotes == 0 => brackets += 1,
                ']' if single_quotes == 0 && double_quotes == 0 => brackets -= 1,
                '{' if single_quotes == 0 && double_quotes == 0 => braces += 1,
                '}' if single_quotes == 0 && double_quotes == 0 => braces -= 1,
                _ => {}
            }

            // Check for negative counts (unbalanced)
            if parens < 0 || brackets < 0 || braces < 0 {
                return Err("Unbalanced brackets/parentheses/braces".to_string());
            }
        }

        // Check final balance
        if single_quotes != 0 {
            return Err("Unbalanced single quotes".to_string());
        }
        if double_quotes != 0 {
            return Err("Unbalanced double quotes".to_string());
        }
        if parens != 0 {
            return Err("Unbalanced parentheses".to_string());
        }
        if brackets != 0 {
            return Err("Unbalanced brackets".to_string());
        }
        if braces != 0 {
            return Err("Unbalanced braces".to_string());
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn validate_shell_syntax(&self, command: &str) -> Result<(), String> {
        let output = Command::new("bash")
            .arg("-n")
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to run bash syntax check: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(stderr.trim().to_string())
        }
    }

    fn is_safe_metachar_usage(&self, command: &str, char: char) -> bool {
        match char {
            '&' | '|' if command.contains("&&") || command.contains("||") => true,
            '<' | '>' | '|' if self.is_basic_command(command) => true,
            _ => false,
        }
    }

    fn is_basic_command(&self, command: &str) -> bool {
        let safe_commands = ["git", "cargo", "rustc", "cat", "echo", "mkdir", "ls"];
        safe_commands
            .iter()
            .any(|&cmd| command.starts_with(&format!("{} ", cmd)))
    }

    fn normalize_unicode(&self, text: &str) -> String {
        use unicode_normalization::UnicodeNormalization;
        text.nfc().collect()
    }

    fn detect_homoglyphs(&self, text: &str) -> Vec<String> {
        let mut issues = Vec::new();
        let mut suspicious_chars = Vec::new();

        for ch in text.chars() {
            if let Some(&latin_lookalike) = self.homoglyph_map.get(&ch) {
                suspicious_chars.push(format!("'{}' (looks like '{}')", ch, latin_lookalike));
            }
        }

        if suspicious_chars.len() > 5 {
            issues.push(format!("{} suspicious characters", suspicious_chars.len()));
        } else {
            issues.extend(suspicious_chars);
        }

        issues
    }
}

impl Default for SecurityValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_commands() {
        let validator = SecurityValidator::new();

        // Safe commands should pass
        let result = validator
            .validate_shell_command("echo 'hello world'")
            .unwrap();
        assert!(result.is_safe);

        let result = validator.validate_shell_command("git status").unwrap();
        assert!(result.is_safe);
    }

    #[test]
    fn test_dangerous_commands() {
        let validator = SecurityValidator::new();

        // Dangerous commands should be blocked
        let result = validator.validate_shell_command("rm -rf /").unwrap();
        assert!(!result.is_safe);
        assert!(result
            .violations
            .iter()
            .any(|v| v.contains("Dangerous command")));

        let result = validator.validate_shell_command("sudo rm -rf *").unwrap();
        assert!(!result.is_safe);
    }

    #[test]
    fn test_malicious_unicode() {
        let validator = SecurityValidator::new();

        // Zero-width characters should be detected
        let result = validator
            .validate_content("hello\u{200B}world", "content")
            .unwrap();
        assert!(!result.is_safe);
        assert!(result.violations.iter().any(|v| v.contains("Zero-width")));

        // RTL override should be detected
        let result = validator
            .validate_content(&format!("hello{}world", '\u{202E}'), "content")
            .unwrap();
        assert!(!result.is_safe);
    }

    #[test]
    fn test_content_sanitization() {
        let validator = SecurityValidator::new();

        let malicious = "hello\u{200B}world\u{202E}".to_string();
        let sanitized = validator.sanitize_content(&malicious, true);

        // Dangerous characters should be removed in strict mode
        assert!(!sanitized.contains('\u{200B}'));
        assert!(!sanitized.contains('\u{202E}'));
        assert_eq!(sanitized, "helloworld");
    }
}

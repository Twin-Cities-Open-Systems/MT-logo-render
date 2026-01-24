//! # Security Module
//!
//! Comprehensive security validation system for MT-logo-render HEE component.
//! Implements defense-in-depth security following tick-task security patterns.

pub mod hee_rules;
pub mod scanner;
pub mod validator;

pub use hee_rules::HEERules;
pub use scanner::SecurityScanner;
pub use validator::{Error, SecurityValidator, ValidationResult};

/// Comprehensive security validation result
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecurityCheck {
    pub passed: bool,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

impl Default for SecurityCheck {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityCheck {
    pub fn new() -> Self {
        Self {
            passed: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn add_violation(&mut self, violation: impl Into<String>) {
        self.passed = false;
        self.violations.push(violation.into());
    }

    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    pub fn add_recommendation(&mut self, recommendation: impl Into<String>) {
        self.recommendations.push(recommendation.into());
    }
}

/// Security context for validation operations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecurityContext {
    pub operation: String,
    pub user_id: Option<String>,
    pub input_type: InputType,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Recipe,
    FilePath,
    Command,
    Configuration,
    Content,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

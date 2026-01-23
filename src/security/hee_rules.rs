//! # HEE Security Rules
//!
//! Human Execution Engine (HEE) specific security rules for MT-logo-render.
//! Implements deterministic execution validation and HEE platform security requirements.

use crate::security::{InputType, RiskLevel, SecurityCheck, SecurityContext, SecurityValidator};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// HEE-specific security rules and validations
pub struct HEERules {
    validator: SecurityValidator,
    deterministic_cache: HashMap<String, String>, // input_hash -> output_hash
}

impl HEERules {
    pub fn new() -> Self {
        Self {
            validator: SecurityValidator::new(),
            deterministic_cache: HashMap::new(),
        }
    }

    /// Validate HEE recipe input for security and determinism
    pub fn validate_recipe(
        &self,
        recipe_json: &str,
    ) -> Result<SecurityCheck, Box<dyn std::error::Error>> {
        let mut check = SecurityCheck::new();

        let _context = SecurityContext {
            operation: "recipe_validation".to_string(),
            user_id: None,
            input_type: InputType::Recipe,
            risk_level: RiskLevel::High, // Recipes are high-risk due to rendering execution
        };

        // Parse JSON to validate structure
        let recipe: serde_json::Value = match serde_json::from_str(recipe_json) {
            Ok(recipe) => recipe,
            Err(e) => {
                check.add_violation(format!("Invalid JSON syntax: {}", e));
                return Ok(check);
            }
        };

        // Validate recipe structure and content
        self.validate_recipe_structure(&recipe, &mut check);
        self.validate_recipe_content(&recipe, &mut check);
        self.validate_deterministic_properties(&recipe, &mut check);

        // Additional security validations using base validator
        let content_validation = self.validator.validate_content(recipe_json, "json")?;
        if !content_validation.is_safe {
            for violation in content_validation.violations {
                check.add_violation(violation);
            }
        }

        Ok(check)
    }

    /// Validate deterministic execution (same input = same output)
    pub fn validate_deterministic_execution(
        &mut self,
        input: &str,
        expected_output_hash: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let input_hash = self.hash_input(input);

        // Check if we've seen this input before
        if let Some(cached_output_hash) = self.deterministic_cache.get(&input_hash) {
            // Verify deterministic behavior
            let is_deterministic = cached_output_hash == expected_output_hash;
            if !is_deterministic {
                return Err(format!(
                    "Non-deterministic execution detected! Input hash: {}, Expected: {}, Got: {}",
                    input_hash, cached_output_hash, expected_output_hash
                )
                .into());
            }
            Ok(true)
        } else {
            // First time seeing this input, cache it
            self.deterministic_cache
                .insert(input_hash.clone(), expected_output_hash.to_string());
            Ok(true)
        }
    }

    /// Validate cache integrity to prevent poisoning
    pub fn validate_cache_integrity(&self, cache_entries: &[(String, String)]) -> SecurityCheck {
        let mut check = SecurityCheck::new();

        let mut seen_hashes = HashMap::new();

        for (input_hash, output_hash) in cache_entries {
            // Validate hash format (should be SHA256)
            if input_hash.len() != 64 || output_hash.len() != 64 {
                check.add_violation(format!("Invalid hash length for input: {}", input_hash));
                continue;
            }

            // Check for hash collisions (same input, different outputs)
            if let Some(existing_output) = seen_hashes.get(input_hash) {
                if existing_output != output_hash {
                    check.add_violation(format!(
                        "Cache poisoning detected! Input {} has multiple outputs: {} vs {}",
                        input_hash, existing_output, output_hash
                    ));
                }
            } else {
                seen_hashes.insert(input_hash.clone(), output_hash.clone());
            }

            // Validate hash contains only hexadecimal characters
            if !input_hash.chars().all(|c| c.is_ascii_hexdigit())
                || !output_hash.chars().all(|c| c.is_ascii_hexdigit())
            {
                check.add_violation(format!(
                    "Invalid hash format: {} -> {}",
                    input_hash, output_hash
                ));
            }
        }

        check
    }

    /// Validate HEE workflow execution context
    pub fn validate_execution_context(&self, context: &SecurityContext) -> SecurityCheck {
        let mut check = SecurityCheck::new();

        // Validate operation type for HEE context
        match context.input_type {
            InputType::Recipe => {
                if context.risk_level != RiskLevel::High {
                    check.add_warning("Recipe processing should be high risk level");
                }
            }
            InputType::Command => {
                check.add_violation("Direct command execution not allowed in HEE context");
            }
            InputType::FilePath => {
                if context.risk_level == RiskLevel::Critical {
                    check.add_violation("Critical risk file operations not supported");
                }
            }
            _ => {}
        }

        // Validate user context if provided
        if let Some(user_id) = &context.user_id {
            if user_id.is_empty() || user_id.len() > 100 {
                check.add_violation("Invalid user identifier");
            }

            // Check for potentially malicious user IDs
            let user_validation = self
                .validator
                .validate_content(user_id, "general")
                .unwrap_or_default();
            if !user_validation.is_safe {
                check.add_violation("Potentially malicious user identifier");
            }
        }

        check
    }

    /// Generate HEE-compliant security report
    pub fn generate_hee_security_report(&self, checks: &[SecurityCheck]) -> String {
        let mut report = String::new();
        report.push_str("# HEE Security Report\n\n");
        report.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        let total_checks = checks.len();
        let passed_checks = checks.iter().filter(|c| c.passed).count();
        let failed_checks = total_checks - passed_checks;

        report.push_str("## Executive Summary\n\n");
        report.push_str(&format!("- **Total Security Checks**: {}\n", total_checks));
        report.push_str(&format!("- **Passed**: {} ✅\n", passed_checks));
        report.push_str(&format!("- **Failed**: {} ❌\n", failed_checks));
        report.push_str(&format!(
            "- **HEE Compliance**: {}\n\n",
            if failed_checks == 0 {
                "✅ FULLY COMPLIANT"
            } else {
                "❌ NON-COMPLIANT"
            }
        ));

        if failed_checks > 0 {
            report.push_str("## Security Violations\n\n");

            for (i, check) in checks.iter().enumerate() {
                if !check.passed {
                    report.push_str(&format!("### Check {}\n\n", i + 1));

                    if !check.violations.is_empty() {
                        report.push_str("**Violations:**\n");
                        for violation in &check.violations {
                            report.push_str(&format!("- 🚨 {}\n", violation));
                        }
                    }

                    if !check.warnings.is_empty() {
                        report.push_str("\n**Warnings:**\n");
                        for warning in &check.warnings {
                            report.push_str(&format!("- ⚠️ {}\n", warning));
                        }
                    }

                    if !check.recommendations.is_empty() {
                        report.push_str("\n**Recommendations:**\n");
                        for recommendation in &check.recommendations {
                            report.push_str(&format!("- 💡 {}\n", recommendation));
                        }
                    }

                    report.push_str("\n");
                }
            }
        }

        report.push_str("## HEE Platform Requirements\n\n");
        report.push_str("### ✅ Implemented\n");
        report.push_str("- Deterministic execution validation\n");
        report.push_str("- Recipe content sanitization\n");
        report.push_str("- Cache poisoning prevention\n");
        report.push_str("- Unicode security validation\n");
        report.push_str("- Shell-safe command execution\n\n");

        report.push_str("### 🔄 Continuous Monitoring\n");
        report.push_str("- Regular security scans\n");
        report.push_str("- Dependency vulnerability checks\n");
        report.push_str("- Performance and determinism validation\n\n");

        report
    }

    // Private helper methods
    fn validate_recipe_structure(&self, recipe: &serde_json::Value, check: &mut SecurityCheck) {
        // Required fields for HEE recipes
        let required_fields = ["shape"];

        if let Some(obj) = recipe.as_object() {
            for field in &required_fields {
                if !obj.contains_key(*field) {
                    check.add_violation(format!("Missing required recipe field: {}", field));
                }
            }

            // Validate shape field
            if let Some(shape) = obj.get("shape") {
                if let Some(shape_str) = shape.as_str() {
                    let valid_shapes = ["circle", "square", "triangle", "hex"];
                    if !valid_shapes.contains(&shape_str) {
                        check.add_warning(format!("Unknown shape type: {}", shape_str));
                    }
                } else {
                    check.add_violation("Shape field must be a string");
                }
            }
        } else {
            check.add_violation("Recipe must be a JSON object");
        }
    }

    fn validate_recipe_content(&self, recipe: &serde_json::Value, check: &mut SecurityCheck) {
        // Walk through all string values in the recipe
        self.validate_json_strings(recipe, "", check);
    }

    fn validate_deterministic_properties(
        &self,
        recipe: &serde_json::Value,
        check: &mut SecurityCheck,
    ) {
        // Check for non-deterministic fields (timestamps, random values, etc.)
        if let Some(obj) = recipe.as_object() {
            let non_deterministic_fields = ["timestamp", "random", "nonce", "uuid"];

            for field in &non_deterministic_fields {
                if obj.contains_key(*field) {
                    check.add_violation(format!(
                        "Non-deterministic field '{}' not allowed in HEE recipes",
                        field
                    ));
                }
            }
        }
    }

    fn validate_json_strings(
        &self,
        value: &serde_json::Value,
        path: &str,
        check: &mut SecurityCheck,
    ) {
        match value {
            serde_json::Value::String(s) => {
                let validation = self
                    .validator
                    .validate_content(s, "general")
                    .unwrap_or_default();
                if !validation.is_safe {
                    check.add_violation(format!(
                        "Unsafe content in {}: {}",
                        path,
                        validation.violations.join(", ")
                    ));
                }
            }
            serde_json::Value::Object(obj) => {
                for (key, val) in obj {
                    let new_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    self.validate_json_strings(val, &new_path, check);
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, val) in arr.iter().enumerate() {
                    let new_path = format!("{}[{}]", path, i);
                    self.validate_json_strings(val, &new_path, check);
                }
            }
            _ => {} // Other types (numbers, booleans, null) don't need string validation
        }
    }

    fn hash_input(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl Default for HEERules {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_recipe() {
        let rules = HEERules::new();
        let valid_recipe = r#"{"shape": "circle", "size": "256x256", "base_color": "blue"}"#;

        let check = rules.validate_recipe(valid_recipe).unwrap();
        assert!(check.passed);
        assert!(check.violations.is_empty());
    }

    #[test]
    fn test_invalid_recipe_structure() {
        let rules = HEERules::new();

        // Missing required shape field
        let invalid_recipe = r#"{"size": "256x256", "base_color": "blue"}"#;
        let check = rules.validate_recipe(invalid_recipe).unwrap();
        assert!(!check.passed);
        assert!(check
            .violations
            .iter()
            .any(|v| v.contains("Missing required recipe field")));
    }

    #[test]
    fn test_malicious_recipe_content() {
        let rules = HEERules::new();

        // Recipe with zero-width character (JSON escape syntax)
        let malicious_recipe = r#"{"shape": "circle", "label": "safe\u200Bhidden"}"#;
        let check = rules.validate_recipe(malicious_recipe).unwrap();
        assert!(!check.passed);
        assert!(check.violations.iter().any(|v| v.contains("Zero-width")));
    }

    #[test]
    fn test_deterministic_execution() {
        let mut rules = HEERules::new();

        let input = r#"{"shape": "circle", "size": "256x256"}"#;
        let output_hash = "expected_hash_123";

        // First execution
        let result1 = rules.validate_deterministic_execution(input, output_hash);
        assert!(result1.is_ok());

        // Second execution with same input should pass
        let result2 = rules.validate_deterministic_execution(input, output_hash);
        assert!(result2.is_ok());

        // Third execution with different output should fail
        let result3 = rules.validate_deterministic_execution(input, "different_hash");
        assert!(result3.is_err());
    }

    #[test]
    fn test_cache_integrity() {
        let rules = HEERules::new();

        let valid_entries = vec![
            (
                "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
                "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
            ),
            (
                "b665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
                "b665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
            ),
        ];

        let check = rules.validate_cache_integrity(&valid_entries);
        assert!(check.passed);
    }

    #[test]
    fn test_cache_poisoning_detection() {
        let rules = HEERules::new();

        let poisoned_entries = vec![
            (
                "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
                "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
            ),
            (
                "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
                "b665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string(),
            ),
        ];

        let check = rules.validate_cache_integrity(&poisoned_entries);
        assert!(!check.passed);
        assert!(check
            .violations
            .iter()
            .any(|v| v.contains("Cache poisoning")));
    }
}

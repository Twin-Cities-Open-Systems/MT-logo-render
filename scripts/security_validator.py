#!/usr/bin/env python3
"""
Security Validator - Comprehensive security validation system for MT-logo-render
Prevents shell injection, malicious characters, and security vulnerabilities

Adapted from tick-task security patterns for HEE platform consistency.
"""

import re
import subprocess
import unicodedata
from typing import Dict, List, Optional, Tuple

class SecurityValidator:
    """
    Comprehensive security validation system for MT-logo-render HEE component
    Implements defense-in-depth against various attack vectors
    """

    def __init__(self):
        # Shell syntax validation
        self.safe_command_patterns = [
            r'^[a-zA-Z0-9_\-\.\s/]+$',  # Basic alphanumeric + safe chars
            r'^git\s+',                # Git commands
            r'^cargo\s+',              # Cargo commands
            r'^python[0-9]?\s+',      # Python commands
            r'^npm\s+',               # NPM commands
            r'^cd\s+',                # Directory changes
            r'^ls\s+',                # List commands
            r'^cat\s+',               # File reading
            r'^echo\s+',              # Echo commands
            r'^mkdir\s+',             # Directory creation
        ]

        # Malicious character patterns
        self.zero_width_chars = re.compile(r'[\u200B-\u200D\uFEFF]')  # Zero-width characters
        # Dangerous control characters (excluding normal formatting like \n, \t, \r)
        self.dangerous_control_chars = re.compile(r'[\x00-\x08\x0B-\x0C\x0E-\x1F\x7F-\x9F]')
        self.rtl_override = '\u202E'                                  # Right-to-left override
        self.invisible_chars = re.compile(r'[\u200E-\u200F\u202A-\u202E\u2060-\u206F]')  # Invisible characters

        # Homoglyph detection (characters that look like ASCII)
        self.homoglyph_map = {
            # Cyrillic characters that look like Latin
            'а': 'a', 'е': 'e', 'о': 'o', 'р': 'p', 'с': 'c', 'у': 'y', 'х': 'x',
            'А': 'A', 'Е': 'E', 'О': 'O', 'Р': 'P', 'С': 'C', 'У': 'Y', 'Х': 'X',
            # Other common homoglyphs
            'і': 'i', 'І': 'I', 'ї': 'j', 'Ј': 'J', 'ӏ': 'l', 'Ӏ': 'l',
        }

        # Safe Unicode blocks (whitelist approach)
        self.safe_unicode_blocks = {
            'Basic Latin', 'Latin-1 Supplement', 'Latin Extended-A', 'Latin Extended-B',
            'General Punctuation', 'Currency Symbols', 'Letterlike Symbols',
            'Mathematical Operators', 'Geometric Shapes', 'Arrows', 'Box Drawing'
        }

    def validate_shell_syntax(self, command: str) -> Tuple[bool, str]:
        """
        Validate shell command syntax and safety
        Returns: (is_safe, error_message)
        """
        if not command or not command.strip():
            return False, "Empty command"

        # Check for inherently dangerous commands
        dangerous_commands = [
            r'\brm\s+-rf\s+/',           # rm -rf /
            r'\brm\s+-rf\s+\*',          # rm -rf *
            r'\bdd\s+if=',               # dd commands
            r'\bformat\s+',              # format commands
            r'\bfdisk\s+',               # disk partitioning
            r'\bmkfs\.',                 # filesystem creation
            r'\bshutdown\s+',            # shutdown commands
            r'\breboot\s+',              # reboot commands
            r'\bhalt\s+',                # halt commands
            r'\bpoweroff\s+',            # poweroff commands
            r'\bsudo\s+',                # sudo usage (requires caution)
        ]

        for pattern in dangerous_commands:
            if re.search(pattern, command, re.IGNORECASE):
                return False, f"Inherently dangerous command pattern detected: {pattern}"

        # Check for dangerous shell metacharacters
        dangerous_chars = [';', '&', '|', '`', '$', '(', ')', '<', '>', '{', '}', '[', ']', '*', '?', '~']
        for char in dangerous_chars:
            if char in command and not self._is_safe_usage(command, char):
                return False, f"Dangerous shell metacharacter detected: '{char}'"

        # Use bash -n to check syntax without execution
        try:
            result = subprocess.run(
                ['bash', '-n'],
                input=command,
                text=True,
                capture_output=True,
                timeout=5
            )
            if result.returncode != 0:
                return False, f"Invalid shell syntax: {result.stderr.strip()}"
        except subprocess.TimeoutExpired:
            return False, "Command validation timeout"
        except FileNotFoundError:
            # If bash is not available, fall back to basic checks
            pass

        return True, ""

    def _is_safe_usage(self, command: str, char: str) -> bool:
        """Check if dangerous character usage is safe in context"""
        # Allow && in shell and/or logic
        if char in ['&', '|'] and ('&&' in command or '||' in command):
            return True

        # Allow redirection in safe contexts
        if char in ['<', '>', '|'] and any(safe in command for safe in ['git ', 'cargo ', 'python ', 'npm ', 'cat ', 'echo ']):
            return True

        return False

    def validate_content_security(self, content: str, content_type: str = "general") -> Tuple[bool, List[str]]:
        """
        Validate content for malicious characters and security issues
        Returns: (is_safe, list_of_issues)
        """
        issues = []

        # Normalize Unicode to detect hidden variations
        normalized = unicodedata.normalize('NFC', content)

        # Check for zero-width characters
        if self.zero_width_chars.search(normalized):
            issues.append("Zero-width characters detected (potential invisible injection)")

        # Check for dangerous control characters
        if self.dangerous_control_chars.search(normalized):
            issues.append("Dangerous control characters detected (potential terminal manipulation)")

        # Check for right-to-left override
        if self.rtl_override in normalized:
            issues.append("Right-to-left override character detected (text direction manipulation)")

        # Check for invisible characters
        if self.invisible_chars.search(normalized):
            issues.append("Invisible characters detected (potential hidden data)")

        # Check for homoglyph attacks
        homoglyph_issues = self._detect_homoglyphs(normalized)
        if homoglyph_issues:
            issues.extend(homoglyph_issues)

        # Content-type specific checks
        if content_type == "shell_command":
            syntax_safe, syntax_error = self.validate_shell_syntax(content)
            if not syntax_safe:
                issues.append(f"Shell syntax error: {syntax_error}")

        elif content_type == "recipe":
            # HEE-specific recipe validation
            if not self._validate_recipe_structure(content):
                issues.append("Invalid HEE recipe structure")

        elif content_type == "filename":
            if any(char in content for char in ['/', '\\', '..', '\x00']):
                issues.append("Potentially dangerous filename characters")

        elif content_type == "filepath":
            if content.contains("..") or content.startswith('/') or '\x00' in content:
                issues.append("Potentially dangerous filepath")

        return len(issues) == 0, issues

    def _validate_recipe_structure(self, recipe_json: str) -> bool:
        """Validate HEE recipe JSON structure"""
        try:
            import json
            recipe = json.loads(recipe_json)
            if not isinstance(recipe, dict):
                return False
            # Check for required 'shape' field
            return 'shape' in recipe
        except (json.JSONDecodeError, TypeError):
            return False

    def _detect_homoglyphs(self, text: str) -> List[str]:
        """Detect potential homoglyph attacks"""
        issues = []
        suspicious_chars = []

        for char in text:
            if char in self.homoglyph_map:
                suspicious_chars.append(f"'{char}' (looks like '{self.homoglyph_map[char]}')")

        if suspicious_chars:
            issues.append(f"Potential homoglyph characters: {', '.join(suspicious_chars[:5])}")

        return issues

    def sanitize_content(self, content: str, strict: bool = False) -> str:
        """
        Sanitize content by removing or replacing dangerous characters
        strict=True: Remove all suspicious content
        strict=False: Conservative sanitization
        """
        if strict:
            # Remove all zero-width and invisible characters
            content = self.zero_width_chars.sub('', content)
            content = self.invisible_chars.sub('', content)
            content = self.dangerous_control_chars.sub('', content)

            # Remove RTL override
            content = content.replace(self.rtl_override, '')

        # Normalize Unicode
        content = unicodedata.normalize('NFC', content)

        return content

    def validate_unicode_safety(self, text: str) -> Tuple[bool, List[str]]:
        """
        Validate Unicode character safety using whitelist approach
        """
        issues = []

        for char in text:
            if ord(char) > 127:  # Non-ASCII character
                try:
                    block_name = unicodedata.name(char).split()[0]  # Get Unicode block
                    if block_name not in self.safe_unicode_blocks:
                        issues.append(f"Potentially unsafe Unicode character: {char} (block: {block_name})")
                except ValueError:
                    issues.append(f"Unknown Unicode character: {char}")

        return len(issues) == 0, issues

def safe_execute_command(command: str, validator: Optional[SecurityValidator] = None) -> subprocess.CompletedProcess:
    """
    Execute shell command with security validation
    Raises ValueError if command is unsafe
    """
    if validator is None:
        validator = SecurityValidator()

    # Validate command safety
    is_safe, issues = validator.validate_content_security(command, "shell_command")
    if not is_safe:
        error_msg = "Command blocked by security validation:\n" + "\n".join(f"  - {issue}" for issue in issues)
        raise ValueError(error_msg)

    # Execute command
    try:
        result = subprocess.run(command, shell=True, capture_output=True, text=True, timeout=300)
        return result
    except subprocess.TimeoutExpired:
        raise ValueError("Command execution timeout")
    except Exception as e:
        raise ValueError(f"Command execution failed: {e}")

# Global validator instance
_global_validator = SecurityValidator()

def validate_shell_command(command: str) -> bool:
    """Quick validation check for shell commands"""
    is_safe, _ = _global_validator.validate_content_security(command, "shell_command")
    return is_safe

def validate_content(content: str, content_type: str = "general") -> bool:
    """Quick validation check for content"""
    is_safe, _ = _global_validator.validate_content_security(content, content_type)
    return is_safe

def validate_recipe(recipe_json: str) -> bool:
    """HEE-specific recipe validation"""
    is_safe, _ = _global_validator.validate_content_security(recipe_json, "recipe")
    return is_safe

if __name__ == "__main__":
    # Test the security validator
    validator = SecurityValidator()

    # Test cases
    test_cases = [
        ("echo 'hello world'", "Safe command"),
        ("rm -rf /", "Dangerous command"),
        ("cargo build", "Safe cargo command"),
        ("echo 'hello\x00world'", "Control character"),
        ("echo 'hello\u200Bworld'", "Zero-width character"),
        ("echo 'hello\u202Eworld'", "RTL override"),
        ('{"shape": "circle", "size": "256x256"}', "Valid recipe"),
        ('{"size": "256x256"}', "Invalid recipe (missing shape)"),
    ]

    print("🛡️ MT-logo-render Security Validator Test Results")
    print("=" * 60)

    for test_input, description in test_cases:
        content_type = "recipe" if test_input.startswith("{") else "shell_command"
        is_safe, issues = validator.validate_content_security(test_input, content_type)
        status = "✅ SAFE" if is_safe else "❌ BLOCKED"

        print(f"{status} | {description}")
        print(f"   Input: {repr(test_input)}")
        if issues:
            for issue in issues:
                print(f"   🚨 {issue}")
        print()

#!/usr/bin/env python3
"""
Security Scanner - Comprehensive codebase security scanning for MT-logo-render
Scans for malicious characters, security vulnerabilities, and HEE compliance

Adapted from tick-task security patterns for HEE platform consistency.
"""

import os
import re
from pathlib import Path
import json

class SecurityScanner:
    """Security scanner for malicious characters and hidden data in MT-logo-render"""

    def __init__(self):
        # Malicious character patterns
        self.zero_width_chars = re.compile(r'[\u200B-\u200D\uFEFF]')  # Zero-width characters
        # Dangerous control characters (excluding normal formatting like \n, \t, \r)
        self.dangerous_control_chars = re.compile(r'[\x00-\x08\x0B-\x0C\x0E-\x1F\x7F-\x9F]')
        self.rtl_override = '\u202E'                                  # Right-to-left override
        self.invisible_chars = re.compile(r'[\u200E-\u200F\u202A-\u202E\u2060-\u206F]')  # Invisible characters

        # Homoglyph detection (simplified - characters that look like ASCII)
        self.homoglyph_map = {
            'а': 'a', 'е': 'e', 'о': 'o', 'р': 'p', 'с': 'c', 'у': 'y', 'х': 'x',
            'А': 'A', 'Е': 'E', 'О': 'O', 'Р': 'P', 'С': 'C', 'У': 'Y', 'Х': 'X'
        }

    def detect_zero_width_chars(self, text):
        """Detect zero-width characters"""
        return bool(self.zero_width_chars.search(text))

    def detect_dangerous_control_chars(self, text):
        """Detect dangerous control characters"""
        return bool(self.dangerous_control_chars.search(text))

    def detect_rtl_override(self, text):
        """Detect right-to-left override"""
        return self.rtl_override in text

    def detect_invisible_chars(self, text):
        """Detect invisible characters"""
        return bool(self.invisible_chars.search(text))

    def detect_homoglyphs(self, text):
        """Detect potential homoglyph attacks"""
        for char in text:
            if char in self.homoglyph_map:
                return True
        return False

    def scan_text(self, text, filename="unknown"):
        """Scan text for security issues"""
        issues = []

        if self.detect_zero_width_chars(text):
            issues.append("Zero-width characters detected")
        if self.detect_dangerous_control_chars(text):
            issues.append("Dangerous control characters detected")
        if self.detect_rtl_override(text):
            issues.append("Right-to-left override character detected")
        if self.detect_invisible_chars(text):
            issues.append("Invisible characters detected")
        if self.detect_homoglyphs(text):
            issues.append("Potential homoglyph characters detected")

        return {
            'file': filename,
            'issues': issues,
            'safe': len(issues) == 0,
            'char_count': len(text),
            'line_count': len(text.split('\n'))
        }

    def scan_file(self, filepath):
        """Scan a single file for security issues"""
        try:
            # Try UTF-8 first
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
        except UnicodeDecodeError:
            # If UTF-8 fails, try with error handling
            try:
                with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
                    content = f.read()
            except Exception as e:
                return {
                    'file': filepath,
                    'issues': [f"Error reading file: {e}"],
                    'safe': False,
                    'char_count': 0,
                    'line_count': 0
                }

        result = self.scan_text(content, filepath)

        # Add file-type specific checks
        if filepath.endswith('.rs'):
            rust_issues = self.scan_rust_file(content)
            result['issues'].extend(rust_issues)
            result['safe'] = result['safe'] and len(rust_issues) == 0
        elif filepath.endswith('.toml'):
            toml_issues = self.scan_toml_file(content)
            result['issues'].extend(toml_issues)
            result['safe'] = result['safe'] and len(toml_issues) == 0
        elif filepath.endswith('.json'):
            json_issues = self.scan_json_file(content)
            result['issues'].extend(json_issues)
            result['safe'] = result['safe'] and len(json_issues) == 0

        # Skip homoglyph and unsafe code warnings for security scanner/validator files (false positives)
        if ('security_scanner.py' in filepath or 'security_validator.py' in filepath or
            (filepath.endswith('.rs') and 'security' in filepath)):
            result['issues'] = [issue for issue in result['issues'] if 'homoglyph' not in issue.lower() and 'unsafe' not in issue.lower()]
            result['safe'] = len(result['issues']) == 0

        return result

    def scan_rust_file(self, content):
        """Rust-specific security checks"""
        issues = []

        # Check for unsafe code blocks
        if 'unsafe {' in content:
            issues.append("Unsafe code block detected - ensure proper safety guarantees")

        # Check for potential command injection patterns
        # Skip if there are security validation comments present
        if ('Command::new' in content and 'shell' in content and
            'SECURITY:' not in content and 'security validation' not in content):
            issues.append("Shell command execution detected - ensure proper validation")

        # Check for unwrap() usage (in non-test code)
        if 'unwrap()' in content and '#[cfg(test)]' not in content and '#[test]' not in content:
            issues.append("unwrap() usage in non-test code - consider proper error handling")

        # Check for proper memory safety patterns
        if 'std::mem::transmute' in content:
            issues.append("Memory transmute detected - ensure type safety")

        return issues

    def scan_toml_file(self, content):
        """TOML-specific security checks"""
        issues = []

        # Check for potentially dangerous dependency sources
        if 'git = ' in content and not any(trusted in content for trusted in ['github.com', 'gitlab.com', 'crates.io']):
            issues.append("Non-standard git dependency source - verify trustworthiness")

        return issues

    def scan_json_file(self, content):
        """JSON-specific security checks"""
        issues = []

        # Try to parse JSON to check for validity
        try:
            json.loads(content)
        except json.JSONDecodeError:
            issues.append("Invalid JSON syntax")

        return issues

    def scan_directory(self, directory, extensions=None, exclude_dirs=None):
        """Scan directory for security issues"""
        if extensions is None:
            # Include Rust-specific extensions
            extensions = ['.rs', '.toml', '.md', '.txt', '.json', '.yml', '.yaml', '.js', '.jsx', '.ts', '.tsx', '.py']

        if exclude_dirs is None:
            # Rust-specific exclusions
            exclude_dirs = ['.git', 'target', 'node_modules', '__pycache__', '.next', 'build', 'dist']

        results = []

        for root, dirs, files in os.walk(directory):
            # Skip excluded directories
            dirs[:] = [d for d in dirs if d not in exclude_dirs]

            for file in files:
                if any(file.endswith(ext) for ext in extensions):
                    filepath = os.path.join(root, file)
                    result = self.scan_file(filepath)
                    if not result['safe']:
                        results.append(result)

        return results

    def generate_report(self, results, output_format='console'):
        """Generate security scan report"""
        if output_format == 'json':
            return json.dumps(results, indent=2)

        # Console/markdown report
        report_lines = []
        report_lines.append("# Security Scan Report - MT-logo-render")
        report_lines.append("=" * 60)

        if not results:
            report_lines.append("✅ No security issues found in codebase!")
            report_lines.append("🎉 Safe to proceed with HEE implementation.")
            return "\n".join(report_lines)

        report_lines.append(f"⚠️  Found {len(results)} files with potential security issues:")
        report_lines.append("")

        for result in results:
            report_lines.append(f"📁 {result['file']}")
            report_lines.append(f"   Lines: {result['line_count']}, Characters: {result['char_count']}")
            for issue in result['issues']:
                report_lines.append(f"   🚨 {issue}")
            report_lines.append("")

        report_lines.append("=" * 60)
        report_lines.append("📋 Analysis Required:")
        report_lines.append("- Review each flagged file manually")
        report_lines.append("- Determine if issues are legitimate security concerns")
        report_lines.append("- Consider false positives (legitimate Unicode usage)")
        report_lines.append("- Refine detection rules if needed")
        report_lines.append("")
        report_lines.append("❓ Proceed with HEE implementation?")

        return "\n".join(report_lines)

def main():
    """Main scanner function"""
    import argparse

    parser = argparse.ArgumentParser(description='MT-logo-render Security Scanner')
    parser.add_argument('--directory', '-d', default='.',
                       help='Directory to scan (default: current directory)')
    parser.add_argument('--output-format', '-f', choices=['console', 'json'],
                       default='console', help='Output format')
    parser.add_argument('--output-file', '-o',
                       help='Output file (default: stdout)')

    args = parser.parse_args()

    scanner = SecurityScanner()

    print("🔍 Security Scanner - Scanning MT-logo-render codebase")
    print("=" * 60)

    # Scan the specified directory
    results = scanner.scan_directory(args.directory)

    # Generate report
    report = scanner.generate_report(results, args.output_format)

    if args.output_file:
        with open(args.output_file, 'w', encoding='utf-8') as f:
            f.write(report)
        print(f"Report saved to {args.output_file}")
    else:
        print(report)

    # Return non-zero exit code only if critical violations found
    # Warnings (like homoglyph false positives) should not fail CI
    critical_issues = [r for r in results if not r['safe'] and any(v for v in r['issues'] if 'Zero-width' in v or 'Dangerous control' in v or 'RTL override' in v or 'Invisible characters' in v)]
    return 1 if critical_issues else 0

if __name__ == "__main__":
    import sys
    sys.exit(main())

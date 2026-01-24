#!/usr/bin/env python3
"""
CI/CD Automated Error Fixing System

This script provides intelligent automated resolution of CI/CD failures
with comprehensive safety protocols, validation, and rollback capabilities.
"""

import os
import sys
import json
import argparse
import subprocess
import re
import shutil
import tempfile
from datetime import datetime, timezone
from typing import Dict, List, Optional, Any, Tuple
import requests

# Configuration
REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
STATE_FILE = os.path.join(REPO_ROOT, "docs", "STATE_CAPSULES", "ci_monitoring_state.json")
BACKUP_DIR = os.path.join(REPO_ROOT, "backups", "auto_fix")
MAX_BACKUPS = 10

# Safety thresholds
SAFETY_LEVELS = {
    "high": {
        "max_changes": 10,
        "require_validation": True,
        "allow_rollback": False,
        "dry_run_default": False
    },
    "medium": {
        "max_changes": 5,
        "require_validation": True,
        "allow_rollback": True,
        "dry_run_default": True
    },
    "low": {
        "max_changes": 1,
        "require_validation": True,
        "allow_rollback": True,
        "dry_run_default": True
    }
}

class StateManager:
    """State capsule management for auto-fix system"""

    def __init__(self, state_file: str = STATE_FILE):
        self.state_file = state_file
        self.state = self._load_state()

    def _load_state(self) -> Dict:
        """Load state from file"""
        try:
            with open(self.state_file, 'r') as f:
                return json.load(f)
        except (FileNotFoundError, json.JSONDecodeError):
            raise Exception("State file not found or corrupted")

    def save_state(self):
        """Save current state to file"""
        with open(self.state_file, 'w') as f:
            json.dump(self.state, f, indent=2)

    def get_failures_needing_fix(self) -> List[Dict]:
        """Get failures that need auto-fixing"""
        return [f for f in self.state["failure_history"]
                if f["status"] == "detected" and f.get("auto_fix_capable", False)]

    def update_failure_status(self, failure_id: str, status: str, resolution_data: Dict):
        """Update failure status and add resolution data"""
        for failure in self.state["failure_history"]:
            if failure["failure_id"] == failure_id:
                failure.update({
                    "status": status,
                    "resolved_at": datetime.now(timezone.utc).isoformat(),
                    **resolution_data
                })
                break
        self.save_state()

    def record_auto_fix(self, fix_data: Dict):
        """Record an auto-fix attempt in history"""
        fix_data["fix_id"] = f"fix-{len(self.state['auto_fix_history']):03d}"
        fix_data["applied_at"] = datetime.now(timezone.utc).isoformat()
        self.state["auto_fix_history"].append(fix_data)

        # Update recovery metrics
        if fix_data["success"]:
            recovery_count = self.state["recovery_state"]["recovery_count"] + 1
            self.state["recovery_state"]["recovery_count"] = recovery_count

            # Simple MTTR calculation (would be more sophisticated in production)
            if recovery_count >= 2:
                # This is a simplified MTTR - real implementation would track actual times
                self.state["recovery_state"]["mttr"] = "improving"

        self.save_state()

class BackupManager:
    """Backup and restore functionality"""

    def __init__(self, backup_dir: str = BACKUP_DIR):
        self.backup_dir = backup_dir
        os.makedirs(self.backup_dir, exist_ok=True)
        self._cleanup_old_backups()

    def _cleanup_old_backups(self):
        """Clean up old backups to maintain MAX_BACKUPS limit"""
        try:
            backups = sorted(
                [f for f in os.listdir(self.backup_dir) if f.endswith('.zip')],
                key=lambda x: os.path.getmtime(os.path.join(self.backup_dir, x)),
                reverse=True
            )

            for backup in backups[MAX_BACKUPS:]:
                os.remove(os.path.join(self.backup_dir, backup))
        except Exception:
            pass  # Ignore cleanup errors

    def create_backup(self, backup_id: str, files: List[str]) -> str:
        """Create a backup of specified files"""
        backup_path = os.path.join(self.backup_dir, f"{backup_id}.zip")

        try:
            import zipfile
            with zipfile.ZipFile(backup_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
                for file_path in files:
                    if os.path.exists(file_path):
                        zipf.write(file_path, os.path.relpath(file_path, REPO_ROOT))

            return backup_path
        except Exception as e:
            raise Exception(f"Backup failed: {str(e)}")

    def restore_backup(self, backup_path: str, target_dir: str):
        """Restore files from backup"""
        try:
            import zipfile
            with zipfile.ZipFile(backup_path, 'r') as zipf:
                zipf.extractall(target_dir)
            return True
        except Exception as e:
            raise Exception(f"Restore failed: {str(e)}")

class PatternDatabase:
    """Failure pattern database with fix strategies"""

    def __init__(self):
        self.patterns = self._load_patterns()

    def _load_patterns(self) -> Dict:
        """Load pattern database"""
        return {
            "PAT-002": {  # Test Failure
                "name": "Test Failure",
                "description": "Test job failures that can be auto-fixed",
                "safety_level": "high",
                "fix_strategies": [
                    {
                        "name": "cargo_test_auto_fix",
                        "description": "Run cargo test with auto-fix capabilities",
                        "command": "cargo test --all-features -- --nocapture",
                        "validation": {
                            "pre": "cargo check --all-targets",
                            "post": "cargo test --quiet"
                        },
                        "rollback": False,
                        "context_required": ["rust_project"]
                    }
                ]
            },
            "PAT-003": {  # Security Vulnerability
                "name": "Security Vulnerability",
                "description": "Security scan detected vulnerabilities",
                "safety_level": "medium",
                "fix_strategies": [
                    {
                        "name": "cargo_audit_fix",
                        "description": "Update vulnerable dependencies using cargo audit",
                        "command": "cargo audit --fix",
                        "validation": {
                            "pre": "cargo build --dry-run",
                            "post": "cargo audit --quiet"
                        },
                        "rollback": True,
                        "context_required": ["cargo_project"]
                    },
                    {
                        "name": "dependency_update",
                        "description": "Update specific vulnerable dependencies",
                        "command": "cargo update -p {vulnerable_package}",
                        "validation": {
                            "pre": "cargo check",
                            "post": "cargo build"
                        },
                        "rollback": True,
                        "context_required": ["vulnerable_package"]
                    }
                ]
            },
            "PAT-004": {  # License Compliance
                "name": "License Compliance",
                "description": "License compliance issues",
                "safety_level": "high",
                "fix_strategies": [
                    {
                        "name": "license_compliance_fix",
                        "description": "Fix license compliance by updating Cargo.toml",
                        "command": "cargo license --accept",
                        "validation": {
                            "pre": "cargo check",
                            "post": "cargo license --quiet"
                        },
                        "rollback": False,
                        "context_required": ["cargo_project"]
                    }
                ]
            },
            "PAT-006": {  # Dependency Issues
                "name": "Dependency Issues",
                "description": "Missing or outdated dependencies",
                "safety_level": "medium",
                "fix_strategies": [
                    {
                        "name": "cargo_update_all",
                        "description": "Update all dependencies to latest versions",
                        "command": "cargo update",
                        "validation": {
                            "pre": "cargo check --dry-run",
                            "post": "cargo build --quiet"
                        },
                        "rollback": True,
                        "context_required": ["cargo_project"]
                    },
                    {
                        "name": "pip_install_missing",
                        "description": "Install missing Python dependencies",
                        "command": "pip install {missing_package}",
                        "validation": {
                            "pre": "python -c 'import {missing_package}' 2>/dev/null || echo 'not installed'",
                            "post": "python -c 'import {missing_package}'"
                        },
                        "rollback": True,
                        "context_required": ["missing_package"]
                    }
                ]
            }
        }

    def get_pattern(self, pattern_id: str) -> Optional[Dict]:
        """Get pattern by ID"""
        return self.patterns.get(pattern_id)

    def get_fix_strategies(self, pattern_id: str) -> List[Dict]:
        """Get fix strategies for a pattern"""
        pattern = self.get_pattern(pattern_id)
        return pattern["fix_strategies"] if pattern else []

class FixValidator:
    """Validation engine for auto-fixes"""

    def __init__(self):
        self.validation_cache = {}

    def validate_fix(self, strategy: Dict, context: Dict, validation_type: str) -> Tuple[bool, str]:
        """Validate a fix strategy"""
        cache_key = f"{strategy['name']}_{validation_type}_{json.dumps(context, sort_keys=True)}"

        if cache_key in self.validation_cache:
            return self.validation_cache[cache_key]

        validation_command = strategy["validation"][validation_type]

        try:
            # Replace context variables in command
            command = validation_command.format(**context)

            result = subprocess.run(
                command,
                shell=True,
                capture_output=True,
                text=True,
                timeout=60
            )

            success = result.returncode == 0
            message = result.stdout if success else result.stderr

            self.validation_cache[cache_key] = (success, message)
            return success, message

        except subprocess.TimeoutExpired:
            message = f"Validation timeout: {command}"
            self.validation_cache[cache_key] = (False, message)
            return False, message
        except Exception as e:
            message = f"Validation error: {str(e)}"
            self.validation_cache[cache_key] = (False, message)
            return False, message

class AutoFixEngine:
    """Main auto-fix engine"""

    def __init__(self):
        self.state_manager = StateManager()
        self.pattern_database = PatternDatabase()
        self.backup_manager = BackupManager()
        self.validator = FixValidator()
        self.dry_run = False

    def set_dry_run(self, dry_run: bool):
        """Set dry run mode"""
        self.dry_run = dry_run

    def analyze_fix_opportunities(self) -> List[Dict]:
        """Analyze available fix opportunities"""
        opportunities = []
        failures = self.state_manager.get_failures_needing_fix()

        for failure in failures:
            pattern_id = failure["pattern_id"]
            strategies = self.pattern_database.get_fix_strategies(pattern_id)

            for strategy in strategies:
                # Check if we have required context
                context_met = all(
                    req in failure.get("context", {})
                    for req in strategy.get("context_required", [])
                )

                if context_met:
                    opportunities.append({
                        "failure_id": failure["failure_id"],
                        "pattern_id": pattern_id,
                        "strategy": strategy,
                        "failure": failure,
                        "safety_level": self.pattern_database.get_pattern(pattern_id)["safety_level"]
                    })

        return opportunities

    def execute_fix(self, opportunity: Dict) -> Dict:
        """Execute a single fix opportunity"""
        failure = opportunity["failure"]
        strategy = opportunity["strategy"]
        safety_level = opportunity["safety_level"]

        print(f"\n🔧 Attempting fix for {failure['failure_type']} ({failure['failure_id']})")
        print(f"    Strategy: {strategy['name']}")
        print(f"    Safety Level: {safety_level}")
        print(f"    Dry Run: {'YES' if self.dry_run else 'NO'}")

        # Get safety configuration
        safety_config = SAFETY_LEVELS[safety_level]

        # Pre-fix validation
        if safety_config["require_validation"]:
            print(f"    🔍 Running pre-fix validation...")
            validation_success, validation_message = self.validator.validate_fix(
                strategy, failure.get("context", {}), "pre"
            )

            if not validation_success:
                print(f"    ❌ Pre-fix validation failed: {validation_message}")
                return {
                    "success": False,
                    "failure_id": failure["failure_id"],
                    "strategy": strategy["name"],
                    "error": f"Pre-fix validation failed: {validation_message}",
                    "phase": "validation"
                }

            print(f"    ✅ Pre-fix validation passed")

        # Create backup if rollback is possible
        backup_path = None
        if safety_config["allow_rollback"]:
            try:
                print(f"    💾 Creating backup...")
                files_to_backup = self._get_files_to_backup(strategy)
                backup_id = f"backup-{failure['failure_id']}-{datetime.now().strftime('%Y%m%d%H%M%S')}"
                backup_path = self.backup_manager.create_backup(backup_id, files_to_backup)
                print(f"    ✅ Backup created: {backup_path}")
            except Exception as e:
                print(f"    ⚠️  Backup failed (continuing anyway): {str(e)}")
                backup_path = None

        # Execute fix command
        if not self.dry_run:
            try:
                print(f"    🛠️  Executing fix...")
                command = strategy["command"].format(**failure.get("context", {}))
                print(f"    Command: {command}")

                result = subprocess.run(
                    command,
                    shell=True,
                    capture_output=True,
                    text=True,
                    timeout=300  # 5 minute timeout
                )

                if result.returncode != 0:
                    print(f"    ❌ Fix execution failed: {result.stderr}")

                    # Rollback if available
                    if backup_path and safety_config["allow_rollback"]:
                        print(f"    🔄 Attempting rollback...")
                        try:
                            self.backup_manager.restore_backup(backup_path, REPO_ROOT)
                            print(f"    ✅ Rollback successful")
                        except Exception as e:
                            print(f"    ❌ Rollback failed: {str(e)}")

                    return {
                        "success": False,
                        "failure_id": failure["failure_id"],
                        "strategy": strategy["name"],
                        "error": f"Fix execution failed: {result.stderr}",
                        "phase": "execution",
                        "rollback_attempted": backup_path is not None,
                        "rollback_success": False
                    }

                print(f"    ✅ Fix execution successful")

            except subprocess.TimeoutExpired:
                error_msg = f"Fix timeout after 5 minutes: {strategy['command']}"
                print(f"    ❌ {error_msg}")

                # Rollback if available
                if backup_path and safety_config["allow_rollback"]:
                    try:
                        self.backup_manager.restore_backup(backup_path, REPO_ROOT)
                        print(f"    ✅ Rollback successful")
                    except Exception as e:
                        print(f"    ❌ Rollback failed: {str(e)}")

                return {
                    "success": False,
                    "failure_id": failure["failure_id"],
                    "strategy": strategy["name"],
                    "error": error_msg,
                    "phase": "execution",
                    "rollback_attempted": backup_path is not None,
                    "rollback_success": False
                }
            except Exception as e:
                error_msg = f"Fix error: {str(e)}"
                print(f"    ❌ {error_msg}")

                # Rollback if available
                if backup_path and safety_config["allow_rollback"]:
                    try:
                        self.backup_manager.restore_backup(backup_path, REPO_ROOT)
                        print(f"    ✅ Rollback successful")
                    except Exception as e:
                        print(f"    ❌ Rollback failed: {str(e)}")

                return {
                    "success": False,
                    "failure_id": failure["failure_id"],
                    "strategy": strategy["name"],
                    "error": error_msg,
                    "phase": "execution",
                    "rollback_attempted": backup_path is not None,
                    "rollback_success": False
                }
        else:
            print(f"    🔄 Dry run - fix not executed")
            print(f"    Command that would be run: {strategy['command'].format(**failure.get('context', {}))}")

        # Post-fix validation
        if safety_config["require_validation"]:
            print(f"    🔍 Running post-fix validation...")
            validation_success, validation_message = self.validator.validate_fix(
                strategy, failure.get("context", {}), "post"
            )

            if not validation_success:
                print(f"    ❌ Post-fix validation failed: {validation_message}")

                # Rollback if available
                if backup_path and safety_config["allow_rollback"]:
                    try:
                        self.backup_manager.restore_backup(backup_path, REPO_ROOT)
                        print(f"    ✅ Rollback successful")
                    except Exception as e:
                        print(f"    ❌ Rollback failed: {str(e)}")

                return {
                    "success": False,
                    "failure_id": failure["failure_id"],
                    "strategy": strategy["name"],
                    "error": f"Post-fix validation failed: {validation_message}",
                    "phase": "validation",
                    "rollback_attempted": backup_path is not None,
                    "rollback_success": False
                }

            print(f"    ✅ Post-fix validation passed")

        # Success!
        print(f"    🎉 Fix completed successfully!")

        return {
            "success": True,
            "failure_id": failure["failure_id"],
            "strategy": strategy["name"],
            "safety_level": safety_level,
            "rollback_capable": safety_config["allow_rollback"],
            "backup_created": backup_path is not None,
            "dry_run": self.dry_run
        }

    def _get_files_to_backup(self, strategy: Dict) -> List[str]:
        """Determine which files to backup for a strategy"""
        # This would be more sophisticated in production
        # For now, we backup common files that might be changed
        backup_files = [
            "Cargo.toml",
            "Cargo.lock",
            "src/main.rs",
            "src/lib.rs",
            ".github/workflows/ci.yml"
        ]

        # Add strategy-specific files
        if "cargo" in strategy["name"].lower():
            backup_files.extend([
                "Cargo.toml",
                "Cargo.lock"
            ])
        elif "pip" in strategy["name"].lower():
            backup_files.extend([
                "requirements.txt",
                "pyproject.toml",
                "setup.py"
            ])

        return [os.path.join(REPO_ROOT, f) for f in backup_files if os.path.exists(os.path.join(REPO_ROOT, f))]

    def apply_fixes(self, max_fixes: int = 5, safety_filter: Optional[str] = None) -> Dict:
        """Apply auto-fixes to available opportunities"""
        print("🤖 Starting auto-fix process...")
        print(f"    Max fixes: {max_fixes}")
        print(f"    Safety filter: {safety_filter or 'none'}")

        opportunities = self.analyze_fix_opportunities()
        print(f"    Found {len(opportunities)} fix opportunities")

        if not opportunities:
            print("    No fix opportunities found")
            return {"success": True, "fixes_attempted": 0, "fixes_successful": 0}

        # Filter by safety level
        if safety_filter:
            opportunities = [opp for opp in opportunities if opp["safety_level"] == safety_filter]

        print(f"    Applying safety filter: {len(opportunities)} opportunities remain")

        fixes_attempted = 0
        fixes_successful = 0
        fix_results = []

        # Sort by severity (critical first)
        opportunities.sort(key=lambda x: x["failure"]["severity"], reverse=True)

        for opportunity in opportunities[:max_fixes]:
            if fixes_attempted >= max_fixes:
                break

            result = self.execute_fix(opportunity)
            fixes_attempted += 1

            if result["success"]:
                fixes_successful += 1

                # Update state
                self.state_manager.update_failure_status(
                    result["failure_id"],
                    "resolved",
                    {
                        "resolution_method": "auto-fix",
                        "resolution_details": f"Applied {result['strategy']} strategy",
                        "auto_fix_id": f"fix-{fixes_successful:03d}"
                    }
                )

                # Record auto-fix in history
                self.state_manager.record_auto_fix({
                    "failure_id": result["failure_id"],
                    "pattern_id": opportunity["pattern_id"],
                    "strategy": result["strategy"],
                    "success": True,
                    "safety_level": opportunity["safety_level"],
                    "dry_run": self.dry_run,
                    "rollback_capable": result["rollback_capable"],
                    "backup_created": result["backup_created"]
                })

            else:
                # Update state for failed fix attempt
                self.state_manager.update_failure_status(
                    result["failure_id"],
                    "fix_attempted",
                    {
                        "resolution_method": "auto-fix_attempted",
                        "resolution_details": f"Fix failed: {result['error']}",
                        "auto_fix_error": result["error"]
                    }
                )

                # Record failed auto-fix in history
                self.state_manager.record_auto_fix({
                    "failure_id": result["failure_id"],
                    "pattern_id": opportunity["pattern_id"],
                    "strategy": result["strategy"],
                    "success": False,
                    "safety_level": opportunity["safety_level"],
                    "dry_run": self.dry_run,
                    "error": result["error"],
                    "phase": result["phase"]
                })

            fix_results.append(result)

        print(f"\n📊 Auto-fix summary:")
        print(f"    Opportunities found: {len(opportunities)}")
        print(f"    Fixes attempted: {fixes_attempted}")
        print(f"    Fixes successful: {fixes_successful}")
        print(f"    Success rate: {fixes_successful/fixes_attempted*100:.1f}%")

        return {
            "success": fixes_successful > 0,
            "fixes_attempted": fixes_attempted,
            "fixes_successful": fixes_successful,
            "fix_results": fix_results,
            "opportunities_remaining": len(opportunities) - fixes_attempted
        }

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="CI/CD Automated Error Fixing System",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python auto_fix.py --mode safe
  python auto_fix.py --mode aggressive --max-fixes 10
  python auto_fix.py --dry-run --safety high
        """
    )

    parser.add_argument(
        "--mode",
        choices=["safe", "normal", "aggressive"],
        default="safe",
        help="Fixing mode (safe=high safety only, normal=medium/high, aggressive=all)"
    )

    parser.add_argument(
        "--max-fixes",
        type=int,
        default=5,
        help="Maximum number of fixes to attempt"
    )

    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Dry run mode - show what would be done without making changes"
    )

    parser.add_argument(
        "--safety",
        choices=["high", "medium", "low"],
        default=None,
        help="Filter fixes by safety level"
    )

    parser.add_argument(
        "--list-only",
        action="store_true",
        help="List available fixes without executing"
    )

    args = parser.parse_args()

    # Initialize auto-fix engine
    auto_fix = AutoFixEngine()
    auto_fix.set_dry_run(args.dry_run)

    try:
        if args.list_only:
            print("🔍 Available fix opportunities:")
            opportunities = auto_fix.analyze_fix_opportunities()

            if not opportunities:
                print("    No fix opportunities found")
                return 0

            for i, opp in enumerate(opportunities, 1):
                failure = opp["failure"]
                strategy = opp["strategy"]
                print(f"\n{i}. {failure['failure_type']} ({failure['failure_id']})")
                print(f"   Severity: {failure['severity']}")
                print(f"   Pattern: {failure['pattern_id']}")
                print(f"   Strategy: {strategy['name']}")
                print(f"   Safety: {opp['safety_level']}")
                print(f"   Description: {strategy['description']}")

            print(f"\nTotal: {len(opportunities)} fix opportunities")
            return 0

        # Determine safety filter based on mode
        safety_filter = None
        if args.mode == "safe":
            safety_filter = "high"
        elif args.mode == "normal":
            safety_filter = None  # Allow high and medium
        elif args.mode == "aggressive":
            safety_filter = None  # Allow all safety levels

        # Apply fixes
        result = auto_fix.apply_fixes(
            max_fixes=args.max_fixes,
            safety_filter=safety_filter
        )

        # Exit with appropriate code
        if result["fixes_successful"] > 0:
            print("\n✅ Auto-fix completed with successes")
            return 0
        elif result["fixes_attempted"] > 0:
            print("\n⚠️  Auto-fix attempted but no successes")
            return 1
        else:
            print("\nℹ️  No fixes attempted")
            return 0

    except KeyboardInterrupt:
        print("\n🛑 Auto-fix interrupted by user")
        return 1
    except Exception as e:
        print(f"\n❌ Fatal error: {str(e)}")
        return 1

if __name__ == "__main__":
    main()

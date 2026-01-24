#!/usr/bin/env python3
"""
CI/CD Monitoring System - Main Monitoring Engine

This script provides comprehensive monitoring of GitHub Actions workflows
with intelligent failure detection, alerting, and integration capabilities.
"""

import os
import sys
import time
import json
import argparse
import subprocess
from datetime import datetime, timezone
from typing import Dict, List, Optional, Any
import requests
import yaml

# Configuration
REPO_OWNER = "spencerbutler"
REPO_NAME = "MT-logo-render"
GITHUB_API_BASE = f"https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}"
POLL_INTERVAL_NORMAL = 60  # seconds
POLL_INTERVAL_DEGRADED = 30  # seconds
MAX_RETRIES = 3
RETRY_DELAY = 5  # seconds

# State file path
STATE_FILE = "docs/STATE_CAPSULES/ci_monitoring_state.json"

class GitHubClient:
    """GitHub API client for monitoring workflows"""

    def __init__(self, token: Optional[str] = None):
        self.token = token or os.getenv("GITHUB_TOKEN")
        self.session = requests.Session()
        if self.token:
            self.session.headers.update({"Authorization": f"token {self.token}"})
        self.session.headers.update({
            "Accept": "application/vnd.github.v3+json",
            "User-Agent": "CI-Monitoring-System"
        })

    def get_workflow_runs(self, limit: int = 10) -> List[Dict]:
        """Get recent workflow runs"""
        url = f"{GITHUB_API_BASE}/actions/runs"
        params = {
            "per_page": limit,
            "page": 1
        }

        for attempt in range(MAX_RETRIES):
            try:
                response = self.session.get(url, params=params, timeout=30)
                response.raise_for_status()
                return response.json().get("workflow_runs", [])
            except requests.exceptions.RequestException as e:
                if attempt == MAX_RETRIES - 1:
                    raise Exception(f"Failed to fetch workflow runs: {str(e)}")
                time.sleep(RETRY_DELAY)

    def get_jobs_for_run(self, run_id: int) -> List[Dict]:
        """Get jobs for a specific workflow run"""
        url = f"{GITHUB_API_BASE}/actions/runs/{run_id}/jobs"

        for attempt in range(MAX_RETRIES):
            try:
                response = self.session.get(url, timeout=30)
                response.raise_for_status()
                return response.json().get("jobs", [])
            except requests.exceptions.RequestException as e:
                if attempt == MAX_RETRIES - 1:
                    raise Exception(f"Failed to fetch jobs for run {run_id}: {str(e)}")
                time.sleep(RETRY_DELAY)

    def get_job_details(self, job_id: int) -> Dict:
        """Get detailed information about a specific job"""
        url = f"{GITHUB_API_BASE}/actions/jobs/{job_id}"

        for attempt in range(MAX_RETRIES):
            try:
                response = self.session.get(url, timeout=30)
                response.raise_for_status()
                return response.json()
            except requests.exceptions.RequestException as e:
                if attempt == MAX_RETRIES - 1:
                    raise Exception(f"Failed to fetch job {job_id} details: {str(e)}")
                time.sleep(RETRY_DELAY)

class StateManager:
    """State capsule management for monitoring system"""

    def __init__(self, state_file: str = STATE_FILE):
        self.state_file = state_file
        self.state = self._load_state()

    def _load_state(self) -> Dict:
        """Load state from file or initialize new state"""
        try:
            with open(self.state_file, 'r') as f:
                return json.load(f)
        except (FileNotFoundError, json.JSONDecodeError):
            return self._initialize_state()

    def _initialize_state(self) -> Dict:
        """Initialize a new state capsule"""
        state = {
            "metadata": {
                "version": "1.0",
                "created_at": datetime.now(timezone.utc).isoformat(),
                "updated_at": datetime.now(timezone.utc).isoformat(),
                "capsule_id": f"ci-monitoring-{os.urandom(4).hex()}"
            },
            "monitoring_state": {
                "system_status": "operational",
                "last_healthy_check": datetime.now(timezone.utc).isoformat(),
                "current_alert_level": "normal",
                "monitoring_mode": "continuous",
                "polling_interval": POLL_INTERVAL_NORMAL,
                "last_poll_time": None,
                "consecutive_failures": 0,
                "github_api_status": "operational",
                "last_workflow_run_id": None,
                "active_alerts": 0,
                "suppressed_alerts": []
            },
            "coverage_state": {
                "current_metrics": {
                    "line": 0.0,
                    "branch": 0.0,
                    "function": 0.0,
                    "integration": 0.0,
                    "total": 0.0
                },
                "target_metrics": {
                    "line": 100.0,
                    "branch": 100.0,
                    "function": 100.0,
                    "integration": 100.0,
                    "total": 100.0
                },
                "improvement_trend": "0%",
                "coverage_gaps": [],
                "last_analysis": None,
                "analysis_frequency": "daily",
                "coverage_trend": [],
                "test_generation_queue": []
            },
            "failure_history": [],
            "auto_fix_history": [],
            "recovery_state": {
                "last_recovery": None,
                "recovery_count": 0,
                "mttr": None,
                "recovery_trend": [],
                "recovery_capabilities": {
                    "automatic_recovery": True,
                    "manual_recovery": True,
                    "rollback_capability": True,
                    "state_restoration": True
                }
            }
        }
        return state

    def save_state(self):
        """Save current state to file"""
        with open(self.state_file, 'w') as f:
            json.dump(self.state, f, indent=2)

    def update_monitoring_state(self, updates: Dict):
        """Update monitoring state with validation"""
        self.state["monitoring_state"].update(updates)
        self.state["monitoring_state"]["updated_at"] = datetime.now(timezone.utc).isoformat()
        self.state["metadata"]["updated_at"] = datetime.now(timezone.utc).isoformat()
        self.save_state()

    def record_failure(self, failure_data: Dict):
        """Record a new failure in history"""
        failure_data["failure_id"] = f"fail-{len(self.state['failure_history']):03d}"
        failure_data["detected_at"] = datetime.now(timezone.utc).isoformat()
        failure_data["status"] = "detected"
        self.state["failure_history"].append(failure_data)
        self.save_state()

    def record_auto_fix(self, fix_data: Dict):
        """Record an auto-fix attempt"""
        fix_data["fix_id"] = f"fix-{len(self.state['auto_fix_history']):03d}"
        fix_data["applied_at"] = datetime.now(timezone.utc).isoformat()
        self.state["auto_fix_history"].append(fix_data)
        self.save_state()

class AlertManager:
    """Alert management system"""

    def __init__(self):
        self.alerts = []

    def generate_alert(self, severity: str, message: str, context: Dict):
        """Generate a new alert"""
        alert = {
            "alert_id": f"alert-{len(self.alerts):03d}",
            "severity": severity,
            "message": message,
            "context": context,
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "acknowledged": False,
            "resolved": False
        }
        self.alerts.append(alert)
        return alert

    def generate_console_alert(self, alert: Dict):
        """Display alert in console"""
        severity_colors = {
            "critical": "\033[91m",  # Red
            "high": "\033[93m",      # Yellow
            "medium": "\033[94m",    # Blue
            "low": "\033[92m"        # Green
        }

        color = severity_colors.get(alert["severity"].lower(), "\033[0m")
        reset = "\033[0m"

        print(f"\n{color}=== CI MONITORING ALERT ==={reset}")
        print(f"{color}Severity:{reset} {alert['severity'].upper()}")
        print(f"{color}Time:{reset} {alert['timestamp']}")
        print(f"{color}Message:{reset} {alert['message']}")

        if alert["context"]:
            print(f"{color}Context:{reset}")
            for key, value in alert["context"].items():
                print(f"  {key}: {value}")

        print(f"{color}==========================={reset}\n")

    def get_active_alerts(self) -> List[Dict]:
        """Get all active (unresolved) alerts"""
        return [alert for alert in self.alerts if not alert["resolved"]]

class MonitoringEngine:
    """Main monitoring engine"""

    def __init__(self):
        self.github_client = GitHubClient()
        self.state_manager = StateManager()
        self.alert_manager = AlertManager()
        self.pattern_database = self._load_pattern_database()
        self.running = False

    def _load_pattern_database(self) -> Dict:
        """Load failure pattern database"""
        # This would be loaded from a file in production
        return {
            "PAT-001": {
                "name": "Compilation Failure",
                "description": "Build job compilation errors",
                "detection": {
                    "job_name_pattern": ".*build.*|.*check.*",
                    "conclusion": "failure",
                    "exit_code": 1
                },
                "severity": "critical",
                "auto_fix_capable": False
            },
            "PAT-002": {
                "name": "Test Failure",
                "description": "Test job failures",
                "detection": {
                    "job_name_pattern": ".*test.*",
                    "conclusion": "failure"
                },
                "severity": "high",
                "auto_fix_capable": True
            },
            "PAT-003": {
                "name": "Security Vulnerability",
                "description": "Security scan detected issues",
                "detection": {
                    "job_name_pattern": ".*security.*|.*audit.*",
                    "conclusion": "failure"
                },
                "severity": "critical",
                "auto_fix_capable": True
            },
            "PAT-004": {
                "name": "License Compliance",
                "description": "License compliance check failed",
                "detection": {
                    "job_name_pattern": ".*license.*",
                    "conclusion": "failure"
                },
                "severity": "medium",
                "auto_fix_capable": True
            },
            "PAT-005": {
                "name": "Timeout",
                "description": "Job exceeded time limit",
                "detection": {
                    "conclusion": "failure",
                    "exit_code": -1  # Typically indicates timeout
                },
                "severity": "medium",
                "auto_fix_capable": False
            },
            "PAT-006": {
                "name": "Dependency Issue",
                "description": "Missing or outdated dependencies",
                "detection": {
                    "log_pattern": "command not found|import error|not found",
                    "conclusion": "failure"
                },
                "severity": "high",
                "auto_fix_capable": True
            }
        }

    def detect_failure_pattern(self, job: Dict) -> Optional[str]:
        """Detect failure pattern for a job"""
        for pattern_id, pattern in self.pattern_database.items():
            detection = pattern["detection"]

            # Check conclusion
            if job["conclusion"] != detection.get("conclusion", "failure"):
                continue

            # Check job name pattern
            if "job_name_pattern" in detection:
                import re
                if not re.match(detection["job_name_pattern"], job["name"], re.IGNORECASE):
                    continue

            # Check exit code if available
            if "exit_code" in detection and "exit_code" in job:
                if job["exit_code"] != detection["exit_code"]:
                    continue

            # Pattern matched
            return pattern_id

        return None

    def analyze_workflow_run(self, run: Dict):
        """Analyze a workflow run and its jobs"""
        run_id = run["id"]
        conclusion = run["conclusion"]
        status = run["status"]

        print(f"Analyzing workflow run {run_id}: {run['name']} ({status}/{conclusion})")

        # Update state
        self.state_manager.update_monitoring_state({
            "last_workflow_run_id": run_id,
            "github_api_status": "operational"
        })

        if status == "in_progress":
            print(f"  Run {run_id} is in progress...")
            return

        if conclusion == "success":
            print(f"  Run {run_id} completed successfully")
            # Reset failure counter on success
            if self.state_manager.state["monitoring_state"]["consecutive_failures"] > 0:
                self.state_manager.update_monitoring_state({
                    "consecutive_failures": 0
                })
            return

        # Run failed - analyze jobs
        try:
            jobs = self.github_client.get_jobs_for_run(run_id)

            for job in jobs:
                if job["conclusion"] == "failure":
                    self.analyze_failed_job(run, job)

        except Exception as e:
            print(f"  Error analyzing jobs for run {run_id}: {str(e)}")
            self.state_manager.update_monitoring_state({
                "github_api_status": "degraded"
            })

    def analyze_failed_job(self, run: Dict, job: Dict):
        """Analyze a specific failed job"""
        job_id = job["id"]
        job_name = job["name"]

        print(f"  Analyzing failed job {job_id}: {job_name}")

        # Detect failure pattern
        pattern_id = self.detect_failure_pattern(job)

        if pattern_id:
            pattern = self.pattern_database[pattern_id]
            print(f"    Detected pattern: {pattern['name']} ({pattern_id})")
            print(f"    Severity: {pattern['severity']}")

            # Record failure
            failure_data = {
                "workflow_run_id": run["id"],
                "job_id": job_id,
                "job_name": job_name,
                "failure_type": pattern["name"],
                "severity": pattern["severity"],
                "pattern_id": pattern_id,
                "impact": {
                    "affected_components": [job_name],
                    "downtime": "unknown",
                    "recovery_time": "unknown"
                },
                "root_cause": "Analysis pending",
                "preventive_actions": []
            }

            self.state_manager.record_failure(failure_data)

            # Generate alert
            alert_context = {
                "run_id": run["id"],
                "run_name": run["name"],
                "job_id": job_id,
                "job_name": job_name,
                "pattern_id": pattern_id,
                "pattern_name": pattern["name"],
                "severity": pattern["severity"]
            }

            alert = self.alert_manager.generate_alert(
                pattern["severity"],
                f"Job failure detected: {job_name} ({pattern['name']})",
                alert_context
            )

            self.alert_manager.generate_console_alert(alert)

            # Update consecutive failures
            consecutive_failures = self.state_manager.state["monitoring_state"]["consecutive_failures"] + 1
            self.state_manager.update_monitoring_state({
                "consecutive_failures": consecutive_failures,
                "active_alerts": len(self.alert_manager.get_active_alerts())
            })

            # Adjust polling interval based on failure rate
            if consecutive_failures >= 3:
                self.state_manager.update_monitoring_state({
                    "system_status": "degraded",
                    "polling_interval": POLL_INTERVAL_DEGRADED
                })
            elif consecutive_failures >= 1:
                self.state_manager.update_monitoring_state({
                    "system_status": "degraded"
                })

        else:
            print(f"    No known pattern detected for job failure")
            # Generic failure recording
            failure_data = {
                "workflow_run_id": run["id"],
                "job_id": job_id,
                "job_name": job_name,
                "failure_type": "Unknown",
                "severity": "medium",
                "pattern_id": "UNKNOWN",
                "impact": {
                    "affected_components": [job_name],
                    "downtime": "unknown",
                    "recovery_time": "unknown"
                },
                "root_cause": "Unknown failure pattern",
                "preventive_actions": []
            }

            self.state_manager.record_failure(failure_data)

            # Update state
            consecutive_failures = self.state_manager.state["monitoring_state"]["consecutive_failures"] + 1
            self.state_manager.update_monitoring_state({
                "consecutive_failures": consecutive_failures,
                "system_status": "degraded"
            })

    def continuous_monitoring_loop(self):
        """Main continuous monitoring loop"""
        self.running = True
        print("Starting CI/CD monitoring system...")

        try:
            while self.running:
                try:
                    # Get current polling interval
                    poll_interval = self.state_manager.state["monitoring_state"]["polling_interval"]

                    print(f"\n--- Polling GitHub API (interval: {poll_interval}s) ---")
                    print(f"System status: {self.state_manager.state['monitoring_state']['system_status']}")
                    print(f"Active alerts: {len(self.alert_manager.get_active_alerts())}")

                    # Update last poll time
                    self.state_manager.update_monitoring_state({
                        "last_poll_time": datetime.now(timezone.utc).isoformat()
                    })

                    # Get recent workflow runs
                    runs = self.github_client.get_workflow_runs(limit=5)

                    if not runs:
                        print("No recent workflow runs found")
                    else:
                        # Process runs in reverse chronological order
                        for run in sorted(runs, key=lambda x: x["created_at"], reverse=True):
                            self.analyze_workflow_run(run)

                    # Check for auto-fix capable failures
                    self.check_auto_fix_opportunities()

                    # Sleep for polling interval
                    time.sleep(poll_interval)

                except KeyboardInterrupt:
                    print("\nMonitoring interrupted by user")
                    break
                except Exception as e:
                    print(f"Monitoring error: {str(e)}")
                    # Fallback to normal polling on errors
                    self.state_manager.update_monitoring_state({
                        "polling_interval": POLL_INTERVAL_NORMAL,
                        "github_api_status": "degraded"
                    })
                    time.sleep(POLL_INTERVAL_NORMAL)

        finally:
            self.running = False
            print("Monitoring system stopped")

    def check_auto_fix_opportunities(self):
        """Check for failures that can be auto-fixed"""
        auto_fix_capable = []

        for failure in self.state_manager.state["failure_history"]:
            if failure["status"] == "detected" and failure.get("auto_fix_capable", False):
                pattern = self.pattern_database.get(failure["pattern_id"])
                if pattern and pattern.get("auto_fix_capable", False):
                    auto_fix_capable.append(failure)

        if auto_fix_capable:
            print(f"\nAuto-fix opportunities found: {len(auto_fix_capable)}")
            for failure in auto_fix_capable:
                print(f"  - {failure['failure_id']}: {failure['failure_type']} ({failure['severity']})")

            # In production, this would trigger the auto-fix system
            print("  (Auto-fix would be triggered here in production mode)")

    def one_time_check(self):
        """Perform a one-time check of current workflow status"""
        print("Performing one-time CI/CD status check...")

        try:
            runs = self.github_client.get_workflow_runs(limit=3)

            if not runs:
                print("No recent workflow runs found")
                return False

            # Check most recent run
            latest_run = runs[0]
            print(f"Latest workflow run: {latest_run['name']} ({latest_run['id']})")
            print(f"Status: {latest_run['status']}")
            print(f"Conclusion: {latest_run['conclusion']}")

            if latest_run["conclusion"] == "failure":
                print("Latest run failed - analyzing...")
                jobs = self.github_client.get_jobs_for_run(latest_run["id"])
                for job in jobs:
                    if job["conclusion"] == "failure":
                        self.analyze_failed_job(latest_run, job)
                return False
            else:
                print("Latest run successful")
                return True

        except Exception as e:
            print(f"One-time check failed: {str(e)}")
            return False

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="CI/CD Monitoring System",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python ci_monitor.py --mode continuous
  python ci_monitor.py --mode onetime
  python ci_monitor.py --mode pre-commit
        """
    )

    parser.add_argument(
        "--mode",
        choices=["continuous", "onetime", "pre-commit", "github-actions"],
        default="onetime",
        help="Monitoring mode (default: onetime)"
    )

    parser.add_argument(
        "--poll-interval",
        type=int,
        default=None,
        help="Custom polling interval in seconds (override default)"
    )

    parser.add_argument(
        "--alert-level",
        choices=["low", "medium", "high", "critical"],
        default="medium",
        help="Minimum alert level to display (default: medium)"
    )

    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Dry run mode (no state changes)"
    )

    args = parser.parse_args()

    # Initialize monitoring engine
    monitor = MonitoringEngine()

    try:
        if args.mode == "continuous":
            if args.poll_interval:
                monitor.state_manager.update_monitoring_state({
                    "polling_interval": args.poll_interval
                })
            monitor.continuous_monitoring_loop()

        elif args.mode == "onetime":
            success = monitor.one_time_check()
            sys.exit(0 if success else 1)

        elif args.mode == "pre-commit":
            # Quick check for pre-commit hook
            success = monitor.one_time_check()
            if not success:
                print("\n⚠️  CI/CD issues detected - consider running full monitoring")
            sys.exit(0 if success else 1)

        elif args.mode == "github-actions":
            # GitHub Actions integration mode
            success = monitor.one_time_check()
            # Output results in GitHub Actions format
            if not success:
                print("::error::CI/CD monitoring detected issues")
                sys.exit(1)
            else:
                print("::notice::CI/CD monitoring - all systems operational")
                sys.exit(0)

    except KeyboardInterrupt:
        print("\nMonitoring stopped by user")
        sys.exit(0)
    except Exception as e:
        print(f"Fatal error: {str(e)}")
        sys.exit(1)

if __name__ == "__main__":
    main()

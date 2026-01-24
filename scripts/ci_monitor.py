#!/usr/bin/env python3
"""
🎯 MT-logo-render CI/CD Monitor
Real-time CI pipeline status with fancy progress indicators!

Adapted from tick-task for HEE platform consistency.
"""

import time
import requests
import json
from datetime import datetime
import sys
import os

# GitHub API configuration
GITHUB_TOKEN = os.getenv('GITHUB_TOKEN')  # Optional: for higher rate limits
REPO_OWNER = "spencerbutler"
REPO_NAME = "MT-logo-render"
BRANCH = "main"  # Default to main branch

# ANSI color codes for fancy output
class Colors:
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    MAGENTA = '\033[95m'
    CYAN = '\033[96m'
    WHITE = '\033[97m'
    BOLD = '\033[1m'
    END = '\033[0m'

# Status emojis
STATUS_EMOJIS = {
    'COMPLETED': {'SUCCESS': '✅', 'FAILURE': '❌', 'NEUTRAL': '⚪'},
    'IN_PROGRESS': '🔄',
    'QUEUED': '⏳',
    'REQUESTED': '📋',
    'WAITING': '⏸️',
    'PENDING': '🟡'
}

# Job status mapping
JOB_STATUS = {
    'success': f"{Colors.GREEN}✅ SUCCESS{Colors.END}",
    'failure': f"{Colors.RED}❌ FAILED{Colors.END}",
    'in_progress': f"{Colors.BLUE}🔄 RUNNING{Colors.END}",
    'queued': f"{Colors.YELLOW}⏳ QUEUED{Colors.END}",
    'neutral': f"{Colors.CYAN}⚪ NEUTRAL{Colors.END}",
    'skipped': f"{Colors.MAGENTA}⏭️ SKIPPED{Colors.END}"
}

def clear_screen():
    """Clear terminal screen"""
    os.system('clear' if os.name == 'posix' else 'cls')

def format_duration(started_at, completed_at=None):
    """Format duration between timestamps"""
    if not started_at:
        return "N/A"

    start = datetime.fromisoformat(started_at.replace('Z', '+00:00'))
    end = datetime.fromisoformat(completed_at.replace('Z', '+00:00')) if completed_at else datetime.now()

    duration = end - start
    total_seconds = int(duration.total_seconds())

    if total_seconds < 60:
        return f"{total_seconds}s"
    elif total_seconds < 3600:
        return f"{total_seconds // 60}m {total_seconds % 60}s"
    else:
        return f"{total_seconds // 3600}h {(total_seconds % 3600) // 60}m"

def get_workflow_runs():
    """Fetch latest workflow runs from GitHub API"""
    url = f"https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/actions/runs"
    params = {
        'branch': BRANCH,
        'per_page': 5  # Get last 5 runs
    }
    headers = {'Accept': 'application/vnd.github.v3+json'}
    if GITHUB_TOKEN:
        headers['Authorization'] = f'token {GITHUB_TOKEN}'

    try:
        response = requests.get(url, params=params, headers=headers)
        response.raise_for_status()
        return response.json()['workflow_runs']
    except requests.RequestException as e:
        print(f"{Colors.RED}❌ Error fetching workflow runs: {e}{Colors.END}")
        return []

def get_workflow_jobs(run_id):
    """Fetch jobs for a specific workflow run"""
    url = f"https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/actions/runs/{run_id}/jobs"
    headers = {'Accept': 'application/vnd.github.v3+json'}
    if GITHUB_TOKEN:
        headers['Authorization'] = f'token {GITHUB_TOKEN}'

    try:
        response = requests.get(url, headers=headers)
        response.raise_for_status()
        return response.json()['jobs']
    except requests.RequestException as e:
        print(f"{Colors.RED}❌ Error fetching workflow jobs: {e}{Colors.END}")
        return []

def display_header():
    """Display fancy header"""
    print(f"""
{Colors.CYAN}{Colors.BOLD}
╔══════════════════════════════════════════════════════════════╗
║                    🚀 MT-logo-render CI/CD Monitor                ║
║                    HEE Platform Security Pipeline                  ║
╚══════════════════════════════════════════════════════════════╝{Colors.END}
""")

def display_workflow_status(workflow_runs):
    """Display workflow run status"""
    if not workflow_runs:
        print(f"{Colors.YELLOW}⏳ No workflow runs found for branch '{BRANCH}'{Colors.END}")
        return None

    latest_run = workflow_runs[0]
    run_id = latest_run['id']
    status = latest_run['status']
    conclusion = latest_run.get('conclusion', 'unknown')
    created_at = latest_run['created_at']
    updated_at = latest_run['updated_at']

    # Status emoji and color
    if status == 'completed':
        if conclusion == 'success':
            status_display = f"{Colors.GREEN}✅ SUCCESS{Colors.END}"
        elif conclusion == 'failure':
            status_display = f"{Colors.RED}❌ FAILURE{Colors.END}"
        else:
            status_display = f"{Colors.YELLOW}⚠️ {conclusion.upper()}{Colors.END}"
    elif status == 'in_progress':
        status_display = f"{Colors.BLUE}🔄 IN PROGRESS{Colors.END}"
    else:
        status_display = f"{Colors.YELLOW}⏳ {status.upper()}{Colors.END}"

    print(f"""
{Colors.BOLD}Latest Workflow Run:{Colors.END}
├── Run ID: {Colors.CYAN}{run_id}{Colors.END}
├── Branch: {Colors.CYAN}{BRANCH}{Colors.END}
├── Status: {status_display}
├── Started: {Colors.WHITE}{created_at}{Colors.END}
└── Updated: {Colors.WHITE}{updated_at}{Colors.END}
""")

    return run_id, status, conclusion

def display_job_status(jobs):
    """Display detailed job status"""
    if not jobs:
        print(f"{Colors.YELLOW}📋 No jobs found{Colors.END}")
        return

    print(f"{Colors.BOLD}Job Status:{Colors.END}")

    for job in jobs:
        name = job['name']
        status = job['status']
        conclusion = job.get('conclusion', 'unknown')
        started_at = job.get('started_at')
        completed_at = job.get('completed_at')

        # Format job status
        if status == 'completed':
            job_status = JOB_STATUS.get(conclusion, f"{Colors.CYAN}❓ {conclusion.upper()}{Colors.END}")
        else:
            job_status = JOB_STATUS.get(status, f"{Colors.YELLOW}❓ {status.upper()}{Colors.END}")

        # Duration
        duration = format_duration(started_at, completed_at)

        print(f"├── {Colors.WHITE}{name}{Colors.END}: {job_status} ({duration})")

def display_security_status():
    """Display HEE security status"""
    print(f"\n{Colors.BOLD}HEE Security Status:{Colors.END}")

    # Check if security scanner exists and run it
    scanner_path = os.path.join(os.path.dirname(__file__), 'security_scanner.py')
    if os.path.exists(scanner_path):
        try:
            import subprocess
            result = subprocess.run([sys.executable, scanner_path, '--directory', '..', '--output-format', 'json'],
                                  capture_output=True, text=True, cwd=os.path.dirname(scanner_path))

            if result.returncode == 0:
                print(f"├── {Colors.GREEN}🔒 Security Scan: PASSED{Colors.END}")
            else:
                security_data = json.loads(result.stdout) if result.stdout.strip() else []
                print(f"├── {Colors.RED}🔒 Security Scan: {len(security_data)} ISSUES FOUND{Colors.END}")
        except Exception as e:
            print(f"├── {Colors.YELLOW}🔒 Security Scan: ERROR - {e}{Colors.END}")
    else:
        print(f"├── {Colors.YELLOW}🔒 Security Scanner: NOT FOUND{Colors.END}")

def main():
    """Main monitoring loop"""
    import argparse

    # Declare global variables first
    global BRANCH, REPO_OWNER, REPO_NAME

    parser = argparse.ArgumentParser(description='MT-logo-render CI/CD Monitor')
    parser.add_argument('--branch', '-b', default=BRANCH,
                       help=f'Branch to monitor (default: {BRANCH})')
    parser.add_argument('--owner', default=REPO_OWNER,
                       help=f'Repository owner (default: {REPO_OWNER})')
    parser.add_argument('--repo', default=REPO_NAME,
                       help=f'Repository name (default: {REPO_NAME})')
    parser.add_argument('--security-check', action='store_true',
                       help='Include HEE security status checks')

    args = parser.parse_args()

    # Update globals from args
    BRANCH = args.branch
    REPO_OWNER = args.owner
    REPO_NAME = args.repo

    print(f"{Colors.MAGENTA}🎯 Starting MT-logo-render CI/CD Monitor... Press Ctrl+C to exit{Colors.END}")
    print(f"{Colors.CYAN}📊 Monitoring branch: {BRANCH}{Colors.END}")
    print(f"{Colors.CYAN}🏠 Repository: {REPO_OWNER}/{REPO_NAME}{Colors.END}")
    if args.security_check:
        print(f"{Colors.CYAN}🔒 HEE Security monitoring: ENABLED{Colors.END}")

    last_status = None

    try:
        while True:
            clear_screen()
            display_header()

            workflow_runs = get_workflow_runs()
            result = display_workflow_status(workflow_runs)

            if result:
                run_id, status, conclusion = result
                jobs = get_workflow_jobs(run_id)
                display_job_status(jobs)

                # Check if status changed
                current_status = f"{status}:{conclusion}"
                if last_status != current_status and last_status is not None:
                    print(f"\n{Colors.GREEN}🔄 Status changed: {last_status} → {current_status}{Colors.END}")
                last_status = current_status

                # Display security status if requested
                if args.security_check:
                    display_security_status()

                # Exit if workflow completed
                if status == 'completed':
                    print(f"\n{Colors.GREEN}🎉 Workflow completed! Final status: {conclusion.upper()}{Colors.END}")
                    if conclusion == 'success':
                        print(f"{Colors.GREEN}✅ All checks passed! HEE security validated.{Colors.END}")
                    else:
                        print(f"{Colors.RED}❌ Some checks failed. Check the details above.{Colors.END}")
                    break

            print(f"\n{Colors.CYAN}🔄 Refreshing in 10 seconds... (Ctrl+C to exit){Colors.END}")
            time.sleep(10)

    except KeyboardInterrupt:
        print(f"\n{Colors.YELLOW}👋 Monitoring stopped by user{Colors.END}")

if __name__ == "__main__":
    main()

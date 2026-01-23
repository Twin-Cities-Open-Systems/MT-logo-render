# GitHub CI Troubleshooting Guide

This guide documents the systematic approach for identifying and resolving CI/CD test failures using GitHub CLI (gh) and jq for selective querying.

## Overview

When CI/CD tests fail, use this structured approach to quickly identify root causes and implement fixes. This method leverages GitHub's API with jq filtering for precise, targeted analysis.

## Prerequisites

- GitHub CLI (`gh`) installed and authenticated
- jq installed for JSON parsing
- Access to the repository with appropriate permissions

## Step-by-Step Troubleshooting Process

### 1. Identify Failed Workflow Run

Get the latest workflow run ID:
```bash
# Get the most recent workflow run
gh api repos/spencerbutler/MT-logo-render/actions/runs --per_page=1 | jq '.workflow_runs[0].id'

# Get all recent runs with status
gh api repos/spencerbutler/MT-logo-render/actions/runs | jq '.workflow_runs[] | {id: .id, name: .name, status: .status, conclusion: .conclusion, created_at: .created_at}'
```

### 2. List Failed Jobs in a Specific Run

Filter for failed jobs only:
```bash
# Replace RUN_ID with actual run ID
RUN_ID=21304793158

# Get all jobs from the run
gh api repos/spencerbutler/MT-logo-render/actions/runs/$RUN_ID/jobs | jq '.jobs[] | {name: .name, conclusion: .conclusion, id: .id}'

# Filter only failed jobs
gh api repos/spencerbutler/MT-logo-render/actions/runs/$RUN_ID/jobs | jq '.jobs[] | select(.conclusion == "failure") | {name: .name, id: .id}'
```

### 3. Analyze Specific Failed Job

Get detailed job information:
```bash
# Replace JOB_ID with actual job ID
JOB_ID=61330289698

# Get job details
gh api repos/spencerbutler/MT-logo-render/actions/jobs/$JOB_ID

# Check job steps to identify which step failed
gh api repos/spencerbutler/MT-logo-render/actions/jobs/$JOB_ID | jq '.steps[] | {name: .name, status: .status, conclusion: .conclusion, number: .number}'
```

### 4. Common Failure Patterns & Solutions

#### Pattern 1: Deprecated GitHub Actions
**Error**: "uses a deprecated version of `actions/upload-artifact: v3`"
**Solution**: Update to latest version (v4)
```yaml
# Before
uses: actions/upload-artifact@v3

# After  
uses: actions/upload-artifact@v4
```

#### Pattern 2: Inverted Test Logic
**Error**: Tests failing on expected invalid inputs
**Solution**: Fix validation logic to only fail on unexpected results
```python
# Before (WRONG)
if not result:  # Any invalid recipe causes failure
    print('❌ Invalid recipe found')
    exit(1)

# After (CORRECT)
if result != is_valid:  # Only fail when validation doesn't match expectation
    validation_errors.append(f'Recipe: got {result}, expected {is_valid}')
```

#### Pattern 3: Missing Dependencies
**Error**: "command not found" or import errors
**Solution**: Add missing dependencies to workflow
```yaml
- name: Install dependencies
  run: |
    pip install requests
    cargo install cargo-audit
```

#### Pattern 4: Environment Issues
**Error**: Platform-specific failures
**Solution**: Check matrix configuration and platform compatibility
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta, nightly]
```

### 5. Proactive Monitoring

Monitor current runs:
```bash
# Check for in-progress runs
gh api repos/spencerbutler/MT-logo-render/actions/runs | jq '.workflow_runs[] | select(.status == "in_progress") | {id: .id, name: .name, created_at: .created_at}'

# Check recent run status
gh api repos/spencerbutler/MT-logo-render/actions/runs | jq '.workflow_runs[0] | {id: .id, conclusion: .conclusion, created_at: .created_at}'
```

## Real-World Example

### Problem: Multiple CI Jobs Failing

**Step 1**: Identify the failing run
```bash
gh api repos/spencerbutler/MT-logo-render/actions/runs --per_page=1 | jq '.workflow_runs[0].id'
# Output: 21304793158
```

**Step 2**: List failed jobs
```bash
gh api repos/spencerbutler/MT-logo-render/actions/runs/21304793158/jobs | jq '.jobs[] | select(.conclusion == "failure") | {name: .name, id: .id}'
# Output: Multiple failed jobs including "HEE Recipe Validation"
```

**Step 3**: Analyze specific job
```bash
gh api repos/spencerbutler/MT-logo-render/actions/jobs/61330289698 | jq '.steps[] | {name: .name, status: .status, conclusion: .conclusion}'
# Output: "Validate HEE Recipe Security" step failed
```

**Step 4**: Fix the issue
- Identified inverted test logic
- Fixed validation to only fail on unexpected results
- Updated workflow with corrected logic

**Step 5**: Verify fix
```bash
# Commit and push changes
git add .github/workflows/ci.yml
git commit -m "fix: Correct HEE Recipe Validation logic"
git push origin feature/phase3-core-implementation
```

## Best Practices

1. **Use selective querying**: Filter for specific failures rather than viewing all data
2. **Document patterns**: Keep a record of common failure patterns and their solutions
3. **Test locally**: Verify fixes work before pushing to CI
4. **Monitor proactively**: Check CI status regularly, don't wait for failures
5. **Update dependencies**: Keep GitHub Actions and tools up to date
6. **Clear error messages**: Ensure test failures provide actionable information

## Useful Commands Reference

```bash
# Get latest workflow run
gh api repos/owner/repo/actions/runs --per_page=1 | jq '.workflow_runs[0].id'

# List all jobs in a run
gh api repos/owner/repo/actions/runs/RUN_ID/jobs | jq '.jobs[] | {name: .name, conclusion: .conclusion}'

# Get job details
gh api repos/owner/repo/actions/jobs/JOB_ID

# Filter failed jobs only
gh api repos/owner/repo/actions/runs/RUN_ID/jobs | jq '.jobs[] | select(.conclusion == "failure")'

# Check job steps
gh api repos/owner/repo/actions/jobs/JOB_ID | jq '.steps[] | {name: .name, conclusion: .conclusion}'
```

## Troubleshooting Tips

- **Start broad, then narrow**: Begin with run-level analysis, then drill down to job and step level
- **Look for patterns**: Multiple similar failures often indicate a common root cause
- **Check recent changes**: Review recent commits that might have introduced the issue
- **Use GitHub Actions logs**: For detailed step-by-step execution information
- **Test in isolation**: Run individual steps locally when possible to isolate issues

## Related Documentation

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [GitHub CLI Documentation](https://cli.github.com/manual/gh_api)
- [jq Manual](https://stedolan.github.io/jq/manual/)

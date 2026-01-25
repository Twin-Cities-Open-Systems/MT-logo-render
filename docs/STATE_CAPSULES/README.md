# State Capsules Directory

This directory contains State Capsules for the MT-logo-render project, providing structured handoff documents between agents and chat sessions.

## Directory Structure

```
docs/STATE_CAPSULES/
├── README.md                    # This file
├── 2026-01-23/
│   ├── MT-logo-render-Phase3-CI-Troubleshooting.md
│   └── template.md
├── 2026-01-24/
│   └── [next-session-name].md
└── archive/
    ├── 2026-01-20/
    │   └── [older-capsules].md
    └── 2026-01-15/
        └── [older-capsules].md
```

## Naming Convention

### Date-Based Organization

- **Primary Organization**: By date (YYYY-MM-DD format)
- **Secondary Organization**: By project and session within each date directory
- **File Naming**: `Project-Session-Description.md`

### Examples

- `2026-01-23/MT-logo-render-Phase3-CI-Troubleshooting.md`
- `2026-01-24/MT-logo-render-Phase3-Doctor-Implementation.md`
- `2026-01-25/Market-Thesis-Dashboard-Integration.md`

## Completion Convention

When all tasks in a session capsule are complete:

1. **Rename File**: Prepend `done.` to filename
   - `Project-Phase-Description.md` → `done.Project-Phase-Description.md`
1. **Note**: Only session-specific capsules are renamed. The canonical `CURRENT_TASKS.md` remains unchanged.

## File Format

Each State Capsule follows the standardized YAML format:

```yaml
chat: <project-name> <phase/session>
purpose: <one-sentence objective>
context:
  - Project: <project description>
  - Current Phase: <current phase or milestone>
  - Status: <current status and recent progress>
  - Constraints: <important constraints or requirements>
  - Dependencies: <key dependencies or blockers>

decisions:
  - <specific decision made>
  - <technical choice with rationale>
  - <architectural decision>

open_threads:
  - <unresolved issue or pending task>
  - <dependency or blocker>
  - <next major milestone>

next_chat_bootstrap:
  - <immediate next step>
  - <how to continue current work>
  - <what to investigate or implement>
```

## Usage Guidelines

### For Agents

1. **Check Latest Capsule**: Always check the most recent date directory first
1. **Read Context**: Review the context section to understand current state
1. **Follow Next Steps**: Use the `next_chat_bootstrap` section as your starting point
1. **Update or Create**: Either update the existing capsule or create a new one

### For Project Management

1. **Archive Old Capsules**: Move completed capsules to the `archive/` directory
1. **Maintain Structure**: Keep the date-based organization consistent
1. **Review Regularly**: Use capsules for project retrospectives and planning
1. **Link to PRs**: Reference relevant capsules in pull requests and releases

## Quick Start

### Creating a New State Capsule

1. **Create Date Directory** (if it doesn't exist):

   ```bash
   mkdir -p docs/STATE_CAPSULES/$(date +%Y-%m-%d)
   ```

1. **Copy Template**:

   ```bash
   cp docs/STATE_CAPSULE_TEMPLATE.md docs/STATE_CAPSULES/$(date +%Y-%m-%d)/template.md
   ```

1. **Create New Capsule**:

   ```bash
   cp docs/STATE_CAPSULES/$(date +%Y-%m-%d)/template.md docs/STATE_CAPSULES/$(date +%Y-%m-%d)/Project-Session-Description.md
   ```

1. **Fill in Details**: Replace template placeholders with actual project information

### Finding the Latest State Capsule

```bash
# Find the most recent date directory
LATEST_DATE=$(ls -dt docs/STATE_CAPSULES/*/ | head -1)

# List capsules in that directory
ls -la "$LATEST_DATE"

# Read the latest capsule
cat "$LATEST_DATE"/*.md
```

## Integration with Workflow

### Git Integration

- State Capsules are version controlled with the project
- Include capsule references in commit messages for major transitions
- Use in pull request templates for context preservation

### CI/CD Integration

- Generate capsule summaries in build artifacts
- Include capsule links in deployment notifications
- Use capsule metadata for automated project status updates

### Project Management

- Reference capsules in sprint planning
- Use for knowledge transfer during team changes
- Include in project documentation and wikis

## Benefits

1. **Immediate Context**: New agents can immediately understand current state
1. **Decision Tracking**: Maintain rationale for important choices
1. **Progress Visibility**: Clear view of project progression over time
1. **Reduced Onboarding**: Faster ramp-up for new team members or agents
1. **Knowledge Preservation**: Prevent loss of critical project knowledge

## Maintenance

### Regular Tasks

- **Daily**: Create new capsules for significant sessions
- **Weekly**: Archive old capsules to keep directory clean
- **Monthly**: Review capsule effectiveness and update templates

### Quality Assurance

- Ensure all required fields are filled
- Verify that next steps are actionable
- Check that context is current and accurate
- Validate that decisions are well-documented

## Troubleshooting

### Common Issues

- **Missing Context**: Always include essential background information
- **Vague Next Steps**: Make actions specific and immediately actionable
- **Outdated Information**: Keep capsules current with project state
- **Inconsistent Format**: Always use the standardized template

### Getting Help

- Refer to `docs/STATE_CAPSULE_GUIDE.md` for detailed guidance
- Use the template in `docs/STATE_CAPSULE_TEMPLATE.md` as a reference
- Check previous capsules for examples of good practices

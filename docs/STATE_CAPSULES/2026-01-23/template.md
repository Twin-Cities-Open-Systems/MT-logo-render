# State Capsule Template

Use this template for creating State Capsules to ensure consistent handoffs between agents or chat sessions.

## Template

```yaml
chat: <project-name> <phase/session>
purpose: <one-sentence objective>
context:
  - Project: <project description>
  - Current Phase: <current phase or milestone>
  - Status: <current status and recent progress>
  - Constraints: <important constraints or requirements>
  - Dependencies: <key dependencies or blockers>
  - Tools/Technologies: <key tools, frameworks, or technologies in use>

decisions:
  - <specific decision made with rationale>
  - <technical choice and why it was chosen>
  - <architectural decision and its impact>
  - <any trade-offs that were considered>

open_threads:
  - <unresolved issue or pending task>
  - <dependency or blocker>
  - <next major milestone>
  - <risk or concern that needs attention>
  - <question that needs answering>

next_chat_bootstrap:
  - <immediate next step to take>
  - <how to continue current work>
  - <what to investigate or implement>
  - <priority order for remaining tasks>
```

## Quick Reference

### Required Fields

- `chat`: Name of the current session
- `purpose`: One-sentence objective
- `context`: Essential background information
- `decisions`: Key decisions made
- `open_threads`: Unresolved items
- `next_chat_bootstrap`: Starting points for continuation

### Formatting Rules

- Use YAML format
- Use bullet points for lists
- Be specific and actionable
- Include rationale for decisions
- Prioritize open threads by importance

### Content Guidelines

- **Context**: Include only critical information
- **Decisions**: Focus on impactful choices
- **Open Threads**: Mark dependencies and blockers
- **Next Steps**: Make immediately actionable

## Example Usage

```yaml
chat: MT-logo-render Phase 3 CI Troubleshooting
purpose: Document current state and remaining work for Phase 3 completion
context:
  - Project: MT-logo-render (Rust CLI tool for deterministic logo generation)
  - Current Phase: 3 (Command completion - doctor/list)
  - Status: Stuck on CI failures, Code Quality job failing due to formatting
  - Constraints: Must maintain backward compatibility, follow Rust best practices
  - Dependencies: Rust 1.89, image crate, serde, chrono, sha2, clap
  - Tools/Technologies: GitHub Actions, pre-commit hooks, cargo fmt, clippy

decisions:
  - Use cargo fmt --all before every push to prevent formatting failures
  - Fix HEE Recipe Validation logic to only fail on unexpected results (inverted test logic)
  - Update mdformat to version 0.7.19 to fix GFM plugin compatibility
  - Use GitHub Actions API with jq for selective CI monitoring

open_threads:
  - HEE Security Scan job still failing (needs investigation)
  - License Compliance job failing (needs investigation)
  - Release Build job failing (needs investigation)
  - Pre-commit Hooks job failing (needs investigation)
  - Need to verify all CI jobs pass after formatting fixes
  - Need to complete doctor/list command implementation

next_chat_bootstrap:
  - Continue CI troubleshooting systematically
  - Fix remaining failed jobs one by one
  - Complete doctor/list command implementation
  - Run comprehensive integration tests
  - Prepare for Phase 3 completion and PR merge
```

## Checklist

Before creating a State Capsule, ensure you have:

- [ ] Identified the current project phase and status
- [ ] Documented all key decisions made during the session
- [ ] Listed all unresolved issues and dependencies
- [ ] Defined clear next steps for continuation
- [ ] Used specific, actionable language
- [ ] Included rationale for important decisions
- [ ] Prioritized open threads by importance
- [ ] Made next steps immediately actionable

## Integration

### Project Documentation

- Include State Capsules in project README or documentation
- Reference in PR descriptions for major changes
- Use in release notes for significant milestones

### Team Workflow

- Establish team conventions for State Capsule usage
- Include in handoff procedures between team members
- Use in sprint planning and retrospectives

### Tool Integration

- Create templates in your preferred documentation tool
- Include in project templates and scaffolding
- Automate State Capsule generation where possible

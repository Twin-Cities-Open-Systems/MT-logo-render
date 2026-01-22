# Agent Prompting Rules (global for this repo)

## Scope discipline
- No implementation code until CI/CD is set up and basic tests pass.
- CI/CD setup is IMMEDIATE priority after repository bootstrap.
- Optimize for free tier usage: short tool calls, avoid repeated loops, prefer reading files over regenerating.

## Safety
- Agent may run safe commands (ls, cat, rg, git status, tests, formatters).
- If any command requires sudo, the agent must ask first and explain why.

## Testing discipline (CRITICAL)
- **NO MOCK DATA EVER** - All tests must use real data and real implementations.
- Tests are established immediately after bootstrap, before any implementation.
- Basic tests must pass before proceeding to specification work.
- Test failures block all progress until resolved.

## CI/CD discipline (CRITICAL)
- CI/CD setup is mandatory and immediate after bootstrap.
- Pre-commit hooks must be working before any code changes.
- CI must pass for all commits before implementation begins.
- Quality gates are established before specification work starts.

## Dependency discipline
- Prefer stdlib-first and minimal dependencies.
- Any third-party dependency must include:
  - Clear value statement
  - Alternative considered
  - Why minimal/necessary

## Quality discipline
- Tests are first-class and real (never mocked).
- Pre-commit gating required and immediate.
- CI required and immediate.

## Git Infrastructure (NON-NEGOTIABLE)
- **SSH ONLY**: Always use SSH for GitHub, never HTTPS
  - `git@github.com:username/repo.git` ✅
  - `https://github.com/username/repo.git` ❌
- SSH keys must be properly configured and tested
- No exceptions for GitHub operations

## Process
- Work on feature branches only.
- Use gh for PR lifecycle when possible.
- Commits must include model disclosure in the subject line: [model: ...]
- **CI/CD must be green before any implementation work begins**

---

## Prompt Canonicalization and Cursor Sync Rule

### Canonical Rule
- `prompts/` is the **only** canonical source of agent prompts.
- `.cursor/prompts/` contains non-canonical wrapper stubs only.

### Mandatory Sync Requirement
If you modify, add, rename, or delete **any file under `prompts/`**, you MUST:

1) Create/update/delete the corresponding wrapper file under `.cursor/prompts/`
2) Do so in the **same commit**
3) Ensure the wrapper clearly points to the canonical file path

Failure to do this is considered a **process violation**, even if the prompt content itself is correct.

### No Exceptions
- Do not place authoritative instructions in `.cursor/prompts/`
- Do not update `prompts/` without updating `.cursor/prompts/`
- Do not assume a future script will "fix it later"

Agents are expected to enforce this rule proactively and call it out explicitly if violated.


# task_progress List (Optional - Plan Mode)

While in PLAN MODE, if you've outlined concrete steps or requirements for the user, you may include a preliminary todo list using the task_progress parameter.

Reminder on how to use the task_progress parameter:


1. To create or update a todo list, include the task_progress parameter in the next tool call
2. Review each item and update its status:
   - Mark completed items with: - [x]
   - Keep incomplete items as: - [ ]
   - Add new items if you discover additional steps
3. Modify the list as needed:
		- Add any new steps you've discovered
		- Reorder if the sequence has changed
4. Ensure the list accurately reflects the current state

**Remember:** Keeping the task_progress list updated helps track progress and ensures nothing is missed.

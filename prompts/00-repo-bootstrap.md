# Agent Prompt 00 — Repo bootstrap (docs/prompts only)

You are operating in a fresh directory for MT-logo-render. This is ALWAYS the first prompt to execute for any new project.

## Objectives
1) Initialize git repository with main branch (never master).
2) Create complete docs/prompts scaffolding with no implementation code.
3) Ensure structure matches README and docs index.
4) Make initial commit with all foundational files before any implementation.

## Critical Requirements
- **ALWAYS start with git init and main branch** - This is mandatory for all projects.
- **Never proceed to implementation without git history** - All decisions must be recorded.
- **Complete bootstrap before any code** - Documentation and process comes first.

## Steps

### Git Setup (MANDATORY - Always First)
```bash
git init
git branch -m main  # Always use main, never master
```

### File Creation
- Create .gitignore with appropriate patterns
- Create README.md with project overview
- Create CHANGELOG.md for version tracking
- Create docs/ROADMAP.md with development phases
- Create complete prompts/ directory with all 18 prompts
- Create docs/ structure with SPEC.md, ARCHITECTURE.md, etc.

### Initial Commit (MANDATORY)
```bash
git add .gitignore README.md CHANGELOG.md docs/ROADMAP.md prompts/ docs/
git commit -m "feat: bootstrap [PROJECT] repository with tick-task methodology

- Add comprehensive README.md with project overview and usage examples
- Add complete 18-prompt library adapted for [TECH_STACK] development
- Add documentation structure: ROADMAP.md, SPEC.md, ARCHITECTURE.md, etc.
- Add .gitignore with [TECH_STACK] and project-specific patterns
- Add CHANGELOG.md for version tracking
- Establish git repository with main branch

[model: <MODEL_NAME>]"
```

### Verification
- Verify all prompts/ files exist (00-17 + PROMPTING_RULES.md)
- Verify docs/ structure is complete
- Verify git log shows the bootstrap commit
- Verify branch is named 'main'

## Constraints
- Optimize for free tier usage: do not generate large files unnecessarily.
- No sudo operations.
- Use gh for PR operations if available.
- Every commit includes model disclosure: [model: <MODEL_NAME>]
- **Never skip git initialization** - This process must be followed for auditability.

## Post-Bootstrap
After this prompt completes successfully:
- Repository has complete documentation foundation
- Git history records all initial setup decisions
- Ready to proceed to specification phase (Prompt 01)
- All future work is tracked in git

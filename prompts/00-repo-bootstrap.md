# Agent Prompt 00 — Repo bootstrap (docs/prompts only)

You are operating in a fresh git repo for MT-logo-render.

## Objectives
1) Ensure the repo contains only docs/prompts scaffolding (no implementation code).
2) Ensure structure matches README and docs index.
3) Ensure the first PR contains: docs/*, prompts/*, .github/*, CHANGELOG.md, README.md.

## Constraints
- Optimize for free tier usage: do not generate large files unnecessarily.
- No sudo.
- Use gh for PR operations if available.
- Every commit includes: [model: <MODEL_NAME>]

## Steps
- Verify directory structure.
- Verify docs and prompts have no code.
- Create/adjust minimal .gitignore (only if needed).
- Prepare a single commit for this PR and open it.

chat: MT-logo-render Phase 1 Completion Convention
purpose: Establish done. prefix convention for completed state capsules
context:

- Project: MT-logo-render (Rust CLI tool for deterministic logo generation)
- Current Phase: 1 (Documentation Setup)
- Status: Establishing workflow conventions before CI fixes
- Constraints: Must maintain existing state capsule structure
- Dependencies: None - standalone documentation task
- Tools/Technologies: State capsule system, documentation

decisions:

- Establish `done.` prefix for completed session capsules
- Only rename session-specific capsules, not canonical CURRENT_TASKS.md
- Apply convention retroactively to existing completed capsules
- Update README.md with clear completion rules

open_threads:

- Complete Phase 1 tasks and verify convention works
- Proceed to Phase 2 CI recovery after merge confirmation

next_chat_bootstrap:

- Wait for PR merge confirmation
- Proceed to Phase 2 CI pipeline recovery
- Begin systematic fixing of CI failures

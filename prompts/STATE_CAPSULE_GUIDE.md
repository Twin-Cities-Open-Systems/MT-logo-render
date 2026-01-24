# State Capsule Guide

## Overview

A State Capsule is a structured handoff document that preserves decision state and context when transitioning between agents or chat sessions. It is designed to prevent context loss and ensure continuity of work.

## Purpose

- **Continuation Vector**: Not a summary, but a structured handoff document
- **Decision Preservation**: Maintains critical decision state across transitions
- **Context Stability**: Prevents loss of important context and requirements
- **Workflow Efficiency**: Enables clean handoffs between agents or sessions

## When to Use

Use a State Capsule when:

- Transitioning between agents or chat sessions
- Need to preserve complex decision state
- Working on multi-step projects with dependencies
- Want to ensure no critical information is lost
- Preparing for context switches or interruptions

## Format

State Capsules must be emitted as final responses in normal chat mode, never as plan-mode output.

### Required Structure

```yaml
chat: <name>
purpose: <one sentence>
context:
  - ...
decisions:
  - ...
open_threads:
  - ...
next_chat_bootstrap:
  - ...
```

## Components

### 1. Chat

- **Purpose**: Name of the current chat or session
- **Format**: Short, descriptive name
- **Example**: `MT-logo-render Phase 3 CI Troubleshooting`

### 2. Purpose

- **Purpose**: One-sentence description of the chat's objective
- **Format**: Single sentence, clear and concise
- **Example**: `Document current state and remaining work for Phase 3 completion`

### 3. Context

- **Purpose**: Key background information and current state
- **Format**: Bullet points of essential context
- **Content**: Project status, current phase, recent actions, constraints
- **Guidelines**: Include only critical information needed for continuity

### 4. Decisions

- **Purpose**: Important decisions made during the session
- **Format**: Bullet points of key decisions
- **Content**: Technical choices, architectural decisions, resolved issues
- **Guidelines**: Focus on decisions that impact future work

### 5. Open Threads

- **Purpose**: Unresolved issues and pending work
- **Format**: Bullet points of remaining tasks
- **Content**: Incomplete tasks, unresolved problems, dependencies
- **Guidelines**: Prioritize by importance and urgency

### 6. Next Chat Bootstrap

- **Purpose**: Starting points for the next session
- **Format**: Bullet points of immediate next steps
- **Content**: What to work on first, how to continue
- **Guidelines**: Actionable items that can be started immediately

## Anti-Patterns to Avoid

❌ **Do NOT say:**

- "Stay in plan mode but output..."
- "Do not respond yet, just write..."
- "Generate but don't finalize..."

These instructions conflict with Cline's execution model and cause failures.

## Best Practices

### 1. Be Specific

- Use concrete, actionable items
- Avoid vague or generic statements
- Include specific details and context

### 2. Focus on Decision State

- Preserve critical decisions and rationale
- Document why choices were made
- Maintain architectural reasoning

### 3. Prioritize Open Threads

- Order by importance and urgency
- Clearly indicate dependencies
- Mark critical path items

### 4. Clear Handoff

- Assume the next agent has no prior context
- Include all necessary background information
- Make next steps immediately actionable

## Example Template

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

## Integration with Workflow

### 1. Standardize Template

- Create reusable State Capsule templates
- Establish team conventions for format and content
- Include in project documentation

### 2. Jump Chat Commands

- Design "jump chat" commands that work reliably in Cline
- Create standardized handoff procedures
- Document transition protocols

### 3. Project Integration

- Include State Capsules in project documentation
- Use as part of PR descriptions or release notes
- Reference in project wikis or knowledge bases

## Guardrails

### 1. Always Emit as Final Response

- State Capsules must be the final response in a normal chat
- Never emit from plan mode
- Use as a proper handoff, not a planning tool

### 2. Maintain Non-Empty Content

- Ensure all sections have meaningful content
- Avoid placeholder or generic entries
- Provide actionable information

### 3. Clear Intent

- Make the purpose and next steps unambiguous
- Avoid leaving the next agent guessing
- Provide clear direction for continuation

## Benefits

1. **Reduced Context Loss**: Preserves critical information across transitions
1. **Improved Continuity**: Ensures work can continue seamlessly
1. **Better Decision Tracking**: Maintains rationale for important choices
1. **Enhanced Collaboration**: Enables effective handoffs between team members
1. **Workflow Stability**: Reduces errors and rework due to lost context

## Common Use Cases

- **Multi-Agent Projects**: Handoffs between specialized agents
- **Long-Running Projects**: Session transitions and breaks
- **Complex Problem Solving**: Multi-step problem resolution
- **Team Collaboration**: Knowledge transfer between team members
- **Project Handoffs**: Transitions between project phases or teams

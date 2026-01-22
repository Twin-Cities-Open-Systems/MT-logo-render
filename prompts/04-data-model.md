# Agent Prompt 04 — Data Model

## Objective
Define the core data structures for render recipes and cache:
- Recipe schema: canvas, palette, layers, transforms
- Canonicalization rules for deterministic hashing
- Cache index format and atomic updates
- Validation rules for recipe inputs

## Output
- Update docs/DATA_MODEL.md with schema definitions
- Document canonicalization algorithms
- Specify cache consistency and integrity checks

## Constraints
- No code.
- Focus on data integrity and deterministic outputs.
- Design for extensibility beyond default shapes.

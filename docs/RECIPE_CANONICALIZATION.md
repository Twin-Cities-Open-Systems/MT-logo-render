# RECIPE_CANONICALIZATION (v0)

## Goal

Enable deterministic hashing of arbitrary recipes so the cache key is stable across machines and runs.

## Canonical representation

A recipe may be provided as YAML or JSON. Before hashing:

1. Parse into an internal object model.
1. Normalize:
   - keys: treat as case-sensitive but require canonical key spellings in validation
   - numbers: integers remain integers; floats normalized to a fixed decimal representation if allowed
   - colors: normalize to either palette name or hex rrggbb (lowercase, no '#')
   - arrays: preserve order (layer order is semantically meaningful)
1. Serialize to canonical JSON:
   - UTF-8
   - no insignificant whitespace
   - object keys sorted lexicographically at every object level
   - arrays preserved in order
1. Hash:
   - recipe_id = sha256(canonical_json_bytes)
   - use `hash8` = first 8 hex chars for filenames: `recipe-<hash8>`

## Determinism notes

- File paths (e.g., font_path) must not affect recipe hash directly unless included as a declared field.
- For fonts, the cache key uses a separate `font_id` hash of font bytes when font is used for PNG glyph rendering.

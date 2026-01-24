# RENDER_ENGINE_SPEC_RUST_NATIVE (v0)

## Purpose

A Rust-native render engine and cache that generates deterministic assets on demand:

- Raster: `.png` (no external binaries; no shelling out)
- Terminal: `.ansi` (truecolor-first), optional `.ansi256`
- Flat cache directory with deterministic filenames
- `index.yaml` records requested vs effective spec + fingerprints

This system is **not limited to shapes**. Shapes are the default recipe family; arbitrary render recipes are supported.

## Non-Negotiables

- Runs anywhere: no ffmpeg, no external runtime deps, no OS graphics APIs required.
- Flat cache: `assets/logo/generated/` contains files directly (no subdirs).
- Deterministic outputs: canonicalized effective spec -> deterministic filename + deterministic bytes.
- Terminal output uses **24-bit truecolor** ANSI as canonical (`.ansi`); optional 256-color fallback (`.ansi256`).
- Raster glyph/text:
  - Default: built-in microfont for short labels (license-free by construction; strokes you own)
  - Optional: user-supplied font path enables Unicode glyph rendering in PNG
  - No bundled third-party fonts (avoid licensing issues)

## Directory Layout

assets/logo/
index.yaml
generated/
<stem>.png
<stem>.ansi
[optional] <stem>.ansi256
[optional] <stem>.md
[optional] <stem>.html

## Targets

### terminal

- Output: `<stem>.ansi` (truecolor), optionally `<stem>.ansi256`
- Unicode glyphs: emitted directly; terminal font determines appearance.
- No terminal image protocols.

### image

- Output: `<stem>.png`
- Rendered in-process.

### markdown/html (optional wrappers)

- Portable default: wrapper references sibling `.png` (relative path).
- Optional strict mode: wrapper embeds base64 PNG (discouraged: large diffs / renderer limits).

## Core abstraction: Render Recipe

The engine renders a recipe to a target.

Minimal recipe model:

- canvas: width, height, background (transparent or color)
- palette: base, optional accent; optional fg (computed if absent)
- layers[] (ordered back-to-front):
  - primitives (circle, rect, triangle, hex, path optional)
  - fills (solid, pie, split, stripe)
  - overlays (mark, badge, label, glyph)
  - transforms (translate/scale/rotate)
  - clip (to primitive/path)
  - composite (source-over only)

## Default recipe family: defaultshape (v0)

Inputs:

- shape: circle | square | triangle | hex
- size: WxH (pixels for PNG; cells for terminal but still represented as WxH)
- base_color: palette name or hex
- accent_color: optional (for split/stripe)
- fill: solid | pie:p | split:n | stripe:k
- mark: none | check | x | dot
- badge: none | corner-dot | corner-check
- label: optional (1–4 chars recommended)
- glyph: optional Unicode string
- font_path: optional filesystem path for PNG text/glyph rendering

Rules:

- Terminal: glyph always supported (printed as Unicode).
- PNG: glyph requires font_path; otherwise degrades (see below).

## Font & glyph policy

### Terminal

- Always supports Unicode output.
- No tool-side fonts; environment font renders glyphs.

### PNG

- Default label rendering uses built-in microfont (ASCII-oriented).
- If user supplies font_path:
  - Use it for label/glyph rendering (Unicode allowed).
- If glyph requested but no font_path:
  - If representable in microfont -> render (likely ASCII only)
  - Else omit glyph or substitute mark; record degradation in effective_spec/notes.

Font identity in cache key:

- When font_path is used for PNG, compute font_id = hash(font bytes).
- Include `font-<hash8>` token in the filename stem.

## Deterministic filename stem

Canonical stem format:
<recipeId>-<base>\[-<accent>\]-<tokens>-<WxH>

recipeId:

- Built-in: `defaultshape`
- Arbitrary recipe: `recipe-<hash8>` from canonical recipe serialization (see RECIPE_CANONICALIZATION)

Colors:

- Palette name OR normalized hex `rrggbb` (lowercase, no '#').

Canonical token order:

1. fill-\<...> (always; default fill-solid)
1. mark-\<...> (always; default mark-none)
1. badge-\<...> (always; default badge-none)
1. label-\<...> (only if present after sanitization)
1. glyph-\<...> (only if present and renderable; encoded as UXXXXXXXX[\_UYYYYYYYY...])
1. font-<hash8> (only if PNG uses user font)

Label sanitization (filename-safe):

- Uppercase
- Keep only [A-Z0-9.\_]
- Truncate to 4 chars
- If empty -> omit label token

Glyph token encoding:

- Do not place raw glyphs into filenames.
- Encode each codepoint as UXXXXXXXX (uppercase hex, 8 digits), join with '\_' for multi-codepoint strings.

## Degradation rules (effective_spec)

- pie:p only valid for circle; else fill-solid.
- split:n constraints:
  - hex: 2/3/6
  - square: 2
  - triangle: 2 (optional); if unsupported -> fill-solid
- Terminal legibility may bucket/omit details based on cell size; must be recorded.
- PNG glyph without font_path:
  - microfont-only if possible; else omit/substitute; must be recorded.

## Index & atomicity

- Write outputs to temp names, fsync/rename into place.
- Update index last (temp + atomic rename).

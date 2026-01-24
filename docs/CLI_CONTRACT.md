# CLI Contract (v1.0)

## Overview

MT-logo-render provides a command-line interface with four core commands. All commands support JSON input/output for machine consumption and human-readable formatting for interactive use.

## Global Options

### Asset Root (`--asset-root <DIR>`)

- **Default**: `assets/logo`
- **Description**: Root directory for cache and generated assets
- **Structure**:
  ```
  <asset-root>/
  ├── index.yaml          # Cache metadata
  └── generated/          # Output files
  ```

### Output Format (`--format <FORMAT>`)

- **Values**: `json`, `yaml`
- **Default**: `json`
- **Description**: Output format for machine-readable responses

### Force Mode (`--force`)

- **Type**: Flag
- **Description**: Bypass cache checks (useful for testing/debugging)

## Command Reference

### 1. Resolve Command

**Purpose**: Compute deterministic filenames and check existence without rendering.

**Usage**:

```bash
logo-render resolve [OPTIONS] <RECIPE>
logo-render resolve [OPTIONS] --file <PATH>
logo-render resolve [OPTIONS] -  # Read from stdin
```

**Arguments**:

- `<RECIPE>`: Inline JSON/YAML recipe string
- `--file <PATH>`: Path to recipe file
- `-`: Read recipe from stdin

**Examples**:

```bash
# Inline recipe
logo-render resolve '{"shape": "circle", "size": "256x256", "base_color": "blue"}'

# From file
logo-render resolve --file recipe.json

# From stdin
echo '{"shape": "square", "size": "128x128", "base_color": "red"}' | logo-render resolve -
```

**Output Schema** (JSON):

```json
{
  "requested_spec": {
    "shape": "circle",
    "size": "256x256",
    "base_color": "blue"
  },
  "effective_spec": {
    "shape": "circle",
    "size": "256x256",
    "base_color": "0000ff",
    "fill": "solid"
  },
  "stem": "defaultshape-0000ff-solid-circle-256x256",
  "outputs": {
    "png": {
      "path": "assets/logo/generated/defaultshape-0000ff-solid-circle-256x256.png",
      "exists": false
    },
    "ansi": {
      "path": "assets/logo/generated/defaultshape-0000ff-solid-circle-256x256.ansi",
      "exists": false
    }
  },
  "notes": []
}
```

**Exit Codes**:

- `0`: Resolution successful (files may or may not exist)
- `1`: Error (invalid recipe, IO failure, etc.)

### 2. Render Command

**Purpose**: Ensure requested outputs exist, generating on cache miss.

**Usage**:

```bash
logo-render render [OPTIONS] <RECIPE>
logo-render render [OPTIONS] --file <PATH>
logo-render render [OPTIONS] --targets png,ansi -
```

**Arguments**:

- `<RECIPE>`: Inline JSON/YAML recipe string
- `--file <PATH>`: Path to recipe file
- `--targets <LIST>`: Comma-separated list of targets (png,ansi,ansi256,md,html)
- `-`: Read recipe from stdin

**Target Options**:

- `png`: PNG image output (default)
- `ansi`: Truecolor ANSI terminal output (default)
- `ansi256`: 256-color ANSI fallback
- `md`: Markdown wrapper referencing PNG
- `html`: HTML wrapper (base64 embed or reference)

**Examples**:

```bash
# Basic render
logo-render render '{"shape": "triangle", "size": "128x128", "base_color": "green"}'

# Specific targets only
logo-render render --targets png,ansi256 --file recipe.json

# Force re-render
logo-render render --force '{"shape": "hex", "size": "256x256", "base_color": "purple"}'
```

**Output Schema** (JSON):

```json
{
  "requested_spec": {
    "shape": "triangle",
    "size": "128x128",
    "base_color": "green"
  },
  "effective_spec": {
    "shape": "triangle",
    "size": "128x128",
    "base_color": "008000",
    "fill": "solid"
  },
  "stem": "defaultshape-008000-solid-triangle-128x128",
  "outputs": {
    "png": {
      "path": "assets/logo/generated/defaultshape-008000-solid-triangle-128x128.png",
      "exists": true,
      "sha256": "a1b2c3d4...",
      "size_bytes": 2048
    },
    "ansi": {
      "path": "assets/logo/generated/defaultshape-008000-solid-triangle-128x128.ansi",
      "exists": true,
      "sha256": "e5f6g7h8...",
      "size_bytes": 256
    }
  },
  "written": [
    "assets/logo/generated/defaultshape-008000-solid-triangle-128x128.png",
    "assets/logo/generated/defaultshape-008000-solid-triangle-128x128.ansi"
  ],
  "fingerprints": {
    "assets/logo/generated/defaultshape-008000-solid-triangle-128x128.png": "a1b2c3d4...",
    "assets/logo/generated/defaultshape-008000-solid-triangle-128x128.ansi": "e5f6g7h8..."
  },
  "notes": []
}
```

**Exit Codes**:

- `0`: All requested outputs exist after command completion
- `1`: Error (invalid recipe, rendering failure, IO error)
- `2`: Partial success (some outputs failed, but command completed)

### 3. Doctor Command

**Purpose**: Environment self-check and capability reporting.

**Usage**:

```bash
logo-render doctor [OPTIONS]
```

**Checks Performed**:

- Asset root directory writable
- Font paths readable (if configured)
- System capabilities summary
- Version and build information

**Output Schema** (JSON):

```json
{
  "version": "1.0.0",
  "build_info": {
    "rust_version": "1.70.0",
    "target": "x86_64-unknown-linux-gnu",
    "build_date": "2026-01-22"
  },
  "environment": {
    "asset_root_writable": true,
    "asset_root_path": "assets/logo",
    "font_support": {
      "builtin_microfont": true,
      "external_fonts": true
    }
  },
  "capabilities": {
    "png_rendering": true,
    "ansi_rendering": true,
    "truecolor_ansi": true,
    "font_loading": true
  },
  "issues": []
}
```

**Issues Format**:

```json
{
  "issues": [
    {
      "severity": "warning",
      "category": "font",
      "message": "Custom font path not configured",
      "suggestion": "Set LOGO_RENDER_FONT_PATH environment variable"
    }
  ]
}
```

**Exit Codes**:

- `0`: Environment OK, all capabilities available
- `1`: Issues detected that may impact functionality

### 4. List Command

**Purpose**: Query cache index with optional filtering.

**Usage**:

```bash
logo-render list [OPTIONS] [FILTERS...]
```

**Filter Options**:

- `--shape <SHAPE>`: Filter by shape (circle, square, triangle, hex)
- `--size <SIZE>`: Filter by size (WxH format)
- `--base-color <COLOR>`: Filter by base color
- `--fill <FILL>`: Filter by fill pattern
- `--mark <MARK>`: Filter by mark type
- `--badge <BADGE>`: Filter by badge type
- `--label <TEXT>`: Filter by label content
- `--exists`: Only show entries with existing files
- `--missing`: Only show entries with missing files

**Examples**:

```bash
# List all cached entries
logo-render list

# Filter by shape and color
logo-render list --shape circle --base-color blue

# Show only missing files
logo-render list --missing

# Find entries with specific label
logo-render list --label "MT"
```

**Output Schema** (JSON):

```json
[
  {
    "stem": "defaultshape-0000ff-solid-circle-256x256",
    "recipe": {
      "requested": {
        "shape": "circle",
        "size": "256x256",
        "base_color": "blue"
      },
      "effective": {
        "shape": "circle",
        "size": "256x256",
        "base_color": "0000ff",
        "fill": "solid"
      }
    },
    "outputs": {
      "png": {
        "path": "assets/logo/generated/defaultshape-0000ff-solid-circle-256x256.png",
        "exists": true,
        "sha256": "a1b2c3d4...",
        "size_bytes": 2048
      },
      "ansi": {
        "path": "assets/logo/generated/defaultshape-0000ff-solid-circle-256x256.ansi",
        "exists": true,
        "sha256": "e5f6g7h8...",
        "size_bytes": 256
      }
    },
    "created_at": "2026-01-22T04:00:00Z",
    "notes": []
  }
]
```

**Exit Codes**:

- `0`: Query successful
- `1`: Error (IO failure, invalid filters)

## Recipe Schema

### Base Recipe Structure

```json
{
  "shape": "circle" | "square" | "triangle" | "hex",
  "size": "<WIDTH>x<HEIGHT>",
  "base_color": "<COLOR>",
  "accent_color": "<COLOR>",
  "fill": "solid" | "pie:<DEGREES>" | "split:<SEGMENTS>" | "stripe:<COUNT>",
  "mark": "check" | "x" | "dot" | null,
  "badge": "corner-dot" | "corner-check" | null,
  "label": "<ASCII_TEXT>",
  "glyph": "<UNICODE_STRING>",
  "font_path": "<FILESYSTEM_PATH>"
}
```

### Color Formats

- **Named colors**: `"red"`, `"blue"`, `"green"`, etc.
- **Hex colors**: `"#ff0000"`, `"00ff00"`, `"0000ff"`
- **Short hex**: `"f00"`, `"0f0"`, `"00f"`

### Size Format

- **Format**: `"<WIDTH>x<HEIGHT>"`
- **Examples**: `"256x256"`, `"128x64"`, `"512x512"`
- **Constraints**: Minimum 16x16, maximum 4096x4096

### Fill Patterns

- **solid**: Single color fill
- **pie:<DEGREES>**: Pie slice from top (circle only, degrades to solid)
- **split:<SEGMENTS>**: Equal divisions (2/3/6 for hex, 2 for others)
- **stripe:<COUNT>**: Vertical stripes alternating base/accent

## Error Handling

### Error Response Format

```json
{
  "error": {
    "type": "InvalidRecipe",
    "message": "Invalid shape: 'pentagon' is not supported",
    "suggestion": "Use one of: circle, square, triangle, hex",
    "context": {
      "field": "shape",
      "provided": "pentagon",
      "valid_values": ["circle", "square", "triangle", "hex"]
    }
  }
}
```

### Common Error Types

- **InvalidRecipe**: Malformed or invalid recipe specification
- **RenderingFailed**: Rendering pipeline error
- **CacheCorrupted**: Cache index inconsistency
- **IoError**: File system access problems
- **FontError**: Font loading or rendering issues

### Error Suggestions

All errors include actionable suggestions:

- "Use one of: [valid_values]"
- "Ensure asset root is writable: chmod +w assets/logo"
- "Check font file exists and is readable"

## Input/Output Formats

### JSON Input

- Standard JSON parsing
- Comments not supported (use YAML for comments)
- Unicode strings supported

### YAML Input

- Alternative to JSON for human editing
- Comments and multi-line strings supported
- Converted to internal JSON representation

### Output Formatting

- **JSON**: Machine-readable, single-line or pretty-printed
- **YAML**: Human-readable with comments
- Consistent field ordering for deterministic diffs

## Environment Variables

### LOGO_RENDER_ASSET_ROOT

- **Default**: `assets/logo`
- **Description**: Override default asset root directory

### LOGO_RENDER_FONT_PATH

- **Default**: None
- **Description**: Default font path for Unicode glyph rendering

### LOGO_RENDER_FORCE_COLOR

- **Values**: `true`, `false`
- **Default**: Auto-detect
- **Description**: Force ANSI color output (for testing)

## Exit Code Summary

| Code | Meaning | Description                            |
| ---- | ------- | -------------------------------------- |
| 0    | Success | Command completed successfully         |
| 1    | Error   | Fatal error, command failed            |
| 2    | Partial | Some operations succeeded, some failed |

## Compatibility Notes

### Platform Support

- **Linux**: Primary target, full feature support
- **macOS**: Full support with native font rendering
- **Windows**: Full support with path separator handling

### Terminal Compatibility

- **Truecolor**: Modern terminals (iTerm2, GNOME Terminal, Windows Terminal)
- **256-color**: Fallback for older terminals
- **Basic**: Graceful degradation to no colors

### File System Requirements

- **Unicode paths**: Full Unicode filename support
- **Atomic renames**: POSIX-compliant file systems
- **Permissions**: Read/write access to asset root

## Development Commands

### Recommended Cargo Commands

For development work, use the `--quiet` flag to reduce output noise:

```bash
# Clean build output
cargo build --quiet

# Run tests with minimal noise
cargo test --quiet

# Lint with reduced verbosity
cargo clippy --quiet

# Format code
cargo fmt --quiet

# Run specific tests
cargo test --quiet -- resolve_command
cargo test --quiet -- render_command

# Benchmark performance
cargo bench --quiet

# Check for security vulnerabilities
cargo audit --quiet
```

### Non-Interactive Command Requirements

**CRITICAL**: All commands must be fully automated and never trigger user's editor or pager.

#### Git Commands

- Always use `git --no-pager` to prevent pager activation
- Use `GIT_EDITOR=true` or `GIT_EDITOR=:` to prevent editor activation
- Add `--no-edit` to skip editor for commit operations
- Use `--quiet` flags to reduce output and prevent pager

#### Safe Command Examples

```bash
git --no-pager status
git --no-pager diff --name-only
git --no-pager log --oneline -10
GIT_EDITOR=true git commit --no-edit -m "message"
git --no-pager push origin HEAD --force-with-lease
```

#### Unsafe Command Examples (AVOID)

```bash
git status          # Can trigger pager
git diff            # Can trigger pager
git commit          # Can trigger editor
git rebase          # Can trigger editor
```

#### Other Commands

- Use `--quiet` flags to reduce output
- Redirect output to `/dev/null` when not needed
- Use `HEAD` instead of branch names to avoid editor prompts
- Use `--force`, `--assume-yes` flags to avoid interactive prompts

### Development Workflow

1. **Build**: `cargo build --quiet`
1. **Test**: `cargo test --quiet`
1. **Lint**: `cargo clippy --quiet`
1. **Format**: `cargo fmt --quiet`
1. **Audit**: `cargo audit --quiet`

### Debug Commands

For debugging, you can override the quiet mode:

```bash
# Verbose build for debugging
cargo build --verbose

# Debug logging
RUST_LOG=debug ./target/debug/logo-render render --file recipe.json

# Profile build
cargo build --profile=dev --quiet
```

This contract provides a stable, machine-readable interface for integrating MT-logo-render into build systems, scripts, and other tools.

# SPEC (v1.0 - Implementation Ready)

## Product Statement

MT-logo-render is a Rust-native CLI application that generates deterministic PNG and ANSI terminal logo assets on demand. It provides a complete rendering pipeline with caching, supporting the Market Thesis ecosystem's asset generation needs.

## Vision

A reliable, high-performance logo asset generator that produces identical outputs from identical inputs, ensuring reproducible builds and consistent branding across the Market Thesis platform.

## Success Criteria (v1.0)

- [ ] All acceptance criteria pass with ≥80% test coverage
- [ ] Application renders in \<100ms for typical logo sizes
- [ ] Memory usage \<50MB during rendering operations
- [ ] Zero crashes or memory safety violations
- [ ] Deterministic output validation (same input = identical bytes)
- [ ] Cross-platform compatibility (Linux/macOS/Windows)

## Requirements Priority Framework

- **Requirements (v1.0)**: Critical for core functionality, cannot be deferred
- **Enhanced (v1.0)**: Important but can be implemented post-v1.0 if time-constrained
- **Future**: Nice-to-have, implement only if time allows
- **Out of Scope**: Explicitly excluded from v1.0

## Functional Requirements

### Core CLI Commands (v1.0 Requirements)

#### CLI Command Structure (MUST-CLI-001)

**Requirement**: Binary provides four core commands with consistent interface.

**Commands**:

- `resolve`: Compute deterministic filenames without rendering
- `render`: Ensure outputs exist (generate on cache miss)
- `doctor`: Environment self-check and capability reporting
- `list`: Query cache index for existing assets

**Interface Requirements**:

- JSON input/output for machine consumption
- Human-readable error messages and help text
- Exit codes: 0=success, 1=error, 2=partial success
- Global options: --asset-root, --format, --force

**Acceptance Criteria**:

- [ ] All commands accept --help with clear usage information
- [ ] JSON output is valid and machine-parseable
- [ ] Exit codes follow Unix conventions
- [ ] Error messages are actionable and descriptive

#### Resolve Command (MUST-RESOLVE-001)

**Requirement**: Compute output paths and existence status without rendering.

**Input**: Recipe specification (JSON/YAML via stdin or file)
**Output**: JSON with requested_spec, effective_spec, stem, outputs array
**Behavior**: Pure computation, no file system writes

**Acceptance Criteria**:

- [ ] Deterministic stem generation from canonical recipe
- [ ] Accurate existence checking for all target formats
- [ ] effective_spec reflects any canonicalization/degradation
- [ ] Exit code 0 if all outputs exist, 2 if any missing

#### Render Command (MUST-RENDER-001)

**Requirement**: Ensure all requested outputs exist, generating on cache miss.

**Input**: Recipe specification (JSON/YAML via stdin or file)
**Output**: JSON with complete metadata plus written files list
**Behavior**: Atomic cache updates, deterministic rendering

**Acceptance Criteria**:

- [ ] Cache hits return immediately without re-rendering
- [ ] Cache misses trigger rendering and atomic cache update
- [ ] --force flag bypasses cache (useful for testing)
- [ ] Output includes SHA256 fingerprints for verification

#### Doctor Command (MUST-DOCTOR-001)

**Requirement**: Validate environment and report capabilities.

**Checks**:

- Asset root directory writable
- Font paths readable (if specified)
- System capabilities summary
- Version and build information

**Acceptance Criteria**:

- [ ] Comprehensive environment validation
- [ ] Clear reporting of degraded capabilities
- [ ] Exit code 0 for fully capable, 1 for issues detected

#### List Command (MUST-LIST-001)

**Requirement**: Query cache index with optional filtering.

**Filters**: shape, size, base_color, fill, mark, badge, label
**Output**: JSON array of matching cache entries
**Behavior**: Read-only cache queries

**Acceptance Criteria**:

- [ ] All specified filters work independently and combined
- [ ] Output includes complete recipe and output metadata
- [ ] Efficient queries on large cache indexes

### Rendering Pipeline (v1.0 Requirements)

#### PNG Generation (MUST-PNG-001)

**Requirement**: Generate high-quality raster images without external dependencies.

**Supported Features**:

- Basic shapes: circle, square, triangle, hexagon
- Fill patterns: solid, pie, split, stripe
- Overlays: mark (check/x/dot), badge (corner variants)
- Labels: ASCII text with built-in font
- Colors: named palettes + hex colors

**Quality Standards**:

- Antialiased rendering
- Consistent color reproduction
- Crisp text at typical sizes (16px-128px)

**Acceptance Criteria**:

- [ ] All basic shapes render correctly
- [ ] Fill patterns work for supported shapes
- [ ] Text rendering legible and properly positioned
- [ ] Colors match specification exactly

#### ANSI Terminal Output (MUST-ANSI-001)

**Requirement**: Generate truecolor ANSI escape sequences for terminal display.

**Formats**:

- `.ansi`: 24-bit truecolor (canonical)
- `.ansi256`: 256-color fallback (optional)

**Capabilities**:

- Unicode glyph support
- Background colors and transparency
- Block character art for shapes

**Acceptance Criteria**:

- [ ] Truecolor ANSI sequences are valid
- [ ] Terminal display matches PNG appearance
- [ ] 256-color fallback preserves readability
- [ ] Unicode glyphs render correctly in supported terminals

#### Font Handling (MUST-FONT-001)

**Requirement**: Support text rendering with safe font handling.

**Default Behavior**:

- Built-in microfont for ASCII labels
- License-safe by construction
- No external font dependencies

**Optional Enhancement**:

- Custom font path support for Unicode
- Font identity in cache key (hash-based)
- Graceful degradation to microfont

**Acceptance Criteria**:

- [ ] ASCII labels render with built-in font
- [ ] Custom fonts load and render Unicode correctly
- [ ] Font hash included in deterministic filenames
- [ ] Graceful fallback when fonts unavailable

### Data Integrity (v1.0 Requirements)

#### Deterministic Outputs (MUST-DETERMINISTIC-001)

**Requirement**: Identical inputs produce byte-for-byte identical outputs.

**Hashing Strategy**:

- Recipe canonicalization (sorted keys, normalized values)
- SHA256 of canonical JSON
- First 8 hex chars as filename component

**Cache Integrity**:

- Atomic writes (temp file + rename)
- Index updated last in sequence
- Corruption detection and recovery

**Acceptance Criteria**:

- [ ] Same recipe always produces same filenames
- [ ] Same recipe always produces same file contents
- [ ] Cache corruption doesn't break subsequent operations
- [ ] Concurrent operations don't corrupt cache

#### Recipe Canonicalization (MUST-CANONICAL-001)

**Requirement**: Normalize recipe inputs for consistent hashing.

**Normalization Rules**:

- Keys sorted lexicographically
- Colors normalized to lowercase hex
- Arrays preserved in order
- Numbers represented consistently

**Validation**:

- Required fields enforced
- Type safety at parse time
- Sensible defaults applied

**Acceptance Criteria**:

- [ ] Equivalent recipes produce identical canonical forms
- [ ] Invalid recipes rejected with clear errors
- [ ] Canonical JSON is deterministic across runs
- [ ] Validation prevents common input errors

### Cache System (v1.0 Requirements)

#### Flat Directory Structure (MUST-CACHE-001)

**Requirement**: Simple, predictable cache layout with atomic operations.

**Structure**:

```
assets/logo/
├── index.yaml    # Cache metadata
└── generated/    # Output files
    ├── recipe1-png
    ├── recipe1-ansi
    └── ...
```

**Atomicity**:

- Write to temp files, rename on success
- Index updated after successful writes
- Crash-safe operations

**Acceptance Criteria**:

- [ ] All outputs in single generated/ directory
- [ ] No subdirectories in cache
- [ ] Atomic operations prevent corruption
- [ ] Index accurately reflects cache contents

#### Index Management (MUST-INDEX-001)

**Requirement**: YAML index tracks all cache entries with metadata.

**Index Contents**:

- Recipe specification (original and effective)
- Output paths and existence status
- Timestamps and fingerprints
- Degradation notes

**Query Support**:

- Filter by any recipe field
- Existence verification
- Cache cleanup utilities

**Acceptance Criteria**:

- [ ] Index contains complete recipe metadata
- [ ] Queries support all specified filters
- [ ] Index updates are atomic with file writes
- [ ] Corruption recovery possible from index

## SHOULD (v1.0): Enhanced Features

#### Advanced Fill Patterns (SHOULD-FILL-001)

**Requirement**: Support complex fill patterns with shape-specific logic.

**Pie Fill**: Valid for circles only
**Split Fill**: 2/3/6 divisions for hexagons
**Stripe Fill**: Configurable stripe count and direction

**Degradation**:

- Unsupported patterns fall back to solid
- Effective spec records degradation reason

**Acceptance Criteria**:

- [ ] Pie fill works correctly on circles
- [ ] Split fill supports valid division counts
- [ ] Stripe fill configurable and deterministic
- [ ] Degradation documented in effective spec

#### Markdown/HTML Wrappers (SHOULD-WRAP-001)

**Requirement**: Generate portable wrapper formats.

**Markdown**: Reference PNG with alt text
**HTML**: Embed or reference PNG
**Configuration**: Strict mode vs portable mode

**Acceptance Criteria**:

- [ ] Markdown includes proper image references
- [ ] HTML validates and displays correctly
- [ ] Strict mode embeds base64 for portability
- [ ] Alt text includes recipe description

#### Batch Operations (SHOULD-BATCH-001)

**Requirement**: Support multiple recipes in single command.

**Input**: Array of recipe specifications
**Output**: Array of results with individual status
**Error Handling**: Continue on individual failures

**Acceptance Criteria**:

- [ ] Batch processing more efficient than individual calls
- [ ] Individual failures don't stop batch processing
- [ ] Output includes per-item status and errors
- [ ] Atomic batch operations where possible

## SHOULD (v1.0): Quality Assurance

#### Comprehensive Testing (SHOULD-TEST-001)

**Requirement**: 80%+ test coverage with multiple testing strategies.

**Unit Tests**: Core algorithms and validation
**Integration Tests**: CLI command workflows
**Property Tests**: Deterministic output verification
**Performance Tests**: Timing and memory benchmarks

**Acceptance Criteria**:

- [ ] ≥80% line coverage across all modules
- [ ] Property tests verify determinism
- [ ] Integration tests cover all CLI commands
- [ ] Performance tests prevent regressions

#### Pre-commit Quality Gates (SHOULD-CI-001)

**Requirement**: Automated quality checks prevent regressions.

**Rust Standards**:

- `rustfmt` for consistent formatting
- `clippy` for linting and best practices
- `cargo check` for compilation verification

**CI Pipeline**:

- Multi-platform testing (Linux/macOS/Windows)
- Coverage reporting
- Release artifact verification

**Acceptance Criteria**:

- [ ] Pre-commit hooks run automatically
- [ ] CI passes for all pushes and PRs
- [ ] Code formatting is consistent
- [ ] Clippy warnings treated as errors

## Performance Requirements

### Performance Targets (v1.0 Requirements)

- **Render Time**: \<100ms for typical logos (256x256 PNG)
- **Memory Usage**: \<50MB during rendering operations
- **Startup Time**: \<50ms CLI startup to command dispatch
- **Cache Query**: \<10ms for index queries
- **Binary Size**: \<10MB release binary

### SHOULD (v1.0): Scalability Targets

- Linear scaling with output resolution
- Efficient memory usage for large batches
- Fast cache lookups on large indexes (10k+ entries)
- Reasonable performance on resource-constrained systems

## Security Requirements

### Security Baseline (v1.0 Requirements)

- **Memory Safety**: Zero unsafe code, Rust compile-time guarantees
- **Input Validation**: All inputs parsed and validated before processing
- **Path Safety**: No directory traversal or path manipulation vulnerabilities
- **No Network Access**: Pure local operation, no external connections

### SHOULD (v1.0): Enhanced Security

- **Input Sanitization**: Strict limits on string lengths and array sizes
- **Error Handling**: No sensitive information in error messages
- **File Permissions**: Appropriate permissions on cache files
- **Audit Trail**: Optional logging of operations for debugging

## Non-Goals (Explicit Exclusions)

- **GUI Interface**: CLI-only, no graphical user interface
- **Network Operations**: Local-only, no cloud or remote rendering
- **Real-time Rendering**: Batch/offline processing only
- **Advanced Image Processing**: Basic logo generation only
- **Font Bundling**: No included third-party fonts
- **Color Management**: Basic RGB color support only
- **Animation Support**: Static images only
- **Vector Output**: Raster-only (PNG) output

## Implementation Constraints

- **Technology Stack**: Pure Rust with minimal dependencies
- **Platform Support**: Linux (primary), macOS, Windows
- **Rust Version**: 1.70+ minimum supported version
- **Dependencies**: Security-audited crates only, minimal count
- **Binary Distribution**: Single static binary, no runtime dependencies

## Implementation Plan

### Phase 2: Specification & Architecture (Current)

**Status**: In Progress - SPEC.md development

**Deliverables**:

- Complete SPEC.md with detailed acceptance criteria ✅
- ARCHITECTURE.md with component design ⏳
- CLI_CONTRACT.md with command specifications ⏳
- DATA_MODEL.md with schema definitions ⏳

**Success Criteria**:

- All acceptance criteria explicitly defined
- Performance and security requirements specified
- Implementation constraints documented
- Ready for Phase 3 development

### Phase 3: Core Implementation

**Status**: Planned - Foundation ready

**Deliverables**:

- Rust CLI framework with clap
- Recipe parsing and canonicalization
- Basic PNG rendering pipeline
- Cache system with atomic operations

### Phase 4: Advanced Features

**Status**: Planned - Core complete

**Deliverables**:

- ANSI terminal output
- Font loading and Unicode text
- Advanced fill patterns
- Comprehensive error handling

### Phase 5: Quality Assurance

**Status**: Planned - Features complete

**Deliverables**:

- 80%+ test coverage
- Cross-platform validation
- Performance benchmarking
- Security audit

### Phase 6: Production Readiness

**Status**: Planned - QA complete

**Deliverables**:

- Documentation completion
- Binary packaging
- Installation guides
- v1.0 release preparation

### Phase 7: Launch & Maintenance

**Status**: Planned - Production ready

**Deliverables**:

- v1.0 release
- Distribution setup
- User support procedures
- Feedback integration

## Success Metrics (v1.0)

- All requirements implemented and tested
- SHOULD requirements implemented where time permits
- Zero crashes in tested scenarios
- Performance targets met on target hardware
- Cross-platform compatibility verified
- Documentation complete and accurate
- Successful integration in Market Thesis ecosystem

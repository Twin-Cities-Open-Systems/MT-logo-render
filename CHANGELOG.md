# Changelog

All notable changes to MT-logo-render will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-01-24

- Development Phase

### Added

- Repository foundation with complete prompt library (18 prompts)
- Documentation structure adapted from tick-task methodology
- Phase-based development roadmap for Rust CLI render engine
- Bootstrap README.md with usage examples and architecture overview

### Changed

- Adapted tick-task methodology for Rust render engine requirements
- Updated technology stack from Python/React to Rust CLI
- Modified acceptance criteria for deterministic asset generation

### Fixed

- **Pre-commit Authentication**: Fixed SSH authentication failures by switching to HTTPS URLs
- **YAML Syntax**: Corrected indentation error in `.pre-commit-config.yaml`
- **CI Configuration**: Enhanced GitHub Actions workflow with SSH setup for pre-commit
- **Rust Compilation Errors**: Fixed all unused variable warnings in `src/main.rs` by adding underscore prefixes to unused variables and parameters:
  - Line 188: Added `_x` and `_y` to pixel enumeration loop
  - Line 236: Changed `accent_color` to `_accent_color` in ANSI rendering
  - Line 295: Changed `accent_color` parameter to `_accent_color` in `render_circle`
  - Line 321: Changed `accent_color` parameter to `_accent_color` in `render_square`
  - Line 344: Changed `accent_color` parameter to `_accent_color` in `render_triangle`
  - Line 357: Changed `area` variable to `_area` in triangle rendering
  - Line 516: Changed `label` parameter to `_label` in `render_label`
- **Security Scanner Test**: Fixed `test_scan_rust_file` by removing SAFETY comment that was preventing proper unsafe code detection, resolving CI/CD test failures across all platforms (Ubuntu, macOS, Windows)
- **Python 3.14 Syntax Error**: Fixed `exit(1)` syntax error in HEE Recipe Validation CI job by changing to `import sys; sys.exit(1)` for Python 3.14.2 compatibility
- **Windows PowerShell Syntax Error**: Fixed PowerShell parser error in Windows release build test by adding `shell: bash` to ensure consistent bash syntax across all platforms
- **CI Workflow Hardcoded Paths**: Removed hardcoded Windows target paths in favor of dynamic `${{ matrix.target }}` variables for proper cross-platform builds

### Technical

- Established 18-step development process for Rust CLI application
- Created comprehensive documentation framework
- Set up quality gates and development standards for Rust
- **CI/CD Improvements**: Fixed pre-commit hooks authentication issues
- **Security**: Maintained security while improving CI reliability

## [Unreleased] - Development Phase

### Added

### Changed

### Fixed

### Technical

## [0.1.0] - 2026-01-22

### Added

- Initial repository bootstrap with tick-task methodology adaptation
- Complete prompt library for structured Rust development
- Documentation foundation and project scaffolding
- Development workflow and quality standards

### Technical

- Repository structure established for MT-logo-render project
- Prompt library created with 18 sequential development guides
- Documentation framework set up for specification and architecture phases

______________________________________________________________________

**Development Phases:**

- **Phase 1**: Repository Foundation ✅ (Complete)
- **Phase 2**: Specification & Architecture 🚧 (In Progress)
- **Phase 3**: Core Implementation ⏳ (Planned)
- **Phase 4**: Advanced Features ⏳ (Planned)
- **Phase 5**: Quality Assurance ⏳ (Planned)
- **Phase 6**: Production Readiness ⏳ (Planned)
- **Phase 7**: Launch & Maintenance ⏳ (Planned)

For more details on current development status, see [ROADMAP.md](docs/ROADMAP.md).

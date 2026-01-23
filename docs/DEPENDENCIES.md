# MT-logo-render Dependencies

This document outlines all dependencies required for the MT-logo-render project, including Rust crates, system dependencies, and external tools.

## Rust Dependencies

### Core Dependencies

#### Image Processing
- **image** (0.25.4) - Image loading, manipulation, and format support
  - PNG, JPEG, BMP, and other format support
  - Image buffer operations and pixel manipulation
  - Used for rendering PNG outputs

#### CLI Framework
- **clap** (4.5.28) - Command-line argument parsing
  - Subcommand support for resolve, render, doctor, list
  - Automatic help generation and validation
  - Structured argument parsing

#### Serialization
- **serde** (1.0.228) - Serialization framework
- **serde_json** (1.0.140) - JSON serialization
- **serde_yaml** (0.9.34) - YAML serialization
  - Used for recipe input/output in multiple formats
  - Cache index serialization

#### Cryptographic Hashing
- **sha2** (0.10.8) - SHA-256 hashing
  - File fingerprinting for cache validation
  - Deterministic asset identification

#### Date/Time Handling
- **chrono** (0.4.38) - Date and time operations
  - Cache entry timestamps
  - ISO 8601 format support

#### Logging
- **tracing** (0.1.42) - Structured logging framework
- **tracing-subscriber** (0.3.19) - Logging subscriber
  - Debug and info level logging
  - Structured log output

### Development Dependencies

#### Testing
- **tempfile** (3.16.0) - Temporary file creation for tests
- **criterion** (0.5.1) - Benchmarking framework
  - Performance testing for rendering operations
  - Cache performance benchmarks

#### Code Quality
- **clippy** (via rustup) - Linting and code quality
- **rustfmt** (via rustup) - Code formatting

## System Dependencies

### Rust Toolchain
- **Rust 1.89.0** - Required Rust version
- **Cargo 1.89.0** - Package manager and build tool
- **rustfmt 1.89.0** - Code formatter
- **clippy 1.89.0** - Linter

### Alternative Toolchain Setup
The project uses `update-alternatives` for managing multiple Rust versions:

```bash
# Install rust-1.89 packages
sudo apt install rust-1.89 rust-1.89-cargo rust-1.89-clippy rust-1.89-fmt

# Configure alternatives
sudo update-alternatives --install /usr/bin/rustc rustc /usr/bin/rustc.1.89 100
sudo update-alternatives --install /usr/bin/cargo cargo /usr/bin/cargo.1.89 100
sudo update-alternatives --install /usr/bin/rustfmt rustfmt /usr/bin/rustfmt.1.89 100
sudo update-alternatives --install /usr/bin/cargo-clippy cargo-clippy /usr/bin/cargo-clippy.1.89 100
```

### Optional Dependencies
- **Git** - Version control and pre-commit hooks
- **Python 3.10+** - For pre-commit hook scripts
- **pre-commit** - Git hook management

## External Tools

### Build and Development
- **cargo** - Primary build tool
  - Use `cargo build --quiet` for clean output
  - Use `cargo test --quiet` for testing
  - Use `cargo clippy --quiet` for linting

### Pre-commit Hooks
- **pre-commit** framework
- **ruff** - Python linting for hook scripts
- **shellcheck** - Shell script linting

## Installation Instructions

### Prerequisites
1. Install Rust toolchain:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   rustup install 1.89.0
   rustup default 1.89.0
   ```

2. Install system dependencies:
   ```bash
   sudo apt update
   sudo apt install git python3-pip
   pip3 install pre-commit
   ```

### Project Setup
1. Clone the repository
2. Install pre-commit hooks:
   ```bash
   pre-commit install
   ```
3. Build the project:
   ```bash
   cargo build --quiet
   ```

## Version Compatibility

### Rust Version Policy
- **Minimum**: Rust 1.89.0
- **Recommended**: Latest stable 1.89.x
- **Testing**: All tests run on 1.89.0

### Dependency Versioning
- Core dependencies use caret versioning (`^1.0.0`) for patch updates
- Development dependencies use exact versions for reproducible builds
- All versions are locked in `Cargo.lock`

## Troubleshooting

### Common Issues

#### Rust Version Mismatch
```bash
# Check current version
rustc --version

# Switch to correct version
rustup default 1.89.0
```

#### Missing Dependencies
```bash
# Update all dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build --quiet
```

#### Pre-commit Hook Failures
```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install

# Run hooks manually
pre-commit run --all-files
```

### Performance Considerations
- Use `--quiet` flag to reduce build output noise
- Enable parallel builds with `CARGO_BUILD_JOBS`
- Consider using `sccache` for faster rebuilds

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

## Security Notes

### Dependency Security
- All dependencies are regularly updated via `cargo update`
- Security vulnerabilities are monitored via `cargo audit`
- Use `cargo audit` before releases

### Build Security
- Build artifacts are reproducible with `cargo build --locked`
- Use `cargo verify-project` for integrity checking
- Consider using `cargo-vet` for additional security analysis

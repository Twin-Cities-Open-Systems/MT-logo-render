# Architecture (v1.0 - Rust Native)

## Overview
MT-logo-render follows a modular, Rust-native architecture designed for performance, safety, and maintainability. The system is built around a clean separation of concerns with strong typing and compile-time guarantees.

## Core Principles

### Memory Safety First
- **Zero unsafe code**: All operations protected by Rust's ownership system
- **Compile-time guarantees**: Memory safety enforced at build time
- **Resource management**: RAII patterns for automatic cleanup

### Performance Optimized
- **Zero-cost abstractions**: Rust idioms without runtime overhead
- **Efficient data structures**: Optimized for cache locality and minimal allocations
- **Streaming processing**: Memory-efficient handling of large operations

### Deterministic by Design
- **Pure functions**: Identical inputs produce identical outputs
- **Immutable data**: Recipe specifications treated as immutable
- **Atomic operations**: File system operations prevent corruption

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    CLI Interface Layer                      │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                 Command Dispatch                     │    │
│  │  ┌─────────────┬─────────────┬─────────────┬─────┐  │    │
│  │  │  Resolve    │   Render    │   Doctor    │List │  │    │
│  │  └─────────────┴─────────────┴─────────────┴─────┘  │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │    Recipe Engine       │
                    │  ┌─────────────────┐   │
                    │  │  Canonicalize   │   │
                    │  │  & Validate     │   │
                    │  └─────────────────┘   │
                    └────────────────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │     Cache System       │
                    │  ┌─────────────────┐   │
                    │  │   Index Mgmt    │   │
                    │  │   Atomic Ops    │   │
                    │  └─────────────────┘   │
                    └────────────────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │   Rendering Pipeline   │
                    │  ┌─────────────┬─────┐  │
                    │  │    PNG      │ANSI │  │
                    │  └─────────────┴─────┘  │
                    └────────────────────────┘
```

## Component Architecture

### 1. CLI Interface Layer

#### Command Dispatch (`cli/`)
**Responsibilities**:
- Parse command-line arguments using `clap`
- Route to appropriate command handlers
- Handle global options (--asset-root, --format, --force)
- Provide consistent error formatting

**Key Components**:
```rust
pub struct Cli {
    pub command: Command,
    pub asset_root: PathBuf,
    pub format: OutputFormat,
    pub force: bool,
}

pub enum Command {
    Resolve(ResolveArgs),
    Render(RenderArgs),
    Doctor(DoctorArgs),
    List(ListArgs),
}
```

**Design Decisions**:
- **clap derive**: Compile-time CLI parsing with type safety
- **Global options**: Consistent behavior across commands
- **Error handling**: User-friendly messages with actionable guidance

#### Command Handlers
**Pattern**: Each command implements a common trait for consistent execution
```rust
pub trait CommandHandler {
    type Args;
    type Output;

    fn execute(&self, args: Self::Args, ctx: &CliContext) -> Result<Self::Output>;
}
```

### 2. Recipe Engine

#### Parsing & Validation (`recipe/`)
**Responsibilities**:
- Parse JSON/YAML recipe specifications
- Validate against schema requirements
- Apply canonicalization rules
- Generate deterministic identifiers

**Key Data Structures**:
```rust
#[derive(Deserialize, Serialize, Clone)]
pub struct Recipe {
    pub shape: Shape,
    pub size: Size,
    pub base_color: Color,
    pub accent_color: Option<Color>,
    pub fill: Fill,
    pub mark: Option<Mark>,
    pub badge: Option<Badge>,
    pub label: Option<String>,
    pub glyph: Option<String>,
    pub font_path: Option<PathBuf>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CanonicalRecipe {
    // Deterministically ordered fields
}
```

**Canonicalization Process**:
1. **Parse**: JSON/YAML to internal representation
2. **Normalize**: Keys sorted, colors standardized, arrays ordered
3. **Validate**: Required fields, type safety, constraint checking
4. **Hash**: SHA256 of canonical JSON for deterministic IDs

#### Effective Recipe Resolution
**Degradation Logic**: When requested features aren't supported
```rust
pub struct EffectiveRecipe {
    pub requested: Recipe,
    pub effective: Recipe,
    pub notes: Vec<String>, // Degradation explanations
}
```

### 3. Cache System

#### Index Management (`cache/index.rs`)
**Responsibilities**:
- Maintain YAML index of all cached assets
- Support efficient querying and filtering
- Ensure atomic updates with file operations

**Index Structure**:
```yaml
entries:
  - stem: "defaultshape-00ff00-solid-circle-256x256"
    recipe:
      requested: {...}
      effective: {...}
    outputs:
      png:
        path: "generated/stem.png"
        exists: true
        sha256: "abc123..."
        size_bytes: 1234
      ansi:
        path: "generated/stem.ansi"
        exists: true
        sha256: "def456..."
        size_bytes: 567
    created_at: "2026-01-22T04:00:00Z"
    notes: []
```

#### Atomic Operations (`cache/atomic.rs`)
**Pattern**: Write to temp files, rename on success
```rust
pub struct AtomicFile {
    temp_path: PathBuf,
    final_path: PathBuf,
}

impl AtomicFile {
    pub fn commit(self) -> Result<()> {
        std::fs::rename(&self.temp_path, &self.final_path)
    }
}
```

### 4. Rendering Pipeline

#### PNG Renderer (`render/png.rs`)
**Based on**: `image` crate for cross-platform raster graphics
**Capabilities**:
- Basic shape primitives (circle, rectangle, polygon)
- Fill patterns with compositing
- Text rendering with font loading
- Antialiased output

**Architecture**:
```rust
pub trait Renderer {
    fn render(&self, recipe: &EffectiveRecipe) -> Result<image::DynamicImage>;
}

pub struct PngRenderer {
    font_cache: FontCache,
}

impl Renderer for PngRenderer {
    fn render(&self, recipe: &EffectiveRecipe) -> Result<image::DynamicImage> {
        let mut canvas = self.create_canvas(&recipe)?;
        self.draw_shape(&mut canvas, &recipe)?;
        self.apply_fill(&mut canvas, &recipe)?;
        self.add_overlays(&mut canvas, &recipe)?;
        Ok(canvas)
    }
}
```

#### ANSI Renderer (`render/ansi.rs`)
**Terminal-Optimized**: Truecolor ANSI escape sequences
**Fallback Support**: 256-color and basic terminal compatibility

**Block Character Art**: Unicode block elements for geometric shapes
```rust
pub struct AnsiRenderer {
    truecolor: bool,
}

impl Renderer for AnsiRenderer {
    fn render(&self, recipe: &EffectiveRecipe) -> Result<String> {
        // Generate ANSI escape sequences
    }
}
```

#### Font System (`render/font.rs`)
**Dual Approach**:
- **Built-in microfont**: ASCII-only, license-safe
- **External fonts**: Unicode support with hash-based identity

**Font Loading**:
```rust
pub enum FontSource {
    BuiltIn,
    External { path: PathBuf, hash: String },
}
```

## Data Flow Architecture

### Resolve Command Flow
```
Input Recipe → Parse → Canonicalize → Generate Stem → Check Cache → JSON Output
     ↓            ↓         ↓             ↓            ↓           ↓
   JSON/YAML   Validate  Normalize     Hash      Exists?    {stem, outputs}
```

### Render Command Flow
```
Input Recipe → Parse → Canonicalize → Check Cache → [Render] → Update Index → JSON Output
     ↓            ↓         ↓             ↓            ↓           ↓           ↓
   JSON/YAML   Validate  Normalize     Exists?     PNG/ANSI   Atomic Write  {written, fingerprints}
```

### Cache Hit Path
- **Fast path**: Index lookup only, no rendering
- **Memory efficient**: No canvas allocation
- **Deterministic**: Same stem generation

### Cache Miss Path
- **Render**: Create canvas, apply operations
- **Write**: Atomic file operations
- **Update**: Index updated last for consistency

## Error Handling Architecture

### Error Types
```rust
#[derive(thiserror::Error, Debug)]
pub enum LogoRenderError {
    #[error("Invalid recipe: {0}")]
    InvalidRecipe(String),

    #[error("Rendering failed: {0}")]
    RenderError(String),

    #[error("Cache corruption: {0}")]
    CacheError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### Error Propagation
- **CLI Layer**: User-friendly messages with suggestions
- **Engine Layer**: Structured errors with context
- **Recovery**: Graceful degradation where possible

## Testing Architecture

### Unit Testing
- **Pure functions**: Recipe canonicalization, stem generation
- **Mocked I/O**: File system operations abstracted
- **Property testing**: Deterministic output verification

### Integration Testing
- **CLI commands**: End-to-end command execution
- **Cache operations**: Atomicity and consistency
- **Cross-platform**: File system and path handling

### Performance Testing
- **Benchmarks**: Rendering time, memory usage
- **Regression detection**: Historical performance tracking
- **Scalability**: Large batch operations

## Performance Characteristics

### Memory Management
- **Stack allocation**: Small data structures where possible
- **Arena allocation**: Efficient for complex rendering operations
- **Streaming**: Large outputs don't require full buffering

### Concurrency Design
- **Single-threaded core**: Deterministic output requirement
- **Parallel processing**: Potential for batch operations
- **Async I/O**: Non-blocking file operations

### Optimization Opportunities
- **SIMD**: Vectorized rendering operations
- **GPU acceleration**: Future graphics hardware utilization
- **Cache optimization**: Memory layout for better locality

## Security Architecture

### Memory Safety
- **Ownership system**: Prevents use-after-free, double-free
- **Borrow checker**: Compile-time lifetime verification
- **No unsafe code**: All operations memory-safe by construction

### Input Validation
- **Schema validation**: Recipe structure verification
- **Path safety**: No directory traversal vulnerabilities
- **Size limits**: Prevent resource exhaustion attacks

### File System Security
- **Atomic operations**: Prevent partial writes
- **Permission handling**: Appropriate file permissions
- **Path canonicalization**: Resolve symlinks and relative paths

## Deployment Architecture

### Binary Distribution
- **Single static binary**: No runtime dependencies
- **Cross-compilation**: Support multiple target platforms
- **Size optimization**: Strip debug symbols, compress

### Configuration
- **Environment-based**: No config files required
- **Command-line options**: All configuration via CLI
- **Sensible defaults**: Works out-of-the-box

### Ecosystem Integration
- **CLI-first design**: Easy integration with scripts and tools
- **JSON interfaces**: Machine-readable for automation
- **Deterministic outputs**: Reliable for build systems

## Evolution Planning

### Extensibility Points
- **New shapes**: Plugin architecture for custom primitives
- **New renderers**: Support for additional output formats
- **Advanced fills**: Complex pattern generation
- **Font system**: Additional font format support

### Backward Compatibility
- **Stable CLI interface**: Command structure preserved
- **Recipe format**: Additive changes only
- **Output determinism**: Guaranteed across versions

### Performance Scaling
- **Memory optimization**: Reduced memory footprint
- **Parallel rendering**: Multi-core utilization
- **Streaming output**: Large asset handling

This architecture provides a solid foundation for reliable, high-performance logo asset generation while maintaining Rust's safety and performance guarantees.

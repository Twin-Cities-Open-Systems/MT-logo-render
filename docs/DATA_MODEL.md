# Data Model (v1.0)

## Overview
MT-logo-render uses structured data models for recipe specifications, canonicalization, and cache management. All data structures are designed for deterministic processing and efficient serialization.

## Core Data Structures

### Recipe Specification

#### Base Recipe Schema
```rust
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Recipe {
    /// Shape type for rendering
    pub shape: Shape,

    /// Output dimensions in pixels (WxH)
    pub size: Size,

    /// Primary fill color
    pub base_color: Color,

    /// Secondary color for patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<Color>,

    /// Fill pattern specification
    #[serde(default)]
    pub fill: Fill,

    /// Overlay mark (check, x, dot)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<Mark>,

    /// Corner badge indicator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<Badge>,

    /// ASCII label text (1-4 chars recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Unicode glyph string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glyph: Option<String>,

    /// Filesystem path to font file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_path: Option<PathBuf>,
}
```

#### Shape Enumeration
```rust
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "triangle")]
    Triangle,
    #[serde(rename = "hex")]
    Hex,
}
```

#### Size Specification
```rust
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            return Err("Size must be in WxH format".to_string());
        }

        let width = parts[0].parse().map_err(|_| "Invalid width")?;
        let height = parts[1].parse().map_err(|_| "Invalid height")?;

        if width < 16 || height < 16 || width > 4096 || height > 4096 {
            return Err("Size must be between 16x16 and 4096x4096".to_string());
        }

        Ok(Size { width, height })
    }
}
```

#### Color Representation
```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Named(String),    // "red", "blue", etc.
    Hex(String),      // "#ff0000", "00ff00", etc.
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            // Validate hex format
            let hex = &s[1..];
            if hex.len() == 3 || hex.len() == 6 {
                for ch in hex.chars() {
                    if !ch.is_ascii_hexdigit() {
                        return Err("Invalid hex color".to_string());
                    }
                }
                Ok(Color::Hex(s.to_string()))
            } else {
                Err("Hex colors must be 3 or 6 digits".to_string())
            }
        } else {
            // Named color - basic validation
            if s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                Ok(Color::Named(s.to_string()))
            } else {
                Err("Invalid named color".to_string())
            }
        }
    }
}
```

#### Fill Patterns
```rust
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Fill {
    Solid,
    Pie(u16),      // Degrees (circle only)
    Split(u8),     // Number of segments
    Stripe(u8),    // Number of stripes
}

impl Default for Fill {
    fn default() -> Self {
        Fill::Solid
    }
}
```

#### Overlay Elements
```rust
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Mark {
    Check,
    X,
    Dot,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Badge {
    CornerDot,
    CornerCheck,
}
```

## Canonicalization Process

### Recipe Canonicalization
Recipes are normalized to ensure deterministic output regardless of input format variations.

#### Step 1: Parse and Validate
```rust
fn parse_recipe(input: &str) -> Result<Recipe, Error> {
    // Parse JSON/YAML
    // Validate required fields
    // Apply default values
}
```

#### Step 2: Normalize Colors
```rust
fn normalize_color(color: &Color) -> String {
    match color {
        Color::Named(name) => {
            // Convert named colors to hex
            match name.as_str() {
                "red" => "ff0000",
                "green" => "00ff00",
                "blue" => "0000ff",
                // ... more named colors
                _ => return Err("Unknown color name".into()),
            }
        }
        Color::Hex(hex) => {
            // Normalize hex format (remove #, lowercase, expand short form)
            let hex = hex.strip_prefix('#').unwrap_or(hex);
            let hex = hex.to_lowercase();
            match hex.len() {
                3 => format!("{}{}{}{}{}{}", &hex[0..1], &hex[0..1], &hex[1..2], &hex[1..2], &hex[2..3], &hex[2..3]),
                6 => hex,
                _ => return Err("Invalid hex length".into()),
            }
        }
    }
}
```

#### Step 3: Sanitize Text Fields
```rust
fn sanitize_label(label: &str) -> Option<String> {
    let sanitized: String = label.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '.' || *c == '-')
        .take(4)
        .collect();

    if sanitized.is_empty() {
        None
    } else {
        Some(sanitized.to_uppercase())
    }
}
```

#### Step 4: Generate Canonical JSON
```rust
fn canonicalize_recipe(recipe: &Recipe) -> Result<String, Error> {
    // Create normalized recipe
    let normalized = Recipe {
        shape: recipe.shape,
        size: recipe.size,
        base_color: normalize_color(&recipe.base_color)?,
        accent_color: recipe.accent_color.as_ref().map(|c| normalize_color(c)).transpose()?,
        fill: recipe.fill.clone(),
        mark: recipe.mark,
        badge: recipe.badge,
        label: recipe.label.as_ref().map(|l| sanitize_label(l)).flatten(),
        glyph: recipe.glyph.clone(),
        font_path: recipe.font_path.clone(),
    };

    // Serialize with sorted keys
    serde_json::to_string(&normalized)
}
```

### Stem Generation
Deterministic filename stems are generated from canonicalized recipes.

#### Stem Format
```
<recipe_id>-<base_color>[-<accent_color>]-<tokens>-<size>
```

#### Recipe ID Generation
```rust
fn generate_recipe_id(canonical_json: &str) -> String {
    use sha2::{Digest, Sha256};

    let hash = Sha256::digest(canonical_json.as_bytes());
    format!("recipe-{:x}", hash[0..8].iter().fold(0u64, |acc, &x| (acc << 8) | x as u64))
}

// For default shapes, use "defaultshape"
fn get_recipe_id(recipe: &Recipe) -> String {
    // Check if recipe uses only default shape features
    if recipe.glyph.is_none() && recipe.font_path.is_none() {
        "defaultshape".to_string()
    } else {
        // Generate hash-based ID for custom recipes
        let canonical = canonicalize_recipe(recipe)?;
        generate_recipe_id(&canonical)
    }
}
```

#### Token Generation
```rust
fn generate_tokens(recipe: &Recipe) -> Vec<String> {
    let mut tokens = Vec::new();

    // Fill token (always present)
    tokens.push(match &recipe.fill {
        Fill::Solid => "fill-solid".to_string(),
        Fill::Pie(degrees) => format!("fill-pie:{}", degrees),
        Fill::Split(segments) => format!("fill-split:{}", segments),
        Fill::Stripe(count) => format!("fill-stripe:{}", count),
    });

    // Mark token (always present)
    tokens.push(match recipe.mark {
        Some(Mark::Check) => "mark-check".to_string(),
        Some(Mark::X) => "mark-x".to_string(),
        Some(Mark::Dot) => "mark-dot".to_string(),
        None => "mark-none".to_string(),
    });

    // Badge token (always present)
    tokens.push(match recipe.badge {
        Some(Badge::CornerDot) => "badge-corner-dot".to_string(),
        Some(Badge::CornerCheck) => "badge-corner-check".to_string(),
        None => "badge-none".to_string(),
    });

    // Label token (only if present)
    if let Some(label) = &recipe.label {
        tokens.push(format!("label-{}", label.to_uppercase()));
    }

    // Glyph token (only if present and renderable)
    if let Some(glyph) = &recipe.glyph {
        let encoded = encode_unicode_glyph(glyph);
        tokens.push(format!("glyph-{}", encoded));
    }

    // Font token (only if external font used)
    if let Some(font_path) = &recipe.font_path {
        let hash = compute_font_hash(font_path)?;
        tokens.push(format!("font-{:x}", hash[0..8].iter().fold(0u64, |acc, &x| (acc << 8) | x as u64)));
    }

    tokens
}

fn encode_unicode_glyph(glyph: &str) -> String {
    glyph.chars()
        .map(|c| format!("U{:08X}", c as u32))
        .collect::<Vec<_>>()
        .join("_")
}
```

#### Complete Stem Generation
```rust
fn generate_stem(recipe: &Recipe) -> Result<String, Error> {
    let recipe_id = get_recipe_id(recipe)?;
    let base_color = normalize_color(&recipe.base_color)?;
    let accent_token = recipe.accent_color
        .as_ref()
        .map(|c| format!("-{}", normalize_color(c)))
        .unwrap_or_default();

    let tokens = generate_tokens(recipe)?;
    let token_string = tokens.join("-");

    let size = format!("{}x{}", recipe.size.width, recipe.size.height);

    Ok(format!("{}-{}-{}-{}-{}", recipe_id, base_color, accent_token, token_string, size))
}
```

## Cache Data Structures

### Cache Index Format
```yaml
entries:
  - stem: "defaultshape-0000ff-solid-circle-256x256"
    recipe:
      requested: {...}
      effective: {...}
    outputs:
      png:
        path: "assets/logo/generated/defaultshape-0000ff-solid-circle-256x256.png"
        exists: true
        sha256: "a1b2c3d4..."
        size_bytes: 2048
      ansi:
        path: "assets/logo/generated/defaultshape-0000ff-solid-circle-256x256.ansi"
        exists: true
        sha256: "e5f6g7h8..."
        size_bytes: 256
    created_at: "2026-01-22T04:00:00Z"
    notes: []
```

### Cache Entry Structure
```rust
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheEntry {
    pub stem: String,
    pub recipe: RecipePair,
    pub outputs: HashMap<OutputFormat, OutputInfo>,
    pub created_at: DateTime<Utc>,
    pub notes: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RecipePair {
    pub requested: Recipe,
    pub effective: Recipe,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OutputInfo {
    pub path: PathBuf,
    pub exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}
```

### Cache Index Management
```rust
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheIndex {
    pub entries: Vec<CacheEntry>,
}

impl CacheIndex {
    pub fn load(path: &Path) -> Result<Self, Error> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            serde_yaml::from_str(&content)
        } else {
            Ok(CacheIndex { entries: Vec::new() })
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let content = serde_yaml::to_string(self)?;
        atomic_write(path, content.as_bytes())
    }

    pub fn find_by_stem(&self, stem: &str) -> Option<&CacheEntry> {
        self.entries.iter().find(|e| e.stem == stem)
    }

    pub fn add_entry(&mut self, entry: CacheEntry) {
        // Remove existing entry with same stem
        self.entries.retain(|e| e.stem != entry.stem);
        self.entries.push(entry);
    }
}
```

## Effective Recipe Resolution

### Degradation Logic
When requested features aren't supported, recipes are gracefully degraded.

```rust
pub fn resolve_effective_recipe(requested: &Recipe) -> (Recipe, Vec<String>) {
    let mut effective = requested.clone();
    let mut notes = Vec::new();

    // Validate and degrade fill patterns
    match (&requested.shape, &requested.fill) {
        (Shape::Circle, Fill::Pie(_)) => {
            // Pie fill is valid for circles
        }
        (_, Fill::Pie(_)) => {
            // Degrade to solid for other shapes
            effective.fill = Fill::Solid;
            notes.push("Pie fill only supported for circles, using solid".to_string());
        }
        (Shape::Hex, Fill::Split(n)) if *n == 2 || *n == 3 || *n == 6 => {
            // Valid split counts for hexagons
        }
        (_, Fill::Split(_)) => {
            // Degrade to solid
            effective.fill = Fill::Solid;
            notes.push("Unsupported split count for shape, using solid".to_string());
        }
        _ => {
            // Other fills are generally supported
        }
    }

    // Validate glyph rendering capability
    if let Some(glyph) = &requested.glyph {
        if requested.font_path.is_none() {
            // Check if glyph is ASCII-renderable with microfont
            if !is_ascii_only(glyph) {
                effective.glyph = None;
                notes.push("Unicode glyph requires font_path, omitted".to_string());
            }
        }
    }

    (effective, notes)
}
```

## Validation Rules

### Recipe Validation
```rust
impl Recipe {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Size constraints
        if self.size.width < 16 || self.size.height < 16 {
            errors.push("Size must be at least 16x16".to_string());
        }
        if self.size.width > 4096 || self.size.height > 4096 {
            errors.push("Size must not exceed 4096x4096".to_string());
        }

        // Label constraints
        if let Some(label) = &self.label {
            if label.is_empty() {
                errors.push("Label cannot be empty".to_string());
            }
            if label.chars().count() > 4 {
                errors.push("Label must be 4 characters or less".to_string());
            }
        }

        // Font path validation
        if let Some(path) = &self.font_path {
            if !path.exists() {
                errors.push(format!("Font path does not exist: {}", path.display()));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## Serialization Compatibility

### JSON Schema Support
All data structures support JSON Schema generation for validation:

```rust
impl Recipe {
    pub fn json_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["shape", "size", "base_color"],
            "properties": {
                "shape": {
                    "enum": ["circle", "square", "triangle", "hex"]
                },
                "size": {
                    "type": "string",
                    "pattern": "^\\d+x\\d+$"
                },
                "base_color": {
                    "type": "string"
                }
                // ... additional properties
            }
        })
    }
}
```

### Version Compatibility
Data structures include version information for future compatibility:

```rust
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VersionedRecipe {
    pub version: String,  // "1.0"
    pub recipe: Recipe,
}
```

This data model provides a solid foundation for deterministic asset generation with comprehensive validation, canonicalization, and caching support.

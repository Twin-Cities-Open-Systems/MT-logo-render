//! MT-logo-render CLI
//!
//! A Human Execution Engine (HEE) for deterministic logo asset generation.

use clap::{Parser, Subcommand};
use mt_logo_render::recipe::{Badge, Mark, Shape};
use mt_logo_render::{Error, Recipe, Result};
use std::path::{Path, PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
enum OutputFormat {
    Json,
    Yaml,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(format!("Invalid format: {}. Use 'json' or 'yaml'", s)),
        }
    }
}

#[derive(Parser)]
#[command(name = "logo-render")]
#[command(about = "MT-logo-render: Human Execution Engine for deterministic logo assets")]
#[command(version, author, long_about = None)]
struct Cli {
    /// Asset root directory
    #[arg(long, default_value = "assets/logo")]
    asset_root: PathBuf,

    /// Output format (json or yaml)
    #[arg(long, default_value = "json")]
    format: OutputFormat,

    /// Force re-rendering (bypass cache)
    #[arg(long)]
    force: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compute deterministic filenames without rendering
    Resolve {
        /// Recipe as JSON string, file path, or stdin
        recipe: Option<String>,

        /// Read recipe from file
        #[arg(short, long)]
        file: Option<PathBuf>,
    },

    /// Ensure outputs exist, generating on cache miss
    Render {
        /// Recipe as JSON string, file path, or stdin
        recipe: Option<String>,

        /// Read recipe from file
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Output targets (comma-separated: png,ansi,ansi256,md,html)
        #[arg(long, default_value = "png,ansi")]
        targets: String,
    },

    /// Environment self-check and capability reporting
    Doctor,

    /// Query cache index with optional filtering
    List {
        /// Filter by shape
        #[arg(long)]
        shape: Option<String>,

        /// Filter by size (WxH format)
        #[arg(long)]
        size: Option<String>,

        /// Filter by base color
        #[arg(long)]
        base_color: Option<String>,

        /// Filter by fill pattern
        #[arg(long)]
        fill: Option<String>,

        /// Only show entries with existing files
        #[arg(long)]
        exists: bool,

        /// Only show entries with missing files
        #[arg(long)]
        missing: bool,
    },
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    // Validate asset root
    if !cli.asset_root.exists() {
        std::fs::create_dir_all(&cli.asset_root)?;
    }

    let ctx = CliContext {
        asset_root: cli.asset_root,
        format: cli.format,
        force: cli.force,
    };

    match cli.command {
        Commands::Resolve { recipe, file } => {
            handle_resolve(recipe, file, &ctx)?;
        }
        Commands::Render {
            recipe,
            file,
            targets,
        } => {
            handle_render(recipe, file, targets, &ctx)?;
        }
        Commands::Doctor => {
            // TODO: Implement doctor command
            println!("Doctor command not yet implemented");
            std::process::exit(1);
        }
        Commands::List {
            shape: _shape,
            size: _size,
            base_color: _base_color,
            fill: _fill,
            exists: _exists,
            missing: _missing,
        } => {
            // TODO: Implement list command
            println!("List command not yet implemented");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Render recipe to PNG format
fn render_png(recipe: &Recipe, output_path: &Path) -> Result<()> {
    use image::{ImageBuffer, Rgba};
    use std::fs;

    // Create directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let canonical = recipe.canonicalize()?;
    let width = canonical.size.width;
    let height = canonical.size.height;

    // Create image buffer
    let mut img = ImageBuffer::new(width, height);

    // Parse base color
    let base_color = parse_hex_color(&canonical.base_color)?;
    let accent_color = canonical
        .accent_color
        .as_ref()
        .and_then(|c| parse_hex_color(c).ok())
        .unwrap_or(base_color);

    // Fill background
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgba([base_color.0, base_color.1, base_color.2, 255]);
    }

    // Render shape
    match canonical.shape {
        Shape::Circle => render_circle(&mut img, &canonical, base_color, accent_color),
        Shape::Square => render_square(&mut img, &canonical, base_color, accent_color),
        Shape::Triangle => render_triangle(&mut img, &canonical, base_color, accent_color),
        Shape::Hex => render_hex(&mut img, &canonical, base_color, accent_color),
    }

    // Render mark if present
    if let Some(mark) = canonical.mark {
        render_mark(&mut img, mark, &canonical, accent_color);
    }

    // Render badge if present
    if let Some(badge) = canonical.badge {
        render_badge(&mut img, badge, &canonical, accent_color);
    }

    // Render label if present
    if let Some(label) = &canonical.label {
        render_label(&mut img, label, &canonical)?;
    }

    // Save PNG directly
    img.save(output_path)?;

    Ok(())
}

/// Render recipe to ANSI format
fn render_ansi(recipe: &Recipe, output_path: &Path) -> Result<()> {
    use std::fs;

    // Create directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let canonical = recipe.canonicalize()?;
    let width = canonical.size.width as usize;
    let height = canonical.size.height as usize;

    // Parse colors
    let base_color = parse_hex_color(&canonical.base_color)?;
    let accent_color = canonical
        .accent_color
        .as_ref()
        .and_then(|c| parse_hex_color(c).ok())
        .unwrap_or(base_color);

    // Generate ANSI art (simplified - just text representation for now)
    let mut ansi_output = format!("Logo: {}x{}\n", width, height);
    ansi_output.push_str(&format!("Shape: {:?}\n", canonical.shape));
    ansi_output.push_str(&format!(
        "Base Color: #{:02x}{:02x}{:02x}\n",
        base_color.0, base_color.1, base_color.2
    ));

    if let Some(accent) = canonical.accent_color {
        ansi_output.push_str(&format!("Accent Color: {}\n", accent));
    }

    if let Some(label) = &canonical.label {
        ansi_output.push_str(&format!("Label: {}\n", label));
    }

    // Write to file atomically
    let temp_path = output_path.with_extension("ansi.tmp");
    fs::write(&temp_path, ansi_output)?;
    std::fs::rename(temp_path, output_path)?;

    Ok(())
}

/// Parse hex color string to RGB tuple
fn parse_hex_color(hex: &str) -> Result<(u8, u8, u8)> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| Error::Validation(format!("Invalid hex color: {}", hex)))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| Error::Validation(format!("Invalid hex color: {}", hex)))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| Error::Validation(format!("Invalid hex color: {}", hex)))?;
        Ok((r, g, b))
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
            .map_err(|_| Error::Validation(format!("Invalid hex color: {}", hex)))?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
            .map_err(|_| Error::Validation(format!("Invalid hex color: {}", hex)))?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
            .map_err(|_| Error::Validation(format!("Invalid hex color: {}", hex)))?;
        Ok((r, g, b))
    } else {
        Err(Error::Validation(format!("Invalid hex color: {}", hex)))
    }
}

/// Render circle shape
fn render_circle(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    recipe: &mt_logo_render::CanonicalRecipe,
    base_color: (u8, u8, u8),
    _accent_color: (u8, u8, u8),
) {
    let width = recipe.size.width as f32;
    let height = recipe.size.height as f32;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = (width.min(height) / 2.0) * 0.8;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= radius {
            // Inside circle - use base color
            *pixel = image::Rgba([base_color.0, base_color.1, base_color.2, 255]);
        }
        // Outside circle - keep background color
    }
}

/// Render square shape
fn render_square(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    recipe: &mt_logo_render::CanonicalRecipe,
    base_color: (u8, u8, u8),
    _accent_color: (u8, u8, u8),
) {
    let width = recipe.size.width;
    let height = recipe.size.height;
    let margin = (width.min(height) as f32 * 0.1) as u32;
    let square_size = width.min(height) - 2 * margin;
    let start_x = (width - square_size) / 2;
    let start_y = (height - square_size) / 2;

    for y in start_y..(start_y + square_size) {
        for x in start_x..(start_x + square_size) {
            if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                *pixel = image::Rgba([base_color.0, base_color.1, base_color.2, 255]);
            }
        }
    }
}

/// Render triangle shape
fn render_triangle(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    recipe: &mt_logo_render::CanonicalRecipe,
    base_color: (u8, u8, u8),
    accent_color: (u8, u8, u8),
) {
    let width = recipe.size.width as f32;
    let height = recipe.size.height as f32;
    let center_x = width / 2.0;
    let base_y = height * 0.9;
    let top_y = height * 0.1;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let xf = x as f32;
        let yf = y as f32;

        // Simple triangle check (point-in-triangle)
        let area = 0.0; // Simplified calculation for now

        if yf >= top_y && yf <= base_y {
            let progress = (yf - top_y) / (base_y - top_y);
            let triangle_width = width * (1.0 - progress) * 0.8;
            let left = center_x - triangle_width / 2.0;
            let right = center_x + triangle_width / 2.0;

            if xf >= left && xf <= right {
                *pixel = image::Rgba([base_color.0, base_color.1, base_color.2, 255]);
            }
        }
    }
}

/// Render hexagon shape
fn render_hex(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    recipe: &mt_logo_render::CanonicalRecipe,
    base_color: (u8, u8, u8),
    accent_color: (u8, u8, u8),
) {
    let width = recipe.size.width as f32;
    let height = recipe.size.height as f32;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = (width.min(height) / 2.0) * 0.8;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;

        // Hexagon distance calculation (simplified)
        let dist = (dx.abs().powf(2.0) + dy.abs().powf(2.0)).sqrt();
        if dist <= radius {
            *pixel = image::Rgba([base_color.0, base_color.1, base_color.2, 255]);
        }
    }
}

/// Render overlay mark
fn render_mark(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    mark: Mark,
    recipe: &mt_logo_render::CanonicalRecipe,
    color: (u8, u8, u8),
) {
    let center_x = recipe.size.width as i32 / 2;
    let center_y = recipe.size.height as i32 / 2;
    let size = (recipe.size.width.min(recipe.size.height) as i32 / 4).max(10);

    match mark {
        Mark::Check => {
            // Draw checkmark
            let points = [
                (center_x - size / 2, center_y),
                (center_x - size / 6, center_y + size / 3),
                (center_x + size / 2, center_y - size / 2),
            ];
            draw_line(img, points[0], points[1], color);
            draw_line(img, points[1], points[2], color);
        }
        Mark::X => {
            // Draw X
            draw_line(
                img,
                (center_x - size / 2, center_y - size / 2),
                (center_x + size / 2, center_y + size / 2),
                color,
            );
            draw_line(
                img,
                (center_x + size / 2, center_y - size / 2),
                (center_x - size / 2, center_y + size / 2),
                color,
            );
        }
        Mark::Dot => {
            // Draw dot
            let radius = size / 4;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx * dx + dy * dy <= radius * radius {
                        let x = center_x + dx;
                        let y = center_y + dy;
                        if x >= 0
                            && x < recipe.size.width as i32
                            && y >= 0
                            && y < recipe.size.height as i32
                        {
                            img.put_pixel(
                                x as u32,
                                y as u32,
                                image::Rgba([color.0, color.1, color.2, 255]),
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Render corner badge
fn render_badge(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    badge: Badge,
    recipe: &mt_logo_render::CanonicalRecipe,
    color: (u8, u8, u8),
) {
    let size = (recipe.size.width.min(recipe.size.height) as i32 / 8).max(8);
    let margin = size / 2;

    let (start_x, start_y) = match badge {
        Badge::CornerDot => (recipe.size.width as i32 - size - margin, margin),
        Badge::CornerCheck => (margin, margin),
    };

    match badge {
        Badge::CornerDot => {
            // Draw small circle
            let radius = size / 2;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx * dx + dy * dy <= radius * radius {
                        let x = start_x + radius + dx;
                        let y = start_y + radius + dy;
                        if x >= 0
                            && x < recipe.size.width as i32
                            && y >= 0
                            && y < recipe.size.height as i32
                        {
                            img.put_pixel(
                                x as u32,
                                y as u32,
                                image::Rgba([color.0, color.1, color.2, 255]),
                            );
                        }
                    }
                }
            }
        }
        Badge::CornerCheck => {
            // Draw small checkmark
            let half_size = size / 2;
            let points = [
                (start_x + half_size / 2, start_y + half_size),
                (start_x + half_size * 3 / 4, start_y + half_size * 3 / 4),
                (start_x + half_size * 3 / 2, start_y + half_size / 2),
            ];
            draw_line(img, points[0], points[1], color);
            draw_line(img, points[1], points[2], color);
        }
    }
}

/// Render text label (simplified - just draw a colored rectangle for now)
fn render_label(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    label: &str,
    recipe: &mt_logo_render::CanonicalRecipe,
) -> Result<()> {
    // For now, just draw a colored rectangle in the bottom area
    // TODO: Implement proper text rendering with rusttype
    let width = recipe.size.width;
    let height = recipe.size.height;
    let label_height = height / 8;
    let start_y = height - label_height;

    for y in start_y..height {
        for x in 0..width {
            if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                // Use a contrasting color for label background
                *pixel = image::Rgba([255, 255, 255, 200]); // Semi-transparent white
            }
        }
    }

    Ok(())
}

/// Draw a line between two points
fn draw_line(
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    start: (i32, i32),
    end: (i32, i32),
    color: (u8, u8, u8),
) {
    let (x1, y1) = start;
    let (x2, y2) = end;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
            img.put_pixel(
                x as u32,
                y as u32,
                image::Rgba([color.0, color.1, color.2, 255]),
            );
        }

        if x == x2 && y == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn handle_render(
    recipe: Option<String>,
    file: Option<PathBuf>,
    targets: String,
    ctx: &CliContext,
) -> Result<()> {
    use mt_logo_render::{Cache, Recipe};
    use std::fs;
    use std::io::{self, Read};

    // Get recipe content
    let recipe_content = if let Some(recipe_str) = recipe {
        recipe_str
    } else if let Some(file_path) = file {
        fs::read_to_string(file_path)?
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Parse recipe
    let recipe: Recipe = if recipe_content.trim().starts_with('{') {
        Recipe::from_json(&recipe_content)?
    } else {
        Recipe::from_yaml(&recipe_content)?
    };

    // Validate recipe
    recipe.validate()?;

    // Generate stem for output paths
    let stem = recipe.generate_stem()?;

    // Initialize cache
    let mut cache = Cache::new(&ctx.asset_root)?;

    // Parse targets
    let targets: Vec<&str> = targets.split(',').map(|s| s.trim()).collect();
    let mut rendered_files = Vec::new();
    let mut fingerprints = std::collections::HashMap::new();

    // Process each target
    for target in targets {
        match target {
            "png" => {
                let output_path = ctx.asset_root.join(format!("{}.png", stem));

                // Check cache first
                if !ctx.force {
                    if let Some(output_info) = cache.check_output(&stem, "png") {
                        if output_info.exists {
                            println!(
                                "PNG already exists (use --force to override): {}",
                                output_path.display()
                            );
                            continue;
                        }
                    }
                }

                // Render PNG
                render_png(&recipe, &output_path)?;

                // Update cache with the actual output path
                cache.update_entry(&stem, &recipe, "png", &output_path)?;

                // Compute fingerprint
                if let Ok(hash) = compute_file_hash(&output_path) {
                    fingerprints.insert(output_path.display().to_string(), hash.clone());
                }

                rendered_files.push(format!("png:{}", output_path.display()));
            }
            "ansi" => {
                let output_path = ctx.asset_root.join(format!("{}.ansi", stem));

                // Check cache first
                if !ctx.force {
                    if let Some(output_info) = cache.check_output(&stem, "ansi") {
                        if output_info.exists {
                            println!(
                                "ANSI already exists (use --force to override): {}",
                                output_path.display()
                            );
                            continue;
                        }
                    }
                }

                // Render ANSI
                render_ansi(&recipe, &output_path)?;

                // Update cache
                cache.update_entry(&stem, &recipe, "ansi", &output_path)?;

                // Compute fingerprint
                if let Ok(hash) = compute_file_hash(&output_path) {
                    fingerprints.insert(output_path.display().to_string(), hash.clone());
                }

                rendered_files.push(format!("ansi:{}", output_path.display()));
            }
            _ => {
                eprintln!("Warning: Unknown target '{}', skipping", target);
            }
        }
    }

    // Prepare output
    let effective = mt_logo_render::resolve_effective_recipe(&recipe);
    let output = serde_json::json!({
        "requested_spec": effective.requested,
        "effective_spec": effective.effective,
        "stem": stem,
        "outputs": {
            "png": cache.check_output(&stem, "png"),
            "ansi": cache.check_output(&stem, "ansi"),
        },
        "written": rendered_files,
        "fingerprints": fingerprints,
        "notes": effective.notes
    });

    // Output in requested format
    match ctx.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&output)?);
        }
    }

    Ok(())
}

/// Compute SHA256 hash of a file
fn compute_file_hash(path: &std::path::Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let content = std::fs::read(path)?;
    let hash = Sha256::digest(&content);
    Ok(format!("{:x}", hash))
}

struct CliContext {
    asset_root: PathBuf,
    format: OutputFormat,
    force: bool,
}

fn handle_resolve(recipe: Option<String>, file: Option<PathBuf>, ctx: &CliContext) -> Result<()> {
    use mt_logo_render::{resolve_effective_recipe, Recipe};
    use std::fs;
    use std::io::{self, Read};

    // Get recipe content
    let recipe_content = if let Some(recipe_str) = recipe {
        recipe_str
    } else if let Some(file_path) = file {
        fs::read_to_string(file_path)?
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Parse recipe
    let recipe: Recipe = if recipe_content.trim().starts_with('{') {
        Recipe::from_json(&recipe_content)?
    } else {
        Recipe::from_yaml(&recipe_content)?
    };

    // Validate recipe
    recipe.validate()?;

    // Resolve effective recipe
    let effective = resolve_effective_recipe(&recipe);

    // Generate stem
    let stem = recipe.generate_stem()?;

    // Prepare output
    let output = serde_json::json!({
        "stem": stem,
        "requested": effective.requested,
        "effective": effective.effective,
        "notes": effective.notes
    });

    // Output in requested format
    match ctx.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&output)?);
        }
    }

    Ok(())
}
<task_progress>
- [x] Analyze CI monitor script
- [x] Check failing tests details
- [x] Implement auto-fixing for HEE Security Scan
- [x] Implement auto-fixing for Pre-commit Hooks
- [x] Implement auto-fixing for Code Quality
- [ ] Monitor CI results and iterate
</task_progress>
</write_to_file>

//! MT-logo-render CLI
//!
//! A Human Execution Engine (HEE) for deterministic logo asset generation.

use clap::{Parser, Subcommand};
use mt_logo_render::{Error, Result, Recipe, resolve_effective_recipe};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        Commands::Render { recipe, file, targets } => {
            // TODO: Implement render command
            println!("Render command not yet implemented");
            std::process::exit(1);
        }
        Commands::Doctor => {
            // TODO: Implement doctor command
            println!("Doctor command not yet implemented");
            std::process::exit(1);
        }
        Commands::List { shape, size, base_color, fill, exists, missing } => {
            // TODO: Implement list command
            println!("List command not yet implemented");
            std::process::exit(1);
        }
    }

    Ok(())
}

struct CliContext {
    asset_root: PathBuf,
    format: OutputFormat,
    force: bool,
}

fn handle_resolve(recipe: Option<String>, file: Option<PathBuf>, ctx: &CliContext) -> Result<()> {
    use mt_logo_render::{Recipe, resolve_effective_recipe};
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

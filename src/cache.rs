//! Cache system for MT-logo-render
//!
//! Manages deterministic asset caching with atomic operations and YAML index.

use crate::recipe::{Color, Fill};
use crate::{Error, Recipe, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Cache index entry for tracking generated assets
#[derive(Deserialize, Serialize, Clone, Debug)]
#[allow(dead_code)]
pub struct CacheEntry {
    pub stem: String,
    pub recipe: RecipePair,
    pub outputs: HashMap<String, OutputInfo>,
    pub created_at: DateTime<Utc>,
    pub notes: Vec<String>,
}

/// Pair of requested and effective recipes
#[derive(Deserialize, Serialize, Clone, Debug)]
#[allow(dead_code)]
pub struct RecipePair {
    pub requested: Recipe,
    pub effective: Recipe,
}

/// Information about a generated output file
#[derive(Deserialize, Serialize, Clone, Debug)]
#[allow(dead_code)]
pub struct OutputInfo {
    pub path: PathBuf,
    pub exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

/// Main cache management system
#[derive(Debug)]
#[allow(dead_code)]
pub struct Cache {
    index_path: PathBuf,
    generated_dir: PathBuf,
    index: CacheIndex,
}

/// Cache index containing all entries
#[derive(Deserialize, Serialize, Clone, Debug)]
#[allow(dead_code)]
pub struct CacheIndex {
    pub entries: Vec<CacheEntry>,
}

impl Cache {
    /// Create a new cache instance
    pub fn new(asset_root: &Path) -> Result<Self> {
        let generated_dir = asset_root.join("generated");
        let index_path = asset_root.join("index.yaml");

        // Ensure directories exist
        fs::create_dir_all(&generated_dir)?;

        // Load existing index or create new one
        let index = CacheIndex::load(&index_path)?;

        // Ensure index file exists (create if it doesn't)
        if !index_path.exists() {
            index.save(&index_path)?;
        }

        Ok(Cache {
            index_path,
            generated_dir,
            index,
        })
    }

    /// Get the path for a generated file
    pub fn get_output_path(&self, stem: &str, format: &str) -> PathBuf {
        self.generated_dir.join(format!("{}.{}", stem, format))
    }

    /// Check if an output file exists and is valid
    pub fn check_output(&self, stem: &str, format: &str) -> Option<OutputInfo> {
        let path = self.get_output_path(stem, format);

        if path.exists() {
            let metadata = path.metadata().ok()?;
            let size_bytes = metadata.len();
            let sha256 = compute_file_hash(&path).ok();

            Some(OutputInfo {
                path,
                exists: true,
                sha256,
                size_bytes: Some(size_bytes),
            })
        } else {
            Some(OutputInfo {
                path,
                exists: false,
                sha256: None,
                size_bytes: None,
            })
        }
    }

    /// Update cache entry for a rendered output
    #[allow(clippy::ptr_arg)]
    pub fn update_entry(
        &mut self,
        stem: &str,
        recipe: &Recipe,
        format: &str,
        path: &PathBuf,
    ) -> Result<()> {
        let effective = crate::resolve_effective_recipe(recipe);
        let recipe_pair = RecipePair {
            requested: recipe.clone(),
            effective: effective.effective,
        };

        let output_info = OutputInfo {
            path: path.clone(),
            exists: true,
            sha256: compute_file_hash(path).ok(),
            size_bytes: Some(path.metadata()?.len()),
        };

        // Check if entry exists
        let entry_exists = self.index.entries.iter().any(|e| e.stem == stem);

        if entry_exists {
            // Update existing entry
            for entry in self.index.entries.iter_mut() {
                if entry.stem == stem {
                    // Update recipe info if it's different
                    if entry.recipe.requested != *recipe {
                        entry.recipe = recipe_pair;
                    }

                    // Update output info
                    entry.outputs.insert(format.to_string(), output_info);
                    break;
                }
            }
        } else {
            // Create new entry
            self.index.entries.push(CacheEntry {
                stem: stem.to_string(),
                recipe: recipe_pair,
                outputs: {
                    let mut outputs = HashMap::new();
                    outputs.insert(format.to_string(), output_info);
                    outputs
                },
                created_at: Utc::now(),
                notes: effective.notes,
            });
        }

        // Save index atomically
        self.index.save(&self.index_path)?;

        Ok(())
    }

    /// Find entries matching filters
    pub fn find_entries(&self, filters: &CacheFilters) -> Vec<&CacheEntry> {
        self.index
            .entries
            .iter()
            .filter(|entry| {
                if let Some(shape) = &filters.shape {
                    let requested_shape = entry.recipe.requested.shape;
                    if format!("{:?}", requested_shape).to_lowercase() != shape.to_lowercase() {
                        return false;
                    }
                }

                if let Some(size) = &filters.size {
                    let expected_size = format!(
                        "{}x{}",
                        entry.recipe.requested.size.width, entry.recipe.requested.size.height
                    );
                    if expected_size != *size {
                        return false;
                    }
                }

                if let Some(base_color) = &filters.base_color {
                    let requested_color = &entry.recipe.requested.base_color;
                    let normalized = normalize_color(requested_color).unwrap_or_default();
                    if normalized != *base_color {
                        return false;
                    }
                }

                if let Some(fill) = &filters.fill {
                    let fill_str = match &entry.recipe.requested.fill {
                        Fill::Solid => "solid",
                        Fill::Pie(_) => "pie",
                        Fill::Split(_) => "split",
                        Fill::Stripe(_) => "stripe",
                    };
                    if fill_str != *fill {
                        return false;
                    }
                }

                if filters.exists && !entry.outputs.values().any(|o| o.exists) {
                    return false;
                }

                if filters.missing && entry.outputs.values().all(|o| o.exists) {
                    return false;
                }

                true
            })
            .collect()
    }
}

/// Filters for cache querying
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CacheFilters {
    pub shape: Option<String>,
    pub size: Option<String>,
    pub base_color: Option<String>,
    pub fill: Option<String>,
    pub exists: bool,
    pub missing: bool,
}

impl CacheIndex {
    /// Load cache index from file
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            serde_yaml::from_str(&content)
                .map_err(|e| Error::Validation(format!("Failed to parse cache index: {}", e)))
        } else {
            Ok(CacheIndex {
                entries: Vec::new(),
            })
        }
    }

    /// Save cache index to file with atomic operations
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| Error::Validation(format!("Failed to serialize cache index: {}", e)))?;

        // Write to temp file first
        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, content)?;

        // Atomic rename
        fs::rename(&temp_path, path)?;

        Ok(())
    }
}

/// Compute SHA256 hash of a file
fn compute_file_hash(path: &Path) -> Result<String> {
    let content = fs::read(path)?;
    let hash = Sha256::digest(&content);
    Ok(format!("{:x}", hash))
}

/// Normalize color to hex format for comparison
fn normalize_color(color: &Color) -> Result<String> {
    match color {
        Color::Named(name) => {
            // Convert named colors to hex
            match name.as_str() {
                "red" => Ok("ff0000".to_string()),
                "green" => Ok("00ff00".to_string()),
                "blue" => Ok("0000ff".to_string()),
                "black" => Ok("000000".to_string()),
                "white" => Ok("ffffff".to_string()),
                "yellow" => Ok("ffff00".to_string()),
                "cyan" => Ok("00ffff".to_string()),
                "magenta" => Ok("ff00ff".to_string()),
                "gray" | "grey" => Ok("808080".to_string()),
                _ => Err(Error::Validation(format!("Unknown color name: {}", name))),
            }
        }
        Color::Hex(hex) => {
            let hex = hex.strip_prefix('#').unwrap_or(hex);
            let hex = hex.to_lowercase();
            match hex.len() {
                3 => Ok(format!(
                    "{}{}{}{}{}{}",
                    &hex[0..1],
                    &hex[0..1],
                    &hex[1..2],
                    &hex[1..2],
                    &hex[2..3],
                    &hex[2..3]
                )),
                6 => Ok(hex),
                _ => Err(Error::Validation("Invalid hex length".to_string())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::{Color, Fill, Shape, Size};
    use tempfile::tempdir;

    #[test]
    fn test_cache_creation() {
        let temp_dir = tempdir().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        assert!(cache.index_path.exists());
        assert!(cache.generated_dir.exists());
    }

    #[test]
    fn test_output_path_generation() {
        let temp_dir = tempdir().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        let path = cache.get_output_path("test-stem", "png");
        assert_eq!(path.file_name().unwrap(), "test-stem.png");
    }

    #[test]
    fn test_cache_entry_update() {
        let temp_dir = tempdir().unwrap();
        let mut cache = Cache::new(temp_dir.path()).unwrap();

        // Create a test recipe
        let recipe = Recipe {
            shape: Shape::Circle,
            size: Size {
                width: 256,
                height: 256,
            },
            base_color: Color::Named("red".to_string()),
            accent_color: None,
            fill: Fill::Solid,
            mark: None,
            badge: None,
            label: None,
            glyph: None,
            font_path: None,
        };

        // Create a test file
        let test_file = temp_dir.path().join("test.png");
        fs::write(&test_file, b"test content").unwrap();

        // Update cache entry
        cache
            .update_entry("test-stem", &recipe, "png", &test_file)
            .unwrap();

        // Verify entry was created
        assert_eq!(cache.index.entries.len(), 1);
        let entry = &cache.index.entries[0];
        assert_eq!(entry.stem, "test-stem");
        assert!(entry.outputs.contains_key("png"));
    }
}

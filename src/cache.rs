//! Cache system for MT-logo-render.
//!
//! Provides atomic file operations and YAML-based index management
//! for deterministic asset caching.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// Output format enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Png,
    Ansi,
    Ansi256,
    Md,
    Html,
}

/// Cache entry metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Deterministic stem identifier
    pub stem: String,
    /// Recipe specification pair (requested vs effective)
    pub recipe: RecipePair,
    /// Output file information by format
    pub outputs: HashMap<OutputFormat, OutputInfo>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Degradation notes
    pub notes: Vec<String>,
}

/// Recipe specification pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipePair {
    /// Original requested recipe
    pub requested: serde_json::Value,
    /// Effective recipe after canonicalization/degradation
    pub effective: serde_json::Value,
}

/// Output file information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputInfo {
    /// Relative path from cache root
    pub path: PathBuf,
    /// Whether file exists on disk
    pub exists: bool,
    /// SHA256 fingerprint (if file exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// File size in bytes (if file exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

/// Cache index containing all entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheIndex {
    /// All cache entries keyed by stem
    pub entries: HashMap<String, CacheEntry>,
}

impl Default for CacheIndex {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

/// Main cache manager with atomic operations.
pub struct Cache {
    /// Root directory for cache (e.g., "assets/logo")
    root: PathBuf,
    /// Path to index file
    index_path: PathBuf,
    /// Path to generated assets directory
    generated_path: PathBuf,
    /// In-memory index
    index: CacheIndex,
}

impl Cache {
    /// Create new cache manager for the given root directory.
    pub fn new(root: PathBuf) -> Result<Self> {
        let index_path = root.join("index.yaml");
        let generated_path = root.join("generated");

        // Create directories if they don't exist
        fs::create_dir_all(&root)?;
        fs::create_dir_all(&generated_path)?;

        // Load existing index or create new one
        let index = if index_path.exists() {
            Self::load_index(&index_path)?
        } else {
            CacheIndex::default()
        };

        Ok(Self {
            root,
            index_path,
            generated_path,
            index,
        })
    }

    /// Get cache root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Get generated assets directory.
    pub fn generated_path(&self) -> &Path {
        &self.generated_path
    }

    /// Check if cache entry exists for the given stem.
    pub fn exists(&self, stem: &str) -> bool {
        self.index.entries.contains_key(stem)
    }

    /// Get cache entry for the given stem.
    pub fn get(&self, stem: &str) -> Option<&CacheEntry> {
        self.index.entries.get(stem)
    }

    /// Check if all outputs exist for the given stem.
    pub fn all_outputs_exist(&self, stem: &str, formats: &[OutputFormat]) -> bool {
        if let Some(entry) = self.get(stem) {
            formats.iter().all(|format| {
                entry.outputs.get(format)
                    .map(|info| info.exists)
                    .unwrap_or(false)
            })
        } else {
            false
        }
    }

    /// Atomically write file content to cache.
    pub fn write_file_atomic(&self, stem: &str, filename: &str, content: &[u8]) -> Result<PathBuf> {
        let target_path = self.generated_path.join(filename);

        // Create temporary file in same directory for atomic rename
        let temp_path = self.generated_path.join(format!(".{}.tmp", filename));

        // Write to temporary file first
        fs::write(&temp_path, content)?;

        // Atomic rename to final location
        fs::rename(&temp_path, &target_path)?;

        Ok(target_path)
    }

    /// Add or update cache entry.
    pub fn add_entry(&mut self, entry: CacheEntry) -> Result<()> {
        self.index.entries.insert(entry.stem.clone(), entry);
        self.save_index()?;
        Ok(())
    }

    /// Remove cache entry and associated files.
    pub fn remove_entry(&mut self, stem: &str) -> Result<()> {
        if let Some(entry) = self.index.entries.remove(stem) {
            // Remove associated files
            for output_info in entry.outputs.values() {
                let full_path = self.root.join(&output_info.path);
                if full_path.exists() {
                    fs::remove_file(&full_path)?;
                }
            }
        }
        self.save_index()?;
        Ok(())
    }

    /// Clean cache entries that no longer have files.
    pub fn clean_orphaned_entries(&mut self) -> Result<usize> {
        let mut removed = 0;
        let stems_to_remove: Vec<String> = self.index.entries.iter()
            .filter(|(_, entry)| {
                // Check if any output files still exist
                !entry.outputs.values().any(|info| {
                    let full_path = self.root.join(&info.path);
                    full_path.exists()
                })
            })
            .map(|(stem, _)| stem.clone())
            .collect();

        for stem in stems_to_remove {
            self.index.entries.remove(&stem);
            removed += 1;
        }

        if removed > 0 {
            self.save_index()?;
        }

        Ok(removed)
    }

    /// Get all cache entries (for listing).
    pub fn entries(&self) -> &HashMap<String, CacheEntry> {
        &self.index.entries
    }

    /// Query entries by filter criteria.
    pub fn query(&self, filters: &CacheQuery) -> Vec<&CacheEntry> {
        self.index.entries.values()
            .filter(|entry| filters.matches(entry))
            .collect()
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        let total_entries = self.index.entries.len();
        let total_files = self.index.entries.values()
            .map(|entry| entry.outputs.len())
            .sum();

        let existing_files = self.index.entries.values()
            .flat_map(|entry| &entry.outputs)
            .filter(|(_, info)| info.exists)
            .count();

        let total_size_bytes = self.index.entries.values()
            .flat_map(|entry| &entry.outputs)
            .filter_map(|(_, info)| info.size_bytes)
            .sum();

        CacheStats {
            total_entries,
            total_files,
            existing_files,
            orphaned_files: total_files.saturating_sub(existing_files),
            total_size_bytes,
        }
    }

    /// Load index from YAML file.
    fn load_index(path: &Path) -> Result<CacheIndex> {
        let content = fs::read_to_string(path)?;
        let index: CacheIndex = serde_yaml::from_str(&content)
            .map_err(|e| Error::Validation(format!("Invalid cache index: {}", e)))?;
        Ok(index)
    }

    /// Save index to YAML file atomically.
    fn save_index(&self) -> Result<()> {
        let content = serde_yaml::to_string(&self.index)
            .map_err(|e| Error::Validation(format!("Failed to serialize cache index: {}", e)))?;

        let temp_path = self.index_path.with_extension("yaml.tmp");
        fs::write(&temp_path, &content)?;
        fs::rename(&temp_path, &self.index_path)?;

        Ok(())
    }
}

/// Cache query filters.
#[derive(Debug, Default)]
pub struct CacheQuery {
    pub shape: Option<String>,
    pub size: Option<String>,
    pub base_color: Option<String>,
    pub fill: Option<String>,
    pub mark: Option<String>,
    pub badge: Option<String>,
    pub label: Option<String>,
    pub exists: Option<bool>,
}

impl CacheQuery {
    pub fn matches(&self, entry: &CacheEntry) -> bool {
        // Parse effective recipe for filtering
        if let Ok(recipe) = serde_json::from_value::<serde_json::Value>(entry.recipe.effective.clone()) {
            if let Some(obj) = recipe.as_object() {
                if let Some(shape) = &self.shape {
                    if obj.get("shape").and_then(|s| s.as_str()) != Some(shape) {
                        return false;
                    }
                }
                if let Some(size) = &self.size {
                    if obj.get("size").and_then(|s| s.as_str()) != Some(size) {
                        return false;
                    }
                }
                if let Some(base_color) = &self.base_color {
                    if obj.get("base_color").and_then(|s| s.as_str()) != Some(base_color) {
                        return false;
                    }
                }
                if let Some(fill) = &self.fill {
                    if obj.get("fill").and_then(|s| s.as_str()) != Some(fill) {
                        return false;
                    }
                }
                if let Some(mark) = &self.mark {
                    if obj.get("mark").and_then(|s| s.as_str()) != Some(mark) {
                        return false;
                    }
                }
                if let Some(badge) = &self.badge {
                    if obj.get("badge").and_then(|s| s.as_str()) != Some(badge) {
                        return false;
                    }
                }
                if let Some(label) = &self.label {
                    if obj.get("label").and_then(|s| s.as_str()) != Some(label) {
                        return false;
                    }
                }
            }
        }

        // Check existence filter
        if let Some(exists) = self.exists {
            let has_existing_files = entry.outputs.values().any(|info| info.exists);
            if has_existing_files != exists {
                return false;
            }
        }

        true
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_files: usize,
    pub existing_files: usize,
    pub orphaned_files: usize,
    pub total_size_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_creation() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path().to_path_buf()).unwrap();

        assert_eq!(cache.root(), temp_dir.path());
        assert!(cache.generated_path().exists());
        assert!(!cache.index_path.exists()); // New cache, no index yet
    }

    #[test]
    fn test_atomic_file_write() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path().to_path_buf()).unwrap();

        let content = b"test content";
        let path = cache.write_file_atomic("test", "test.txt", content).unwrap();

        assert!(path.exists());
        assert_eq!(fs::read(&path).unwrap(), content);
    }

    #[test]
    fn test_cache_entry_management() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = Cache::new(temp_dir.path().to_path_buf()).unwrap();

        let entry = CacheEntry {
            stem: "test-stem".to_string(),
            recipe: RecipePair {
                requested: serde_json::json!({"shape": "circle"}),
                effective: serde_json::json!({"shape": "circle", "size": "256x256"}),
            },
            outputs: HashMap::new(),
            created_at: Utc::now(),
            notes: vec![],
        };

        cache.add_entry(entry.clone()).unwrap();
        assert!(cache.exists("test-stem"));
        assert_eq!(cache.get("test-stem").unwrap().stem, "test-stem");

        cache.remove_entry("test-stem").unwrap();
        assert!(!cache.exists("test-stem"));
    }

    #[test]
    fn test_cache_query() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = Cache::new(temp_dir.path().to_path_buf()).unwrap();

        let entry = CacheEntry {
            stem: "circle-red".to_string(),
            recipe: RecipePair {
                requested: serde_json::json!({"shape": "circle", "base_color": "red"}),
                effective: serde_json::json!({"shape": "circle", "size": "256x256", "base_color": "red"}),
            },
            outputs: HashMap::new(),
            created_at: Utc::now(),
            notes: vec![],
        };

        cache.add_entry(entry).unwrap();

        let query = CacheQuery {
            shape: Some("circle".to_string()),
            ..Default::default()
        };

        let results = cache.query(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].stem, "circle-red");
    }
}

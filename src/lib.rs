//! # MT-logo-render
//!
//! A Human Execution Engine (HEE) for deterministic logo asset generation.
//! This crate provides secure, deterministic rendering of logo assets from recipes.

pub mod security;
pub mod recipe;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Security validation failed: {0}")]
    Security(#[from] security::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Font loading error: {0}")]
    Font(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, Error>;

// Re-export security types
pub use security::{SecurityValidator, ValidationResult};

//! # Origin Design Adapter Core
//!
//! Common infrastructure for all design platform adapters.
//! Provides IR, primitive mapping, constraint generation, and binary format.

#![warn(clippy::all)]

use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Intermediate Representation (IR) for design files
pub mod ir;

/// Primitive mapping from design elements to Origin primitives
pub mod primitive_mapper;

/// Constraint generation from layout information
pub mod constraint_generator;

/// Texture atlas builder
pub mod atlas_builder;

/// Binary format writer (.origin files)
pub mod binary_writer;

/// Common trait for design platform adapters
pub trait DesignAdapter {
    /// Parse a design file into IR
    fn parse(&self, source: AdapterSource) -> Result<ir::OriginIR, AdapterError>;
    
    /// Check if this adapter supports hot reload
    fn supports_hot_reload(&self) -> bool {
        false
    }
}

/// Source for design file
#[derive(Debug, Clone)]
pub enum AdapterSource {
    /// Local file path
    FilePath(std::path::PathBuf),
    /// Remote URL
    Url(String),
    /// Raw data
    Data(Vec<u8>),
}

/// Adapter error types
#[derive(Debug)]
pub enum AdapterError {
    /// Parse error
    ParseError(String),
    /// IO error
    IoError(std::io::Error),
    /// Network error
    NetworkError(String),
    /// Unsupported feature
    UnsupportedFeature(String),
}

impl From<std::io::Error> for AdapterError {
    fn from(err: std::io::Error) -> Self {
        AdapterError::IoError(err)
    }
}

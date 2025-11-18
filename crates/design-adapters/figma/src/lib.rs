//! # OriginFigma
//!
//! Figma design file adapter for Origin.
//! Transforms Figma files into .origin binaries with pixel-perfect accuracy.

#![warn(clippy::all)]

use origin_adapter_core::{AdapterError, AdapterSource, DesignAdapter};

/// Figma API client
pub mod api;

/// Figma file parser
pub mod parser;

/// Figma to Origin converter
pub mod converter;

/// OriginFigma adapter implementation
pub struct FigmaAdapter {
    /// API token (if using Figma API)
    api_token: Option<String>,
}

impl FigmaAdapter {
    /// Create new Figma adapter
    pub fn new() -> Self {
        Self { api_token: None }
    }
    
    /// Create new Figma adapter with API token
    pub fn with_token(token: String) -> Self {
        Self {
            api_token: Some(token),
        }
    }
}

impl Default for FigmaAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl DesignAdapter for FigmaAdapter {
    fn parse(&self, source: AdapterSource) -> Result<origin_adapter_core::ir::OriginIR, AdapterError> {
        // TODO: Implement Figma parsing
        Err(AdapterError::UnsupportedFeature(
            "Figma parsing not yet implemented".to_string(),
        ))
    }
    
    fn supports_hot_reload(&self) -> bool {
        true // Figma supports hot reload via plugin
    }
}

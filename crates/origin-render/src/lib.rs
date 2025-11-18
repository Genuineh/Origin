//! # Origin Render Core
//!
//! Pure wgpu rendering with single mega-shader.
//! Instance-only rendering, no per-primitive shaders.

#![warn(clippy::all)]

/// GPU context and management
pub mod gpu;

/// Instance buffer management
pub mod instance;

/// Uniform buffer system
pub mod uniform;

/// Texture atlas management
pub mod texture;

/// Render pipeline setup
pub mod pipeline;

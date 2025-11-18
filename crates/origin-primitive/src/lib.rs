//! # Origin Primitive Layer
//!
//! The 6 core visual primitives, all SDF-based:
//! 1. RoundedRect
//! 2. Circle
//! 3. Line
//! 4. Arc
//! 5. Path
//! 6. Text

#![warn(clippy::all)]

/// Rounded rectangle primitive
pub mod rounded_rect;

/// Circle primitive
pub mod circle;

/// Line primitive
pub mod line;

/// Arc primitive
pub mod arc;

/// Path primitive with boolean operations
pub mod path;

/// Text primitive with MSDF rendering
pub mod text;

/// Common types for all primitives
pub mod types;

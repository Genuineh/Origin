//! # Origin Layout Engine
//!
//! Mathematical constraint-based layout, no trees.
//! Runtime constraint solving with Gauss-Seidel method.

#![warn(clippy::all)]

/// Constraint types and expressions
pub mod constraint;

/// Constraint solver (Gauss-Seidel)
pub mod solver;

/// Column layout primitive
pub mod column;

/// Row layout primitive
pub mod row;

/// Grid layout primitive
pub mod grid;

/// Relative coordinate stack
pub mod coordinate;

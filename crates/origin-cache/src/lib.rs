//! # Origin Cache & Optimizer
//!
//! Minimizes GPU traffic through intelligent caching.
//! StateHash → Instance buffer sub-updates + draw call merging.

#![warn(clippy::all)]

/// State hash for change detection
pub mod state_hash;

/// Differential buffer updates
pub mod diff;

/// Draw call batching
pub mod batch;

/// Dirty rectangle tracking
pub mod dirty_rect;

//! # Origin Platform Abstraction Layer
//!
//! Provides unified entry points for all platforms with minimal platform-specific code.
//! Target: < 600 lines total, zero platform-specific rendering code.

#![warn(clippy::all)]

/// Platform-specific implementations
#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android"), not(target_os = "ios")))]
pub mod desktop;

/// Common platform trait
pub trait Platform {
    /// Initialize the platform
    fn init() -> Self;
    
    /// Run the main event loop
    fn run(self);
}

//! # Origin
//!
//! Origin is a revolutionary UI runtime that transforms design files into 120fps,
//! pixel-perfect, interactive native apps across all platforms.
//!
//! ## The 15 Iron Rules
//!
//! 1. No trees: Widget/Element/Node/Tree terminology is forbidden
//! 2. One-frame regeneration: All UI must regenerate in 1 frame
//! 3. Simple state: Only f32/u32/bool, packed into Uniforms
//! 4. No native controls: Zero platform-native UI controls
//! 5. Pixel perfect: ≤ 1px error across platforms
//! 6. Fast loading: < 50ms for design files
//! 7. Instant web: < 100ms cold start
//! 8. Memory efficient: < 60MB peak
//! 9. Pure shader effects: All effects via SDF + shaders
//! 10. Mathematical layout: Equations only, no trees
//! 11. u64 tags: High 32 bits type, low 32 bits instance ID
//! 12. No external UI libs: egui, iced, etc. forbidden
//! 13. Single wgpu instance: One instance across all platforms
//! 14. Small binary: < 12MB (release + strip)
//! 15. The mission: Origin — Where design returns to its origin.

#![deny(missing_docs)]
#![warn(clippy::all)]

pub use origin_cache as cache;
pub use origin_input as input;
pub use origin_layout as layout;
pub use origin_platform as platform;
pub use origin_primitive as primitive;
pub use origin_render as render;
pub use origin_tag as tag;
pub use origin_ui as ui;

/// Re-exports of commonly used types and traits
pub mod prelude {
    //! Common imports for Origin applications
    
    pub use crate::cache::*;
    pub use crate::input::*;
    pub use crate::layout::*;
    pub use crate::primitive::*;
    pub use crate::render::*;
    pub use crate::tag::*;
    pub use crate::ui::*;
}

#[cfg(feature = "figma")]
pub use origin_figma as figma;

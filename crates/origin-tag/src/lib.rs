//! # Origin Tag System
//!
//! u64 tag system for element identification.
//! High 32 bits = type, low 32 bits = instance ID.

#![warn(clippy::all)]

/// A tag identifying an instance
pub type Tag = u64;

/// Create a tag from type and instance ID
pub fn make_tag(type_id: u32, instance_id: u32) -> Tag {
    ((type_id as u64) << 32) | (instance_id as u64)
}

/// Extract type ID from tag
pub fn extract_type(tag: Tag) -> u32 {
    (tag >> 32) as u32
}

/// Extract instance ID from tag
pub fn extract_id(tag: Tag) -> u32 {
    tag as u32
}

/// Pick buffer for GPU-based picking
pub mod pick_buffer;

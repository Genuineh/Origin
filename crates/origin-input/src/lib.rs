//! # Origin Input System
//!
//! Cross-platform unified input handling.
//! Mouse, keyboard, touch, and scroll events.

#![warn(clippy::all)]

use glam::Vec2;
use std::collections::HashSet;

/// Mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
}

/// Touch phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    /// Touch started
    Started,
    /// Touch moved
    Moved,
    /// Touch ended
    Ended,
    /// Touch cancelled
    Cancelled,
}

/// Input event
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Mouse moved
    MouseMove { x: f32, y: f32 },
    /// Mouse button pressed
    MouseDown { button: MouseButton, x: f32, y: f32 },
    /// Mouse button released
    MouseUp { button: MouseButton, x: f32, y: f32 },
    /// Mouse wheel scrolled
    Scroll { delta_x: f32, delta_y: f32 },
    /// Touch event
    Touch { id: u64, phase: TouchPhase, x: f32, y: f32 },
}

/// Input state tracker
#[derive(Debug, Default)]
pub struct InputState {
    /// Current mouse position
    pub mouse_pos: Vec2,
    /// Pressed mouse buttons
    pub pressed_buttons: HashSet<MouseButton>,
}

impl InputState {
    /// Create new input state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update state with an event
    pub fn update(&mut self, event: &InputEvent) {
        match event {
            InputEvent::MouseMove { x, y } => {
                self.mouse_pos = Vec2::new(*x, *y);
            }
            InputEvent::MouseDown { button, .. } => {
                self.pressed_buttons.insert(*button);
            }
            InputEvent::MouseUp { button, .. } => {
                self.pressed_buttons.remove(button);
            }
            _ => {}
        }
    }
}

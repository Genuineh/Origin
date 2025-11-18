//! Intermediate Representation (IR) for design files

use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Origin IR - platform-agnostic representation of design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginIR {
    /// Version of IR format
    pub version: u32,
    /// All artboards/pages in the design
    pub artboards: Vec<Artboard>,
    /// Reusable components
    pub components: HashMap<String, Component>,
    /// Asset references (images, fonts, etc.)
    pub assets: Assets,
}

/// An artboard/page in the design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artboard {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Size of the artboard
    pub size: Vec2,
    /// Background color
    pub background: Color,
    /// All visual nodes in this artboard
    pub nodes: Vec<Node>,
}

/// A visual node (any design element)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier
    pub id: String,
    /// Transform (position, rotation, scale)
    pub transform: Transform,
    /// Visual properties
    pub visual: VisualProperties,
    /// Layout properties
    pub layout: LayoutProperties,
    /// Interactive properties
    pub interactions: Vec<Interaction>,
}

/// Transformation matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    /// Position
    pub position: Vec2,
    /// Rotation in radians
    pub rotation: f32,
    /// Scale
    pub scale: Vec2,
}

/// Visual properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualProperties {
    /// Fill color or gradient
    pub fill: Option<Fill>,
    /// Stroke properties
    pub stroke: Option<Stroke>,
    /// Corner radii (for rectangles)
    pub corner_radii: Option<[f32; 4]>,
    /// Effects (shadow, blur, etc.)
    pub effects: Vec<Effect>,
    /// Opacity
    pub opacity: f32,
}

/// Fill types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fill {
    /// Solid color
    Solid(Color),
    /// Linear gradient
    LinearGradient(LinearGradient),
    /// Radial gradient
    RadialGradient(RadialGradient),
}

/// Stroke properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    /// Color
    pub color: Color,
    /// Width
    pub width: f32,
    /// Cap style
    pub cap: StrokeCap,
    /// Join style
    pub join: StrokeJoin,
}

/// Stroke cap styles
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StrokeCap {
    /// Butt cap
    Butt,
    /// Round cap
    Round,
    /// Square cap
    Square,
}

/// Stroke join styles
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StrokeJoin {
    /// Miter join
    Miter,
    /// Round join
    Round,
    /// Bevel join
    Bevel,
}

/// Color (RGBA)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    /// Red (0-1)
    pub r: f32,
    /// Green (0-1)
    pub g: f32,
    /// Blue (0-1)
    pub b: f32,
    /// Alpha (0-1)
    pub a: f32,
}

/// Linear gradient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearGradient {
    /// Start point
    pub start: Vec2,
    /// End point
    pub end: Vec2,
    /// Color stops
    pub stops: Vec<ColorStop>,
}

/// Radial gradient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadialGradient {
    /// Center point
    pub center: Vec2,
    /// Radius
    pub radius: f32,
    /// Color stops
    pub stops: Vec<ColorStop>,
}

/// Color stop in gradient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorStop {
    /// Position (0-1)
    pub position: f32,
    /// Color at this stop
    pub color: Color,
}

/// Visual effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    /// Drop shadow
    DropShadow {
        /// Offset
        offset: Vec2,
        /// Blur radius
        blur: f32,
        /// Color
        color: Color,
    },
    /// Inner shadow
    InnerShadow {
        /// Offset
        offset: Vec2,
        /// Blur radius
        blur: f32,
        /// Color
        color: Color,
    },
    /// Blur
    Blur {
        /// Blur radius
        radius: f32,
    },
}

/// Layout properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutProperties {
    /// Width constraint
    pub width: SizeConstraint,
    /// Height constraint
    pub height: SizeConstraint,
    /// Padding
    pub padding: Padding,
    /// Layout mode (if container)
    pub layout_mode: Option<LayoutMode>,
}

/// Size constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SizeConstraint {
    /// Fixed size
    Fixed(f32),
    /// Fill available space
    Fill,
    /// Hug contents
    Hug,
}

/// Padding
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Padding {
    /// Top
    pub top: f32,
    /// Right
    pub right: f32,
    /// Bottom
    pub bottom: f32,
    /// Left
    pub left: f32,
}

/// Layout mode (for containers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutMode {
    /// Vertical stack
    Column {
        /// Spacing between items
        spacing: f32,
        /// Alignment
        align: Alignment,
    },
    /// Horizontal stack
    Row {
        /// Spacing between items
        spacing: f32,
        /// Alignment
        align: Alignment,
    },
    /// Grid layout
    Grid {
        /// Columns
        columns: usize,
        /// Spacing
        spacing: f32,
    },
}

/// Alignment
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Alignment {
    /// Start
    Start,
    /// Center
    Center,
    /// End
    End,
    /// Stretch
    Stretch,
}

/// Interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// Trigger
    pub trigger: InteractionTrigger,
    /// Action
    pub action: InteractionAction,
}

/// Interaction trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionTrigger {
    /// On click
    Click,
    /// On hover
    Hover,
    /// On key press
    KeyPress(String),
}

/// Interaction action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionAction {
    /// Navigate to artboard
    NavigateTo(String),
    /// Toggle state
    ToggleState(String),
}

/// Component definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Component ID
    pub id: String,
    /// Component name
    pub name: String,
    /// Component nodes
    pub nodes: Vec<Node>,
    /// Variant properties
    pub variants: HashMap<String, Vec<String>>,
}

/// Assets (fonts, images, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assets {
    /// Font files
    pub fonts: Vec<FontAsset>,
    /// Image files
    pub images: Vec<ImageAsset>,
}

/// Font asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontAsset {
    /// Font ID
    pub id: String,
    /// Font family name
    pub family: String,
    /// Font data (embedded)
    pub data: Vec<u8>,
}

/// Image asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAsset {
    /// Image ID
    pub id: String,
    /// Image format
    pub format: ImageFormat,
    /// Image data (embedded)
    pub data: Vec<u8>,
}

/// Image format
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageFormat {
    /// PNG
    Png,
    /// JPEG
    Jpeg,
    /// WebP
    Webp,
}

//! Color palette for the UI.

use bevy::color::{Color, Srgba};
use bevy::prelude::Val;

// Button colors
pub const BUTTON_BACKGROUND: Color = Color::Srgba(Srgba::new(0.15, 0.15, 0.15, 0.95));
pub const BUTTON_HOVERED_BACKGROUND: Color = Color::Srgba(Srgba::new(0.25, 0.25, 0.25, 0.95));
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::Srgba(Srgba::new(0.35, 0.35, 0.35, 0.95));
pub const BUTTON_TEXT: Color = Color::Srgba(Srgba::new(0.9, 0.9, 0.9, 1.0));

// Panel colors
pub const PANEL_BACKGROUND: Color = Color::Srgba(Srgba::new(0.1, 0.1, 0.1, 0.8));
pub const PANEL_BORDER: Color = Color::Srgba(Srgba::new(0.3, 0.3, 0.3, 0.9));

// Text colors
pub const TEXT_PRIMARY: Color = Color::Srgba(Srgba::new(0.9, 0.9, 0.9, 1.0));
pub const TEXT_SECONDARY: Color = Color::Srgba(Srgba::new(0.7, 0.7, 0.7, 1.0));
pub const TEXT_HEADER: Color = Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 1.0));

// Helper function for percentage-based values
pub const fn percent(value: f32) -> Val {
    Val::Percent(value)
}

// Helper function for pixel-based values
pub const fn px(value: f32) -> Val {
    Val::Px(value)
}

use bevy::prelude::*;

// Panel styling constants (shared with left panel pattern)
pub const PANEL_PADDING: Val = Val::Px(15.0);
pub const PANEL_MARGIN: Val = Val::Px(5.0);
pub const PANEL_BORDER_WIDTH: Val = Val::Px(2.0);
pub const PANEL_BORDER_RADIUS: Val = Val::Px(8.0);

// Color constants
pub const PANEL_BACKGROUND: Color = Color::srgba(0.176, 0.176, 0.176, 1.0); // #2d2d2d
pub const PANEL_BORDER: Color = Color::srgba(0.267, 0.267, 0.267, 1.0); // #444444
pub const TITLE_COLOR: Color = Color::srgba(1.0, 0.8, 0.0, 1.0); // #ffcc00
pub const TEXT_PRIMARY: Color = Color::WHITE;
pub const TEXT_SECONDARY: Color = Color::srgba(0.8, 0.8, 0.8, 1.0); // #cccccc
pub const TEXT_TERTIARY: Color = Color::srgba(0.533, 0.533, 0.533, 1.0); // #888888

// Text sizes
pub const TITLE_FONT_SIZE: f32 = 20.0;
pub const SUBTITLE_FONT_SIZE: f32 = 16.0;
pub const BODY_FONT_SIZE: f32 = 14.0;

// Spacing
pub const TITLE_MARGIN_BOTTOM: Val = Val::Px(15.0);
pub const SECTION_MARGIN_BOTTOM: Val = Val::Px(10.0);
pub const TEXT_MARGIN_BOTTOM: Val = Val::Px(5.0);

// Section heights
pub const STATISTICS_PANEL_MIN_HEIGHT: Val = Val::Px(150.0);
pub const HOVERED_TILE_PANEL_MIN_HEIGHT: Val = Val::Px(120.0);
pub const CIVILIZATIONS_PANEL_MIN_HEIGHT: Val = Val::Px(200.0);
pub const MINIMAP_PANEL_MIN_HEIGHT: Val = Val::Px(150.0);

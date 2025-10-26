use bevy::prelude::*;

// Panel styling constants
pub const PANEL_PADDING: Val = Val::Px(15.0);
pub const PANEL_MARGIN: Val = Val::Px(5.0);
pub const PANEL_BORDER_WIDTH: Val = Val::Px(2.0);
pub const PANEL_BORDER_RADIUS: Val = Val::Px(8.0);

// Color constants
pub const PANEL_BACKGROUND: Color = Color::srgba(0.165, 0.165, 0.165, 1.0); // #2a2a2a
pub const STATS_CONTAINER_BACKGROUND: Color = Color::srgba(0.176, 0.176, 0.176, 1.0); // #2d2d2d
pub const PANEL_BORDER: Color = Color::srgba(0.267, 0.267, 0.267, 1.0); // #444444
pub const TITLE_COLOR: Color = Color::WHITE;
pub const TEXT_PRIMARY: Color = Color::srgba(1.0, 0.8, 0.0, 1.0); // #ffcc00

// Text sizes
pub const TITLE_FONT_SIZE: f32 = 24.0;
pub const STATS_FONT_SIZE: f32 = 18.0;

// Stats container
pub const STATS_CONTAINER_MIN_WIDTH: Val = Val::Px(500.0);

use bevy::prelude::*;

// Panel styling constants
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
pub const TEXT_TERTIARY: Color = Color::srgba(0.6, 0.6, 0.6, 1.0); // #999999

// Button styling constants
pub const BUTTON_HEIGHT: Val = Val::Px(40.0);
pub const BUTTON_PADDING: Val = Val::Px(10.0);
pub const BUTTON_MARGIN: Val = Val::Px(5.0);
pub const BUTTON_BORDER_WIDTH: Val = Val::Px(2.0);
pub const BUTTON_BORDER_RADIUS: Val = Val::Px(5.0);

// Button colors
pub const BUTTON_BACKGROUND: Color = Color::srgba(0.176, 0.176, 0.176, 1.0); // #2d2d2d
pub const BUTTON_BORDER: Color = Color::srgba(0.4, 0.4, 0.4, 1.0); // #666666
pub const BUTTON_HOVER_BACKGROUND: Color = Color::srgba(0.251, 0.251, 0.251, 1.0); // #404040
pub const BUTTON_HOVER_BORDER: Color = Color::srgba(1.0, 0.8, 0.0, 1.0); // #ffcc00
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgba(0.0, 0.667, 0.667, 1.0); // #0aa

// Text sizes
pub const TITLE_FONT_SIZE: f32 = 20.0;
pub const SUBTITLE_FONT_SIZE: f32 = 16.0;
pub const BODY_FONT_SIZE: f32 = 14.0;
pub const SMALL_FONT_SIZE: f32 = 12.0;

// Spacing
pub const TITLE_MARGIN_BOTTOM: Val = Val::Px(15.0);
pub const SECTION_MARGIN_BOTTOM: Val = Val::Px(10.0);
pub const TEXT_MARGIN_BOTTOM: Val = Val::Px(5.0);
pub const SEPARATOR_HEIGHT: Val = Val::Px(2.0);
pub const SEPARATOR_MARGIN: Val = Val::Px(10.0);

// Production menu specific
pub const PRODUCTION_MENU_MAX_HEIGHT: Val = Val::Px(500.0);
pub const NEXT_TURN_BUTTON_HEIGHT: Val = Val::Px(50.0);
pub const GAME_PANEL_MIN_HEIGHT: Val = Val::Px(200.0);

// Button text constants
pub const BUTTON_TEXT_END_TURN: &str = "End Turn";
pub const BUTTON_TEXT_START_YOUR_TURN: &str = "Start Your Turn";
pub const BUTTON_TEXT_NEXT_TURN: &str = "Next Turn";
pub const BUTTON_TEXT_PROCESSING: &str = "Processing...";

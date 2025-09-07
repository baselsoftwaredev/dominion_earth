use super::constants::display_layout;
use bevy::prelude::*;

pub struct UiPanelBounds {
    pub window_width: f32,
    pub window_height: f32,
    pub header_height: f32,
    pub left_sidebar_width: f32,
    pub right_sidebar_width: f32,
}

impl UiPanelBounds {
    pub fn from_window(window: &Window) -> Self {
        Self {
            window_width: window.width(),
            window_height: window.height(),
            header_height: display_layout::HEADER_HEIGHT,
            left_sidebar_width: display_layout::LEFT_SIDEBAR_WIDTH,
            right_sidebar_width: display_layout::RIGHT_SIDEBAR_WIDTH,
        }
    }
}

/// Check if cursor position is over any UI panel (header, left sidebar, or right sidebar)
pub fn is_cursor_over_ui_panel(cursor_screen_position: Vec2, ui_bounds: &UiPanelBounds) -> bool {
    // Convert cursor position to match Bevy coordinate system (origin at bottom-left)
    let cursor_y_from_top = ui_bounds.window_height - cursor_screen_position.y;

    // Check if cursor is over header (top of screen)
    if cursor_y_from_top <= ui_bounds.header_height {
        return true;
    }

    // Check if cursor is over left sidebar
    if cursor_screen_position.x <= ui_bounds.left_sidebar_width {
        return true;
    }

    // Check if cursor is over right sidebar
    if cursor_screen_position.x >= (ui_bounds.window_width - ui_bounds.right_sidebar_width) {
        return true;
    }

    false
}

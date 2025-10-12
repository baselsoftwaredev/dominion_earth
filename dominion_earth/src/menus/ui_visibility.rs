//! Utilities for managing UI panel visibility during menu transitions.

use bevy::prelude::*;
use bevy_hui::prelude::*;

use crate::debug_utils::DebugLogging;

/// Hide all gameplay UI panels (those with HtmlNode component).
/// Called when entering a menu during gameplay to provide clean menu presentation.
pub fn hide_gameplay_ui_panels(
    mut ui_panels: Query<&mut Visibility, With<HtmlNode>>,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(debug_logging, "🙈 Hiding gameplay UI panels");

    for mut panel_visibility in &mut ui_panels {
        set_panel_hidden(&mut panel_visibility);
    }
}

/// Show all gameplay UI panels (those with HtmlNode component).
/// Called when exiting a menu to restore gameplay UI visibility.
pub fn show_gameplay_ui_panels(
    mut ui_panels: Query<&mut Visibility, With<HtmlNode>>,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(debug_logging, "👁️  Showing gameplay UI panels");

    for mut panel_visibility in &mut ui_panels {
        set_panel_visible(&mut panel_visibility);
    }
}

fn set_panel_hidden(visibility: &mut Visibility) {
    *visibility = Visibility::Hidden;
}

fn set_panel_visible(visibility: &mut Visibility) {
    *visibility = Visibility::Visible;
}

//! Utilities for managing UI panel visibility during menu transitions.

use bevy::prelude::*;
use bevy_hui::prelude::HtmlNode;

use crate::debug_utils::DebugLogging;

/// Hide all gameplay UI panels (those with HtmlNode component).
/// Called when entering a menu during gameplay to provide clean menu presentation.
pub fn hide_gameplay_ui_panels(
    mut html_ui_panels: Query<
        &mut Visibility,
        (
            With<HtmlNode>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
        ),
    >,
    mut top_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::top_panel::TopPanel>,
            Without<HtmlNode>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
        ),
    >,
    mut right_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::right_panel::RightPanel>,
            Without<HtmlNode>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
        ),
    >,
    mut left_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::left_panel::LeftPanel>,
            Without<HtmlNode>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
        ),
    >,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(debug_logging, "🙈 Hiding gameplay UI panels");

    // Hide HTML-based panels
    for mut panel_visibility in &mut html_ui_panels {
        set_panel_hidden(&mut panel_visibility);
    }

    // Hide native top panel
    for mut panel_visibility in &mut top_panel {
        set_panel_hidden(&mut panel_visibility);
    }

    // Hide native right panel
    for mut panel_visibility in &mut right_panel {
        set_panel_hidden(&mut panel_visibility);
    }

    // Hide native left panel
    for mut panel_visibility in &mut left_panel {
        set_panel_hidden(&mut panel_visibility);
    }
}

/// Show all gameplay UI panels (those with HtmlNode component).
/// Called when exiting a menu to restore gameplay UI visibility.
pub fn show_gameplay_ui_panels(
    mut html_ui_panels: Query<
        &mut Visibility,
        (
            With<HtmlNode>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
        ),
    >,
    mut top_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::top_panel::TopPanel>,
            Without<HtmlNode>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
        ),
    >,
    mut right_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::right_panel::RightPanel>,
            Without<HtmlNode>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
        ),
    >,
    mut left_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::left_panel::LeftPanel>,
            Without<HtmlNode>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
        ),
    >,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(debug_logging, "👁️  Showing gameplay UI panels");

    // Show HTML-based panels
    for mut panel_visibility in &mut html_ui_panels {
        set_panel_visible(&mut panel_visibility);
    }

    // Show native top panel
    for mut panel_visibility in &mut top_panel {
        set_panel_visible(&mut panel_visibility);
    }

    // Show native right panel
    for mut panel_visibility in &mut right_panel {
        set_panel_visible(&mut panel_visibility);
    }

    // Show native left panel
    for mut panel_visibility in &mut left_panel {
        set_panel_visible(&mut panel_visibility);
    }
}

fn set_panel_hidden(visibility: &mut Visibility) {
    *visibility = Visibility::Hidden;
}

fn set_panel_visible(visibility: &mut Visibility) {
    *visibility = Visibility::Visible;
}

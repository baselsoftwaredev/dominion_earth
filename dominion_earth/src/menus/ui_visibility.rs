//! Utilities for managing UI panel visibility during menu transitions.

use bevy::prelude::*;

use crate::debug_utils::DebugLogging;

/// Hide all gameplay UI panels.
/// Called when entering a menu during gameplay to provide clean menu presentation.
pub fn hide_gameplay_ui_panels(
    mut top_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::capital_labels::CapitalLabel>,
            Without<crate::ui::unit_labels::UnitLabel>,
        ),
    >,
    mut right_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::capital_labels::CapitalLabel>,
            Without<crate::ui::unit_labels::UnitLabel>,
        ),
    >,
    mut left_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::capital_labels::CapitalLabel>,
            Without<crate::ui::unit_labels::UnitLabel>,
        ),
    >,
    mut capital_labels: Query<
        &mut Visibility,
        (
            With<crate::ui::capital_labels::CapitalLabel>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::unit_labels::UnitLabel>,
        ),
    >,
    mut unit_labels: Query<
        &mut Visibility,
        (
            With<crate::ui::unit_labels::UnitLabel>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::capital_labels::CapitalLabel>,
        ),
    >,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(debug_logging, "🙈 Hiding gameplay UI panels");

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

    // Hide capital labels
    for mut label_visibility in &mut capital_labels {
        set_panel_hidden(&mut label_visibility);
    }

    // Hide unit labels
    for mut label_visibility in &mut unit_labels {
        set_panel_hidden(&mut label_visibility);
    }
}

/// Show all gameplay UI panels.
/// Called when exiting a menu to restore gameplay UI visibility.
pub fn show_gameplay_ui_panels(
    mut top_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::capital_labels::CapitalLabel>,
            Without<crate::ui::unit_labels::UnitLabel>,
        ),
    >,
    mut right_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::capital_labels::CapitalLabel>,
            Without<crate::ui::unit_labels::UnitLabel>,
        ),
    >,
    mut left_panel: Query<
        &mut Visibility,
        (
            With<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::capital_labels::CapitalLabel>,
            Without<crate::ui::unit_labels::UnitLabel>,
        ),
    >,
    mut capital_labels: Query<
        &mut Visibility,
        (
            With<crate::ui::capital_labels::CapitalLabel>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::unit_labels::UnitLabel>,
        ),
    >,
    mut unit_labels: Query<
        &mut Visibility,
        (
            With<crate::ui::unit_labels::UnitLabel>,
            Without<crate::ui::top_panel::TopPanel>,
            Without<crate::ui::right_panel::RightPanel>,
            Without<crate::ui::left_panel::LeftPanel>,
            Without<crate::ui::capital_labels::CapitalLabel>,
        ),
    >,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(debug_logging, "👁️  Showing gameplay UI panels");

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

    // Show capital labels
    for mut label_visibility in &mut capital_labels {
        set_panel_visible(&mut label_visibility);
    }

    // Show unit labels
    for mut label_visibility in &mut unit_labels {
        set_panel_visible(&mut label_visibility);
    }
}

fn set_panel_hidden(visibility: &mut Visibility) {
    *visibility = Visibility::Hidden;
}

fn set_panel_visible(visibility: &mut Visibility) {
    *visibility = Visibility::Visible;
}

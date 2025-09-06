// Production Menu implementation using bevy_hui
use crate::production_input::SelectedCapital;
use bevy::prelude::*;
use bevy_hui::prelude::*;
use core_sim::{Civilization, ProductionQueue};

/// Production menu functionality is now handled through bevy_hui templates
/// The UI template is at assets/ui/production_menu.html
///
/// Key features:
/// - Shows/hides based on selected_capital.show_production_menu
/// - Displays capital name, civilization info, gold, production
/// - Shows current production progress
/// - Shows production queue length
/// - Provides instructions for queueing units
///
/// The production menu state is managed through the SelectedCapital resource
/// and updated in the main UI update system in bevy_hui_impl.rs

pub fn update_production_menu_visibility(
    selected_capital: Res<SelectedCapital>,
    mut ui_nodes: Query<&mut TemplateProperties, With<HtmlNode>>,
) {
    // This function can be used for additional production menu logic if needed
    // The main visibility and data updates are handled in bevy_hui_impl.rs

    if selected_capital.is_changed() {
        for mut properties in ui_nodes.iter_mut() {
            let show_menu = if selected_capital.show_production_menu {
                "flex"
            } else {
                "none"
            };
            properties.insert("show_production_menu".to_string(), show_menu.to_string());
        }
    }
}

use bevy::prelude::*;
use bevy::ui::Overflow;
use bevy_hui::prelude::*;

use super::constants::ui_component_names;

/// Marker component to indicate a panel has had scroll components added
#[derive(Component)]
pub struct ScrollEnabled;

/// System to add Overflow and ScrollPosition components to UI panels that need scrolling
/// This runs continuously to catch panels after they're fully compiled
pub fn setup_scrollable_panels(
    mut commands: Commands,
    // Query for panel entities with Node component (compiled panels)
    panel_query: Query<(Entity, Option<&Name>, &Node), (With<HtmlNode>, Without<ScrollEnabled>)>,
) {
    for (entity, opt_name, _node) in panel_query.iter() {
        let name_str = opt_name.map(|n| n.as_str()).unwrap_or("unnamed");

        // Check if this is a panel or component that should be scrollable
        let should_scroll = match name_str {
            ui_component_names::LEFT_SIDE_PANEL_NAME
            | ui_component_names::RIGHT_SIDE_PANEL_NAME
            | ui_component_names::PRODUCTION_MENU_NAME => true,
            _ => false,
        };

        if should_scroll {
            info!("Enabling scroll for panel: {}", name_str);

            // Add ScrollPosition component - the Node already exists
            commands
                .entity(entity)
                .insert(ScrollPosition::default())
                .insert(ScrollEnabled);

            info!("Added ScrollPosition to panel: {}", name_str);
        }
    }
}

/// System to modify the Node overflow property for panels after they're compiled
/// This must run after the Node component exists
pub fn apply_scroll_overflow(
    mut node_query: Query<(&mut Node, &Name), (With<HtmlNode>, With<ScrollEnabled>, Changed<Node>)>,
) {
    for (mut node, name) in node_query.iter_mut() {
        if node.overflow != Overflow::scroll_y() {
            node.overflow = Overflow::scroll_y();
            info!("Applied scroll overflow to panel: {}", name);
        }
    }
}

pub mod capital_labels;
pub mod constants;
pub mod production_orders;
pub mod property_updates;
pub mod scroll_setup;

use crate::entity_utils;
use crate::game::GameState;
use crate::ui::traits::*;
use bevy::prelude::*;
use core_sim::components::MilitaryUnit;

pub use capital_labels::{spawn_capital_labels, update_capital_labels, CapitalLabel};
pub use property_updates::{should_update_ui_this_frame, update_ui_properties_system};

/// Bevy HUI implementation of the UI system
pub struct BevyUiSystem;

impl BevyUiSystem {
    pub fn setup_plugins(app: &mut App) {
        app.add_systems(
                Startup,
                (
                    crate::ui::top_panel::spawn_top_panel,
                    crate::ui::right_panel::spawn_right_panel,
                    crate::ui::left_panel::spawn_left_panel,
                ),
            )
            .add_systems(
                Update,
                (
                    update_ui_properties_system.run_if(should_update_ui_this_frame),
                    spawn_capital_labels,
                    update_capital_labels,
                    // Top panel updates
                    crate::ui::top_panel::update_player_resources,
                    crate::ui::top_panel::update_turn_display,
                    // Right panel updates
                    crate::ui::right_panel::update_statistics_panel,
                    crate::ui::right_panel::update_hovered_tile_info,
                    crate::ui::right_panel::update_civilizations_list,
                    // Left panel button interactions
                    crate::ui::left_panel::handle_next_turn_button,
                    crate::ui::left_panel::handle_infantry_button,
                    crate::ui::left_panel::handle_archer_button,
                    crate::ui::left_panel::handle_cavalry_button,
                    crate::ui::left_panel::update_production_button_visuals,
                    // Left panel updates
                    crate::ui::left_panel::update_production_menu,
                    crate::ui::left_panel::update_unit_info,
                ),
            );
    }

    pub fn setup_plugins_for_screen<S: States>(app: &mut App, screen: S) {
        app.add_systems(
                OnEnter(screen.clone()),
                (
                    crate::ui::top_panel::spawn_top_panel,
                    crate::ui::right_panel::spawn_right_panel,
                    crate::ui::left_panel::spawn_left_panel,
                ),
            )
            .add_systems(OnExit(screen.clone()), cleanup_ui)
            .add_systems(
                Update,
                (
                    update_ui_properties_system.run_if(should_update_ui_this_frame),
                    spawn_capital_labels,
                    update_capital_labels,
                    // Top panel updates
                    crate::ui::top_panel::update_player_resources,
                    crate::ui::top_panel::update_turn_display,
                    // Right panel updates
                    crate::ui::right_panel::update_statistics_panel,
                    crate::ui::right_panel::update_hovered_tile_info,
                    crate::ui::right_panel::update_civilizations_list,
                    // Left panel button interactions
                    crate::ui::left_panel::handle_next_turn_button,
                    crate::ui::left_panel::handle_infantry_button,
                    crate::ui::left_panel::handle_archer_button,
                    crate::ui::left_panel::handle_cavalry_button,
                    crate::ui::left_panel::update_production_button_visuals,
                    // Left panel updates
                    crate::ui::left_panel::update_production_menu,
                    crate::ui::left_panel::update_unit_info,
                )
                    .run_if(in_state(screen)),
            );
    }
}

fn cleanup_ui(
    mut commands: Commands,
    top_panel: Query<Entity, With<crate::ui::top_panel::TopPanel>>,
    right_panel: Query<Entity, With<crate::ui::right_panel::RightPanel>>,
    left_panel: Query<Entity, With<crate::ui::left_panel::LeftPanel>>,
    children_query: Query<&Children>,
) {
    let mut despawned = std::collections::HashSet::new();

    // Despawn native top panel
    for entity in &top_panel {
        entity_utils::recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
            &mut despawned,
        );
    }

    // Despawn native right panel
    for entity in &right_panel {
        entity_utils::recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
            &mut despawned,
        );
    }

    // Despawn native left panel
    for entity in &left_panel {
        entity_utils::recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
            &mut despawned,
        );
    }
}

impl UiSystem for BevyUiSystem {
    fn initialize(&self, app: &mut App) {
        Self::setup_plugins(app);
    }

    fn render_main_game_panel(&self, _data: &GamePanelData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_production_menu(&self, _data: &ProductionMenuData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_statistics_panel(&self, _data: &StatisticsPanelData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_tile_info(&self, _data: &TileInfoData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_minimap(&self, _data: &MinimapData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_resources(&self, _data: &ResourcesData) {
        // For bevy_hui, rendering is handled by the component system
    }
}

/// HuiComponent struct for UI system integration
#[derive(Component)]
pub struct HuiComponent {
    id: String,
}

impl UiComponent for HuiComponent {
    type Id = String;

    fn get_id(&self) -> Self::Id {
        self.id.clone()
    }

    fn update_data(&mut self, _data: Box<dyn std::any::Any + Send + Sync>) {
        // HUI components update themselves via reactive systems
    }
}

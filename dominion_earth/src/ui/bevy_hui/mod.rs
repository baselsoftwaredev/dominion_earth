pub mod capital_labels;
pub mod constants;
pub mod main_ui;
pub mod production_orders;
pub mod property_updates;

use crate::game::GameState;
use crate::ui::traits::*;
use bevy::prelude::*;
use bevy_hui::prelude::*;
use core_sim::components::MilitaryUnit;

pub use capital_labels::{spawn_capital_labels, update_capital_labels, CapitalLabel};
pub use main_ui::setup_main_ui;
pub use property_updates::{should_update_ui_this_frame, update_ui_properties_system};

/// Bevy HUI implementation of the UI system
pub struct BevyHuiSystem;

impl BevyHuiSystem {
    pub fn setup_plugins(app: &mut App) {
        app.add_plugins((HuiPlugin, HuiAutoLoadPlugin::new(&["ui"])))
            .add_systems(Startup, setup_main_ui)
            .add_systems(
                Update,
                (
                    update_ui_properties_system.run_if(should_update_ui_this_frame),
                    spawn_capital_labels,
                    update_capital_labels,
                ),
            );
    }

    pub fn setup_plugins_for_screen<S: States>(app: &mut App, screen: S) {
        app.add_plugins((HuiPlugin, HuiAutoLoadPlugin::new(&["ui"])))
            .add_systems(OnEnter(screen.clone()), setup_main_ui)
            .add_systems(OnExit(screen.clone()), cleanup_ui)
            .add_systems(
                Update,
                (
                    update_ui_properties_system.run_if(should_update_ui_this_frame),
                    spawn_capital_labels,
                    update_capital_labels,
                )
                    .run_if(in_state(screen)),
            );
    }
}

/// Clean up all UI panels when leaving the gameplay screen
fn cleanup_ui(mut commands: Commands, ui_panels: Query<Entity, With<HtmlNode>>) {
    for entity in &ui_panels {
        commands.entity(entity).despawn();
    }
}

impl UiSystem for BevyHuiSystem {
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

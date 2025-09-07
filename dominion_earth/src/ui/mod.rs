pub mod constants;
// pub mod game_panel; // Commented out - needs bevy_hui implementation
// pub mod minimap; // Commented out - needs bevy_hui implementation
// pub mod production_menu; // Commented out - needs bevy_hui implementation
pub mod resources;
// pub mod statistics_panel; // Commented out - needs bevy_hui implementation
// pub mod tile_info; // Commented out - needs bevy_hui implementation
pub mod bevy_hui_impl;
pub mod traits;
pub mod utilities;

// pub use game_panel::*; // Commented out
// pub use minimap::*; // Commented out
// pub use production_menu::*; // Commented out
pub use resources::*;
// pub use statistics_panel::*; // Commented out
// pub use tile_info::*; // Commented out
pub use bevy_hui_impl::*;
pub use traits::*;
pub use utilities::*;

use crate::debug_utils::DebugLogging;
use crate::game::GameState;
use crate::production_input::SelectedCapital;
use bevy::prelude::*;
use core_sim::{
    components::{Capital, MilitaryUnit},
    resources::{CurrentTurn, WorldMap},
    Civilization, Position, ProductionQueue,
};

/// New system that works with the UI abstraction layer
pub fn initialize_ui_system(
    ui_system: Res<UiSystemResource>,
    current_turn: Res<CurrentTurn>,
    game_state: Res<GameState>,
    world_map: Res<WorldMap>,
    terrain_counts: Res<TerrainCounts>,
    civs: Query<&Civilization>,
    player_civs: Query<&Civilization, With<core_sim::PlayerControlled>>,
    selected_tile: Res<SelectedTile>,
    selected_unit: Res<core_sim::SelectedUnit>,
    last_logged_tile: Res<LastLoggedTile>,
    debug_logging: Res<DebugLogging>,
    capitals: Query<(&Capital, &Position)>,
    units: Query<(&MilitaryUnit, &Position)>,
    selected_capital: Res<SelectedCapital>,
    production_query: Query<&ProductionQueue>,
) {
    // Collect data for UI rendering
    let game_panel_data = GamePanelData {
        current_turn: current_turn.0,
        // game_state: (*game_state).clone(), // Removed for now
        world_map: (*world_map).clone(),
        terrain_counts: (*terrain_counts).clone(),
        civilizations: civs.iter().cloned().collect(),
        player_civilizations: player_civs.iter().cloned().collect(),
        selected_tile: selected_tile.position.map(|pos| (pos.x, pos.y)),
        selected_unit: selected_unit.unit_entity,
        last_logged_tile: last_logged_tile.position.map(|pos| (pos.x, pos.y)),
        debug_logging: debug_logging.0,
        capitals: capitals.iter().map(|(c, p)| (c.clone(), *p)).collect(),
        units: units.iter().map(|(u, p)| (u.clone(), *p)).collect(),
        selected_capital: selected_capital.capital_entity,
        production_queues: production_query.iter().cloned().collect(),
    };

    // For bevy_hui, the actual rendering is handled by the update systems
    // This function serves as a data collection point for the abstraction layer
}

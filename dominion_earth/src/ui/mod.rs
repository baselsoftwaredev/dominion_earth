pub mod capital_labels;
pub mod constants;
pub mod left_panel;
pub mod resources;
pub mod right_panel;
pub mod system_setup;
pub mod top_panel;
pub mod traits;
pub mod unit_labels;
pub mod utilities;
pub use capital_labels::*;
pub use left_panel::*;
pub use resources::*;
pub use right_panel::*;
pub use system_setup::*;
pub use top_panel::*;
pub use traits::*;
pub use unit_labels::*;
pub use utilities::*;

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
        capitals: capitals.iter().map(|(c, p)| (c.clone(), *p)).collect(),
        units: units.iter().map(|(u, p)| (u.clone(), *p)).collect(),
        selected_capital: selected_capital.capital_entity,
        production_queues: production_query.iter().cloned().collect(),
    };
}

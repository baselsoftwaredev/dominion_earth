pub mod constants;
pub mod game_panel;
pub mod minimap;
pub mod production_menu;
pub mod resources;
pub mod statistics_panel;
pub mod tile_info;

pub use game_panel::*;
pub use minimap::*;
pub use production_menu::*;
pub use resources::*;
pub use statistics_panel::*;
pub use tile_info::*;

use crate::debug_utils::DebugLogging;
use crate::game::GameState;
use crate::production_input::SelectedCapital;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use core_sim::{
    components::{Capital, MilitaryUnit},
    resources::{CurrentTurn, WorldMap},
    Civilization, Position, ProductionQueue,
};

pub fn initialize_ui_system(
    mut contexts: EguiContexts,
    current_turn: Res<CurrentTurn>,
    mut game_state: ResMut<GameState>,
    world_map: Res<WorldMap>,
    terrain_counts: Res<TerrainCounts>,
    civs: Query<&Civilization>,
    player_civs: Query<&Civilization, With<core_sim::PlayerControlled>>,
    selected_tile: Res<SelectedTile>,
    selected_unit: Res<core_sim::SelectedUnit>,
    mut last_logged_tile: ResMut<LastLoggedTile>,
    debug_logging: Res<DebugLogging>,
    world_tile_query: Query<(
        &core_sim::tile::tile_components::WorldTile,
        &core_sim::tile::tile_components::TileNeighbors,
    )>,
    capitals: Query<(&Capital, &Position)>,
    units: Query<(&MilitaryUnit, &Position)>,
    selected_capital: Res<SelectedCapital>,
    production_query: Query<&ProductionQueue>,
) {
    if let Ok(ctx) = contexts.ctx_mut() {
        render_main_game_panel(
            ctx,
            &current_turn,
            &mut game_state,
            &selected_tile,
            &selected_unit,
            &mut last_logged_tile,
            &debug_logging,
            &world_tile_query,
            &civs,
            &player_civs,
            &world_map,
            &capitals,
            &units,
            &selected_capital,
            &production_query,
        );

        render_world_statistics_panel(ctx, &world_map, &terrain_counts, &civs);

        render_world_minimap_window(ctx, &civs);
    }
}

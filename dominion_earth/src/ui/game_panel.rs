use super::constants::display_layout;
use super::tile_info::render_selected_tile_information_panel;
use crate::debug_utils::DebugLogging;
use crate::game::GameState;
use crate::production_input::SelectedCapital;
use crate::ui::resources::{LastLoggedTile, SelectedTile};
use bevy::prelude::*;
use bevy_egui::egui;
use core_sim::{
    components::{Capital, MilitaryUnit},
    resources::{CurrentTurn, WorldMap},
    Civilization, Position, ProductionQueue,
};

pub fn render_main_game_panel(
    ctx: &egui::Context,
    current_turn: &CurrentTurn,
    game_state: &mut GameState,
    selected_tile: &SelectedTile,
    selected_unit: &core_sim::SelectedUnit,
    last_logged_tile: &mut LastLoggedTile,
    debug_logging: &DebugLogging,
    world_tile_query: &Query<(
        &core_sim::tile::tile_components::WorldTile,
        &core_sim::tile::tile_components::TileNeighbors,
    )>,
    civs: &Query<&Civilization>,
    player_civs: &Query<&Civilization, With<core_sim::PlayerControlled>>,
    world_map: &Res<WorldMap>,
    capitals: &Query<(&Capital, &Position)>,
    units: &Query<(&MilitaryUnit, &Position)>,
    selected_capital: &SelectedCapital,
    production_query: &Query<&ProductionQueue>,
) {
    egui::SidePanel::left("game_panel")
        .resizable(true)
        .default_width(display_layout::GAME_PANEL_DEFAULT_WIDTH)
        .min_width(display_layout::GAME_PANEL_MINIMUM_WIDTH)
        .max_width(display_layout::GAME_PANEL_MAXIMUM_WIDTH)
        .show(ctx, |ui| {
            ui.heading("Dominion Earth");
            ui.separator();

            render_current_turn_information(ui, current_turn, game_state);
            ui.separator();

            render_civilization_summary_information(ui, civs, player_civs);
            ui.separator();

            render_unit_selection_information(ui, selected_unit);
            ui.separator();

            render_selected_tile_information_panel(
                ui,
                selected_tile,
                last_logged_tile,
                debug_logging,
                world_tile_query,
                world_map,
                capitals,
                units,
                selected_capital,
                production_query,
                civs,
            );
        });
}

fn render_current_turn_information(
    ui: &mut egui::Ui,
    current_turn: &CurrentTurn,
    game_state: &mut GameState,
) {
    ui.heading("Game Status");
    ui.label(format!("Turn: {}", current_turn.0));
    ui.label(format!(
        "Game State: {}",
        if game_state.paused {
            "Paused"
        } else {
            "Running"
        }
    ));
    ui.label(format!(
        "Auto-advance: {} (debug: {})",
        if game_state.auto_advance { "On" } else { "Off" },
        game_state.auto_advance
    ));
}

fn render_civilization_summary_information(
    ui: &mut egui::Ui,
    civs: &Query<&Civilization>,
    player_civs: &Query<&Civilization, With<core_sim::PlayerControlled>>,
) {
    ui.heading("Civilizations");
    let total_civilization_count = civs.iter().count();
    let player_civilization_count = player_civs.iter().count();

    ui.label(format!("Total: {}", total_civilization_count));
    ui.label(format!("Player: {}", player_civilization_count));

    for civilization in player_civs.iter() {
        ui.label(format!("• {} (Player)", civilization.name));
        ui.label(format!("  Gold: {:.0}", civilization.economy.gold));
        ui.label(format!(
            "  Production: {:.1}",
            civilization.economy.production
        ));
    }
}

fn render_unit_selection_information(ui: &mut egui::Ui, selected_unit: &core_sim::SelectedUnit) {
    ui.heading("Selected Unit");
    if let Some(unit_id) = selected_unit.unit_id {
        ui.label(format!("Unit ID: {}", unit_id));
    } else {
        ui.label("No unit selected");
    }
}

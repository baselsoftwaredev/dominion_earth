// TODO: Replace with bevy_hui implementation
/*
use super::production_menu::render_capital_production_interface;
use crate::debug_utils::{DebugLogging, DebugUtils};
use crate::production_input::SelectedCapital;
use crate::ui::resources::{LastLoggedTile, SelectedTile};
use bevy::prelude::*;
use bevy_egui::egui;
use core_sim::{
    components::{Capital, MilitaryUnit},
    resources::WorldMap,
    Civilization, Position, ProductionQueue,
};

pub fn render_selected_tile_information_panel(
    ui: &mut egui::Ui,
    selected_tile: &SelectedTile,
    last_logged_tile: &mut LastLoggedTile,
    debug_logging: &DebugLogging,
    world_tile_query: &Query<(
        &core_sim::tile::tile_components::WorldTile,
        &core_sim::tile::tile_components::TileNeighbors,
    )>,
    world_map: &Res<WorldMap>,
    capitals: &Query<(&Capital, &Position)>,
    units: &Query<(&MilitaryUnit, &Position)>,
    selected_capital: &SelectedCapital,
    production_query: &Query<&ProductionQueue>,
    civs: &Query<&Civilization>,
) {
    ui.heading("Selected Tile Info");

    if let Some(position) = selected_tile.position {
        ui.label(format!("Position: ({}, {})", position.x, position.y));

        let should_log_tile_information = last_logged_tile.position != Some(position);
        if should_log_tile_information {
            last_logged_tile.position = Some(position);
        }

        let mut found_ecs_tile_data = false;
        let mut ecs_terrain_type = None;
        let mut neighbor_terrain_information = Vec::new();

        for (world_tile, tile_neighbors) in world_tile_query.iter() {
            if world_tile.grid_pos == position {
                ecs_terrain_type = Some(world_tile.terrain_type.clone());
                ui.label(format!("ECS Terrain: {:?}", world_tile.terrain_type));

                neighbor_terrain_information =
                    collect_neighbor_terrain_information(tile_neighbors, world_tile_query);

                found_ecs_tile_data = true;
                break;
            }
        }

        if !found_ecs_tile_data {
            ui.label("No ECS tile data found.");
        }

        ui.separator();
        render_tile_structure_information(
            ui,
            &position,
            should_log_tile_information,
            debug_logging,
            capitals,
            units,
        );

        if found_ecs_tile_data {
            if selected_capital.show_production_menu {
                render_capital_production_interface(
                    ui,
                    selected_capital,
                    production_query,
                    civs,
                );
            } else {
                render_tile_neighbor_information(ui, &neighbor_terrain_information);
            }
        }

        render_world_map_terrain_comparison(
            ui,
            &position,
            world_map,
            should_log_tile_information,
            debug_logging,
            ecs_terrain_type.as_ref(),
            &neighbor_terrain_information,
        );

        ui.separator();
    } else {
        ui.label("No tile selected.");
        last_logged_tile.position = None;
    }
}

fn collect_neighbor_terrain_information(
    tile_neighbors: &core_sim::tile::tile_components::TileNeighbors,
    world_tile_query: &Query<(
        &core_sim::tile::tile_components::WorldTile,
        &core_sim::tile::tile_components::TileNeighbors,
    )>,
) -> Vec<(String, String)> {
    let mut neighbor_information = Vec::new();

    collect_single_neighbor_terrain_information(
        "North",
        tile_neighbors.north,
        world_tile_query,
        &mut neighbor_information,
    );
    collect_single_neighbor_terrain_information(
        "South",
        tile_neighbors.south,
        world_tile_query,
        &mut neighbor_information,
    );
    collect_single_neighbor_terrain_information(
        "East",
        tile_neighbors.east,
        world_tile_query,
        &mut neighbor_information,
    );
    collect_single_neighbor_terrain_information(
        "West",
        tile_neighbors.west,
        world_tile_query,
        &mut neighbor_information,
    );

    neighbor_information
}

fn collect_single_neighbor_terrain_information(
    direction_name: &str,
    neighbor_entity_option: Option<Entity>,
    world_tile_query: &Query<(
        &core_sim::tile::tile_components::WorldTile,
        &core_sim::tile::tile_components::TileNeighbors,
    )>,
    neighbor_information: &mut Vec<(String, String)>,
) {
    if let Some(neighbor_entity) = neighbor_entity_option {
        if let Ok((world_tile, _)) = world_tile_query.get(neighbor_entity) {
            neighbor_information.push((
                direction_name.to_string(),
                format!("{:?}", world_tile.terrain_type),
            ));
        }
    }
}

fn render_tile_structure_information(
    ui: &mut egui::Ui,
    position: &Position,
    should_log_tile_information: bool,
    debug_logging: &DebugLogging,
    capitals: &Query<(&Capital, &Position)>,
    units: &Query<(&MilitaryUnit, &Position)>,
) {
    ui.label("Structures:");
    let mut found_any_structures = false;

    if should_log_tile_information {
        DebugUtils::log_tile_check(debug_logging, position);

        let capitals_collection: Vec<_> = capitals.iter().collect();
        DebugUtils::log_capitals(debug_logging, &capitals_collection);

        let units_collection: Vec<_> = units.iter().collect();
        DebugUtils::log_units(debug_logging, &units_collection);
    }

    for (capital, capital_position) in capitals.iter() {
        if capital_position.x == position.x && capital_position.y == position.y {
            ui.label(format!("  🏛️ Capital (Civ {})", capital.owner.0));
            found_any_structures = true;
        }
    }

    for (unit, unit_position) in units.iter() {
        if unit_position.x == position.x && unit_position.y == position.y {
            ui.label(format!(
                "  ⚔️ {:?} (Civ {})",
                unit.unit_type, unit.owner.0
            ));
            found_any_structures = true;
        }
    }

    if !found_any_structures {
        ui.label("  (none)");
    }
}

fn render_tile_neighbor_information(
    ui: &mut egui::Ui,
    neighbor_terrain_information: &[(String, String)],
) {
    ui.separator();
    ui.label("Neighbors:");
    for (direction, terrain) in neighbor_terrain_information {
        ui.label(format!("  {}: {:?}", direction, terrain));
    }
}

fn render_world_map_terrain_comparison(
    ui: &mut egui::Ui,
    position: &Position,
    world_map: &Res<WorldMap>,
    should_log_tile_information: bool,
    debug_logging: &DebugLogging,
    ecs_terrain_type: Option<&core_sim::components::terrain::TerrainType>,
    neighbor_terrain_information: &[(String, String)],
) {
    let mut world_map_terrain_type = None;
    if let Some(world_map_tile) = world_map.get_tile(*position) {
        world_map_terrain_type = Some(world_map_tile.terrain.clone());
        ui.label(format!(
            "WorldMap Terrain: {:?}",
            world_map_tile.terrain
        ));
    } else {
        ui.label("No WorldMap tile data found.");
    }

    if ecs_terrain_type.is_some() && should_log_tile_information {
        DebugUtils::log_terrain_comparison(
            debug_logging,
            position,
            ecs_terrain_type,
            world_map_terrain_type.as_ref(),
            neighbor_terrain_information,
        );
    }
}
*/

use super::coordinates::convert_cursor_position_to_tile_coordinates;
use crate::debug_println;
use crate::debug_utils::{DebugLogging, DebugUtils};
use crate::production_input::SelectedCapital;
use crate::ui::SelectedTile;
use bevy::prelude::*;
use core_sim::components::Position;
use core_sim::components::{Capital, PlayerControlled};
use core_sim::tile::tile_components::{TileNeighbors, WorldTile};

fn locate_tile_entity_at_position<'a>(
    target_position: Position,
    tile_query: &'a Query<(Entity, &WorldTile, &TileNeighbors)>,
) -> Option<(Entity, &'a WorldTile, &'a TileNeighbors)> {
    tile_query
        .iter()
        .find(|(_, world_tile, _)| world_tile.grid_pos == target_position)
}

fn display_terrain_neighbor_debug_information(
    world_tile: &WorldTile,
    neighbors: &TileNeighbors,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
    debug_logging: &DebugLogging,
) {
    let position = world_tile.grid_pos;
    DebugUtils::log_neighbors_header(
        debug_logging,
        position.x,
        position.y,
        &format!("{:?}", world_tile.terrain_type),
    );

    display_directional_neighbor("North", neighbors.north, tile_query, debug_logging);
    display_directional_neighbor("South", neighbors.south, tile_query, debug_logging);
    display_directional_neighbor("East", neighbors.east, tile_query, debug_logging);
    display_directional_neighbor("West", neighbors.west, tile_query, debug_logging);

    DebugUtils::log_neighbors_footer(debug_logging);
}

fn display_directional_neighbor(
    direction_name: &str,
    neighbor_entity: Option<Entity>,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
    debug_logging: &DebugLogging,
) {
    if let Some(entity) = neighbor_entity {
        if let Ok((_, tile, _)) = tile_query.get(entity) {
            DebugUtils::log_single_neighbor(
                debug_logging,
                direction_name,
                Some(&format!("{:?}", tile.terrain_type)),
                Some(tile.grid_pos.x),
                Some(tile.grid_pos.y),
            );
        }
    } else {
        DebugUtils::log_single_neighbor(debug_logging, direction_name, None, None, None);
    }
}

fn process_tile_selection(
    position: Position,
    world_map: &core_sim::resources::WorldMap,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
    selected_tile: &mut SelectedTile,
    debug_logging: &DebugLogging,
    capitals_query: &Query<(Entity, &Capital, &Position)>,
    player_civs_query: &Query<Entity, With<PlayerControlled>>,
    selected_capital: &mut SelectedCapital,
) {
    if world_map.get_tile(position).is_some() {
        println!("DEBUG TILE SELECTION: Tile exists in world map.");
        selected_tile.position = Some(position);

        // Check if there's a capital at this position
        let player_civs: Vec<Entity> = player_civs_query.iter().collect();

        let capitals_count = capitals_query.iter().count();
        println!(
            "DEBUG TILE SELECTION: Found {} capitals total",
            capitals_count
        );

        for (capital_entity, capital, capital_position) in capitals_query.iter() {
            if capital_position.x == position.x && capital_position.y == position.y {
                println!(
                    "DEBUG TILE SELECTION: Found capital at clicked position! Capital owner: {}",
                    capital.owner.0
                );

                // Find the player entity that corresponds to this civilization
                for &player_civ_entity in &player_civs {
                    // Note: The capital.owner.0 is the civilization ID, and we need to check if this player entity represents that civilization
                    // For now, let's assume the first player entity (index 0 in our civilization system) corresponds to civ 0
                    if capital.owner.0 == 0
                        && player_civs.iter().position(|&e| e == player_civ_entity) == Some(0)
                    {
                        println!("DEBUG TILE SELECTION: Player capital clicked - opening production menu");
                        selected_capital.capital_entity = Some(capital_entity);
                        selected_capital.civ_entity = Some(player_civ_entity);
                        selected_capital.show_production_menu = true;
                        break;
                    }
                }
                break;
            }
        }

        if let Some((_, world_tile, neighbors)) =
            locate_tile_entity_at_position(position, tile_query)
        {
            if debug_logging.0 {
                display_terrain_neighbor_debug_information(
                    world_tile,
                    neighbors,
                    tile_query,
                    debug_logging,
                );
            }
        }
    } else {
        println!("DEBUG TILE SELECTION: No tile data found at this position.");
        selected_tile.position = None;
    }
}

pub fn select_tile_on_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut selected_tile: ResMut<SelectedTile>,
    world_map: Res<core_sim::resources::WorldMap>,
    tile_query: Query<(Entity, &WorldTile, &TileNeighbors)>,
    debug_logging: Res<DebugLogging>,
    capitals_query: Query<(Entity, &Capital, &Position)>,
    player_civs_query: Query<Entity, With<PlayerControlled>>,
    mut selected_capital: ResMut<SelectedCapital>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    match convert_cursor_position_to_tile_coordinates(cursor_pos, camera, camera_transform) {
        Ok(position) => {
            DebugUtils::log_tile_click(&debug_logging, position.x, position.y);
            process_tile_selection(
                position,
                &world_map,
                &tile_query,
                &mut selected_tile,
                &debug_logging,
                &capitals_query,
                &player_civs_query,
                &mut selected_capital,
            );
        }
        Err(error_msg) => {
            DebugUtils::log_info(&debug_logging, error_msg);
            selected_tile.position = None;
        }
    }
}

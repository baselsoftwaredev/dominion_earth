use super::coordinates::convert_cursor_position_to_tile_coordinates;
use crate::debug_println;
use crate::debug_utils::DebugUtils;
use crate::production_input::SelectedCapital;
use crate::ui::resources::{HoveredTile, SelectedTile};
use crate::ui::utilities::{is_cursor_over_ui_panel, UiPanelBounds};
use bevy::prelude::*;
use core_sim::components::Position;
use core_sim::components::{Capital, PlayerControlled};
use core_sim::tile::tile_components::{TileNeighbors, WorldTile};

pub mod constants {
    pub const FIRST_PLAYER_CIVILIZATION_INDEX: usize = 0;
    pub const EGYPT_CIVILIZATION_ID: u32 = 0;
}

fn locate_tile_entity_at_specified_position<'a>(
    target_position: Position,
    tile_query: &'a Query<(Entity, &WorldTile, &TileNeighbors)>,
) -> Option<(Entity, &'a WorldTile, &'a TileNeighbors)> {
    tile_query
        .iter()
        .find(|(_, world_tile, _)| world_tile.grid_pos == target_position)
}

fn display_terrain_neighbor_debug_information_for_tile(
    world_tile: &WorldTile,
    neighbors: &TileNeighbors,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
) {
    let tile_position = world_tile.grid_pos;
    DebugUtils::log_neighbors_header(
        tile_position.x,
        tile_position.y,
        &format!("{:?}", world_tile.terrain_type),
    );

    display_directional_neighbor_terrain_information("North", neighbors.north, tile_query);
    display_directional_neighbor_terrain_information("South", neighbors.south, tile_query);
    display_directional_neighbor_terrain_information("East", neighbors.east, tile_query);
    display_directional_neighbor_terrain_information("West", neighbors.west, tile_query);

    DebugUtils::log_neighbors_footer();
}

fn display_directional_neighbor_terrain_information(
    direction_name: &str,
    neighbor_entity_option: Option<Entity>,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
) {
    if let Some(neighbor_entity) = neighbor_entity_option {
        if let Ok((_, neighbor_tile, _)) = tile_query.get(neighbor_entity) {
            DebugUtils::log_single_neighbor(
                direction_name,
                Some(&format!("{:?}", neighbor_tile.terrain_type)),
                Some(neighbor_tile.grid_pos.x),
                Some(neighbor_tile.grid_pos.y),
            );
        }
    } else {
        DebugUtils::log_single_neighbor(direction_name, None, None, None);
    }
}

fn check_and_activate_capital_production_menu_if_player_capital_clicked(
    clicked_position: Position,
    capitals_query: &Query<(Entity, &Capital, &Position)>,
    player_civilizations_query: &Query<Entity, With<PlayerControlled>>,
    selected_capital: &mut SelectedCapital,
    selected_unit: &mut core_sim::SelectedUnit,
    commands: &mut Commands,
    units_query: &Query<(Entity, &core_sim::MilitaryUnit, &core_sim::Position)>,
) -> bool {
    let player_civilization_entities: Vec<Entity> = player_civilizations_query.iter().collect();

    let total_capitals_count = capitals_query.iter().count();
    debug_println!(
        "DEBUG TILE SELECTION: Found {} capitals total",
        total_capitals_count
    );

    for (capital_entity, capital, capital_position) in capitals_query.iter() {
        if capital_position.x == clicked_position.x && capital_position.y == clicked_position.y {
            debug_println!(
                "DEBUG TILE SELECTION: Found capital at clicked position! Capital owner: {}",
                capital.owner.0
            );

            if let Some(matching_player_civilization_entity) =
                find_player_civilization_entity_matching_capital_owner(
                    capital.owner.0,
                    &player_civilization_entities,
                )
            {
                debug_println!(
                    "DEBUG TILE SELECTION: Player capital clicked - opening production menu"
                );
                selected_capital.capital_entity = Some(capital_entity);
                selected_capital.civ_entity = Some(matching_player_civilization_entity);
                selected_capital.show_production_menu = true;

                // Only clear unit selection if there's no unit at this position
                let unit_at_position = units_query
                    .iter()
                    .any(|(_, _, pos)| pos.x == clicked_position.x && pos.y == clicked_position.y);

                if !unit_at_position {
                    if let Some(prev_unit_entity) = selected_unit.unit_entity {
                        if let Some(mut entity_commands) =
                            commands.get_entity(prev_unit_entity).ok()
                        {
                            entity_commands.remove::<core_sim::UnitSelected>();
                        }
                    }
                    selected_unit.unit_entity = None;
                    selected_unit.unit_id = None;
                    selected_unit.owner = None;
                }

                return true;
            }
        }
    }

    false
}

fn find_player_civilization_entity_matching_capital_owner(
    capital_owner_id: u32,
    player_civilization_entities: &[Entity],
) -> Option<Entity> {
    if capital_owner_id == constants::EGYPT_CIVILIZATION_ID {
        if let Some(&first_player_entity) =
            player_civilization_entities.get(constants::FIRST_PLAYER_CIVILIZATION_INDEX)
        {
            return Some(first_player_entity);
        }
    }
    None
}

fn process_tile_selection_and_update_ui_state(
    clicked_position: Position,
    world_map: &core_sim::resources::WorldMap,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
    selected_tile: &mut SelectedTile,
    capitals_query: &Query<(Entity, &Capital, &Position)>,
    player_civilizations_query: &Query<Entity, With<PlayerControlled>>,
    selected_capital: &mut SelectedCapital,
    selected_unit: &mut core_sim::SelectedUnit,
    commands: &mut Commands,
    units_query: &Query<(Entity, &core_sim::MilitaryUnit, &core_sim::Position)>,
) {
    if world_map.get_tile(clicked_position).is_some() {
        debug_println!("DEBUG TILE SELECTION: Tile exists in world map.");
        selected_tile.position = Some(clicked_position);

        let capital_was_clicked =
            check_and_activate_capital_production_menu_if_player_capital_clicked(
                clicked_position,
                capitals_query,
                player_civilizations_query,
                selected_capital,
                selected_unit,
                commands,
                units_query,
            );

        let unit_at_position = units_query
            .iter()
            .any(|(_, _, pos)| pos.x == clicked_position.x && pos.y == clicked_position.y);

        if !capital_was_clicked && !unit_at_position {
            debug_println!("DEBUG TILE SELECTION: Empty tile clicked - clearing selections");

            if let Some(prev_unit_entity) = selected_unit.unit_entity {
                if let Some(mut entity_commands) = commands.get_entity(prev_unit_entity).ok() {
                    entity_commands.remove::<core_sim::UnitSelected>();
                }
            }
            selected_unit.unit_entity = None;
            selected_unit.unit_id = None;
            selected_unit.owner = None;

            selected_capital.show_production_menu = false;
            selected_capital.capital_entity = None;
            selected_capital.civ_entity = None;
        }

        if let Some((_, world_tile, tile_neighbors)) =
            locate_tile_entity_at_specified_position(clicked_position, tile_query)
        {
            display_terrain_neighbor_debug_information_for_tile(
                world_tile,
                tile_neighbors,
                tile_query,
            );
        }
    } else {
        debug_println!("DEBUG TILE SELECTION: No tile data found at this position.");
        selected_tile.position = None;
    }
}

pub fn handle_tile_selection_on_mouse_click(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut selected_tile: ResMut<SelectedTile>,
    world_map: Res<core_sim::resources::WorldMap>,
    tile_query: Query<(Entity, &WorldTile, &TileNeighbors)>,
    capitals_query: Query<(Entity, &Capital, &Position)>,
    player_civilizations_query: Query<Entity, With<PlayerControlled>>,
    mut selected_capital: ResMut<SelectedCapital>,
    mut selected_unit: ResMut<core_sim::SelectedUnit>,
    units_query: Query<(Entity, &core_sim::MilitaryUnit, &core_sim::Position)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(primary_window) = windows_query.single() else {
        return;
    };

    let Some(cursor_screen_position) = primary_window.cursor_position() else {
        return;
    };

    let ui_bounds = UiPanelBounds::from_window(primary_window);

    if is_cursor_over_ui_panel(cursor_screen_position, &ui_bounds) {
        debug_println!(
            "DEBUG TILE SELECTION: Cursor over UI panel (x: {}, y: {}), skipping tile selection",
            cursor_screen_position.x,
            ui_bounds.window_height - cursor_screen_position.y
        );
        return;
    }

    let Ok(primary_window) = windows_query.single() else {
        return;
    };

    let Some(cursor_screen_position) = primary_window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };

    match convert_cursor_position_to_tile_coordinates(
        cursor_screen_position,
        camera,
        camera_global_transform,
    ) {
        Ok(tile_world_position) => {
            DebugUtils::log_tile_click(tile_world_position.x, tile_world_position.y);
            process_tile_selection_and_update_ui_state(
                tile_world_position,
                &world_map,
                &tile_query,
                &mut selected_tile,
                &capitals_query,
                &player_civilizations_query,
                &mut selected_capital,
                &mut selected_unit,
                &mut commands,
                &units_query,
            );
        }
        Err(coordinate_conversion_error_message) => {
            DebugUtils::log_info(coordinate_conversion_error_message);
            selected_tile.position = None;
        }
    }
}

pub fn handle_tile_hover_on_mouse_move(
    windows_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut hovered_tile: ResMut<HoveredTile>,
    tile_query: Query<(Entity, &WorldTile, &TileNeighbors)>,
) {
    let Ok(primary_window) = windows_query.single() else {
        return;
    };

    let Some(cursor_screen_position) = primary_window.cursor_position() else {
        hovered_tile.position = None;
        hovered_tile.terrain_type = None;
        return;
    };

    let ui_bounds = UiPanelBounds::from_window(primary_window);
    if is_cursor_over_ui_panel(cursor_screen_position, &ui_bounds) {
        hovered_tile.position = None;
        hovered_tile.terrain_type = None;
        return;
    }

    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };

    match convert_cursor_position_to_tile_coordinates(
        cursor_screen_position,
        camera,
        camera_global_transform,
    ) {
        Ok(tile_position) => {
            if let Some((_, world_tile, _)) =
                locate_tile_entity_at_specified_position(tile_position, &tile_query)
            {
                hovered_tile.position = Some(tile_position);
                hovered_tile.terrain_type = Some(world_tile.terrain_type.clone());
            } else {
                hovered_tile.position = None;
                hovered_tile.terrain_type = None;
            }
        }
        Err(_) => {
            hovered_tile.position = None;
            hovered_tile.terrain_type = None;
        }
    }
}

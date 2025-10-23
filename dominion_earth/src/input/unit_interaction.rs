use super::constants;
use super::coordinates::convert_cursor_position_to_tile_coordinates;
use crate::debug_utils::{DebugLogging, DebugUtils};
use crate::game::GameState;
use crate::production_input::SelectedCapital;
use crate::ui::utilities::{is_cursor_over_ui_panel, UiPanelBounds};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use core_sim::CivId;

pub fn handle_player_unit_interaction(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_unit: ResMut<core_sim::SelectedUnit>,
    mut selected_capital: ResMut<SelectedCapital>,
    mut units_query: Query<(Entity, &mut core_sim::MilitaryUnit, &core_sim::Position)>,
    pending_movements_query: Query<Entity, With<core_sim::PlayerMovementOrder>>,
    player_civs: Query<&core_sim::Civilization, With<core_sim::PlayerControlled>>,
    world_map: Res<core_sim::resources::WorldMap>,
    debug_logging: Res<DebugLogging>,
    game_state: Res<GameState>,
    capitals_query: Query<(Entity, &core_sim::Capital, &core_sim::Position)>,
    player_civilizations_query: Query<Entity, With<core_sim::PlayerControlled>>,
) {
    if game_state.ai_only {
        return;
    }

    if player_civs.is_empty() {
        return;
    }

    let player_civ_id = if let Some(player_civ) = player_civs.iter().next() {
        player_civ.id
    } else {
        return;
    };

    if mouse_button.just_pressed(MouseButton::Right) {
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                let ui_bounds = UiPanelBounds::from_window(window);

                if is_cursor_over_ui_panel(cursor_pos, &ui_bounds) {
                    return;
                }
            }
        }

        handle_unit_movement_command(
            &mut commands,
            &windows,
            &camera_query,
            &mut units_query,
            &pending_movements_query,
            &selected_unit,
            &world_map,
            player_civ_id,
            &debug_logging,
        );
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                let ui_bounds = UiPanelBounds::from_window(window);

                if is_cursor_over_ui_panel(cursor_pos, &ui_bounds) {
                    return;
                }
            }
        }

        handle_unit_selection(
            &mut commands,
            &windows,
            &camera_query,
            &units_query,
            &mut selected_unit,
            &mut selected_capital,
            player_civ_id,
            &debug_logging,
            &capitals_query,
            &player_civilizations_query,
        );
    }

    handle_unit_keyboard_actions(
        &keyboard_input,
        &mut units_query,
        &selected_unit,
        player_civ_id,
        &debug_logging,
    );
}

fn handle_unit_movement_command(
    commands: &mut Commands,
    windows: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera>>,
    units_query: &mut Query<(Entity, &mut core_sim::MilitaryUnit, &core_sim::Position)>,
    pending_movements_query: &Query<Entity, With<core_sim::PlayerMovementOrder>>,
    selected_unit: &ResMut<core_sim::SelectedUnit>,
    world_map: &Res<core_sim::resources::WorldMap>,
    player_civ_id: CivId,
    debug_logging: &Res<DebugLogging>,
) {
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
        Ok(target_position) => {
            if let Some(selected_entity) = selected_unit.unit_entity {
                if let Ok((entity, unit, current_pos)) = units_query.get_mut(selected_entity) {
                    if unit.owner == player_civ_id && unit.can_move() {
                        if pending_movements_query.get(entity).is_ok() {
                            DebugUtils::log_info(
                                debug_logging,
                                &format!(
                                    "Unit {} already has a pending movement order - ignoring new order",
                                    unit.id
                                ),
                            );
                            return;
                        }

                        match validate_movement_target_and_get_cost(
                            &target_position,
                            current_pos,
                            world_map,
                        ) {
                            Ok(movement_cost) => {
                                if unit.movement_remaining >= movement_cost {
                                    commands
                                        .entity(entity)
                                        .insert(core_sim::PlayerMovementOrder { target_position });
                                    DebugUtils::log_info(
                                        debug_logging,
                                        &format!(
                                            "Ordered unit {} to move to ({}, {}) - Cost: {} - Available: {}",
                                            unit.id, target_position.x, target_position.y, movement_cost, unit.movement_remaining
                                        ),
                                    );
                                } else {
                                    DebugUtils::log_info(
                                        debug_logging,
                                        &format!(
                                            "Unit {} cannot move - insufficient movement points. Required: {}, Available: {}",
                                            unit.id, movement_cost, unit.movement_remaining
                                        ),
                                    );
                                }
                            }
                            Err(reason) => {
                                DebugUtils::log_info(
                                    debug_logging,
                                    &format!("Invalid movement target: {}", reason),
                                );
                            }
                        }
                    } else {
                        DebugUtils::log_info(
                            debug_logging,
                            "Selected unit cannot move this turn or is not player-controlled",
                        );
                    }
                }
            }
        }
        Err(error_msg) => {
            DebugUtils::log_info(debug_logging, error_msg);
        }
    }
}

fn handle_unit_selection(
    commands: &mut Commands,
    windows: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera>>,
    units_query: &Query<(Entity, &mut core_sim::MilitaryUnit, &core_sim::Position)>,
    selected_unit: &mut ResMut<core_sim::SelectedUnit>,
    selected_capital: &mut ResMut<SelectedCapital>,
    player_civ_id: CivId,
    debug_logging: &Res<DebugLogging>,
    capitals_query: &Query<(Entity, &core_sim::Capital, &core_sim::Position)>,
    player_civilizations_query: &Query<Entity, With<core_sim::PlayerControlled>>,
) {
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
        Ok(click_position) => {
            let player_civilization_entities: Vec<Entity> =
                player_civilizations_query.iter().collect();
            let capital_at_position = capitals_query.iter().any(|(_, capital, capital_pos)| {
                capital_pos.x == click_position.x
                    && capital_pos.y == click_position.y
                    && player_civilization_entities
                        .iter()
                        .enumerate()
                        .any(|(idx, _)| capital.owner.0 == 0)
            });

            if capital_at_position {
                DebugUtils::log_info(
                    debug_logging,
                    "Unit interaction: Capital at position, skipping unit selection",
                );
                return;
            }

            let mut found_unit = false;
            for (entity, unit, position) in units_query.iter() {
                if *position == click_position && unit.owner == player_civ_id {
                    if let Some(prev_entity) = selected_unit.unit_entity {
                        commands
                            .entity(prev_entity)
                            .remove::<core_sim::UnitSelected>();
                    }

                    selected_unit.unit_entity = Some(entity);
                    selected_unit.unit_id = Some(unit.id);
                    selected_unit.owner = Some(unit.owner);
                    commands.entity(entity).insert(core_sim::UnitSelected);
                    found_unit = true;

                    selected_capital.show_production_menu = false;
                    selected_capital.capital_entity = None;
                    selected_capital.civ_entity = None;

                    DebugUtils::log_info(
                        debug_logging,
                        &format!(
                            "Selected unit {} at ({}, {})",
                            unit.id, position.x, position.y
                        ),
                    );
                    break;
                }
            }

            if !found_unit {
                if let Some(prev_entity) = selected_unit.unit_entity {
                    commands
                        .entity(prev_entity)
                        .remove::<core_sim::UnitSelected>();
                }
                selected_unit.unit_entity = None;
                selected_unit.unit_id = None;
                selected_unit.owner = None;

                selected_capital.show_production_menu = false;
                selected_capital.capital_entity = None;
                selected_capital.civ_entity = None;
            }
        }
        Err(error_msg) => {
            DebugUtils::log_info(debug_logging, error_msg);
        }
    }
}

fn handle_unit_keyboard_actions(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    units_query: &mut Query<(Entity, &mut core_sim::MilitaryUnit, &core_sim::Position)>,
    selected_unit: &ResMut<core_sim::SelectedUnit>,
    player_civ_id: CivId,
    debug_logging: &Res<DebugLogging>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(selected_entity) = selected_unit.unit_entity {
            if let Ok((_, mut unit, _)) = units_query.get_mut(selected_entity) {
                if unit.owner == player_civ_id {
                    unit.movement_remaining = constants::unit_actions::SKIP_TURN_MOVEMENT_REMAINING;
                    DebugUtils::log_info(debug_logging, &format!("Unit {} skipped turn", unit.id));
                }
            }
        }
    }
}

fn validate_movement_target_and_get_cost(
    target_position: &core_sim::Position,
    current_position: &core_sim::Position,
    world_map: &core_sim::resources::WorldMap,
) -> Result<u32, &'static str> {
    let horizontal_distance = (target_position.x - current_position.x).abs();
    let vertical_distance = (target_position.y - current_position.y).abs();
    let total_distance = horizontal_distance + vertical_distance;

    if total_distance != constants::movement::MINIMUM_ADJACENT_DISTANCE {
        return Err("Must be exactly 1 tile away (adjacent)");
    }

    if let Some(tile) = world_map.get_tile(*target_position) {
        match tile.terrain {
            core_sim::TerrainType::Ocean => Err("Cannot move into ocean"),
            _ => {
                let movement_cost = tile.movement_cost as u32;
                if movement_cost == constants::movement::NO_MOVEMENT_REMAINING {
                    Ok(constants::movement::DEFAULT_MOVEMENT_COST)
                } else {
                    Ok(movement_cost)
                }
            }
        }
    } else {
        Err("Target position is outside map boundaries")
    }
}

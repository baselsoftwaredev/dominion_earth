use crate::{
    components::position::MovementOrder,
    constants::{movement_validation, terrain_stats},
    debug_utils::CoreDebugUtils,
    MilitaryUnit, PlayerMovementOrder, Position, TerrainType, WorldMap,
};
use bevy::prelude::*;

fn calculate_manhattan_distance_between_positions(from: Position, to: Position) -> u32 {
    let x_distance = (to.x - from.x).abs() as u32;
    let y_distance = (to.y - from.y).abs() as u32;
    x_distance + y_distance
}

fn validate_movement_to_adjacent_tile(
    from: Position,
    to: Position,
    world_map: &WorldMap,
) -> Result<u32, &'static str> {
    let distance = calculate_manhattan_distance_between_positions(from, to);
    if distance != movement_validation::ADJACENT_TILE_DISTANCE {
        return Err("Can only move to adjacent tiles");
    }

    if let Some(tile) = world_map.get_tile(to) {
        match tile.terrain {
            TerrainType::Ocean => Err("Cannot move into ocean"),
            _ => {
                let movement_cost = tile.movement_cost as u32;
                if movement_cost == 0 {
                    Ok(movement_validation::DEFAULT_MOVEMENT_COST_WHEN_ZERO)
                } else {
                    Ok(movement_cost)
                }
            }
        }
    } else {
        Err("Target position is outside map boundaries")
    }
}

pub fn execute_movement_orders(
    mut commands: Commands,
    mut movement_query: Query<(
        Entity,
        &mut MilitaryUnit,
        &mut Position,
        &PlayerMovementOrder,
    )>,
    world_map: Res<WorldMap>,
) {
    for (entity, mut unit, mut position, movement_order) in movement_query.iter_mut() {
        let current_position = *position;
        let target_position = movement_order.target_position;

        commands.entity(entity).remove::<PlayerMovementOrder>();

        match validate_movement_to_adjacent_tile(current_position, target_position, &world_map) {
            Ok(movement_cost) => {
                if unit.movement_remaining >= movement_cost {
                    *position = target_position;
                    unit.movement_remaining -= movement_cost;

                    CoreDebugUtils::log_unit_movement_success(
                        unit.id,
                        current_position.x,
                        current_position.y,
                        target_position.x,
                        target_position.y,
                        movement_cost,
                        unit.movement_remaining,
                    );
                } else {
                    CoreDebugUtils::log_insufficient_movement_points(
                        unit.id,
                        movement_cost,
                        unit.movement_remaining,
                    );
                }
            }
            Err(reason) => {
                CoreDebugUtils::log_unit_movement_failure(unit.id, reason);
            }
        }
    }
}

pub fn clear_completed_movement_orders(
    mut commands: Commands,
    query: Query<Entity, With<PlayerMovementOrder>>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<PlayerMovementOrder>();
    }
}

/// Execute AI movement orders (similar to player movement but uses MovementOrder instead of PlayerMovementOrder)
pub fn execute_ai_movement_orders(
    mut commands: Commands,
    mut movement_query: Query<(Entity, &mut MilitaryUnit, &mut Position, &MovementOrder)>,
    world_map: Res<WorldMap>,
) {
    for (entity, mut unit, mut position, movement_order) in movement_query.iter_mut() {
        let current_position = *position;

        // Process the next step in the movement order
        if let Some(next_position) = movement_order.next_position() {
            commands.entity(entity).remove::<MovementOrder>();

            match validate_movement_to_adjacent_tile(current_position, next_position, &world_map) {
                Ok(movement_cost) => {
                    if unit.movement_remaining >= movement_cost {
                        *position = next_position;
                        unit.movement_remaining -= movement_cost;

                        CoreDebugUtils::log_unit_movement_success(
                            unit.id,
                            current_position.x,
                            current_position.y,
                            next_position.x,
                            next_position.y,
                            movement_cost,
                            unit.movement_remaining,
                        );
                    } else {
                        CoreDebugUtils::log_insufficient_movement_points(
                            unit.id,
                            movement_cost,
                            unit.movement_remaining,
                        );
                    }
                }
                Err(reason) => {
                    CoreDebugUtils::log_unit_movement_failure(unit.id, reason);
                }
            }
        } else {
            // Movement order is complete, remove it
            commands.entity(entity).remove::<MovementOrder>();
        }
    }
}

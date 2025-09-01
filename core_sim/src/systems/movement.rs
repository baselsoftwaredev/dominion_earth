use crate::{Position, MilitaryUnit, PlayerMovementOrder, WorldMap, TerrainType};
use bevy::prelude::*;

/// Calculate distance between two positions (Manhattan distance for grid-based movement)
fn calculate_movement_distance(from: Position, to: Position) -> u32 {
    let dx = (to.x - from.x).abs() as u32;
    let dy = (to.y - from.y).abs() as u32;
    dx + dy
}

/// Check if movement is valid (adjacent tile and terrain allows movement)
fn is_valid_movement(
    from: Position,
    to: Position,
    world_map: &WorldMap,
) -> Result<u32, &'static str> {
    // Check if target is adjacent (distance of 1)
    let distance = calculate_movement_distance(from, to);
    if distance != 1 {
        return Err("Can only move to adjacent tiles");
    }
    
    // Check if target tile exists and is moveable
    if let Some(tile) = world_map.get_tile(to) {
        match tile.terrain {
            TerrainType::Ocean => Err("Cannot move into ocean"),
            _ => {
                // Return movement cost for this terrain
                let movement_cost = tile.movement_cost as u32;
                if movement_cost == 0 {
                    Ok(1) // Default cost if not set
                } else {
                    Ok(movement_cost)
                }
            }
        }
    } else {
        Err("Target position is outside map boundaries")
    }
}

/// System to process player movement orders
pub fn process_player_movement_orders(
    mut commands: Commands,
    mut movement_query: Query<(Entity, &mut MilitaryUnit, &mut Position, &PlayerMovementOrder)>,
    world_map: Res<WorldMap>,
) {
    for (entity, mut unit, mut position, movement_order) in movement_query.iter_mut() {
        let current_pos = *position;
        let target_pos = movement_order.target_position;
        
        // Always remove the movement order first
        commands.entity(entity).remove::<PlayerMovementOrder>();
        
        // Validate movement
        match is_valid_movement(current_pos, target_pos, &world_map) {
            Ok(movement_cost) => {
                // Check if unit has enough movement points
                if unit.movement_remaining >= movement_cost {
                    // Execute movement
                    *position = target_pos;
                    unit.movement_remaining -= movement_cost;
                    
                    info!(
                        "Unit {} moved from ({}, {}) to ({}, {}) - Cost: {} - Remaining: {}", 
                        unit.id, 
                        current_pos.x, current_pos.y,
                        target_pos.x, target_pos.y,
                        movement_cost,
                        unit.movement_remaining
                    );
                } else {
                    warn!(
                        "Unit {} cannot move - insufficient movement points. Required: {}, Available: {}", 
                        unit.id, movement_cost, unit.movement_remaining
                    );
                }
            }
            Err(reason) => {
                warn!("Unit {} movement blocked: {}", unit.id, reason);
            }
        }
    }
}

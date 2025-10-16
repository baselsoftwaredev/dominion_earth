use bevy_ecs::prelude::*;

use crate::{
    components::{
        City, Civilization, FogOfWarMaps, MilitaryUnit, Position, ProvidesVision, VisibilityMap,
    },
    CivId, WorldMap,
};

/// System to update fog of war for all civilizations at the start of each turn
/// This runs every frame to detect position changes but only updates when necessary
pub fn update_fog_of_war(
    mut fog_of_war: ResMut<FogOfWarMaps>,
    world_map: Res<WorldMap>,
    civilizations: Query<&Civilization>,
    // Use change detection to only update when units/cities move or are added
    units: Query<(&Position, &CivId, &ProvidesVision), With<MilitaryUnit>>,
    cities: Query<(&Position, &CivId, &ProvidesVision), With<City>>,
    // Check for position changes
    changed_positions: Query<&Position, Or<(Changed<Position>, Added<MilitaryUnit>, Added<City>)>>,
) {
    // Initialize fog of war maps for any new civilizations
    let mut new_civ_initialized = false;
    for civ in civilizations.iter() {
        if fog_of_war.get(civ.id).is_none() {
            fog_of_war.init_for_civ(civ.id, world_map.width, world_map.height);
            println!("FOG_OF_WAR: Initialized map for civ {:?}", civ.id);
            new_civ_initialized = true;
        }
    }

    // Only update if something changed: new civ, position changed, or entity added
    let should_update = new_civ_initialized || !changed_positions.is_empty();

    if !should_update {
        return; // Skip update if nothing changed
    }

    // For each civilization, reset visible tiles to explored, then recalculate
    for civ in civilizations.iter() {
        if let Some(vis_map) = fog_of_war.get_mut(civ.id) {
            // Reset all visible tiles to explored (preserves explored state)
            vis_map.reset_visibility();

            let mut unit_count = 0;
            let mut city_count = 0;

            // Add vision from all units belonging to this civilization
            for (pos, civ_id, provides_vision) in units.iter() {
                if *civ_id == civ.id {
                    vis_map.mark_visible(*pos, provides_vision.range);
                    unit_count += 1;
                }
            }

            // Add vision from all cities belonging to this civilization
            for (pos, civ_id, provides_vision) in cities.iter() {
                if *civ_id == civ.id {
                    vis_map.mark_visible(*pos, provides_vision.range);
                    city_count += 1;
                }
            }

            // Only log on first initialization or when explicitly debugging
            // Remove verbose per-frame logging
            // println!(
            //     "FOG_OF_WAR: Updated civ {:?} - {} units, {} cities providing vision",
            //     civ.id, unit_count, city_count
            // );
        }
    }
}

/// Initialize fog of war for a specific civilization (called when spawning a new civ)
pub fn initialize_fog_of_war_for_civ(
    civ_id: CivId,
    fog_of_war: &mut FogOfWarMaps,
    world_map: &WorldMap,
) {
    fog_of_war.init_for_civ(civ_id, world_map.width, world_map.height);
}

/// Helper function: Filter units visible to a specific civilization
pub fn filter_visible_units<'a>(
    units: impl Iterator<Item = (&'a Position, Entity)>,
    civ_id: CivId,
    fog_of_war: &'a FogOfWarMaps,
) -> Vec<Entity> {
    units
        .filter_map(|(pos, entity)| {
            if fog_of_war.is_visible_to(civ_id, *pos) {
                Some(entity)
            } else {
                None
            }
        })
        .collect()
}

/// Helper function: Filter cities visible to a specific civilization
pub fn filter_visible_cities<'a>(
    cities: impl Iterator<Item = (&'a Position, Entity)>,
    civ_id: CivId,
    fog_of_war: &'a FogOfWarMaps,
) -> Vec<Entity> {
    cities
        .filter_map(|(pos, entity)| {
            if fog_of_war.is_visible_to(civ_id, *pos) {
                Some(entity)
            } else {
                None
            }
        })
        .collect()
}

/// Helper function: Check if a position is visible to a civilization
pub fn is_position_visible(pos: Position, civ_id: CivId, fog_of_war: &FogOfWarMaps) -> bool {
    fog_of_war.is_visible_to(civ_id, pos)
}

/// Helper function: Check if a position has been explored by a civilization
pub fn is_position_explored(pos: Position, civ_id: CivId, fog_of_war: &FogOfWarMaps) -> bool {
    fog_of_war.is_explored_by(civ_id, pos)
}

/// Helper function: Get all visible positions for a civilization
pub fn get_visible_positions(civ_id: CivId, fog_of_war: &FogOfWarMaps) -> Vec<Position> {
    if let Some(vis_map) = fog_of_war.get(civ_id) {
        let mut visible_positions = Vec::new();
        for x in 0..vis_map.width {
            for y in 0..vis_map.height {
                let pos = Position::new(x as i32, y as i32);
                if vis_map.is_visible(pos) {
                    visible_positions.push(pos);
                }
            }
        }
        visible_positions
    } else {
        Vec::new()
    }
}

/// Helper function: Get all explored positions for a civilization
pub fn get_explored_positions(civ_id: CivId, fog_of_war: &FogOfWarMaps) -> Vec<Position> {
    if let Some(vis_map) = fog_of_war.get(civ_id) {
        let mut explored_positions = Vec::new();
        for x in 0..vis_map.width {
            for y in 0..vis_map.height {
                let pos = Position::new(x as i32, y as i32);
                if vis_map.is_explored(pos) {
                    explored_positions.push(pos);
                }
            }
        }
        explored_positions
    } else {
        Vec::new()
    }
}

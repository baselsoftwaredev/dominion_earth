use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::{CivId, FogOfWarMaps, PlayerControlled, Position, VisibilityState};

/// Component that links a tile sprite entity to its grid position
/// This allows us to update tile visibility based on fog of war
#[derive(Component, Debug, Clone)]
pub struct TileSprite {
    pub position: Position,
}

/// System to apply fog of war visibility to tile sprites
/// This runs after fog of war is updated and modifies tile sprite colors
pub fn apply_fog_of_war_to_tiles(
    fog_of_war: Res<FogOfWarMaps>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    mut tile_query: Query<(&TileSprite, &mut TileColor)>,
) {
    // Get the player's civilization ID
    let player_civ_id = if let Ok(player_civ) = player_query.single() {
        player_civ.id
    } else {
        return; // No player, nothing to render
    };

    // Get the player's visibility map
    let visibility_map = if let Some(map) = fog_of_war.get(player_civ_id) {
        map
    } else {
        return; // No visibility map yet
    };

    // Update each tile sprite based on visibility
    for (tile_sprite, mut tile_color) in tile_query.iter_mut() {
        let visibility = visibility_map
            .get(tile_sprite.position)
            .unwrap_or(VisibilityState::Unexplored);

        tile_color.0 = match visibility {
            VisibilityState::Unexplored => Color::srgba(0.0, 0.0, 0.0, 1.0), // Completely black
            VisibilityState::Explored => Color::srgba(0.4, 0.4, 0.4, 1.0),   // Dimmed gray
            VisibilityState::Visible => Color::WHITE,                        // Full brightness
        };
    }
}

/// System to hide entities on unexplored tiles
/// This hides units, cities, etc. that are on tiles the player hasn't seen
pub fn hide_entities_in_fog(
    fog_of_war: Res<FogOfWarMaps>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    entity_query: Query<(Entity, &Position, Option<&PlayerControlled>)>,
    mut visibility_query: Query<&mut Visibility>,
) {
    // Get the player's civilization ID
    let player_civ_id = if let Ok(player_civ) = player_query.single() {
        player_civ.id
    } else {
        return; // No player, nothing to hide
    };

    // Get the player's visibility map
    let visibility_map = if let Some(map) = fog_of_war.get(player_civ_id) {
        map
    } else {
        return; // No visibility map yet
    };

    // Hide/show entities based on visibility
    for (entity, position, is_player_controlled) in entity_query.iter() {
        if let Ok(mut visibility) = visibility_query.get_mut(entity) {
            let tile_visibility = visibility_map
                .get(*position)
                .unwrap_or(VisibilityState::Unexplored);

            // Always show player-controlled entities
            // Hide other entities if tile is unexplored or only explored (not currently visible)
            let should_be_visible = is_player_controlled.is_some()
                || matches!(tile_visibility, VisibilityState::Visible);

            *visibility = if should_be_visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

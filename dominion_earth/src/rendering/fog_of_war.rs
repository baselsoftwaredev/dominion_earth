use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::{CivId, FogOfWarMaps, PlayerControlled, Position, VisibilityState};

use crate::ui::capital_labels::CapitalLabel;
use crate::ui::unit_labels::UnitLabel;

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
    // Query entities with sprites (units, capitals)
    entity_query: Query<(
        &Position,
        Option<&PlayerControlled>,
        Option<&core_sim::SpriteEntityReference>,
        Option<&CivId>,
    )>,
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

    // Hide/show sprite entities based on visibility
    for (position, is_player_controlled, sprite_ref, civ_id) in entity_query.iter() {
        let tile_visibility = visibility_map
            .get(*position)
            .unwrap_or(VisibilityState::Unexplored);

        // Determine if entity should be visible:
        // 1. Player-controlled entities are always visible
        // 2. Entities belonging to the player's civ are always visible
        // 3. Other entities only visible if tile is currently visible (not just explored)
        let belongs_to_player = civ_id.map_or(false, |cid| *cid == player_civ_id);
        let should_be_visible = is_player_controlled.is_some()
            || belongs_to_player
            || matches!(tile_visibility, VisibilityState::Visible);

        // Set visibility on the sprite entity (the actual visual representation)
        if let Some(sprite_ref) = sprite_ref {
            if let Ok(mut visibility) = visibility_query.get_mut(sprite_ref.sprite_entity) {
                *visibility = if should_be_visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

/// System to hide capital labels for cities not visible to the player
/// This hides the Text2d labels above capitals that are in fog of war
pub fn hide_capital_labels_in_fog(
    fog_of_war: Res<FogOfWarMaps>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    capital_query: Query<(&Position, &core_sim::Capital)>,
    mut label_query: Query<(Entity, &CapitalLabel, &mut Visibility)>,
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

    // Hide/show capital labels based on visibility
    for (_label_entity, capital_label, mut label_visibility) in label_query.iter_mut() {
        // Get the capital entity's position and owner
        if let Ok((position, capital)) = capital_query.get(capital_label.capital_entity) {
            let tile_visibility = visibility_map
                .get(*position)
                .unwrap_or(VisibilityState::Unexplored);

            // Show label if:
            // 1. Capital belongs to the player's civ, OR
            // 2. Tile is currently visible (not just explored)
            let belongs_to_player = capital.owner == player_civ_id;
            let should_be_visible =
                belongs_to_player || matches!(tile_visibility, VisibilityState::Visible);

            *label_visibility = if should_be_visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// System to hide unit labels for units not visible to the player
/// This hides the Text2d labels above units that are in fog of war
pub fn hide_unit_labels_in_fog(
    fog_of_war: Res<FogOfWarMaps>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    unit_query: Query<(&Position, &core_sim::MilitaryUnit)>,
    mut label_query: Query<(Entity, &UnitLabel, &mut Visibility)>,
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

    // Hide/show unit labels based on visibility
    for (_label_entity, unit_label, mut label_visibility) in label_query.iter_mut() {
        // Get the unit entity's position and owner
        if let Ok((position, unit)) = unit_query.get(unit_label.unit_entity) {
            let tile_visibility = visibility_map
                .get(*position)
                .unwrap_or(VisibilityState::Unexplored);

            // Show label if:
            // 1. Unit belongs to the player's civ, OR
            // 2. Tile is currently visible (not just explored)
            let belongs_to_player = unit.owner == player_civ_id;
            let should_be_visible =
                belongs_to_player || matches!(tile_visibility, VisibilityState::Visible);

            *label_visibility = if should_be_visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

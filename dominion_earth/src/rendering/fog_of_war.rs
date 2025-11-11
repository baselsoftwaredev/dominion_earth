use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::military::MilitaryUnit;
use core_sim::components::rendering::SpriteEntityReference;
use core_sim::{CivId, FogOfWarMaps, PlayerControlled, Position, VisibilityState};

use crate::ui::capital_labels::CapitalLabel;
use crate::ui::debug_toolbox::FogOfWarToggle;
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
    fog_toggle: Res<FogOfWarToggle>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    mut tile_query: Query<(&TileSprite, &mut TileColor)>,
) {
    // If fog of war is disabled, skip
    if !fog_toggle.enabled {
        // Set all tiles to full brightness when FOW is disabled
        for (_, mut tile_color) in tile_query.iter_mut() {
            tile_color.0 = Color::WHITE;
        }
        return;
    }

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
    fog_toggle: Res<FogOfWarToggle>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    // Query units with sprites
    units_query: Query<(
        &Position,
        &core_sim::components::military::MilitaryUnit,
        Option<&PlayerControlled>,
        Option<&SpriteEntityReference>,
    )>,
    // Query capitals with sprites
    capitals_query: Query<(
        &Position,
        &core_sim::Capital,
        Option<&PlayerControlled>,
        Option<&SpriteEntityReference>,
    )>,
    mut visibility_query: Query<&mut Visibility>,
) {
    // If fog of war is disabled, show all entities
    if !fog_toggle.enabled {
        for (_, _, _, sprite_ref) in units_query.iter() {
            if let Some(sprite_ref) = sprite_ref {
                if let Ok(mut visibility) = visibility_query.get_mut(sprite_ref.sprite_entity) {
                    *visibility = Visibility::Inherited;
                }
            }
        }
        for (_, _, _, sprite_ref) in capitals_query.iter() {
            if let Some(sprite_ref) = sprite_ref {
                if let Ok(mut visibility) = visibility_query.get_mut(sprite_ref.sprite_entity) {
                    *visibility = Visibility::Inherited;
                }
            }
        }
        return;
    }

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

    // Hide/show unit sprites based on visibility
    for (position, unit, is_player_controlled, sprite_ref) in units_query.iter() {
        let tile_visibility = visibility_map
            .get(*position)
            .unwrap_or(VisibilityState::Unexplored);

        // Determine if unit should be visible:
        // 1. Player-controlled units are always visible
        // 2. Units belonging to the player's civ are always visible
        // 3. Other units only visible if tile is currently visible (not just explored)
        let belongs_to_player = unit.owner == player_civ_id;
        let should_be_visible = is_player_controlled.is_some()
            || belongs_to_player
            || matches!(tile_visibility, VisibilityState::Visible);

        // Set visibility on the sprite entity (the actual visual representation)
        if let Some(sprite_ref) = sprite_ref {
            if let Ok(mut visibility) = visibility_query.get_mut(sprite_ref.sprite_entity) {
                let new_visibility = if should_be_visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };

                // Debug log for entities that should be hidden
                if matches!(new_visibility, Visibility::Hidden) {
                    crate::debug_println!(
                        "🫙 Hiding unit at {:?} (visibility={:?}, belongs_to_player={})",
                        position,
                        tile_visibility,
                        belongs_to_player
                    );
                }

                *visibility = new_visibility;
            }
        }
    }

    // Hide/show capital sprites based on visibility
    for (position, capital, is_player_controlled, sprite_ref) in capitals_query.iter() {
        let tile_visibility = visibility_map
            .get(*position)
            .unwrap_or(VisibilityState::Unexplored);

        // Determine if capital should be visible:
        // 1. Player-controlled capitals are always visible
        // 2. Capitals belonging to the player's civ are always visible
        // 3. Other capitals only visible if tile is currently visible (not just explored)
        let belongs_to_player = capital.owner == player_civ_id;
        let should_be_visible = is_player_controlled.is_some()
            || belongs_to_player
            || matches!(tile_visibility, VisibilityState::Visible);

        // Set visibility on the sprite entity (the actual visual representation)
        if let Some(sprite_ref) = sprite_ref {
            if let Ok(mut visibility) = visibility_query.get_mut(sprite_ref.sprite_entity) {
                let new_visibility = if should_be_visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };

                // Debug log for entities that should be hidden
                if matches!(new_visibility, Visibility::Hidden) {
                    crate::debug_println!(
                        "🫙 Hiding capital at {:?} (visibility={:?}, belongs_to_player={})",
                        position,
                        tile_visibility,
                        belongs_to_player
                    );
                }

                *visibility = new_visibility;
            }
        }
    }
}

/// System to hide capital labels for cities not visible to the player
/// This hides the Text2d labels above capitals that are in fog of war
pub fn hide_capital_labels_in_fog(
    fog_of_war: Res<FogOfWarMaps>,
    fog_toggle: Res<FogOfWarToggle>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    capital_query: Query<(&Position, &core_sim::Capital)>,
    mut label_query: Query<(Entity, &CapitalLabel, &mut Visibility)>,
) {
    // If fog of war is disabled, show all capital labels
    if !fog_toggle.enabled {
        for (_, _, mut label_visibility) in label_query.iter_mut() {
            *label_visibility = Visibility::Inherited;
        }
        return;
    }

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

/// System to despawn sprite entities that are hidden by fog of war to save memory
/// This prevents accumulation of invisible entities
pub fn despawn_hidden_entity_sprites(
    mut commands: Commands,
    fog_of_war: Res<FogOfWarMaps>,
    fog_toggle: Res<FogOfWarToggle>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    // Query units with sprites
    units_query: Query<(
        &Position,
        &core_sim::components::military::MilitaryUnit,
        Option<&PlayerControlled>,
        Option<&SpriteEntityReference>,
    )>,
    // Query capitals with sprites
    capitals_query: Query<(
        &Position,
        &core_sim::Capital,
        Option<&PlayerControlled>,
        Option<&SpriteEntityReference>,
    )>,
    visibility_query: Query<&Visibility>,
) {
    // Don't despawn if fog of war is disabled
    if !fog_toggle.enabled {
        return;
    }

    // Get the player's civilization ID
    let player_civ_id = if let Ok(player_civ) = player_query.single() {
        player_civ.id
    } else {
        return; // No player, nothing to do
    };

    // Get the player's visibility map
    let visibility_map = if let Some(map) = fog_of_war.get(player_civ_id) {
        map
    } else {
        return; // No visibility map yet
    };

    // Despawn hidden unit sprites
    for (position, unit, is_player_controlled, sprite_ref) in units_query.iter() {
        if is_player_controlled.is_some() {
            continue; // Keep player-controlled unit sprites
        }

        let belongs_to_player = unit.owner == player_civ_id;
        if belongs_to_player {
            continue; // Keep units belonging to player's civ
        }

        let tile_visibility = visibility_map
            .get(*position)
            .unwrap_or(VisibilityState::Unexplored);

        // Only despawn if tile is not visible (explored or unexplored)
        if matches!(tile_visibility, VisibilityState::Visible) {
            continue;
        }

        // Check if sprite is actually hidden
        if let Some(sprite_ref) = sprite_ref {
            if let Ok(visibility) = visibility_query.get(sprite_ref.sprite_entity) {
                if matches!(visibility, Visibility::Hidden) {
                    commands.entity(sprite_ref.sprite_entity).despawn();
                }
            }
        }
    }

    // Despawn hidden capital sprites
    for (position, capital, is_player_controlled, sprite_ref) in capitals_query.iter() {
        if is_player_controlled.is_some() {
            continue; // Keep player-controlled capital sprites
        }

        let belongs_to_player = capital.owner == player_civ_id;
        if belongs_to_player {
            continue; // Keep capitals belonging to player's civ
        }

        let tile_visibility = visibility_map
            .get(*position)
            .unwrap_or(VisibilityState::Unexplored);

        // Only despawn if tile is not visible (explored or unexplored)
        if matches!(tile_visibility, VisibilityState::Visible) {
            continue;
        }

        // Check if sprite is actually hidden
        if let Some(sprite_ref) = sprite_ref {
            if let Ok(visibility) = visibility_query.get(sprite_ref.sprite_entity) {
                if matches!(visibility, Visibility::Hidden) {
                    commands.entity(sprite_ref.sprite_entity).despawn();
                }
            }
        }
    }
}

/// System to hide unit labels for units not visible to the player
/// This hides the Text2d labels above units that are in fog of war
pub fn hide_unit_labels_in_fog(
    fog_of_war: Res<FogOfWarMaps>,
    fog_toggle: Res<FogOfWarToggle>,
    player_query: Query<&core_sim::Civilization, With<PlayerControlled>>,
    unit_query: Query<(&Position, &core_sim::MilitaryUnit)>,
    mut label_query: Query<(Entity, &UnitLabel, &mut Visibility)>,
) {
    // If fog of war is disabled, show all unit labels
    if !fog_toggle.enabled {
        for (_, _, mut label_visibility) in label_query.iter_mut() {
            *label_visibility = Visibility::Inherited;
        }
        return;
    }

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

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::{city::Capital, civilization::Civilization, military::MilitaryUnit, position::Position};
use core_sim::{FogOfWarMaps, PlayerControlled, VisibilityState};
use crate::constants::rendering::borders;
use super::common::calculate_world_position_for_gizmo;

pub fn render_civilization_borders(
    mut gizmos: Gizmos,
    fog_of_war: Res<FogOfWarMaps>,
    player_query: Query<&Civilization, With<PlayerControlled>>,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    units: Query<(&MilitaryUnit, &Position)>,
    capitals: Query<(&Capital, &Position)>,
    civilizations: Query<&Civilization>,
) {
    let Ok((_tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    // Get the player's civilization ID for fog of war checks
    let player_civ_id = if let Ok(player_civ) = player_query.single() {
        player_civ.id
    } else {
        return; // No player, don't render borders
    };

    // Get the player's visibility map
    let visibility_map = if let Some(map) = fog_of_war.get(player_civ_id) {
        map
    } else {
        return; // No visibility map yet
    };

    for (unit, position) in units.iter() {
        // Check fog of war visibility
        let tile_visibility = visibility_map
            .get(*position)
            .unwrap_or(VisibilityState::Unexplored);

        // Only render borders for units that:
        // 1. Belong to the player's civ, OR
        // 2. Are on tiles currently visible to the player
        let belongs_to_player = unit.owner == player_civ_id;
        let should_render = belongs_to_player || matches!(tile_visibility, VisibilityState::Visible);

        if !should_render {
            continue;
        }

        if let Some(civ) = civilizations.iter().find(|civ| civ.id == unit.owner) {
            let world_pos = calculate_world_position_for_gizmo(
                *position, map_size, tile_size, grid_size, map_type, anchor,
            );
            let border_color = Color::srgb(civ.color[0], civ.color[1], civ.color[2]);

            let half_width = tile_size.x * borders::UNIT_BORDER_HALF_WIDTH_FACTOR;
            let half_height = tile_size.y * borders::UNIT_BORDER_HALF_HEIGHT_FACTOR;
            let center = world_pos.truncate();

            let corners = [
                center + Vec2::new(-half_width, -half_height),
                center + Vec2::new(half_width, -half_height),
                center + Vec2::new(half_width, half_height),
                center + Vec2::new(-half_width, half_height),
                center + Vec2::new(-half_width, -half_height),
            ];

            gizmos.linestrip_2d(corners, border_color);
        }
    }

    for (capital, position) in capitals.iter() {
        // Check fog of war visibility
        let tile_visibility = visibility_map
            .get(*position)
            .unwrap_or(VisibilityState::Unexplored);

        // Only render borders for capitals that:
        // 1. Belong to the player's civ, OR
        // 2. Are on tiles currently visible to the player
        let belongs_to_player = capital.owner == player_civ_id;
        let should_render = belongs_to_player || matches!(tile_visibility, VisibilityState::Visible);

        if !should_render {
            continue;
        }

        if let Some(civ) = civilizations.iter().find(|civ| civ.id == capital.owner) {
            let world_pos = calculate_world_position_for_gizmo(
                *position, map_size, tile_size, grid_size, map_type, anchor,
            );
            let border_color = Color::srgb(civ.color[0], civ.color[1], civ.color[2]);

            let half_width = tile_size.x * borders::CAPITAL_OUTER_BORDER_HALF_WIDTH_FACTOR;
            let half_height = tile_size.y * borders::CAPITAL_OUTER_BORDER_HALF_HEIGHT_FACTOR;
            let center = world_pos.truncate();

            let outer_corners = [
                center + Vec2::new(-half_width, -half_height),
                center + Vec2::new(half_width, -half_height),
                center + Vec2::new(half_width, half_height),
                center + Vec2::new(-half_width, half_height),
                center + Vec2::new(-half_width, -half_height),
            ];

            gizmos.linestrip_2d(outer_corners, border_color);

            let inner_half_width = tile_size.x * borders::CAPITAL_INNER_BORDER_HALF_WIDTH_FACTOR;
            let inner_half_height = tile_size.y * borders::CAPITAL_INNER_BORDER_HALF_HEIGHT_FACTOR;

            let inner_corners = [
                center + Vec2::new(-inner_half_width, -inner_half_height),
                center + Vec2::new(inner_half_width, -inner_half_height),
                center + Vec2::new(inner_half_width, inner_half_height),
                center + Vec2::new(-inner_half_width, inner_half_height),
                center + Vec2::new(-inner_half_width, -inner_half_height),
            ];

            gizmos.linestrip_2d(inner_corners, border_color);
        }
    }
}

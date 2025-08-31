use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::{city::Capital, civilization::Civilization, military::MilitaryUnit, position::Position};
use crate::constants::rendering::borders;
use super::common::calculate_world_position_for_gizmo;

pub fn render_civilization_borders(
    mut gizmos: Gizmos,
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

    for (unit, position) in units.iter() {
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

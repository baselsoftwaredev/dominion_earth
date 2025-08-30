use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::Position;
use crate::constants::rendering::z_layers;

#[derive(Resource, Clone)]
pub struct TilemapIdResource(pub TilemapId);

pub fn calculate_world_position_for_gizmo(
    position: Position,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Vec3 {
    let tile_pos = TilePos {
        x: position.x as u32,
        y: position.y as u32,
    };

    let tile_center = tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
    tile_center.extend(z_layers::UNIT_Z + 1.0)
}

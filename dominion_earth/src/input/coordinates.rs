use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::Position;

/// Convert cursor position to tile coordinates using the tilemap's coordinate system
/// This properly handles isometric and other non-square coordinate systems
pub fn convert_cursor_position_to_tile_coordinates(
    cursor_position: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    tilemap_transform: &Transform,
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
    tile_size: &TilemapTileSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Result<Position, &'static str> {
    // Convert cursor position to world position
    match camera.viewport_to_world_2d(camera_transform, cursor_position) {
        Ok(world_position) => {
            // Transform world position relative to the tilemap's transform
            let cursor_in_map_pos: Vec2 = {
                let cursor_pos = Vec4::from((world_position, 0.0, 1.0));
                let cursor_in_map_pos = tilemap_transform.to_matrix().inverse() * cursor_pos;
                cursor_in_map_pos.xy()
            };

            // Use bevy_ecs_tilemap's coordinate conversion for proper isometric handling
            if let Some(tile_pos) = TilePos::from_world_pos(
                &cursor_in_map_pos,
                map_size,
                grid_size,
                tile_size,
                map_type,
                anchor,
            ) {
                Ok(Position::new(tile_pos.x as i32, tile_pos.y as i32))
            } else {
                Err("Cursor position is outside the tilemap bounds")
            }
        }
        Err(_) => Err("Failed to convert cursor position to world position"),
    }
}

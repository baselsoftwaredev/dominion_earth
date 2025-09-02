use crate::constants::input::coordinates;
use bevy::prelude::*;
use core_sim::components::Position;

pub fn convert_cursor_position_to_tile_coordinates(
    cursor_position: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Result<Position, &'static str> {
    match camera.viewport_to_world_2d(camera_transform, cursor_position) {
        Ok(world_position) => {
            let tile_x_coordinate =
                (world_position.x / coordinates::TILE_SIZE_FOR_INPUT).round() as i32;
            let tile_y_coordinate =
                (world_position.y / coordinates::TILE_SIZE_FOR_INPUT).round() as i32;
            Ok(Position::new(tile_x_coordinate, tile_y_coordinate))
        }
        Err(_) => Err("Failed to convert cursor position to world position"),
    }
}

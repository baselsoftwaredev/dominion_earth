use crate::constants::rendering::camera as camera_constants;
use crate::screens::Screen;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::{city::Capital, position::Position};

/// Plugin for camera setup and control
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            OnEnter(Screen::Gameplay),
            center_camera_on_player_capital
                .after(crate::game::setup_game)
                .after(crate::rendering::capitals::spawn_animated_capital_tiles),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d).insert(Transform::from_xyz(
        camera_constants::INITIAL_CAMERA_X,
        camera_constants::INITIAL_CAMERA_Y,
        camera_constants::INITIAL_CAMERA_Z,
    ));
}

/// Centers the camera on the player's capital after civilizations are spawned
fn center_camera_on_player_capital(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    capitals_query: Query<&Position, (With<Capital>, With<core_sim::PlayerControlled>)>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
) {
    // Only run if we have a player capital and haven't already centered
    if let Ok(capital_position) = capitals_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            if let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_query.single() {
                // Convert the capital's tile position to world coordinates
                let tile_pos = TilePos {
                    x: capital_position.x as u32,
                    y: capital_position.y as u32,
                };

                let world_pos =
                    tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

                // Center camera on the player's capital
                camera_transform.translation.x = world_pos.x;
                camera_transform.translation.y = world_pos.y;

                println!(
                    "Camera centered on player capital at tile ({}, {}) -> world ({}, {})",
                    capital_position.x, capital_position.y, world_pos.x, world_pos.y
                );
            }
        }
    }
}

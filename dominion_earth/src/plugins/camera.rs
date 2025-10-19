use crate::constants::rendering::camera as camera_constants;
use crate::debug_utils::DebugLogging;
use crate::screens::Screen;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::{city::Capital, position::Position};

pub struct CameraPlugin;

#[derive(Resource, Default)]
struct CameraCentered {
    centered: bool,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraCentered::default())
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                center_camera_on_player_capital.run_if(in_state(Screen::Gameplay)),
            )
            .add_systems(OnExit(Screen::Gameplay), reset_camera_centered);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d).insert(Transform::from_xyz(
        camera_constants::INITIAL_CAMERA_X,
        camera_constants::INITIAL_CAMERA_Y,
        camera_constants::INITIAL_CAMERA_Z,
    ));
}

fn reset_camera_centered(mut camera_centered: ResMut<CameraCentered>) {
    camera_centered.centered = false;
}

fn center_camera_on_player_capital(
    mut camera_centered: ResMut<CameraCentered>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    capitals_query: Query<&Position, (With<Capital>, With<core_sim::PlayerControlled>)>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    debug_logging: Res<DebugLogging>,
) {
    if camera_centered.centered {
        return;
    }

    let Some(capital_position) = capitals_query.iter().next() else {
        return;
    };

    let Some(mut camera_transform) = camera_query.iter_mut().next() else {
        return;
    };

    let Some((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_query.iter().next()
    else {
        return;
    };

    let tile_pos = TilePos {
        x: capital_position.x as u32,
        y: capital_position.y as u32,
    };

    let world_pos = tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

    camera_transform.translation.x = world_pos.x;
    camera_transform.translation.y = world_pos.y;

    camera_centered.centered = true;

    crate::debug_println!(
        debug_logging,
        "Camera centered on player capital at tile ({}, {}) -> world ({}, {})",
        capital_position.x,
        capital_position.y,
        world_pos.x,
        world_pos.y
    );
}

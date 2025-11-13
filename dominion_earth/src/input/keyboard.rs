use crate::constants::input::camera;
use crate::debug_utils::DebugUtils;
use crate::game::GameState;
use bevy::prelude::*;

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        game_state.paused = !game_state.paused;
        DebugUtils::log_game_state_change("paused", game_state.paused);
    }

    handle_camera_controls(&keyboard_input, &mut camera_query, &time);
}

fn handle_camera_controls(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
    time: &Res<Time>,
) {
    handle_camera_movement_controls(keyboard_input, camera_query, time);
    handle_camera_zoom_key_controls(keyboard_input, camera_query, time);
}

fn handle_camera_movement_controls(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
    time: &Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.y += camera::MOVEMENT_SPEED * time.delta_secs();
        }
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.y -= camera::MOVEMENT_SPEED * time.delta_secs();
        }
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.x -= camera::MOVEMENT_SPEED * time.delta_secs();
        }
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.x += camera::MOVEMENT_SPEED * time.delta_secs();
        }
    }
}

fn handle_camera_zoom_key_controls(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
    time: &Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::KeyQ) {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.scale *= camera::ZOOM_RATE + time.delta_secs();
            camera_transform.scale = camera_transform
                .scale
                .clamp(Vec3::splat(0.1), Vec3::splat(5.0));
        }
    }

    if keyboard_input.pressed(KeyCode::KeyE) {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.scale *= camera::ZOOM_RATE - time.delta_secs();
            camera_transform.scale = camera_transform
                .scale
                .clamp(Vec3::splat(0.1), Vec3::splat(5.0));
        }
    }
}

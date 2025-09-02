use crate::constants::input::{camera, simulation};
use crate::debug_utils::{DebugLogging, DebugUtils};
use crate::game::GameState;
use bevy::prelude::*;

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    debug_logging: Res<DebugLogging>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        game_state.paused = !game_state.paused;
        DebugUtils::log_game_state_change(&debug_logging, "paused", game_state.paused);
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        game_state.auto_advance = !game_state.auto_advance;
        DebugUtils::log_game_state_change(&debug_logging, "auto-advance", game_state.auto_advance);
    }

    handle_simulation_speed_increase(&keyboard_input, &mut game_state, &debug_logging);
    handle_simulation_speed_decrease(&keyboard_input, &mut game_state, &debug_logging);
    handle_camera_controls(&keyboard_input, &mut camera_query, &time);
}

fn handle_simulation_speed_increase(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    game_state: &mut ResMut<GameState>,
    debug_logging: &Res<DebugLogging>,
) {
    if keyboard_input.just_pressed(KeyCode::Equal)
        || keyboard_input.just_pressed(KeyCode::NumpadAdd)
    {
        game_state.simulation_speed =
            (game_state.simulation_speed * simulation::SPEED_MULTIPLIER).min(simulation::MAX_SPEED);
        let speed = game_state.simulation_speed;
        game_state
            .turn_timer
            .set_duration(std::time::Duration::from_secs_f32(
                simulation::BASE_TURN_DURATION / speed,
            ));
        DebugUtils::log_simulation_speed(debug_logging, speed);
    }
}

fn handle_simulation_speed_decrease(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    game_state: &mut ResMut<GameState>,
    debug_logging: &Res<DebugLogging>,
) {
    if keyboard_input.just_pressed(KeyCode::Minus)
        || keyboard_input.just_pressed(KeyCode::NumpadSubtract)
    {
        game_state.simulation_speed =
            (game_state.simulation_speed / simulation::SPEED_MULTIPLIER).max(simulation::MIN_SPEED);
        let speed = game_state.simulation_speed;
        game_state
            .turn_timer
            .set_duration(std::time::Duration::from_secs_f32(
                simulation::BASE_TURN_DURATION / speed,
            ));
        DebugUtils::log_simulation_speed(debug_logging, speed);
    }
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

use crate::ui::SelectedTile;
use core_sim::components::Position;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// System to detect tile clicks and update SelectedTile resource
pub fn select_tile_on_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut selected_tile: ResMut<SelectedTile>,
    world_map: Res<core_sim::resources::WorldMap>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        // Get cursor position in world coordinates
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_query.single() {
                    // Convert screen to world coordinates
                    match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        Ok(world_pos) => {
                            let tile_x = (world_pos.x / 64.0).floor() as i32;
                            let tile_y = (world_pos.y / 64.0).floor() as i32;
                            let pos = Position::new(tile_x, tile_y);
                            println!("Tile clicked: ({}, {})", tile_x, tile_y);
                            // Check if tile exists in world map
                            if world_map.get_tile(pos).is_some() {
                                println!("Tile exists in world map.");
                                selected_tile.position = Some(pos);
                            } else {
                                println!("No tile data found at this position.");
                                selected_tile.position = None;
                            }
                        }
                        Err(_) => {
                            println!("Failed to convert cursor position to world position.");
                            selected_tile.position = None;
                        }
                    }
                }
            }
        }
    }
}
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::game::GameState;

/// Handle keyboard input for game controls
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    // Game controls
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        game_state.paused = !game_state.paused;
        println!("Game {}", if game_state.paused { "paused" } else { "resumed" });
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        game_state.auto_advance = !game_state.auto_advance;
        println!("Auto-advance {}", if game_state.auto_advance { "enabled" } else { "disabled" });
    }

    if keyboard_input.just_pressed(KeyCode::Equal) || keyboard_input.just_pressed(KeyCode::NumpadAdd) {
        game_state.simulation_speed = (game_state.simulation_speed * 1.5).min(5.0);
        let speed = game_state.simulation_speed;
        game_state.turn_timer.set_duration(std::time::Duration::from_secs_f32(2.0 / speed));
        println!("Simulation speed: {:.1}x", speed);
    }

    if keyboard_input.just_pressed(KeyCode::Minus) || keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
        game_state.simulation_speed = (game_state.simulation_speed / 1.5).max(0.2);
        let speed = game_state.simulation_speed;
        game_state.turn_timer.set_duration(std::time::Duration::from_secs_f32(2.0 / speed));
        println!("Simulation speed: {:.1}x", speed);
    }

    // Camera controls
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        let mut movement = Vec3::ZERO;
        let camera_speed = 200.0 * time.delta_secs();

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += camera_speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= camera_speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= camera_speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += camera_speed;
        }

        camera_transform.translation += movement;

        // Zoom controls
        if keyboard_input.pressed(KeyCode::KeyQ) {
            camera_transform.scale *= 1.0 + time.delta_secs();
            camera_transform.scale = camera_transform.scale.clamp(Vec3::splat(0.1), Vec3::splat(5.0));
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            camera_transform.scale *= 1.0 - time.delta_secs();
            camera_transform.scale = camera_transform.scale.clamp(Vec3::splat(0.1), Vec3::splat(5.0));
        }
    }
}

/// Handle mouse input for camera panning and clicking
pub fn handle_mouse_input(
    mut mouse_wheel: EventReader<bevy::input::mouse::MouseWheel>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut last_cursor_pos: Local<Option<Vec2>>,
    mut egui_contexts: EguiContexts,
) {
    // Handle mouse wheel zoom only if mouse is NOT over egui UI
    if let Ok(ctx) = egui_contexts.ctx_mut() {
        if !ctx.is_pointer_over_area() && !ctx.wants_pointer_input() {
            for wheel_event in mouse_wheel.read() {
                if let Ok(mut camera_transform) = camera_query.single_mut() {
                    let zoom_step = 0.1;
                    if wheel_event.y > 0.0 {
                        camera_transform.scale *= 1.0 - zoom_step; // zoom out
                    } else if wheel_event.y < 0.0 {
                        camera_transform.scale *= 1.0 + zoom_step; // zoom in
                    }
                    camera_transform.scale = camera_transform.scale.clamp(Vec3::splat(0.5), Vec3::splat(3.0));
                }
            }
        } else {
            // If mouse is over UI, just consume the events
            mouse_wheel.clear();
        }
    }

    // Handle mouse panning
    if mouse_button.pressed(MouseButton::Left) {
        for cursor_event in cursor_moved.read() {
            if let Some(last_pos) = *last_cursor_pos {
                let delta = cursor_event.position - last_pos;
                
                if let Ok(mut camera_transform) = camera_query.single_mut() {
                    // Store scale to avoid borrow checker issue
                    let scale_x = camera_transform.scale.x;
                    // Invert delta to make panning feel natural
                    camera_transform.translation += Vec3::new(-delta.x, delta.y, 0.0) * scale_x;
                }
            }
            *last_cursor_pos = Some(cursor_event.position);
        }
    } else {
        // Update cursor position even when not dragging
        for cursor_event in cursor_moved.read() {
            *last_cursor_pos = Some(cursor_event.position);
        }
    }

    // Reset last position when mouse button is released
    if mouse_button.just_released(MouseButton::Left) {
        *last_cursor_pos = None;
    }
}

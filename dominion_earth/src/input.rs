use crate::ui::{DebugLogging, SelectedTile};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::Position;
use core_sim::tile::tile_components::{TileNeighbors, WorldTile};

/// System to detect tile clicks and update SelectedTile resource
pub fn select_tile_on_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut selected_tile: ResMut<SelectedTile>,
    world_map: Res<core_sim::resources::WorldMap>,
    tile_query: Query<(Entity, &WorldTile, &TileNeighbors)>,
    debug_logging: Res<DebugLogging>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        // Get cursor position in world coordinates
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_query.single() {
                    // Convert screen to world coordinates
                    match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        Ok(world_pos) => {
                            let tile_x = (world_pos.x / 64.0).round() as i32;
                            let tile_y = (world_pos.y / 64.0).round() as i32;
                            let pos = Position::new(tile_x, tile_y);
                            println!("Tile clicked: ({}, {})", tile_x, tile_y);

                            // Check if tile exists in world map
                            if world_map.get_tile(pos).is_some() {
                                if debug_logging.0 {
                                    println!("Tile exists in world map.");
                                }
                                selected_tile.position = Some(pos);

                                // Find the tile entity and show neighbor information
                                for (_entity, world_tile, neighbors) in tile_query.iter() {
                                    if world_tile.grid_pos == pos {
                                        if debug_logging.0 {
                                            println!(
                                                "=== Tile ({}, {}) Neighbors ===",
                                                tile_x, tile_y
                                            );
                                            println!(
                                                "Center tile: {:?} (viewpoint: {:?})",
                                                world_tile.terrain_type,
                                                world_tile.default_view_point
                                            );

                                            // Show North neighbor
                                            if let Some(north_entity) = neighbors.north {
                                                if let Ok((_, north_tile, _)) =
                                                    tile_query.get(north_entity)
                                                {
                                                    println!(
                                                        "North: {:?} at ({}, {}) (viewpoint: {:?})",
                                                        north_tile.terrain_type,
                                                        north_tile.grid_pos.x,
                                                        north_tile.grid_pos.y,
                                                        north_tile.default_view_point
                                                    );
                                                }
                                            } else {
                                                println!("North: OutOfBounds");
                                            }

                                            // Show South neighbor
                                            if let Some(south_entity) = neighbors.south {
                                                if let Ok((_, south_tile, _)) =
                                                    tile_query.get(south_entity)
                                                {
                                                    println!(
                                                        "South: {:?} at ({}, {}) (viewpoint: {:?})",
                                                        south_tile.terrain_type,
                                                        south_tile.grid_pos.x,
                                                        south_tile.grid_pos.y,
                                                        south_tile.default_view_point
                                                    );
                                                }
                                            } else {
                                                println!("South: OutOfBounds");
                                            }

                                            // Show East neighbor
                                            if let Some(east_entity) = neighbors.east {
                                                if let Ok((_, east_tile, _)) =
                                                    tile_query.get(east_entity)
                                                {
                                                    println!(
                                                        "East: {:?} at ({}, {}) (viewpoint: {:?})",
                                                        east_tile.terrain_type,
                                                        east_tile.grid_pos.x,
                                                        east_tile.grid_pos.y,
                                                        east_tile.default_view_point
                                                    );
                                                }
                                            } else {
                                                println!("East: OutOfBounds");
                                            }

                                            // Show West neighbor
                                            if let Some(west_entity) = neighbors.west {
                                                if let Ok((_, west_tile, _)) =
                                                    tile_query.get(west_entity)
                                                {
                                                    println!(
                                                        "West: {:?} at ({}, {}) (viewpoint: {:?})",
                                                        west_tile.terrain_type,
                                                        west_tile.grid_pos.x,
                                                        west_tile.grid_pos.y,
                                                        west_tile.default_view_point
                                                    );
                                                }
                                            } else {
                                                println!("West: OutOfBounds");
                                            }

                                            println!("===============================");
                                        }
                                        break;
                                    }
                                }
                            } else {
                                if debug_logging.0 {
                                    println!("No tile data found at this position.");
                                }
                                selected_tile.position = None;
                            }
                        }
                        Err(_) => {
                            if debug_logging.0 {
                                println!("Failed to convert cursor position to world position.");
                            }
                            selected_tile.position = None;
                        }
                    }
                }
            }
        }
    }
}
use crate::game::GameState;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

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
        println!(
            "Game {}",
            if game_state.paused {
                "paused"
            } else {
                "resumed"
            }
        );
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        game_state.auto_advance = !game_state.auto_advance;
        println!(
            "Auto-advance {}",
            if game_state.auto_advance {
                "enabled"
            } else {
                "disabled"
            }
        );
    }

    if keyboard_input.just_pressed(KeyCode::Equal)
        || keyboard_input.just_pressed(KeyCode::NumpadAdd)
    {
        game_state.simulation_speed = (game_state.simulation_speed * 1.5).min(5.0);
        let speed = game_state.simulation_speed;
        game_state
            .turn_timer
            .set_duration(std::time::Duration::from_secs_f32(2.0 / speed));
        println!("Simulation speed: {:.1}x", speed);
    }

    if keyboard_input.just_pressed(KeyCode::Minus)
        || keyboard_input.just_pressed(KeyCode::NumpadSubtract)
    {
        game_state.simulation_speed = (game_state.simulation_speed / 1.5).max(0.2);
        let speed = game_state.simulation_speed;
        game_state
            .turn_timer
            .set_duration(std::time::Duration::from_secs_f32(2.0 / speed));
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
            camera_transform.scale = camera_transform
                .scale
                .clamp(Vec3::splat(0.1), Vec3::splat(5.0));
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            camera_transform.scale *= 1.0 - time.delta_secs();
            camera_transform.scale = camera_transform
                .scale
                .clamp(Vec3::splat(0.1), Vec3::splat(5.0));
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
                    camera_transform.scale = camera_transform
                        .scale
                        .clamp(Vec3::splat(0.5), Vec3::splat(3.0));
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

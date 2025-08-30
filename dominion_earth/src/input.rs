use crate::constants::input::{camera, coordinates, simulation};
use crate::debug_utils::{DebugLogging, DebugUtils};
use crate::ui::SelectedTile;
use bevy::prelude::*;
use core_sim::components::Position;
use core_sim::tile::tile_components::{TileNeighbors, WorldTile};

/// Convert cursor position to tile coordinates
fn cursor_to_tile_position(
    cursor_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Result<Position, &'static str> {
    match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
        Ok(world_pos) => {
            let tile_x = (world_pos.x / coordinates::TILE_SIZE_FOR_INPUT).round() as i32;
            let tile_y = (world_pos.y / coordinates::TILE_SIZE_FOR_INPUT).round() as i32;
            Ok(Position::new(tile_x, tile_y))
        }
        Err(_) => Err("Failed to convert cursor position to world position"),
    }
}

/// Find the tile entity at the given position
fn find_tile_entity<'a>(
    position: Position,
    tile_query: &'a Query<(Entity, &WorldTile, &TileNeighbors)>,
) -> Option<(Entity, &'a WorldTile, &'a TileNeighbors)> {
    tile_query
        .iter()
        .find(|(_, world_tile, _)| world_tile.grid_pos == position)
}

/// Display neighbor information for debugging
fn display_neighbor_info(
    world_tile: &WorldTile,
    neighbors: &TileNeighbors,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
    debug_logging: &DebugLogging,
) {
    let pos = world_tile.grid_pos;
    DebugUtils::log_neighbors_header(debug_logging, pos.x, pos.y, &format!("{:?}", world_tile.terrain_type));

    // Show North neighbor
    display_single_neighbor("North", neighbors.north, tile_query, debug_logging);

    // Show South neighbor
    display_single_neighbor("South", neighbors.south, tile_query, debug_logging);

    // Show East neighbor
    display_single_neighbor("East", neighbors.east, tile_query, debug_logging);

    // Show West neighbor
    display_single_neighbor("West", neighbors.west, tile_query, debug_logging);

    DebugUtils::log_neighbors_footer(debug_logging);
}

/// Display information for a single neighbor
fn display_single_neighbor(
    direction: &str,
    neighbor_entity: Option<Entity>,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
    debug_logging: &DebugLogging,
) {
    if let Some(entity) = neighbor_entity {
        if let Ok((_, tile, _)) = tile_query.get(entity) {
            DebugUtils::log_single_neighbor(
                debug_logging,
                direction,
                Some(&format!("{:?}", tile.terrain_type)),
                Some(tile.grid_pos.x),
                Some(tile.grid_pos.y),
            );
        }
    } else {
        DebugUtils::log_single_neighbor(debug_logging, direction, None, None, None);
    }
}

/// Process tile selection and handle debug output
fn process_tile_selection(
    position: Position,
    world_map: &core_sim::resources::WorldMap,
    tile_query: &Query<(Entity, &WorldTile, &TileNeighbors)>,
    selected_tile: &mut SelectedTile,
    debug_logging: &DebugLogging,
) {
    // Check if tile exists in world map
    if world_map.get_tile(position).is_some() {
        DebugUtils::log_info(debug_logging, "Tile exists in world map.");
        selected_tile.position = Some(position);

        // Find the tile entity and show neighbor information
        if let Some((_, world_tile, neighbors)) = find_tile_entity(position, tile_query) {
            if debug_logging.0 {
                display_neighbor_info(world_tile, neighbors, tile_query, debug_logging);
            }
        }
    } else {
        DebugUtils::log_info(debug_logging, "No tile data found at this position.");
        selected_tile.position = None;
    }
}

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
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    match cursor_to_tile_position(cursor_pos, camera, camera_transform) {
        Ok(position) => {
            DebugUtils::log_tile_click(&debug_logging, position.x, position.y);
            process_tile_selection(
                position,
                &world_map,
                &tile_query,
                &mut selected_tile,
                &debug_logging,
            );
        }
        Err(error_msg) => {
            DebugUtils::log_info(&debug_logging, error_msg);
            selected_tile.position = None;
        }
    }
}
use crate::game::GameState;
use bevy_egui::EguiContexts;

/// Handle keyboard input for game controls
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    debug_logging: Res<DebugLogging>,
) {
    // Game controls
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        game_state.paused = !game_state.paused;
        DebugUtils::log_game_state_change(&debug_logging, "paused", game_state.paused);
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        game_state.auto_advance = !game_state.auto_advance;
        DebugUtils::log_game_state_change(&debug_logging, "auto-advance", game_state.auto_advance);
    }

    if keyboard_input.just_pressed(KeyCode::Equal)
        || keyboard_input.just_pressed(KeyCode::NumpadAdd)
    {
        game_state.simulation_speed = (game_state.simulation_speed * simulation::SPEED_MULTIPLIER).min(simulation::MAX_SPEED);
        let speed = game_state.simulation_speed;
        game_state
            .turn_timer
            .set_duration(std::time::Duration::from_secs_f32(simulation::BASE_TURN_DURATION / speed));
        DebugUtils::log_simulation_speed(&debug_logging, speed);
    }

    if keyboard_input.just_pressed(KeyCode::Minus)
        || keyboard_input.just_pressed(KeyCode::NumpadSubtract)
    {
        game_state.simulation_speed = (game_state.simulation_speed / simulation::SPEED_MULTIPLIER).max(simulation::MIN_SPEED);
        let speed = game_state.simulation_speed;
        game_state
            .turn_timer
            .set_duration(std::time::Duration::from_secs_f32(simulation::BASE_TURN_DURATION / speed));
        DebugUtils::log_simulation_speed(&debug_logging, speed);
    }

    // Camera controls
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        let mut movement = Vec3::ZERO;
        let camera_speed = camera::MOVEMENT_SPEED * time.delta_secs();

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
            camera_transform.scale *= camera::ZOOM_RATE + time.delta_secs();
            camera_transform.scale = camera_transform
                .scale
                .clamp(Vec3::splat(0.1), Vec3::splat(5.0));
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            camera_transform.scale *= camera::ZOOM_RATE - time.delta_secs();
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

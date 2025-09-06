use super::constants;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use core_sim::tile::tile_components::WorldTile;
// use bevy_hui::EguiContexts; // Commented out for now
use core_sim::{Position, TerrainType};

pub fn handle_mouse_input(
    mut mouse_wheel: EventReader<bevy::input::mouse::MouseWheel>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut last_cursor_pos: Local<Option<Vec2>>,
) {
    handle_camera_zoom_controls(&mut mouse_wheel, &mut camera_query);
    handle_camera_panning_controls(
        &mouse_button,
        &mut cursor_moved,
        &mut last_cursor_pos,
        &mut camera_query,
    );
}

fn handle_camera_zoom_controls(
    mouse_wheel: &mut EventReader<MouseWheel>,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
) {
    // Process all mouse wheel events for camera zoom
    for wheel_event in mouse_wheel.read() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            apply_camera_zoom_from_wheel_event(wheel_event, &mut camera_transform);
        }
    }
}

fn apply_camera_zoom_from_wheel_event(wheel_event: &MouseWheel, camera_transform: &mut Transform) {
    if wheel_event.y > 0.0 {
        camera_transform.scale *= 1.0 - constants::camera::ZOOM_STEP_SIZE;
    } else if wheel_event.y < 0.0 {
        camera_transform.scale *= 1.0 + constants::camera::ZOOM_STEP_SIZE;
    }
    camera_transform.scale = camera_transform.scale.clamp(
        Vec3::splat(constants::camera::MINIMUM_ZOOM_SCALE),
        Vec3::splat(constants::camera::MAXIMUM_ZOOM_SCALE),
    );
}

fn handle_camera_panning_controls(
    mouse_button: &Res<ButtonInput<MouseButton>>,
    cursor_moved: &mut EventReader<CursorMoved>,
    last_cursor_position: &mut Local<Option<Vec2>>,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        for cursor_event in cursor_moved.read() {
            if let Some(previous_position) = **last_cursor_position {
                let delta = cursor_event.position - previous_position;

                if let Ok(mut camera_transform) = camera_query.single_mut() {
                    let scale_x = camera_transform.scale.x;
                    camera_transform.translation += Vec3::new(-delta.x, delta.y, 0.0) * scale_x;
                }
            }
            **last_cursor_position = Some(cursor_event.position);
        }
    } else {
        for cursor_event in cursor_moved.read() {
            **last_cursor_position = Some(cursor_event.position);
        }
    }

    if mouse_button.just_released(MouseButton::Left) {
        **last_cursor_position = None;
    }
}

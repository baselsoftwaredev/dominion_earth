use bevy::prelude::*;
use core_sim::chunking::{ChunkComponent, ChunkId, ChunkManager};

pub mod constants {
    pub const CHUNK_POSITION_CHANGE_THRESHOLD: f32 = 64.0;
}

/// System to update chunk manager based on camera position
/// This tracks the camera and marks which chunks should be active
pub fn update_chunk_manager_from_camera(
    mut chunk_manager: ResMut<ChunkManager>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    // Get camera position
    let camera_pos = camera_query
        .iter()
        .next()
        .map(|t| (t.translation.x, t.translation.y));

    let Some(camera_pos) = camera_pos else {
        return;
    };

    // Update chunk manager with camera position
    let prev_pos = chunk_manager.last_camera_pos;
    chunk_manager.last_camera_pos = Some(camera_pos);

    // Skip if camera hasn't moved much (to avoid constant updates)
    if let Some(prev) = prev_pos {
        let dx = (camera_pos.0 - prev.0).abs();
        let dy = (camera_pos.1 - prev.1).abs();
        if dx < constants::CHUNK_POSITION_CHANGE_THRESHOLD
            && dy < constants::CHUNK_POSITION_CHANGE_THRESHOLD
        {
            return;
        }
    }

    // Calculate chunks that should be loaded around camera
    let chunks_in_radius = chunk_manager.get_chunks_in_radius(camera_pos);
    let chunks_in_radius_set: std::collections::HashSet<_> =
        chunks_in_radius.iter().copied().collect();

    // Unload chunks that are outside the radius
    let chunks_to_unload: Vec<ChunkId> = chunk_manager
        .loaded_chunks
        .iter()
        .copied()
        .filter(|chunk_id| !chunks_in_radius_set.contains(chunk_id))
        .collect();

    for chunk_id in chunks_to_unload {
        chunk_manager.mark_chunk_unloaded(chunk_id);
    }

    // Load chunks that should be in radius but aren't loaded yet
    for chunk_id in chunks_in_radius {
        if !chunk_manager.is_chunk_loaded(chunk_id) {
            // Use entity 0 as a placeholder - actual tile rendering happens elsewhere
            chunk_manager.mark_chunk_loaded(chunk_id, Entity::PLACEHOLDER);
        }
    }

    if should_log_chunk_position_change(prev_pos, camera_pos) {
        log_camera_chunk_position_change(camera_pos, &chunk_manager);
    }
}

fn should_log_chunk_position_change(prev_pos: Option<(f32, f32)>, camera_pos: (f32, f32)) -> bool {
    prev_pos.is_none()
        || prev_pos.map_or(false, |p| {
            let dx = (camera_pos.0 - p.0).abs();
            let dy = (camera_pos.1 - p.1).abs();
            dx >= constants::CHUNK_POSITION_CHANGE_THRESHOLD
                || dy >= constants::CHUNK_POSITION_CHANGE_THRESHOLD
        })
}

fn log_camera_chunk_position_change(camera_pos: (f32, f32), chunk_manager: &ChunkManager) {
    let current_chunk = ChunkId::from_position(
        core_sim::Position::new(camera_pos.0 as i32, camera_pos.1 as i32),
        chunk_manager.config.chunk_size,
    );
    println!(
        "📍 Camera moved to chunk {:?} (world pos: {:.0}, {:.0}) - {} chunks now active",
        current_chunk,
        camera_pos.0,
        camera_pos.1,
        chunk_manager.loaded_chunks.len()
    );
}

/// System to provide debug info about loaded chunks
/// Tracks and logs only when the number of active chunks changes
pub fn debug_chunk_info(mut chunk_manager: ResMut<ChunkManager>) {
    if chunk_manager.loaded_chunks.len() > 0 {
        if let Some(last_count) = chunk_manager.last_logged_chunk_count {
            if last_count != chunk_manager.loaded_chunks.len() {
                log_loaded_chunks_changed(
                    last_count,
                    chunk_manager.loaded_chunks.len(),
                    &chunk_manager,
                );
                chunk_manager.last_logged_chunk_count = Some(chunk_manager.loaded_chunks.len());
            }
        } else {
            log_loaded_chunks_initialized(chunk_manager.loaded_chunks.len(), &chunk_manager);
            chunk_manager.last_logged_chunk_count = Some(chunk_manager.loaded_chunks.len());
        }
    }
}

fn log_loaded_chunks_changed(
    previous_count: usize,
    current_count: usize,
    chunk_manager: &ChunkManager,
) {
    println!(
        "📊 Loaded chunks changed: {} → {} (config: {} size, {} radius)",
        previous_count,
        current_count,
        chunk_manager.config.chunk_size,
        chunk_manager.config.load_radius
    );
}

fn log_loaded_chunks_initialized(chunk_count: usize, chunk_manager: &ChunkManager) {
    println!(
        "📊 Loaded chunks: {} (config: {} size, {} radius)",
        chunk_count, chunk_manager.config.chunk_size, chunk_manager.config.load_radius
    );
}

// Unused placeholder functions removed

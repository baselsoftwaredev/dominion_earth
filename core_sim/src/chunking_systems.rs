use crate::chunking::{ChunkComponent, ChunkId, ChunkManager};
use bevy_ecs::prelude::*;

pub fn update_chunks_system(
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
    chunk_query: Query<(Entity, &ChunkComponent)>,
) {
    let Some(current_pos) = chunk_manager.last_camera_pos else {
        return;
    };

    let (to_load, to_unload) = chunk_manager.get_chunk_updates(current_pos);

    for chunk_id in to_unload {
        if let Some(entity) = chunk_manager.get_chunk_entity(chunk_id) {
            commands.entity(entity).despawn();
            chunk_manager.mark_chunk_unloaded(chunk_id);
        }
    }

    for _chunk_id in to_load {}
}

pub fn get_loaded_chunks(chunk_manager: &ChunkManager) -> Vec<ChunkId> {
    chunk_manager.loaded_chunks.iter().copied().collect()
}

pub fn is_position_in_loaded_chunk(chunk_manager: &ChunkManager, pos: (f32, f32)) -> bool {
    let chunk_id = ChunkId::from_position(
        crate::Position::new(pos.0 as i32, pos.1 as i32),
        chunk_manager.config.chunk_size,
    );
    chunk_manager.is_chunk_loaded(chunk_id)
}

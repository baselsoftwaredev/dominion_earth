use crate::{Position, WorldMap};
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub mod constants {
    pub const DEFAULT_CHUNK_SIZE_IN_TILES: u32 = 8;
    pub const DEFAULT_CHUNK_LOAD_RADIUS: u32 = 3;
    pub const DEFAULT_CHUNK_UNLOAD_DISTANCE: f32 = 1500.0;
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkConfig {
    pub chunk_size: u32,
    pub load_radius: u32,
    pub unload_distance: f32,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: constants::DEFAULT_CHUNK_SIZE_IN_TILES,
            load_radius: constants::DEFAULT_CHUNK_LOAD_RADIUS,
            unload_distance: constants::DEFAULT_CHUNK_UNLOAD_DISTANCE,
        }
    }
}

/// Unique identifier for a chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ChunkId {
    pub x: i32,
    pub y: i32,
}

impl ChunkId {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Calculate chunk ID from world position
    pub fn from_position(pos: Position, chunk_size: u32) -> Self {
        Self {
            x: pos.x / chunk_size as i32,
            y: pos.y / chunk_size as i32,
        }
    }

    pub fn center(&self, chunk_size: u32) -> (f32, f32) {
        let center_x = (self.x as f32 * chunk_size as f32) + (chunk_size as f32 / 2.0);
        let center_y = (self.y as f32 * chunk_size as f32) + (chunk_size as f32 / 2.0);
        (center_x, center_y)
    }

    pub fn distance_to(&self, pos: (f32, f32), chunk_size: u32) -> f32 {
        let (center_x, center_y) = self.center(chunk_size);
        let dx = center_x - pos.0;
        let dy = center_y - pos.1;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Data about a single chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    pub id: ChunkId,
    pub tiles: Vec<Vec<ChunkTile>>,
}

/// Minimal tile data stored in chunks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkTile {
    pub terrain: String,
    pub owner: Option<u32>,
}

/// Manages all active chunks
#[derive(Resource, Debug)]
pub struct ChunkManager {
    pub config: ChunkConfig,
    pub loaded_chunks: HashSet<ChunkId>,
    pub chunk_entities: std::collections::HashMap<ChunkId, Entity>,
    pub last_camera_pos: Option<(f32, f32)>,
    pub last_logged_chunk_count: Option<usize>,
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self {
            config: ChunkConfig::default(),
            loaded_chunks: HashSet::new(),
            chunk_entities: std::collections::HashMap::new(),
            last_camera_pos: None,
            last_logged_chunk_count: None,
        }
    }
}

impl ChunkManager {
    pub fn new(config: ChunkConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Get all chunk IDs that should be loaded around a position
    pub fn get_chunks_in_radius(&self, pos: (f32, f32)) -> Vec<ChunkId> {
        let center_chunk = ChunkId::from_position(
            Position::new(pos.0 as i32, pos.1 as i32),
            self.config.chunk_size,
        );

        let mut chunks = Vec::new();
        let radius = self.config.load_radius as i32;

        for x in (center_chunk.x - radius)..=(center_chunk.x + radius) {
            for y in (center_chunk.y - radius)..=(center_chunk.y + radius) {
                chunks.push(ChunkId::new(x, y));
            }
        }

        chunks
    }

    pub fn get_chunk_updates(&self, pos: (f32, f32)) -> (Vec<ChunkId>, Vec<ChunkId>) {
        let chunks_to_load = self.get_chunks_in_radius(pos);
        let chunks_to_load_set: HashSet<_> = chunks_to_load.iter().copied().collect();

        let to_unload: Vec<ChunkId> = self
            .loaded_chunks
            .iter()
            .copied()
            .filter(|chunk_id| !chunks_to_load_set.contains(chunk_id))
            .collect();

        let to_load: Vec<ChunkId> = chunks_to_load
            .into_iter()
            .filter(|chunk_id| !self.loaded_chunks.contains(chunk_id))
            .collect();

        (to_load, to_unload)
    }

    pub fn mark_chunk_loaded(&mut self, chunk_id: ChunkId, entity: Entity) {
        self.loaded_chunks.insert(chunk_id);
        self.chunk_entities.insert(chunk_id, entity);
    }

    pub fn mark_chunk_unloaded(&mut self, chunk_id: ChunkId) {
        self.loaded_chunks.remove(&chunk_id);
        self.chunk_entities.remove(&chunk_id);
    }

    pub fn is_chunk_loaded(&self, chunk_id: ChunkId) -> bool {
        self.loaded_chunks.contains(&chunk_id)
    }

    pub fn get_chunk_entity(&self, chunk_id: ChunkId) -> Option<Entity> {
        self.chunk_entities.get(&chunk_id).copied()
    }
}

/// Component to mark an entity as a chunk entity
#[derive(Component, Debug, Clone, Copy)]
pub struct ChunkComponent {
    pub chunk_id: ChunkId,
}

/// Helper function to extract tiles from WorldMap for a chunk
pub fn extract_chunk_from_world(
    world_map: &WorldMap,
    chunk_id: ChunkId,
    chunk_size: u32,
) -> ChunkData {
    let mut tiles = Vec::new();

    let start_x = chunk_id.x as u32 * chunk_size;
    let start_y = chunk_id.y as u32 * chunk_size;

    for x in start_x..(start_x + chunk_size) {
        let mut column = Vec::new();
        for y in start_y..(start_y + chunk_size) {
            let pos = Position::new(x as i32, y as i32);
            let chunk_tile = create_chunk_tile_from_world_map(world_map, pos);
            column.push(chunk_tile);
        }
        tiles.push(column);
    }

    ChunkData {
        id: chunk_id,
        tiles,
    }
}

fn create_chunk_tile_from_world_map(world_map: &WorldMap, pos: Position) -> ChunkTile {
    if let Some(tile) = world_map.get_tile(pos) {
        ChunkTile {
            terrain: format!("{:?}", tile.terrain),
            owner: tile.owner.map(|id| id.0),
        }
    } else {
        ChunkTile {
            terrain: "Unknown".to_string(),
            owner: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_id_from_position() {
        let pos = Position::new(15, 15);
        let chunk_id = ChunkId::from_position(pos, 8);
        assert_eq!(chunk_id, ChunkId::new(1, 1));
    }

    #[test]
    fn test_chunk_id_center() {
        let chunk_id = ChunkId::new(0, 0);
        let (cx, cy) = chunk_id.center(8);
        assert_eq!(cx, 4.0);
        assert_eq!(cy, 4.0);
    }

    #[test]
    fn test_chunks_in_radius() {
        let manager = ChunkManager::new(ChunkConfig {
            chunk_size: constants::DEFAULT_CHUNK_SIZE_IN_TILES,
            load_radius: 1,
            unload_distance: constants::DEFAULT_CHUNK_UNLOAD_DISTANCE,
        });

        let chunks = manager.get_chunks_in_radius((4.0, 4.0));
        assert_eq!(chunks.len(), 9);
    }
}

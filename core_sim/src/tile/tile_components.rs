#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultViewPoint {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
/// System to update tile asset index when terrain changes
pub fn update_tile_asset_on_terrain_change(
    mut events: EventReader<TileTerrainChanged>,
    mut query: Query<(&mut TileTextureIndex, &mut WorldTile)>,
    tile_assets: Res<TileAssets>,
) {
    for event in events.read() {
        if let Ok((mut texture_index, mut world_tile)) = query.get_mut(event.entity) {
            let new_index = tile_assets.get_index_for_terrain(&event.new_terrain);
            texture_index.0 = new_index;
            world_tile.terrain_type = event.new_terrain.clone();
        }
    }
}
use bevy::prelude::*;

/// Event emitted when a tile's terrain type changes
#[derive(Debug, Clone, Event)]
pub struct TileTerrainChanged {
    pub entity: Entity,
    pub new_terrain: TerrainType,
}
use crate::resources::WorldMap;
use crate::tile::tile_assets::TileAssets;
use crate::tile::tile_passes::{
    assign_tile_neighbors_pass, spawn_world_tiles_pass, update_coast_tiles_pass,
};
use bevy::prelude::{Component, Entity};
use bevy_ecs_tilemap::prelude::*;

/// System to setup world tiles, assign terrain, and link neighbors
pub fn setup_world_tiles(
    commands: &mut Commands,
    tilemap_id: TilemapId,
    tile_assets: &impl TileAssetProvider,
    world_map: &mut WorldMap,
) -> TileStorage {
    let map_size = TilemapSize {
        x: world_map.width,
        y: world_map.height,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let mut tile_entities =
        vec![vec![Entity::PLACEHOLDER; map_size.y as usize]; map_size.x as usize];
    let mut terrain_types =
        vec![vec![TerrainType::Ocean; map_size.y as usize]; map_size.x as usize];

    spawn_world_tiles_pass(
        commands,
        tilemap_id,
        tile_assets,
        world_map,
        &mut tile_storage,
        &mut tile_entities,
        &mut terrain_types,
    );
    assign_tile_neighbors_pass(commands, &tile_entities, &map_size);
    update_coast_tiles_pass(
        commands,
        tile_assets,
        &tile_entities,
        &mut terrain_types,
        &map_size,
        world_map,
    );

    tile_storage
}

/// Trait to abstract asset index lookup for core_sim
pub trait TileAssetProvider {
    fn get_index_for_terrain(&self, terrain: &TerrainType) -> u32;
    fn get_coast_index(&self) -> u32;
}
use crate::{CivId, Position, TerrainType};

#[derive(Component)]
pub struct WorldTile {
    pub grid_pos: Position,
    pub terrain_type: TerrainType,
}

#[derive(Component, Clone)]
pub struct TileNeighbors {
    pub north: Option<Entity>,
    pub south: Option<Entity>,
    pub east: Option<Entity>,
    pub west: Option<Entity>,
}

#[derive(Component)]
pub struct UnitSprite {
    pub unit_entity: Entity,
}

#[derive(Component)]
pub struct CapitalSprite {
    pub civ_id: CivId,
}

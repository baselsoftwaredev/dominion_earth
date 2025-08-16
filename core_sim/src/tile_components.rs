use bevy_ecs_tilemap::prelude::*;
use crate::resources::WorldMap;
use bevy::{
    ecs::{
        entity::{EntityMapper, MapEntities},
        reflect::ReflectMapEntities,
    },
    math::{UVec2, Vec2},
    prelude::{
        Component, Deref, DerefMut, Entity, Handle, Image, Reflect, ReflectComponent, Res, ResMut,
    },
    render::{
        render_resource::TextureUsages,
        view::{VisibilityClass, add_visibility_class},
    },
};


/// System to setup world tiles, assign terrain, and link neighbors
pub fn setup_world_tiles(
    commands: &mut Commands,
    tilemap_id: TilemapId,
    tile_assets: &impl TileAssetProvider,
    world_map: &WorldMap,
) -> TileStorage {
    let map_size = TilemapSize {
        x: world_map.width,
        y: world_map.height,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let mut tile_entities = vec![vec![Entity::PLACEHOLDER; map_size.y as usize]; map_size.x as usize];
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let world_pos = Position::new(x as i32, y as i32);

            let terrain_type = world_map
                .get_tile(world_pos)
                .map(|t| &t.terrain)
                .unwrap_or(&TerrainType::Ocean);

            let mut texture_index = tile_assets.get_index_for_terrain(terrain_type);

            // Coast detection (South-facing coast, index 8)
            if !matches!(terrain_type, TerrainType::Ocean | TerrainType::Coast) {
                let south = Position::new(x as i32, y as i32 + 1);
                let left = Position::new(x as i32 - 1, y as i32);
                let right = Position::new(x as i32 + 1, y as i32);
                let north = Position::new(x as i32, y as i32 - 1);

                let south_is_ocean = world_map.get_tile(south).map_or(false, |t| matches!(t.terrain, TerrainType::Ocean));
                let left_is_ocean = world_map.get_tile(left).map_or(false, |t| matches!(t.terrain, TerrainType::Ocean));
                let right_is_ocean = world_map.get_tile(right).map_or(false, |t| matches!(t.terrain, TerrainType::Ocean));
                let north_is_land = world_map.get_tile(north).map_or(false, |t| !matches!(t.terrain, TerrainType::Ocean | TerrainType::Coast));

                if south_is_ocean && left_is_ocean && right_is_ocean && north_is_land {
                    texture_index = tile_assets.get_coast_index();
                }
            }

            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index: TileTextureIndex(texture_index),
                        ..Default::default()
                    },
                    WorldTile {
                        grid_pos: world_pos,
                        terrain_type: terrain_type.clone(),
                    },
                ))
                .id();

            tile_entities[x as usize][y as usize] = tile_entity;
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Second pass: add TileNeighbors component to each tile
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_entity = tile_entities[x as usize][y as usize];
            let north = if y > 0 { Some(tile_entities[x as usize][(y - 1) as usize]) } else { None };
            let south = if (y + 1) < map_size.y { Some(tile_entities[x as usize][(y + 1) as usize]) } else { None };
            let east = if (x + 1) < map_size.x { Some(tile_entities[(x + 1) as usize][y as usize]) } else { None };
            let west = if x > 0 { Some(tile_entities[(x - 1) as usize][y as usize]) } else { None };
            commands.entity(tile_entity).insert(TileNeighbors {
                north,
                south,
                east,
                west,
            });
        }
    }

    tile_storage
}

/// Trait to abstract asset index lookup for core_sim
pub trait TileAssetProvider {
    fn get_index_for_terrain(&self, terrain: &TerrainType) -> u32;
    fn get_coast_index(&self) -> u32;
}
use bevy_ecs::prelude::*;
use crate::{Position, TerrainType, CivId};

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

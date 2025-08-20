use crate::tile::tile_components::{DefaultViewPoint, TileAssetProvider, TileNeighbors, WorldTile};
use crate::{Position, TerrainType};
use bevy::prelude::{Commands, Entity, Transform};
use bevy_ecs_tilemap::prelude::*;

/// First pass: spawn tiles, assign terrain, and store entities/terrain
pub fn spawn_world_tiles_pass(
    commands: &mut Commands,
    tilemap_id: TilemapId,
    tile_assets: &impl TileAssetProvider,
    world_map: &crate::resources::WorldMap,
    tile_storage: &mut TileStorage,
    tile_entities: &mut Vec<Vec<Entity>>,
    terrain_types: &mut Vec<Vec<TerrainType>>,
) {
    let map_size = TilemapSize {
        x: world_map.width,
        y: world_map.height,
    };
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let world_pos = Position::new(x as i32, y as i32);
            let terrain_type = world_map
                .get_tile(world_pos)
                .map(|t| t.terrain.clone())
                .unwrap_or(TerrainType::Ocean);
            terrain_types[x as usize][y as usize] = terrain_type.clone();
            let texture_index = tile_assets.get_index_for_terrain(&terrain_type);
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
                        default_view_point: DefaultViewPoint::North,
                    },
                    Transform::default(), // Add Transform component for rotation system
                ))
                .id();
            tile_entities[x as usize][y as usize] = tile_entity;
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}

/// Second pass: assign TileNeighbors to each tile
pub fn assign_tile_neighbors_pass(
    commands: &mut Commands,
    tile_entities: &Vec<Vec<Entity>>,
    map_size: &TilemapSize,
) {
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_entity = tile_entities[x as usize][y as usize];
            let north = if (y + 1) < map_size.y {
                Some(tile_entities[x as usize][(y + 1) as usize])
            } else {
                None
            };
            let south = if y > 0 {
                Some(tile_entities[x as usize][(y - 1) as usize])
            } else {
                None
            };
            let east = if (x + 1) < map_size.x {
                Some(tile_entities[(x + 1) as usize][y as usize])
            } else {
                None
            };
            let west = if x > 0 {
                Some(tile_entities[(x - 1) as usize][y as usize])
            } else {
                None
            };
            commands.entity(tile_entity).insert(TileNeighbors {
                north,
                south,
                east,
                west,
            });
        }
    }
}

/// Third pass: Check land tiles for ocean neighbors and assign specific coast tile indices
pub fn update_coast_tiles_pass(
    commands: &mut Commands,
    tile_assets: &impl TileAssetProvider,
    tile_entities: &Vec<Vec<Entity>>,
    terrain_types: &mut Vec<Vec<TerrainType>>,
    map_size: &TilemapSize,
    world_map: &mut crate::resources::WorldMap,
) {
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_entity = tile_entities[x as usize][y as usize];
            let terrain = &terrain_types[x as usize][y as usize];

            // Only process land tiles (not ocean or existing coast)
            if !matches!(terrain, TerrainType::Ocean | TerrainType::Coast) {
                // Check which sides have ocean neighbors
                let has_north_ocean = (y + 1) < map_size.y
                    && terrain_types[x as usize][(y + 1) as usize] == TerrainType::Ocean;
                let has_south_ocean =
                    y > 0 && terrain_types[x as usize][(y - 1) as usize] == TerrainType::Ocean;
                let has_east_ocean = (x + 1) < map_size.x
                    && terrain_types[(x + 1) as usize][y as usize] == TerrainType::Ocean;
                let has_west_ocean =
                    x > 0 && terrain_types[(x - 1) as usize][y as usize] == TerrainType::Ocean;

                // Determine tile index based on ocean neighbor pattern
                let (tile_index, should_convert_to_coast) = match (
                    has_north_ocean,
                    has_east_ocean,
                    has_south_ocean,
                    has_west_ocean,
                ) {
                    // Ocean on all sides: tile index 2
                    (true, true, true, true) => (2, true),
                    // North, east, and south ocean: tile index 1
                    (true, true, true, false) => (1, true),
                    // East and south ocean: tile index 9
                    (false, true, true, false) => (9, true),
                    // Only south ocean: tile index 8
                    (false, false, true, false) => (8, true),
                    // Any other ocean adjacency - use default coast index
                    (n, e, s, w) if n || e || s || w => (tile_assets.get_coast_index(), true),
                    // No ocean neighbors - keep as land
                    _ => (tile_assets.get_index_for_terrain(terrain), false),
                };

                if should_convert_to_coast {
                    println!(
                        "Converting land tile at ({}, {}) from {:?} to Coast with tile index {}. Ocean neighbors - North: {}, East: {}, South: {}, West: {}",
                        x, y, terrain, tile_index, has_north_ocean, has_east_ocean, has_south_ocean, has_west_ocean
                    );

                    // Determine view point based on ocean neighbors
                    let view_point = if has_south_ocean {
                        DefaultViewPoint::South
                    } else if has_north_ocean {
                        DefaultViewPoint::North
                    } else if has_east_ocean {
                        DefaultViewPoint::East
                    } else if has_west_ocean {
                        DefaultViewPoint::West
                    } else {
                        DefaultViewPoint::North
                    };

                    // Update tile to coast with the specific tile index
                    commands
                        .entity(tile_entity)
                        .insert(TileTextureIndex(tile_index))
                        .insert(WorldTile {
                            grid_pos: Position::new(x as i32, y as i32),
                            terrain_type: TerrainType::Coast,
                            default_view_point: view_point,
                        });

                    // Update the terrain_types array to keep it synchronized with ECS
                    terrain_types[x as usize][y as usize] = TerrainType::Coast;

                    // Update the WorldMap resource to keep UI in sync
                    let world_pos = Position::new(x as i32, y as i32);
                    if let Some(map_tile) = world_map.get_tile_mut(world_pos) {
                        map_tile.terrain = TerrainType::Coast;
                    }
                }
            }
        }
    }
}

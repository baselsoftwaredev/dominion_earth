/// Pass: Remove internal ocean tiles (lakes) by converting any ocean tile surrounded by land to land
pub fn remove_internal_oceans_pass(
    terrain_types: &mut Vec<Vec<TerrainType>>,
    map_size: &TilemapSize,
) {
    // Copy of terrain_types to avoid modifying while iterating
    let original = terrain_types.clone();
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            if original[x as usize][y as usize] == TerrainType::Ocean {
                let mut land_neighbors = 0;
                // Check north
                if y > 0 && original[x as usize][(y - 1) as usize] != TerrainType::Ocean {
                    land_neighbors += 1;
                }
                // Check south
                if (y + 1) < map_size.y
                    && original[x as usize][(y + 1) as usize] != TerrainType::Ocean
                {
                    land_neighbors += 1;
                }
                // Check east
                if (x + 1) < map_size.x
                    && original[(x + 1) as usize][y as usize] != TerrainType::Ocean
                {
                    land_neighbors += 1;
                }
                // Check west
                if x > 0 && original[(x - 1) as usize][y as usize] != TerrainType::Ocean {
                    land_neighbors += 1;
                }
                // If all 4 neighbors are land, convert to land
                if land_neighbors == 4 {
                    terrain_types[x as usize][y as usize] = TerrainType::Plains;
                    // Default to Plains
                }
            }
        }
    }
}
use crate::tile::tile_components::{TileAssetProvider, TileNeighbors, WorldTile};
use crate::{Position, TerrainType};
use bevy::prelude::{Commands, Entity};
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
                    },
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
            let north = if y > 0 {
                Some(tile_entities[x as usize][(y - 1) as usize])
            } else {
                None
            };
            let south = if (y + 1) < map_size.y {
                Some(tile_entities[x as usize][(y + 1) as usize])
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

/// Third pass: update coast status using terrain_types array
pub fn update_coast_tiles_pass(
    commands: &mut Commands,
    tile_assets: &impl TileAssetProvider,
    tile_entities: &Vec<Vec<Entity>>,
    terrain_types: &Vec<Vec<TerrainType>>,
    map_size: &TilemapSize,
) {
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_entity = tile_entities[x as usize][y as usize];
            let terrain = &terrain_types[x as usize][y as usize];
            if !matches!(terrain, TerrainType::Ocean | TerrainType::Coast) {
                let neighbors = [
                    if y > 0 {
                        Some((x as usize, (y - 1) as usize))
                    } else {
                        None
                    },
                    if (y + 1) < map_size.y {
                        Some((x as usize, (y + 1) as usize))
                    } else {
                        None
                    },
                    if (x + 1) < map_size.x {
                        Some(((x + 1) as usize, y as usize))
                    } else {
                        None
                    },
                    if x > 0 {
                        Some(((x - 1) as usize, y as usize))
                    } else {
                        None
                    },
                ];
                let mut is_adjacent_to_ocean = false;
                for neighbor in neighbors.iter().flatten() {
                    let neighbor_terrain = &terrain_types[neighbor.0][neighbor.1];
                    if matches!(neighbor_terrain, TerrainType::Ocean) {
                        is_adjacent_to_ocean = true;
                        break;
                    }
                }
                if is_adjacent_to_ocean {
                    // Update the tile's texture index to coast
                    commands
                        .entity(tile_entity)
                        .insert(TileTextureIndex(tile_assets.get_coast_index()));
                }
            }
        }
    }
}

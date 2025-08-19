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
use crate::tile::tile_components::{DefaultViewPoint, TileAssetProvider, TileNeighbors, WorldTile};
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
                        default_view_point: DefaultViewPoint::North,
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
    terrain_types: &mut Vec<Vec<TerrainType>>,
    map_size: &TilemapSize,
    world_map: &mut crate::resources::WorldMap,
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
                    // Print all neighbors and their tile types for debugging
                    let north_terrain = if y > 0 {
                        format!("{:?}", terrain_types[x as usize][(y - 1) as usize])
                    } else {
                        "OutOfBounds".to_string()
                    };
                    let south_terrain = if (y + 1) < map_size.y {
                        format!("{:?}", terrain_types[x as usize][(y + 1) as usize])
                    } else {
                        "OutOfBounds".to_string()
                    };
                    let east_terrain = if (x + 1) < map_size.x {
                        format!("{:?}", terrain_types[(x + 1) as usize][y as usize])
                    } else {
                        "OutOfBounds".to_string()
                    };
                    let west_terrain = if x > 0 {
                        format!("{:?}", terrain_types[(x - 1) as usize][y as usize])
                    } else {
                        "OutOfBounds".to_string()
                    };

                    println!(
                        "Converting tile at ({}, {}) from {:?} to Coast. Neighbors - North: {}, South: {}, East: {}, West: {}",
                        x, y, terrain, north_terrain, south_terrain, east_terrain, west_terrain
                    );

                    // Determine coast facing direction based on land neighbors
                    let mut land_neighbors = vec![];
                    // Check North neighbor (y-1) - if land, coast faces North
                    if y > 0
                        && !matches!(
                            terrain_types[x as usize][(y - 1) as usize],
                            TerrainType::Ocean | TerrainType::Coast
                        )
                    {
                        land_neighbors.push(DefaultViewPoint::North);
                    }
                    // Check South neighbor (y+1) - if land, coast faces South
                    if (y + 1) < map_size.y
                        && !matches!(
                            terrain_types[x as usize][(y + 1) as usize],
                            TerrainType::Ocean | TerrainType::Coast
                        )
                    {
                        land_neighbors.push(DefaultViewPoint::South);
                    }
                    // Check East neighbor (x+1) - if land, coast faces East
                    if (x + 1) < map_size.x
                        && !matches!(
                            terrain_types[(x + 1) as usize][y as usize],
                            TerrainType::Ocean | TerrainType::Coast
                        )
                    {
                        land_neighbors.push(DefaultViewPoint::East);
                    }
                    // Check West neighbor (x-1) - if land, coast faces West
                    if x > 0
                        && !matches!(
                            terrain_types[(x - 1) as usize][y as usize],
                            TerrainType::Ocean | TerrainType::Coast
                        )
                    {
                        land_neighbors.push(DefaultViewPoint::West);
                    }

                    let view_point = match land_neighbors.as_slice() {
                        [dir] => *dir, // exactly one land neighbor
                        dirs if dirs.contains(&DefaultViewPoint::South) => DefaultViewPoint::South,
                        dirs if dirs.contains(&DefaultViewPoint::North) => DefaultViewPoint::North,
                        dirs if dirs.contains(&DefaultViewPoint::East) => DefaultViewPoint::East,
                        dirs if dirs.contains(&DefaultViewPoint::West) => DefaultViewPoint::West,
                        _ => DefaultViewPoint::North,
                    };
                    println!(
                        "Coast tile at ({}, {}) land_neighbors: {:?} -> view_point: {:?}",
                        x, y, land_neighbors, view_point
                    );

                    // Update both the tile's texture index and terrain type to coast, with facing
                    commands
                        .entity(tile_entity)
                        .insert(TileTextureIndex(tile_assets.get_coast_index()))
                        .insert(WorldTile {
                            grid_pos: Position::new(x as i32, y as i32),
                            terrain_type: TerrainType::Coast,
                            default_view_point: view_point,
                        });

                    // IMPORTANT: Also update the terrain_types array to keep it synchronized with ECS
                    terrain_types[x as usize][y as usize] = TerrainType::Coast;

                    // CRUCIAL: Also update the WorldMap resource to keep UI in sync
                    let world_pos = Position::new(x as i32, y as i32);
                    if let Some(map_tile) = world_map.get_tile_mut(world_pos) {
                        map_tile.terrain = TerrainType::Coast;
                    }
                }
            }
        }
    }
}

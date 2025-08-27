use core_sim::tile::tile_components::TileNeighbors;
use core_sim::tile::tile_components::{DefaultViewPoint, WorldTile};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::tile::tile_assets::TileAssets;
use core_sim::*;

#[derive(Resource, Clone)]
pub struct TilemapIdResource(pub TilemapId);

/// Setup isometric diamond tilemap using bevy_ecs_tilemap
pub fn setup_tilemap(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    mut world_map: ResMut<WorldMap>,
) {
    // Create tilemap entity early - we need its ID for tile references
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    // Store the tilemap ID as a resource for other systems to access
    commands.insert_resource(TilemapIdResource(tilemap_id));

    // Use core_sim's setup_world_tiles to create tile entities and neighbors
    let tile_storage = core_sim::tile::tile_components::setup_world_tiles(
        &mut commands,
        tilemap_id,
        &*tile_assets,
        &mut *world_map,
    );

    // Configure tilemap for square rendering (can switch to isometric later)
    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = TilemapGridSize { x: 64.0, y: 64.0 };
    let map_type = TilemapType::Square;

    // Add the tilemap bundle to the tilemap entity
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: TilemapSize {
            x: world_map.width,
            y: world_map.height,
        },
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_assets.sprite_sheet.clone()),
        tile_size,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

/// Spawn a unit/capital/city as a child of the correct tile entity in the tilemap
/// Spawn a capital/unit/building at world coordinates aligned with tilemap positioning
pub fn spawn_entity_on_tile(
    commands: &mut Commands,
    tile_assets: &TileAssets,
    tile_storage: &TileStorage,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
    position: Position,
    sprite_index: usize,
) -> Option<Entity> {
    // Convert game position to tilemap position to verify tile exists
    let tile_pos = TilePos {
        x: position.x as u32,
        y: position.y as u32,
    };

    // Verify the tile exists in storage
    if let Some(_tile_entity) = tile_storage.get(&tile_pos) {
        // Calculate world position properly using bevy_ecs_tilemap's coordinate system
        let tile_center =
            tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
        let world_pos = tile_center.extend(10.0); // Higher z-coordinate to render on top of tiles

        // Spawn the sprite at the calculated world coordinates
        let sprite_entity = commands
            .spawn((
                Sprite::from_atlas_image(
                    tile_assets.sprite_sheet.clone(),
                    TextureAtlas {
                        layout: tile_assets.texture_atlas_layout.clone(),
                        index: sprite_index,
                    },
                ),
                Transform::from_translation(world_pos),
            ))
            .id();

        println!("DEBUG: Spawned sprite at world coords ({:.2}, {:.2}, {:.2}) for tile position ({}, {}), sprite index: {}", 
                 world_pos.x, world_pos.y, world_pos.z, position.x, position.y, sprite_index);

        Some(sprite_entity)
    } else {
        eprintln!("Warning: Could not find tile at position {:?}", position);
        None
    }
}

/// System to spawn all world tiles using the new tilemap setup
pub fn spawn_world_tiles(
    commands: Commands,
    tile_assets: Res<TileAssets>,
    world_map: ResMut<WorldMap>,
) {
    setup_tilemap(commands, tile_assets, world_map);
}

/// System to spawn all unit sprites on their respective tiles
pub fn spawn_unit_sprites(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    units: Query<(Entity, &Position), With<core_sim::components::MilitaryUnit>>,
    tilemap_q: Query<
        (
            &TileStorage,
            &TilemapSize,
            &TilemapTileSize,
            &TilemapGridSize,
            &TilemapType,
            &TilemapAnchor,
        ),
        With<TilemapId>,
    >,
) {
    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    for (_, position) in units.iter() {
        spawn_entity_on_tile(
            &mut commands,
            &tile_assets,
            tile_storage,
            map_size,
            tile_size,
            grid_size,
            map_type,
            anchor,
            *position,
            tile_assets.ancient_infantry_index,
        );
    }
}

/// System to spawn all capital sprites on their respective tiles
pub fn spawn_capital_sprites(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    capitals: Query<(
        &core_sim::components::Capital,
        &core_sim::components::Position,
    )>,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
) {
    let capital_count = capitals.iter().count();
    println!(
        "DEBUG: spawn_capital_sprites called with {} capitals",
        capital_count
    );

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        println!("DEBUG: Could not get tilemap components");
        return;
    };

    println!(
        "DEBUG: Got tilemap components, map size: {}x{}",
        map_size.x, map_size.y
    );

    for (capital, pos) in capitals.iter() {
        println!(
            "DEBUG: Processing capital at position ({}, {}) with sprite index {}",
            pos.x, pos.y, capital.sprite_index
        );
        spawn_entity_on_tile(
            &mut commands,
            &tile_assets,
            tile_storage,
            map_size,
            tile_size,
            grid_size,
            map_type,
            anchor,
            *pos,
            capital.sprite_index as usize,
        );
    }
}

/// System to update unit sprites (stub for future logic)
/// Optimized to only run when units actually change
pub fn update_unit_sprites(
    // Only query for units that have changed position or other components
    _units: Query<(), (With<core_sim::MilitaryUnit>, Changed<core_sim::Position>)>,
) {
    // Only process if there are actually changed units
    // Currently empty implementation - will be filled when unit movement is implemented
}

/// System to update capital sprites when they evolve
pub fn update_capital_sprites(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    capitals: Query<
        (
            &core_sim::components::Capital,
            &core_sim::components::Position,
        ),
        Changed<core_sim::components::Capital>,
    >,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
) {
    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    for (capital, pos) in capitals.iter() {
        // Remove old sprite and spawn new one with updated sprite index
        // TODO: This is a simple approach - could be optimized to just update the texture index
        spawn_entity_on_tile(
            &mut commands,
            &tile_assets,
            tile_storage,
            map_size,
            tile_size,
            grid_size,
            map_type,
            anchor,
            *pos,
            capital.sprite_index as usize,
        );
    }
}

// System to render overlays (stub for future logic)
// pub fn render_world_overlays() {
//     // Implement logic to render overlays if needed
// }

// Example usage for units, capitals, cities:
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, unit_assets.ancient_infantry.clone(), unit_position, 1.0);
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, tile_assets.capital_ancient.clone(), capital_position, 2.0);
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, city_asset, city_position, 3.0);

// /// Generate a unique color for each civilization based on their ID
// fn get_civ_color(civ_id: &CivId) -> Color {
//     // Simple hash-based color generation for consistent colors per civilization
//     let hash = civ_id.0.wrapping_mul(31);

//     // Convert hash to HSV for better color distribution
//     let hue = (hash % 360) as f32;
//     let saturation = 0.7;
//     let value = 0.9;

//     // Convert HSV to RGB
//     let c = value * saturation;
//     let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
//     let m = value - c;

//     let (r, g, b) = if hue < 60.0 {
//         (c, x, 0.0)
//     } else if hue < 120.0 {
//         (x, c, 0.0)
//     } else if hue < 180.0 {
//         (0.0, c, x)
//     } else if hue < 240.0 {
//         (0.0, x, c)
//     } else if hue < 300.0 {
//         (x, 0.0, c)
//     } else {
//         (c, 0.0, x)
//     };

//     Color::srgb(r + m, g + m, b + m)
// }

// Final pass to ensure all coast viewpoints are correctly assigned after all setup is complete
// TODO: Replace with TileFlip-based system when default_view_point is removed
/*
pub fn finalize_coast_viewpoints(
    mut commands: Commands,
    tile_query: Query<(Entity, &WorldTile, &TileNeighbors)>,
    mut world_map: ResMut<core_sim::resources::WorldMap>,
    debug_logging: Res<crate::ui::DebugLogging>,
) {
    if debug_logging.0 {
        println!("=== FINALIZING COAST VIEWPOINTS ===");
    }

    // Collect all coast tiles that need viewpoint updates
    let mut updates = Vec::new();

    for (entity, world_tile, neighbors) in tile_query.iter() {
        if world_tile.terrain_type == core_sim::TerrainType::Coast {
            // Re-calculate viewpoint based on current neighbors
            // Coast tiles should face toward the land (not toward ocean)

            // Check all neighbors to find land directions
            let mut land_directions = Vec::new();
            let mut neighbor_debug = Vec::new();

            if let Some(north_entity) = neighbors.north {
                if let Ok((_, neighbor_tile, _)) = tile_query.get(north_entity) {
                    neighbor_debug.push(format!("North: {:?}", neighbor_tile.terrain_type));
                    if !matches!(neighbor_tile.terrain_type, core_sim::TerrainType::Ocean) {
                        land_directions.push("North");
                    }
                }
            } else {
                neighbor_debug.push("North: OutOfBounds".to_string());
            }

            if let Some(south_entity) = neighbors.south {
                if let Ok((_, neighbor_tile, _)) = tile_query.get(south_entity) {
                    neighbor_debug.push(format!("South: {:?}", neighbor_tile.terrain_type));
                    if !matches!(neighbor_tile.terrain_type, core_sim::TerrainType::Ocean) {
                        land_directions.push("South");
                    }
                }
            } else {
                neighbor_debug.push("South: OutOfBounds".to_string());
            }

            if let Some(east_entity) = neighbors.east {
                if let Ok((_, neighbor_tile, _)) = tile_query.get(east_entity) {
                    neighbor_debug.push(format!("East: {:?}", neighbor_tile.terrain_type));
                    if !matches!(neighbor_tile.terrain_type, core_sim::TerrainType::Ocean) {
                        land_directions.push("East");
                    }
                }
            } else {
                neighbor_debug.push("East: OutOfBounds".to_string());
            }

            if let Some(west_entity) = neighbors.west {
                if let Ok((_, neighbor_tile, _)) = tile_query.get(west_entity) {
                    neighbor_debug.push(format!("West: {:?}", neighbor_tile.terrain_type));
                    if !matches!(neighbor_tile.terrain_type, core_sim::TerrainType::Ocean) {
                        land_directions.push("West");
                    }
                }
            } else {
                neighbor_debug.push("West: OutOfBounds".to_string());
            }

            // Assign viewpoint to face toward the primary land direction
            let new_viewpoint = if land_directions.contains(&"South") {
                DefaultViewPoint::South
            } else if land_directions.contains(&"North") {
                DefaultViewPoint::North
            } else if land_directions.contains(&"East") {
                DefaultViewPoint::East
            } else if land_directions.contains(&"West") {
                DefaultViewPoint::West
            } else {
                // If no land neighbors (surrounded by ocean), default to North
                DefaultViewPoint::North
            };

            // Only schedule update if viewpoint has changed
            if world_tile.default_view_point != new_viewpoint {
                if debug_logging.0 {
                    println!(
                        "UPDATING coast tile at ({}, {}) viewpoint from {:?} to {:?}",
                        world_tile.grid_pos.x,
                        world_tile.grid_pos.y,
                        world_tile.default_view_point,
                        new_viewpoint
                    );
                    println!("  Neighbors: {}", neighbor_debug.join(", "));
                    println!("  Land directions: {:?}", land_directions);
                }
                updates.push((
                    entity,
                    new_viewpoint,
                    world_tile.grid_pos,
                    world_tile.terrain_type.clone(),
                ));
            }
        }
    }

    // Apply all updates
    for (entity, new_viewpoint, grid_pos, terrain_type) in updates {
        // Update ECS WorldTile entity
        commands.entity(entity).insert(WorldTile {
            grid_pos,
            terrain_type: terrain_type.clone(),
            default_view_point: new_viewpoint,
        });

        // Also update the WorldMap resource to keep it synchronized
        if let Some(map_tile) = world_map.get_tile_mut(grid_pos) {
            map_tile.terrain = terrain_type;
        }
    }

    if debug_logging.0 {
        println!("=== COAST VIEWPOINT FINALIZATION COMPLETE ===");
    }
}
*/

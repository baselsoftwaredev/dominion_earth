use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::*;
mod tile_assets;
pub use tile_assets::{setup_tile_assets, TileAssets};
pub use core_sim::tile_components::{WorldTile, TileNeighbors, UnitSprite, CapitalSprite};

#[derive(Resource, Clone)]
pub struct TilemapIdResource(pub TilemapId);

/// Setup isometric diamond tilemap using bevy_ecs_tilemap
pub fn setup_tilemap(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    world_map: Res<WorldMap>,
) {
    let map_size = TilemapSize {
        x: world_map.width,
        y: world_map.height,
    };

    // Create tilemap entity early - we need its ID for tile references
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    // Store the tilemap ID as a resource for other systems to access
    commands.insert_resource(TilemapIdResource(tilemap_id));

    // Create tile storage to track all tiles
    let mut tile_storage = TileStorage::empty(map_size);

    // First pass: spawn all tiles and store their entities in a 2D Vec
    let mut tile_entities = vec![vec![Entity::PLACEHOLDER; map_size.y as usize]; map_size.x as usize];
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let world_pos = Position::new(x as i32, y as i32);

            // Get terrain type from world map
            let terrain_type = world_map
                .get_tile(world_pos)
                .map(|t| &t.terrain)
                .unwrap_or(&TerrainType::Ocean);

            // Default texture index
            let mut texture_index = match terrain_type {
                TerrainType::Plains => tile_assets.plains_index as u32,
                TerrainType::Hills => tile_assets.plains_index as u32, // Use plains for now
                TerrainType::Mountains => tile_assets.plains_index as u32, // Use plains for now
                TerrainType::Forest => tile_assets.plains_index as u32, // Use plains for now
                TerrainType::Desert => tile_assets.plains_index as u32, // Use plains for now
                TerrainType::Coast => tile_assets.plains_index as u32, // Use plains for now
                TerrainType::Ocean => tile_assets.ocean_index as u32,
                TerrainType::River => tile_assets.plains_index as u32, // Use plains for now
            };

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
                    texture_index = 8; // Coast tile index
                }
            }

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture_index: TileTextureIndex(texture_index),
                    ..Default::default()
                })
                .insert(WorldTile {
                    grid_pos: world_pos,
                    terrain_type: terrain_type.clone(),
                })
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

    // Configure tilemap for square rendering (can switch to isometric later)
    // Note: Using square tiles (64x64) for now, will create proper isometric tiles later
    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = TilemapGridSize { x: 64.0, y: 64.0 };
    let map_type = TilemapType::Square;

    // Add the tilemap bundle to the tilemap entity
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_assets.sprite_sheet.clone()),
        tile_size,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

/// Spawn a unit/capital/city as a child of the correct tile entity in the tilemap
pub fn spawn_entity_on_tile(
    mut commands: Commands,
    tilemap_query: Query<&TileStorage>,
    tilemap_id: TilemapId,
    tile_assets: &TileAssets,
    sprite_index: usize,
    position: Position,
) {
    let tile_pos = TilePos {
        x: position.x as u32,
        y: position.y as u32,
    };
    if let Ok(tile_storage) = tilemap_query.get(tilemap_id.0) {
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            commands.entity(tile_entity).with_children(|parent| {
                parent.spawn((
                    Sprite::from_atlas_image(
                        tile_assets.sprite_sheet.clone(),
                        TextureAtlas {
                            layout: tile_assets.texture_atlas_layout.clone(),
                            index: sprite_index,
                        },
                    ),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // Slightly above the tile
                ));
            });
        }
    }
}

/// System to spawn all world tiles using the new tilemap setup
pub fn spawn_world_tiles(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    world_map: Res<WorldMap>,
) {
    setup_tilemap(commands, tile_assets, world_map);
}

/// System to spawn all unit sprites on their respective tiles
pub fn spawn_unit_sprites(
    mut commands: Commands,
    tilemap_query: Query<&TileStorage>,
    tilemap_id: Res<TilemapIdResource>,
    tile_assets: Res<TileAssets>,
    units: Query<(Entity, &Position), With<core_sim::components::MilitaryUnit>>,
) {
    for (_, position) in units.iter() {
        spawn_entity_on_tile(
            commands.reborrow(),
            tilemap_query,
            tilemap_id.0,
            &tile_assets,
            tile_assets.ancient_infantry_index,
            *position,
        );
    }
}

/// System to spawn all capital sprites on their respective tiles
pub fn spawn_capital_sprites(
    mut commands: Commands,
    tilemap_query: Query<&TileStorage>,
    tilemap_id: Res<TilemapIdResource>,
    tile_assets: Res<TileAssets>,
    civs: Query<(Entity, &core_sim::components::Civilization)>,
) {
    for (_, civ) in civs.iter() {
        if let Some(pos) = civ.capital {
            spawn_entity_on_tile(
                commands.reborrow(),
                tilemap_query,
                tilemap_id.0,
                &tile_assets,
                tile_assets.capital_ancient_index,
                pos,
            );
        }
    }
}

/// System to update unit sprites (stub for future logic)
pub fn update_unit_sprites() {
    // Implement logic to update unit sprites if needed
}

/// System to render overlays (stub for future logic)
pub fn render_world_overlays() {
    // Implement logic to render overlays if needed
}

// Example usage for units, capitals, cities:
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, unit_assets.ancient_infantry.clone(), unit_position, 1.0);
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, tile_assets.capital_ancient.clone(), capital_position, 2.0);
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, city_asset, city_position, 3.0);

/// Generate a unique color for each civilization based on their ID
fn get_civ_color(civ_id: &CivId) -> Color {
    // Simple hash-based color generation for consistent colors per civilization
    let hash = civ_id.0.wrapping_mul(31);

    // Convert hash to HSV for better color distribution
    let hue = (hash % 360) as f32;
    let saturation = 0.7;
    let value = 0.9;

    // Convert HSV to RGB
    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::srgb(r + m, g + m, b + m)
}

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::{resources::WorldMap, Position, TerrainType};

/// Resource to store the tilemap entity for efficient access
#[derive(Resource)]
pub struct IsometricTilemapEntity(pub Entity);

/// Component to track which terrain type a tile represents
#[derive(Component, Debug, Clone)]
pub struct TileTerrainType(pub TerrainType);

/// Plugin for isometric tilemap functionality
pub struct IsometricTilemapPlugin;

impl Plugin for IsometricTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin).add_systems(
            Startup,
            setup_tilemap_system.after(crate::game::setup_world),
        );
    }
}

/// Create the isometric tilemap from the world map
pub fn setup_tilemap_system(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    tile_assets: Res<crate::rendering::TileAssets>,
) {
    info!(
        "Setting up isometric tilemap with dimensions {}x{}",
        world_map.width, world_map.height
    );

    let map_size = TilemapSize {
        x: world_map.width as u32,
        y: world_map.height as u32,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    // Spawn individual tiles
    for x in 0..world_map.width {
        for y in 0..world_map.height {
            let tile_pos = TilePos {
                x: x as u32,
                y: y as u32,
            };

            let position = Position::new(x as i32, y as i32);
            let terrain = world_map
                .get_tile(position)
                .map(|tile| tile.terrain.clone())
                .unwrap_or(TerrainType::Ocean);

            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index: get_texture_index_for_terrain(terrain.clone()),
                        ..Default::default()
                    },
                    TileTerrainType(terrain),
                ))
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Configure isometric tilemap settings
    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 }; // Standard isometric ratio
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    // Create the tilemap bundle - position the tilemap so center tile aligns with entities
    // For a 100x50 tilemap, the center is at tile (50, 25)
    // Using isometric formula: center_world = (50-25)*32, (50+25)*16 = (800, 1200)
    let center_tile_x = map_size.x as f32 / 2.0;
    let center_tile_y = map_size.y as f32 / 2.0;
    let center_world_x = (center_tile_x - center_tile_y) * tile_size.x / 2.0;
    let center_world_y = (center_tile_x + center_tile_y) * tile_size.y / 2.0;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_assets.plains.clone()),
        tile_size,
        map_type,
        anchor: TilemapAnchor::Center,
        transform: Transform::from_translation(Vec3::new(center_world_x, center_world_y, 0.0)),
        ..Default::default()
    });

    // Store the tilemap entity for later access
    commands.insert_resource(IsometricTilemapEntity(tilemap_entity));

    info!(
        "Isometric tilemap setup complete with {} tiles",
        world_map.width * world_map.height
    );
}

/// Convert terrain type to texture index for isometric tilemap
pub fn get_texture_index_for_terrain(terrain: TerrainType) -> TileTextureIndex {
    match terrain {
        TerrainType::Plains => TileTextureIndex(0),
        TerrainType::Hills => TileTextureIndex(1),
        TerrainType::Mountains => TileTextureIndex(2),
        TerrainType::Forest => TileTextureIndex(3),
        TerrainType::Desert => TileTextureIndex(4),
        TerrainType::Coast => TileTextureIndex(5),
        TerrainType::Ocean => TileTextureIndex(6),
        TerrainType::River => TileTextureIndex(7),
    }
}

/// Update a specific tile's appearance
pub fn update_tile_terrain(
    tile_pos: TilePos,
    new_terrain: TerrainType,
    tilemap_entity: Entity,
    tile_storage_query: &Query<&TileStorage>,
    tile_texture_query: &mut Query<(&mut TileTextureIndex, &mut TileTerrainType)>,
) {
    if let Ok(tile_storage) = tile_storage_query.get(tilemap_entity) {
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            if let Ok((mut texture_index, mut terrain_type)) =
                tile_texture_query.get_mut(tile_entity)
            {
                texture_index.0 = get_texture_index_for_terrain(new_terrain.clone()).0;
                terrain_type.0 = new_terrain;
            }
        }
    }
}

/// System to handle tile updates when the world map changes
pub fn sync_tiles_with_world_map(
    world_map: Res<WorldMap>,
    tilemap_entity: Res<IsometricTilemapEntity>,
    tile_storage_query: Query<&TileStorage>,
    mut tile_query: Query<(&mut TileTextureIndex, &mut TileTerrainType, &TilePos)>,
) {
    if !world_map.is_changed() {
        return;
    }

    if let Ok(_tile_storage) = tile_storage_query.get(tilemap_entity.0) {
        for (mut texture_index, mut terrain_type, tile_pos) in tile_query.iter_mut() {
            let x = tile_pos.x as i32;
            let y = tile_pos.y as i32;

            if x >= 0 && y >= 0 && (x as u32) < world_map.width && (y as u32) < world_map.height {
                let position = Position::new(x, y);
                if let Some(map_tile) = world_map.get_tile(position) {
                    if terrain_type.0 != map_tile.terrain {
                        texture_index.0 = get_texture_index_for_terrain(map_tile.terrain.clone()).0;
                        terrain_type.0 = map_tile.terrain.clone();
                    }
                }
            }
        }
    }
}

/// Helper function to convert world coordinates to tile coordinates
pub fn world_to_tile_pos(world_pos: Vec2, tile_size: TilemapTileSize) -> Option<TilePos> {
    // Standard isometric world-to-tile conversion
    let tile_x = ((world_pos.x / tile_size.x) + (world_pos.y / tile_size.y)) / 2.0;
    let tile_y = ((world_pos.y / tile_size.y) - (world_pos.x / tile_size.x)) / 2.0;

    if tile_x >= 0.0 && tile_y >= 0.0 {
        Some(TilePos {
            x: tile_x as u32,
            y: tile_y as u32,
        })
    } else {
        None
    }
}

/// Helper function to convert tile coordinates to world coordinates
pub fn tile_to_world_pos(tile_pos: TilePos, tile_size: TilemapTileSize) -> Vec2 {
    // Standard isometric tile-to-world conversion for diamond layout
    // For diamond isometric, we need to account for the diamond orientation
    let world_x = (tile_pos.x as f32 - tile_pos.y as f32) * tile_size.x / 2.0;
    let world_y = (tile_pos.x as f32 + tile_pos.y as f32) * tile_size.y / 2.0;

    // Add a small offset to center the coordinate system
    let offset_x = 0.0;
    let offset_y = 0.0;

    Vec2::new(world_x + offset_x, world_y + offset_y)
}

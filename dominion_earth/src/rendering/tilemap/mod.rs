use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::tile::tile_assets::TileAssets;
use core_sim::WorldMap;
use crate::constants::rendering::tile_size;
use crate::debug_utils::DebugLogging;
use super::common::TilemapIdResource;

pub fn setup_tilemap(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    mut world_map: ResMut<WorldMap>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    commands.insert_resource(TilemapIdResource(tilemap_id));

    let tile_storage = core_sim::tile::tile_components::setup_world_tiles(
        &mut commands,
        tilemap_id,
        &*tile_assets,
        &mut *world_map,
    );

    let tile_size = TilemapTileSize {
        x: tile_size::TILE_WIDTH,
        y: tile_size::TILE_HEIGHT,
    };
    let grid_size = TilemapGridSize {
        x: tile_size::GRID_WIDTH,
        y: tile_size::GRID_HEIGHT,
    };
    let map_type = TilemapType::Square;

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
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });
}

pub fn spawn_world_tiles(
    _commands: Commands,
    _tilemap_id_resource: Res<TilemapIdResource>,
) {
    // This function was originally load_world_tiles, but it doesn't exist in core_sim
    // For now, just leave it empty as tiles are already loaded in setup_tilemap
}

pub fn spawn_entity_on_tile(
    commands: &mut Commands,
    tile_assets: &TileAssets,
    tile_storage: &TileStorage,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
    position: core_sim::Position,
    sprite_index: usize,
    z_offset: f32,
    debug_logging: &DebugLogging,
) -> Option<Entity> {
    let tile_pos = TilePos {
        x: position.x as u32,
        y: position.y as u32,
    };

    if let Some(_tile_entity) = tile_storage.get(&tile_pos) {
        let tile_center =
            tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
        let world_pos = tile_center.extend(z_offset);

        let sprite_entity = commands.spawn((
            Sprite::from_atlas_image(
                tile_assets.sprite_sheet.clone(),
                TextureAtlas {
                    layout: tile_assets.texture_atlas_layout.clone(),
                    index: sprite_index,
                },
            ),
            Transform::from_translation(world_pos),
        )).id();

        crate::debug_println!(debug_logging,
            "DEBUG: Spawned entity at position ({}, {}) with sprite index {} at world pos ({}, {}, {})",
            position.x, position.y, sprite_index, world_pos.x, world_pos.y, world_pos.z
        );

        Some(sprite_entity)
    } else {
        crate::debug_println!(debug_logging, "Warning: Could not find tile at position {:?}", position);
        None
    }
}

use super::common::TilemapIdResource;
use crate::constants::rendering::tile_size;
use crate::screens::{LoadingState, Screen};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::tile::tile_assets::TileAssets;
use core_sim::WorldMap;

pub fn setup_tilemap(
    mut commands: Commands,
    tile_assets: Option<Res<TileAssets>>,
    mut world_map: ResMut<WorldMap>,
    tilemap_id_resource: Option<Res<TilemapIdResource>>,
) {
    // Only run once - if tilemap already exists, skip
    if tilemap_id_resource.is_some() {
        return;
    }

    // Wait for TileAssets to be loaded
    let Some(tile_assets) = tile_assets else {
        return;
    };

    println!("Setting up tilemap with loaded TileAssets!");

    // DISABLED: We're using static TMX loading via bevy_ecs_tiled
    // No need to create our own tilemap - bevy_ecs_tiled handles it
    // The old setup_world_tiles call was creating a duplicate map

    // Mark that we've set up the tilemap (even though bevy_ecs_tiled is doing the actual work)
    let dummy_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(dummy_entity);
    commands.insert_resource(TilemapIdResource(tilemap_id));
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
) -> Option<Entity> {
    spawn_entity_on_tile_with_parent(
        commands,
        tile_assets,
        tile_storage,
        map_size,
        tile_size,
        grid_size,
        map_type,
        anchor,
        position,
        sprite_index,
        z_offset,
        None,
    )
}

/// Spawn an entity sprite on a tile, optionally as a child of the tile
pub fn spawn_entity_on_tile_with_parent(
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
    parent_tile_entity: Option<Entity>,
) -> Option<Entity> {
    let tile_pos = TilePos {
        x: position.x as u32,
        y: position.y as u32,
    };

    if let Some(tile_entity) = tile_storage.get(&tile_pos) {
        let tile_center =
            tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
        let world_pos = tile_center.extend(z_offset);

        // Always spawn as independent entity for proper rendering
        // The parent_tile_entity parameter is kept for API compatibility but not used
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
                DespawnOnExit(Screen::Gameplay), // Auto-despawn when leaving Gameplay
                DespawnOnEnter(LoadingState::Loading), // Auto-despawn when loading starts
            ))
            .id();

        crate::debug_println!(
            "DEBUG: Spawned entity at position ({}, {}) with sprite index {} at world pos ({}, {}, {})",
            position.x, position.y, sprite_index, world_pos.x, world_pos.y, world_pos.z
        );

        Some(sprite_entity)
    } else {
        crate::debug_println!("Warning: Could not find tile at position {:?}", position);
        None
    }
}

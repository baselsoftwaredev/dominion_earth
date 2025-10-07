use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::AnimatedTile;
use core_sim::tile::tile_assets::TileAssets;
use core_sim::components::{city::Capital, position::Position};
use crate::constants::rendering::{animation, z_layers};
use crate::debug_utils::DebugLogging;
use super::tilemap::spawn_entity_on_tile;

#[derive(Component, Debug, Clone)]
pub struct SpriteAnimationTimer {
    pub animated_tile: AnimatedTile,
    pub timer: f32,
}

impl SpriteAnimationTimer {
    pub fn new(start: u32, end: u32, speed: f32) -> Self {
        Self {
            animated_tile: AnimatedTile { start, end, speed },
            timer: animation::ANIMATION_TIMER_RESET_VALUE,
        }
    }
}

pub fn spawn_animated_capital_sprite(
    commands: &mut Commands,
    tile_assets: &TileAssets,
    tile_storage: &TileStorage,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
    position: Position,
    sprite_index: u32,
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

        let should_animate = matches!(sprite_index, animation::ANCIENT_CAPITAL_START_FRAME..=animation::ANCIENT_CAPITAL_END_FRAME);
        
        let sprite_entity = if should_animate {
            let entity = commands
                .spawn((
                    Sprite::from_atlas_image(
                        tile_assets.sprite_sheet.clone(),
                        TextureAtlas {
                            layout: tile_assets.texture_atlas_layout.clone(),
                            index: sprite_index as usize,
                        },
                    ),
                    Transform::from_translation(world_pos),
                    SpriteAnimationTimer::new(
                        animation::ANCIENT_CAPITAL_START_FRAME,
                        animation::ANCIENT_CAPITAL_END_FRAME,
                        animation::ANCIENT_CAPITAL_ANIMATION_SPEED,
                    ),
                ))
                .id();

            crate::debug_println!(debug_logging, 
                "DEBUG: Spawned animated capital sprite at ({}, {}) with animation range {}-{}, speed {}", 
                position.x, position.y, animation::ANCIENT_CAPITAL_START_FRAME, animation::ANCIENT_CAPITAL_END_FRAME, animation::ANCIENT_CAPITAL_ANIMATION_SPEED
            );

            entity
        } else {
            let entity = commands
                .spawn((
                    Sprite::from_atlas_image(
                        tile_assets.sprite_sheet.clone(),
                        TextureAtlas {
                            layout: tile_assets.texture_atlas_layout.clone(),
                            index: sprite_index as usize,
                        },
                    ),
                    Transform::from_translation(world_pos),
                ))
                .id();

            crate::debug_println!(debug_logging, 
                "DEBUG: Spawned static capital sprite at ({}, {}) with index {}", 
                position.x, position.y, sprite_index
            );

            entity
        };

        Some(sprite_entity)
    } else {
        crate::debug_println!(debug_logging, "Warning: Could not find tile at position {:?}", position);
        None
    }
}

pub fn spawn_animated_capital_tiles(
    mut commands: Commands,
    tile_assets: Option<Res<TileAssets>>,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    // Also check for capitals without sprite references (e.g., loaded from save or spawned before tilemap was ready)
    capitals: Query<
        (Entity, &Capital, &Position),
        Or<(
            Added<Capital>,
            Without<core_sim::components::rendering::SpriteEntityReference>,
        )>,
    >,
    debug_logging: Res<DebugLogging>,
) {
    // Wait for TileAssets to be loaded
    let Some(tile_assets) = tile_assets else {
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    for (capital_entity, capital, pos) in capitals.iter() {
        crate::debug_println!(
            debug_logging,
            "DEBUG: spawn_animated_capital_tiles processing capital at ({}, {}) with sprite index {}",
            pos.x,
            pos.y,
            capital.sprite_index
        );

        if let Some(sprite_entity) = spawn_animated_capital_sprite(
            &mut commands,
            &tile_assets,
            tile_storage,
            map_size,
            tile_size,
            grid_size,
            map_type,
            anchor,
            *pos,
            capital.sprite_index,
            z_layers::CAPITAL_Z,
            &debug_logging,
        ) {
            commands
                .entity(capital_entity)
                .insert(core_sim::components::rendering::SpriteEntityReference { sprite_entity });
        }
    }
}

pub fn spawn_capital_sprites(
    mut commands: Commands,
    tile_assets: Option<Res<TileAssets>>,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    world_tile_q: Query<&core_sim::tile::tile_components::WorldTile>,
    capitals: Query<(Entity, &Capital, &Position), Added<Capital>>,
    debug_logging: Res<DebugLogging>,
) {
    // Wait for TileAssets to be loaded
    let Some(tile_assets) = tile_assets else {
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    for (_capital_entity, capital, pos) in capitals.iter() {
        crate::debug_println!(
            debug_logging,
            "DEBUG: spawn_capital_sprites processing capital at ({}, {}) with sprite index {}",
            pos.x,
            pos.y,
            capital.sprite_index
        );

        let tile_pos = TilePos {
            x: pos.x as u32,
            y: pos.y as u32,
        };

        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            if let Ok(world_tile) = world_tile_q.get(tile_entity) {
                crate::debug_println!(
                    debug_logging,
                    "DEBUG: Spawning capital on {:?} tile at ({}, {})",
                    world_tile.terrain_type,
                    pos.x,
                    pos.y
                );
            }
        }

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
            z_layers::CAPITAL_Z,
            &debug_logging,
        );
    }
}

pub fn update_capital_sprites(
    mut commands: Commands,
    tile_assets: Option<Res<TileAssets>>,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    capitals: Query<(&Capital, &Position), bevy::ecs::query::Changed<Capital>>,
    debug_logging: Res<DebugLogging>,
) {
    // Wait for TileAssets to be loaded
    let Some(tile_assets) = tile_assets else {
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    for (capital, pos) in capitals.iter() {
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
            z_layers::CAPITAL_Z,
            &debug_logging,
        );
    }
}

pub fn update_animated_capital_sprites(
    time: Res<Time>,
    mut animated_query: Query<(&mut Sprite, &mut SpriteAnimationTimer)>,
    debug_logging: Res<DebugLogging>,
) {
    for (mut sprite, mut animation_timer) in animated_query.iter_mut() {
        animation_timer.timer += time.delta_secs();

        if animation_timer.timer >= animation_timer.animated_tile.speed {
            animation_timer.timer = animation::ANIMATION_TIMER_RESET_VALUE;

            if let Some(texture_atlas) = &mut sprite.texture_atlas {
                let current_index = texture_atlas.index;
                let next_index = if current_index >= animation_timer.animated_tile.end as usize {
                    animation_timer.animated_tile.start as usize
                } else {
                    current_index + 1
                };

                texture_atlas.index = next_index;
            }
        }
    }
}

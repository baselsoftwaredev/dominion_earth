use core_sim::tile::tile_components::WorldTile;

use crate::constants::rendering::{animation, borders, tile_size, transform, z_layers};
use crate::debug_utils::DebugLogging;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::AnimatedTile;
use core_sim::tile::tile_assets::TileAssets;
use core_sim::*;

#[derive(Resource, Clone)]
pub struct TilemapIdResource(pub TilemapId);

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
        transform: Transform::from_translation(Vec3::new(
            transform::DEFAULT_X,
            transform::DEFAULT_Y,
            transform::DEFAULT_Z,
        )),
        ..Default::default()
    });
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
    position: Position,
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

        crate::debug_log!(debug_logging, "DEBUG: Spawned sprite at world coords ({:.2}, {:.2}, {:.2}) for tile position ({}, {}), sprite index: {}", 
                 world_pos.x, world_pos.y, world_pos.z, position.x, position.y, sprite_index);

        Some(sprite_entity)
    } else {
        crate::debug_println!(debug_logging, "Warning: Could not find tile at position {:?}", position);
        None
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
                "DEBUG: Spawned regular capital sprite at ({}, {}) with sprite index {}", 
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

pub fn spawn_world_tiles(
    commands: Commands,
    tile_assets: Res<TileAssets>,
    world_map: ResMut<WorldMap>,
) {
    setup_tilemap(commands, tile_assets, world_map);
}

pub fn spawn_unit_sprites(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    units: Query<
        (
            Entity,
            &core_sim::components::MilitaryUnit,
            &core_sim::components::Position,
        ),
        Without<core_sim::components::SpriteEntityReference>,
    >,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    debug_logging: Res<DebugLogging>,
) {
    let unit_count = units.iter().count();
    crate::debug_log!(
        debug_logging,
        "DEBUG: spawn_unit_sprites called with {} units",
        unit_count
    );

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        crate::debug_log!(
            debug_logging,
            "DEBUG: Could not get tilemap components for units"
        );
        return;
    };

    crate::debug_log!(
        debug_logging,
        "DEBUG: Got tilemap components for units, map size: {}x{}",
        map_size.x,
        map_size.y
    );

    for (unit_entity, unit, position) in units.iter() {
        crate::debug_log!(
            debug_logging,
            "DEBUG: Processing unit {:?} at position ({}, {})",
            unit.unit_type,
            position.x,
            position.y
        );

        let sprite_index = match unit.unit_type {
            core_sim::UnitType::Infantry => tile_assets.ancient_infantry_index,
            core_sim::UnitType::Cavalry => tile_assets.ancient_infantry_index,
            core_sim::UnitType::Archer => tile_assets.ancient_infantry_index,
            core_sim::UnitType::Siege => tile_assets.ancient_infantry_index,
            core_sim::UnitType::Naval => tile_assets.ancient_infantry_index,
        };

        crate::debug_log!(
            debug_logging,
            "DEBUG: Spawning unit sprite with index {} at ({}, {})",
            sprite_index,
            position.x,
            position.y
        );

        if let Some(sprite_entity) = spawn_entity_on_tile(
            &mut commands,
            &tile_assets,
            tile_storage,
            map_size,
            tile_size,
            grid_size,
            map_type,
            anchor,
            *position,
            sprite_index,
            z_layers::UNIT_Z,
            &debug_logging,
        ) {
            commands.entity(unit_entity).insert(
                core_sim::components::SpriteEntityReference::create_new_reference(sprite_entity),
            );
        }
    }
}

pub fn spawn_animated_capital_tiles(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    capitals: Query<
        (Entity, &core_sim::Position, &core_sim::Capital),
        Without<core_sim::components::rendering::SpriteEntityReference>,
    >,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    debug_logging: Res<DebugLogging>,
) {
    if capitals.is_empty() {
        return;
    }

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single() else {
        return;
    };

    crate::debug_println!(
        debug_logging,
        "DEBUG: spawn_animated_capital_tiles called with {} capitals",
        capitals.iter().count()
    );

    for (capital_entity, pos, capital) in capitals.iter() {
        crate::debug_println!(
            debug_logging,
            "DEBUG: Processing animated capital at position ({}, {}) with sprite index {}",
            pos.x,
            pos.y,
            capital.sprite_index
        );

        if let Some(sprite_entity) = spawn_animated_capital_sprite(
            &mut commands,
            &tile_assets,
            &tile_storage,
            &map_size,
            &tile_size,
            &grid_size,
            &map_type,
            &anchor,
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

                crate::debug_println!(
                    debug_logging,
                    "DEBUG: Animated capital sprite frame updated from {} to {}",
                    current_index,
                    next_index
                );
            }
        }
    }
}
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
    tile_q: Query<&WorldTile>,
    debug_logging: Res<DebugLogging>,
) {
    let capital_count = capitals.iter().count();
    crate::debug_log!(
        debug_logging,
        "DEBUG: spawn_capital_sprites called with {} capitals",
        capital_count
    );

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        crate::debug_log!(debug_logging, "DEBUG: Could not get tilemap components");
        return;
    };

    crate::debug_log!(
        debug_logging,
        "DEBUG: Got tilemap components, map size: {}x{}",
        map_size.x,
        map_size.y
    );

    for (capital, pos) in capitals.iter() {
        crate::debug_log!(
            debug_logging,
            "DEBUG: Processing capital at position ({}, {}) with sprite index {}",
            pos.x,
            pos.y,
            capital.sprite_index
        );

        let tile_pos = TilePos {
            x: pos.x as u32,
            y: pos.y as u32,
        };

        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            if let Ok(world_tile) = tile_q.get(tile_entity) {
                crate::debug_log!(
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

pub fn update_unit_sprites(
    mut transform_q: Query<&mut Transform>,
    tile_assets: Res<TileAssets>,
    query: Query<
        (
            Entity,
            &core_sim::Position,
            &core_sim::MilitaryUnit,
            &core_sim::components::rendering::SpriteEntityReference,
        ),
        Changed<core_sim::Position>,
    >,
    debug_logging: Res<DebugLogging>,
) {
    let changed_unit_count = query.iter().count();
    if changed_unit_count == 0 {
        return;
    }

    crate::debug_log!(
        debug_logging,
        "DEBUG: update_unit_sprites called with {} changed units",
        changed_unit_count
    );

    for (_entity, position, unit, sprite_reference) in query.iter() {
        crate::debug_log!(
            debug_logging,
            "DEBUG: Moving unit {:?} sprite at position ({}, {})",
            unit.unit_type,
            position.x,
            position.y
        );

        if let Ok(mut transform) = transform_q.get_mut(sprite_reference.sprite_entity) {
            let new_world_position = calculate_sprite_world_position(position, &tile_assets);
            transform.translation = new_world_position;

            crate::debug_log!(
                debug_logging,
                "DEBUG: Successfully moved sprite to world position ({}, {}, {})",
                new_world_position.x,
                new_world_position.y,
                new_world_position.z
            );
        } else {
            crate::debug_log!(
                debug_logging,
                "WARN: Could not find sprite entity for unit at position ({}, {})",
                position.x,
                position.y
            );
        }
    }
}

fn calculate_sprite_world_position(
    position: &core_sim::Position,
    _tile_assets: &TileAssets,
) -> Vec3 {
    Vec3::new(
        position.x as f32 * tile_size::TILE_WIDTH,
        position.y as f32 * tile_size::TILE_HEIGHT,
        z_layers::UNIT_Z,
    )
}

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
    debug_logging: Res<DebugLogging>,
) {
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

pub fn render_civilization_borders(
    mut gizmos: Gizmos,
    units: Query<(
        &core_sim::components::military::MilitaryUnit,
        &core_sim::components::position::Position,
    )>,
    capitals: Query<(
        &core_sim::components::city::Capital,
        &core_sim::components::position::Position,
    )>,
    civilizations: Query<&core_sim::components::civilization::Civilization>,
    tilemap_q: Query<(
        &TileStorage,
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    debug_logging: Res<DebugLogging>,
) {
    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    for (unit, position) in units.iter() {
        if let Some(civ) = civilizations.iter().find(|civ| civ.id == unit.owner) {
            let world_pos = calculate_world_position_for_gizmo(
                *position, map_size, tile_size, grid_size, map_type, anchor,
            );
            let border_color = Color::srgb(civ.color[0], civ.color[1], civ.color[2]);

            let half_width = tile_size.x * borders::UNIT_BORDER_HALF_WIDTH_FACTOR;
            let half_height = tile_size.y * borders::UNIT_BORDER_HALF_HEIGHT_FACTOR;
            let center = world_pos.truncate();

            let corners = [
                center + Vec2::new(-half_width, -half_height),
                center + Vec2::new(half_width, -half_height),
                center + Vec2::new(half_width, half_height),
                center + Vec2::new(-half_width, half_height),
                center + Vec2::new(-half_width, -half_height),
            ];

            gizmos.linestrip_2d(corners, border_color);

            crate::debug_log!(
                debug_logging,
                "DEBUG: Drew unit border at ({}, {}) with color {:?}",
                world_pos.x,
                world_pos.y,
                border_color
            );
        }
    }

    for (capital, position) in capitals.iter() {
        if let Some(civ) = civilizations.iter().find(|civ| civ.id == capital.owner) {
            let world_pos = calculate_world_position_for_gizmo(
                *position, map_size, tile_size, grid_size, map_type, anchor,
            );
            let border_color = Color::srgb(civ.color[0], civ.color[1], civ.color[2]);

            let half_width = tile_size.x * borders::CAPITAL_OUTER_BORDER_HALF_WIDTH_FACTOR;
            let half_height = tile_size.y * borders::CAPITAL_OUTER_BORDER_HALF_HEIGHT_FACTOR;
            let center = world_pos.truncate();

            let outer_corners = [
                center + Vec2::new(-half_width, -half_height),
                center + Vec2::new(half_width, -half_height),
                center + Vec2::new(half_width, half_height),
                center + Vec2::new(-half_width, half_height),
                center + Vec2::new(-half_width, -half_height),
            ];

            gizmos.linestrip_2d(outer_corners, border_color);

            let inner_half_width = tile_size.x * borders::CAPITAL_INNER_BORDER_HALF_WIDTH_FACTOR;
            let inner_half_height = tile_size.y * borders::CAPITAL_INNER_BORDER_HALF_HEIGHT_FACTOR;

            let inner_corners = [
                center + Vec2::new(-inner_half_width, -inner_half_height),
                center + Vec2::new(inner_half_width, -inner_half_height),
                center + Vec2::new(inner_half_width, inner_half_height),
                center + Vec2::new(-inner_half_width, inner_half_height),
                center + Vec2::new(-inner_half_width, -inner_half_height),
            ];

            gizmos.linestrip_2d(inner_corners, border_color);
        }
    }
}

fn calculate_world_position_for_gizmo(
    position: core_sim::components::position::Position,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Vec3 {
    let tile_pos = TilePos {
        x: position.x as u32,
        y: position.y as u32,
    };

    let tile_center = tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
    tile_center.extend(z_layers::UNIT_Z + 1.0)
}

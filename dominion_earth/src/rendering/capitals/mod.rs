use super::tilemap::spawn_entity_on_tile_with_parent;
use crate::constants::rendering::{animation, z_layers};
use crate::screens::{LoadingState, Screen};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::AnimatedTile;
use core_sim::components::{city::Capital, position::Position};
use core_sim::tile::tile_assets::TileAssets;

/// Marker component for capital sprite entities (for cleanup)
#[derive(Component, Debug, Clone)]
pub struct CapitalSprite;

/// Tracks animation state for capital sprites
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

/// Helper struct to reduce function parameter bloat
pub struct TilemapContext<'a> {
    pub tile_storage: &'a TileStorage,
    pub map_size: &'a TilemapSize,
    pub tile_size: &'a TilemapTileSize,
    pub grid_size: &'a TilemapGridSize,
    pub map_type: &'a TilemapType,
    pub anchor: &'a TilemapAnchor,
}

/// Creates the base sprite with texture atlas
fn create_capital_sprite(
    tile_assets: &TileAssets,
    sprite_index: u32,
    world_pos: Vec3,
) -> (Sprite, Transform) {
    let sprite = Sprite::from_atlas_image(
        tile_assets.sprite_sheet.clone(),
        TextureAtlas {
            layout: tile_assets.texture_atlas_layout.clone(),
            index: sprite_index as usize,
        },
    );
    let transform = Transform::from_translation(world_pos);
    (sprite, transform)
}

/// Checks if a sprite index should be animated
fn should_animate_sprite(sprite_index: u32) -> bool {
    matches!(
        sprite_index,
        animation::ANCIENT_CAPITAL_START_FRAME..=animation::ANCIENT_CAPITAL_END_FRAME
    )
}

pub fn spawn_animated_capital_sprite(
    commands: &mut Commands,
    tile_assets: &TileAssets,
    tilemap: &TilemapContext,
    position: Position,
    sprite_index: u32,
    z_offset: f32,
) -> Option<Entity> {
    let tile_pos = TilePos {
        x: position.x as u32,
        y: position.y as u32,
    };

    // Ensure the tile exists in the tilemap
    tilemap.tile_storage.get(&tile_pos)?;

    let tile_center = tile_pos.center_in_world(
        tilemap.map_size,
        tilemap.grid_size,
        tilemap.tile_size,
        tilemap.map_type,
        tilemap.anchor,
    );
    let world_pos = tile_center.extend(z_offset);

    let (sprite, transform) = create_capital_sprite(tile_assets, sprite_index, world_pos);

    let sprite_entity = if should_animate_sprite(sprite_index) {
        // Spawn animated capital sprite
        let entity = commands
            .spawn((
                sprite,
                transform,
                Visibility::Hidden, // Start hidden - fog_of_war system will show if appropriate
                SpriteAnimationTimer::new(
                    animation::ANCIENT_CAPITAL_START_FRAME,
                    animation::ANCIENT_CAPITAL_END_FRAME,
                    animation::ANCIENT_CAPITAL_ANIMATION_SPEED,
                ),
                CapitalSprite,
                DespawnOnExit(Screen::Gameplay),
                DespawnOnEnter(LoadingState::Loading),
            ))
            .id();

        crate::debug_println!(
            "Spawned animated capital at ({}, {}) with animation range {}-{}, speed {}",
            position.x,
            position.y,
            animation::ANCIENT_CAPITAL_START_FRAME,
            animation::ANCIENT_CAPITAL_END_FRAME,
            animation::ANCIENT_CAPITAL_ANIMATION_SPEED
        );

        entity
    } else {
        // Spawn static capital sprite
        let entity = commands
            .spawn((
                sprite,
                transform,
                Visibility::Hidden, // Start hidden - fog_of_war system will show if appropriate
                CapitalSprite,
                DespawnOnExit(Screen::Gameplay),
                DespawnOnEnter(LoadingState::Loading),
            ))
            .id();

        crate::debug_println!(
            "Spawned static capital at ({}, {}) with sprite index {}",
            position.x,
            position.y,
            sprite_index
        );

        entity
    };

    Some(sprite_entity)
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
    capitals: Query<
        (Entity, &Capital, &Position),
        Or<(
            Added<Capital>,
            Without<core_sim::components::rendering::SpriteEntityReference>,
        )>,
    >,
) {
    let Some(tile_assets) = tile_assets else {
        // Silently skip if tile_assets not ready yet
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        // Silently skip if tilemap not ready - will retry next frame
        return;
    };

    let capital_count = capitals.iter().count();
    if capital_count == 0 {
        return;
    }

    crate::debug_println!(
        "🏛️ spawn_animated_capital_tiles: Found {} capitals to process",
        capital_count
    );

    let tilemap = TilemapContext {
        tile_storage,
        map_size,
        tile_size,
        grid_size,
        map_type,
        anchor,
    };

    for (capital_entity, capital, pos) in capitals.iter() {
        crate::debug_println!(
            "🏛️ Processing capital {:?} at ({}, {}) with sprite index {}",
            capital_entity,
            pos.x,
            pos.y,
            capital.sprite_index
        );

        if let Some(sprite_entity) = spawn_animated_capital_sprite(
            &mut commands,
            &tile_assets,
            &tilemap,
            *pos,
            capital.sprite_index,
            z_layers::CAPITAL_Z,
        ) {
            crate::debug_println!(
                "✅ Added SpriteEntityReference to capital {:?}: sprite_entity={:?}",
                capital_entity,
                sprite_entity
            );
            commands
                .entity(capital_entity)
                .insert(core_sim::components::rendering::SpriteEntityReference { sprite_entity });
        } else {
            crate::debug_println!(
                "❌ Failed to spawn sprite for capital {:?} at ({}, {})",
                capital_entity,
                pos.x,
                pos.y
            );
        }
    }
}

/// Retry spawning capital sprites for any capitals that still don't have sprite references
/// This handles cases where the tilemap wasn't ready on the first attempt after load
pub fn recreate_missing_capital_sprites(
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
    capitals: Query<
        (Entity, &Capital, &Position),
        Without<core_sim::components::rendering::SpriteEntityReference>,
    >,
) {
    let Some(tile_assets) = tile_assets else {
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        // Silently skip if tilemap not ready yet
        return;
    };

    let tilemap = TilemapContext {
        tile_storage,
        map_size,
        tile_size,
        grid_size,
        map_type,
        anchor,
    };

    for (capital_entity, capital, pos) in capitals.iter() {
        crate::debug_println!(
            "🔄 Recreating missing sprite for capital {:?} at ({}, {})",
            capital_entity,
            pos.x,
            pos.y
        );

        if let Some(sprite_entity) = spawn_animated_capital_sprite(
            &mut commands,
            &tile_assets,
            &tilemap,
            *pos,
            capital.sprite_index,
            z_layers::CAPITAL_Z,
        ) {
            crate::debug_println!(
                "✅ Restored SpriteEntityReference to capital {:?}: sprite_entity={:?}",
                capital_entity,
                sprite_entity
            );
            commands
                .entity(capital_entity)
                .insert(core_sim::components::rendering::SpriteEntityReference { sprite_entity });
        }
    }
}

/// Recreate capital sprites that have invalid sprite references
/// This handles stale references that were saved before being properly cleared
pub fn validate_and_recreate_capital_sprites(
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
    capitals: Query<
        (
            Entity,
            &Capital,
            &Position,
            Option<&core_sim::components::rendering::SpriteEntityReference>,
        ),
        With<Capital>,
    >,
    sprites: Query<&Transform>, // Query to check if sprite entities exist
) {
    let Some(tile_assets) = tile_assets else {
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    let tilemap = TilemapContext {
        tile_storage,
        map_size,
        tile_size,
        grid_size,
        map_type,
        anchor,
    };

    for (capital_entity, capital, pos, sprite_ref) in capitals.iter() {
        let needs_new_sprite = if let Some(sprite_ref) = sprite_ref {
            // Check if the sprite entity still exists (not just if it's hidden)
            // A hidden sprite is still valid - don't recreate it
            sprites.get(sprite_ref.sprite_entity).is_err()
        } else {
            false // No reference means it's OK, will be handled by recreate_missing_capital_sprites
        };

        if needs_new_sprite {
            crate::debug_println!(
                "🏛️ Capital {:?} at ({}, {}) has invalid sprite reference, recreating",
                capital_entity,
                pos.x,
                pos.y
            );

            // Remove the stale reference
            commands
                .entity(capital_entity)
                .remove::<core_sim::components::rendering::SpriteEntityReference>();

            // Spawn new sprite
            if let Some(sprite_entity) = spawn_animated_capital_sprite(
                &mut commands,
                &tile_assets,
                &tilemap,
                *pos,
                capital.sprite_index,
                z_layers::CAPITAL_Z,
            ) {
                crate::debug_println!(
                    "✅ Recreated sprite for capital {:?}: sprite_entity={:?}",
                    capital_entity,
                    sprite_entity
                );
                commands.entity(capital_entity).insert(
                    core_sim::components::rendering::SpriteEntityReference { sprite_entity },
                );
            }
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
) {
    let Some(tile_assets) = tile_assets else {
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    let tilemap = TilemapContext {
        tile_storage,
        map_size,
        tile_size,
        grid_size,
        map_type,
        anchor,
    };

    for (_capital_entity, capital, pos) in capitals.iter() {
        crate::debug_println!(
            "Spawning capital at ({}, {}) with sprite index {}",
            pos.x,
            pos.y,
            capital.sprite_index
        );

        let tile_pos = TilePos {
            x: pos.x as u32,
            y: pos.y as u32,
        };

        if let Some(tile_entity) = tilemap.tile_storage.get(&tile_pos) {
            // Log terrain type for debugging
            if let Ok(world_tile) = world_tile_q.get(tile_entity) {
                crate::debug_println!(
                    "Capital placed on {:?} terrain at ({}, {})",
                    world_tile.terrain_type,
                    pos.x,
                    pos.y
                );
            }

            spawn_entity_on_tile_with_parent(
                &mut commands,
                &tile_assets,
                tilemap.tile_storage,
                tilemap.map_size,
                tilemap.tile_size,
                tilemap.grid_size,
                tilemap.map_type,
                tilemap.anchor,
                *pos,
                capital.sprite_index as usize,
                z_layers::CAPITAL_Z,
                Some(tile_entity),
            );
        }
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
) {
    let Some(tile_assets) = tile_assets else {
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    let tilemap = TilemapContext {
        tile_storage,
        map_size,
        tile_size,
        grid_size,
        map_type,
        anchor,
    };

    for (capital, pos) in capitals.iter() {
        let tile_pos = TilePos {
            x: pos.x as u32,
            y: pos.y as u32,
        };

        if let Some(tile_entity) = tilemap.tile_storage.get(&tile_pos) {
            spawn_entity_on_tile_with_parent(
                &mut commands,
                &tile_assets,
                tilemap.tile_storage,
                tilemap.map_size,
                tilemap.tile_size,
                tilemap.grid_size,
                tilemap.map_type,
                tilemap.anchor,
                *pos,
                capital.sprite_index as usize,
                z_layers::CAPITAL_Z,
                Some(tile_entity),
            );
        }
    }
}

pub fn update_animated_capital_sprites(
    time: Res<Time>,
    mut animated_query: Query<(&mut Sprite, &mut SpriteAnimationTimer)>,
) {
    for (mut sprite, mut anim_timer) in animated_query.iter_mut() {
        anim_timer.timer += time.delta_secs();

        // Advance animation frame when timer exceeds animation speed
        if anim_timer.timer >= anim_timer.animated_tile.speed {
            anim_timer.timer = animation::ANIMATION_TIMER_RESET_VALUE;

            if let Some(texture_atlas) = &mut sprite.texture_atlas {
                // Wrap animation back to start when reaching the end frame
                let next_index = if texture_atlas.index >= anim_timer.animated_tile.end as usize {
                    anim_timer.animated_tile.start as usize
                } else {
                    texture_atlas.index + 1
                };

                texture_atlas.index = next_index;
            }
        }
    }
}

/// Cleans up capital sprites when loading starts
/// Ensures old sprites don't persist when transitioning to a new game
pub fn cleanup_capital_sprites_on_load(
    mut commands: Commands,
    loading_state: Res<State<LoadingState>>,
    capital_sprites: Query<Entity, With<CapitalSprite>>,
) {
    if *loading_state == LoadingState::Loading && loading_state.is_changed() {
        for sprite_entity in &capital_sprites {
            crate::debug_println!("Despawning capital sprite {:?} on load", sprite_entity);
            commands.entity(sprite_entity).despawn();
        }
    }
}

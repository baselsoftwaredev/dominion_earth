use super::tilemap::spawn_entity_on_tile_with_parent;
use crate::constants::rendering::z_layers;
use crate::screens::LoadingState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::{
    military::{FacingDirection, MilitaryUnit},
    position::Position,
};
use core_sim::tile::tile_assets::TileAssets;

/// Marker component for unit sprite entities (for cleanup)
#[derive(Component, Debug, Clone)]
pub struct UnitSprite;

mod constants {
    pub const SPRITE_SCALE_FACING_LEFT: f32 = -1.0;
    pub const SPRITE_SCALE_FACING_RIGHT: f32 = 1.0;
}

fn apply_unit_facing_to_sprite_scale(transform: &mut Transform, facing: FacingDirection) {
    transform.scale.x = match facing {
        FacingDirection::Left => constants::SPRITE_SCALE_FACING_LEFT,
        FacingDirection::Right => constants::SPRITE_SCALE_FACING_RIGHT,
    };
}

pub fn spawn_unit_sprites(
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
    units: Query<
        (Entity, &MilitaryUnit, &Position),
        Or<(
            Added<MilitaryUnit>,
            Without<core_sim::components::rendering::SpriteEntityReference>,
        )>,
    >,
    mut transforms: Query<&mut Transform>,
) {
    let Some(tile_assets) = tile_assets else {
        crate::debug_println!("⚠️ spawn_unit_sprites: tile_assets not ready");
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    let unit_count = units.iter().count();
    if unit_count == 0 {
        return;
    }

    crate::debug_println!("🎖️ spawn_unit_sprites: Found {} units to spawn", unit_count);

    for (unit_entity, unit, pos) in units.iter() {
        spawn_unit_sprite(
            &mut commands,
            &tile_assets,
            tile_storage,
            map_size,
            tile_size,
            grid_size,
            map_type,
            anchor,
            unit_entity,
            unit,
            pos,
            &mut transforms,
        );
    }
}

pub fn recreate_missing_unit_sprites(
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
    units: Query<
        (
            Entity,
            &MilitaryUnit,
            &Position,
            Option<&core_sim::components::rendering::SpriteEntityReference>,
        ),
        With<MilitaryUnit>,
    >,
    mut transforms: Query<&mut Transform>,
) {
    let Some(tile_assets) = tile_assets else {
        crate::debug_println!("⚠️ recreate_missing_unit_sprites: tile_assets not ready");
        return;
    };

    let Ok((tile_storage, map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single()
    else {
        return;
    };

    // First pass: check if any units actually need sprite recreation
    let mut units_needing_sprites = Vec::new();
    for (unit_entity, unit, pos, sprite_ref) in units.iter() {
        let needs_new_sprite = if let Some(sprite_ref) = sprite_ref {
            transforms.get(sprite_ref.sprite_entity).is_err()
        } else {
            true
        };

        if needs_new_sprite {
            units_needing_sprites.push((unit_entity, unit, pos, sprite_ref));
        }
    }

    // Only do work and print debug message if there are units that actually need sprites
    if !units_needing_sprites.is_empty() {
        crate::debug_println!(
            "🎖️ recreate_missing_unit_sprites: Recreating sprites for {} of {} units",
            units_needing_sprites.len(),
            units.iter().count()
        );

        for (unit_entity, unit, pos, sprite_ref) in units_needing_sprites {
            crate::debug_println!(
                "🎖️ Unit {:?} at ({}, {}) needs sprite (has_ref: {})",
                unit_entity,
                pos.x,
                pos.y,
                sprite_ref.is_some()
            );

            if sprite_ref.is_some() {
                commands
                    .entity(unit_entity)
                    .remove::<core_sim::components::rendering::SpriteEntityReference>();
            }

            spawn_unit_sprite(
                &mut commands,
                &tile_assets,
                tile_storage,
                map_size,
                tile_size,
                grid_size,
                map_type,
                anchor,
                unit_entity,
                unit,
                pos,
                &mut transforms,
            );
        }
    }
}

fn spawn_unit_sprite(
    commands: &mut Commands,
    tile_assets: &TileAssets,
    tile_storage: &TileStorage,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
    unit_entity: Entity,
    unit: &MilitaryUnit,
    pos: &Position,
    transforms: &mut Query<&mut Transform>,
) {
    let sprite_index = match unit.unit_type {
        core_sim::components::military::UnitType::Infantry => tile_assets.ancient_infantry_index,
        core_sim::components::military::UnitType::Archer => tile_assets.ancient_infantry_index,
        core_sim::components::military::UnitType::Cavalry => tile_assets.ancient_infantry_index,
        _ => tile_assets.ancient_infantry_index,
    };

    crate::debug_println!(
        "Spawning unit sprite for {:?} at ({}, {}) with sprite index {} facing {:?}",
        unit.unit_type,
        pos.x,
        pos.y,
        sprite_index,
        unit.facing
    );

    // Get the tile entity at this position to use as parent
    let tile_pos = TilePos {
        x: pos.x as u32,
        y: pos.y as u32,
    };
    let parent_tile_entity = tile_storage.get(&tile_pos);

    if let Some(sprite_entity) = spawn_entity_on_tile_with_parent(
        commands,
        tile_assets,
        tile_storage,
        map_size,
        tile_size,
        grid_size,
        map_type,
        anchor,
        *pos,
        sprite_index,
        z_layers::UNIT_Z,
        parent_tile_entity,
    ) {
        let scale_x = match unit.facing {
            FacingDirection::Left => constants::SPRITE_SCALE_FACING_LEFT,
            FacingDirection::Right => constants::SPRITE_SCALE_FACING_RIGHT,
        };

        if let Ok(mut transform) = transforms.get_mut(sprite_entity) {
            transform.scale.x = scale_x;
            crate::debug_println!(
                "Applied facing {:?} (scale.x = {}) to sprite immediately",
                unit.facing,
                scale_x
            );
        } else {
            crate::debug_println!(
                "Transform not available yet for sprite, will be updated by update_unit_sprites"
            );
        }

        // Add marker component for cleanup and add SpriteEntityReference
        commands.entity(sprite_entity).insert(UnitSprite); // Marker for cleanup on load

        commands
            .entity(unit_entity)
            .insert(core_sim::components::rendering::SpriteEntityReference { sprite_entity });
    }
}

pub fn update_unit_sprites(
    mut transform_q: Query<&mut Transform>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    query: Query<
        (
            Entity,
            &Position,
            &MilitaryUnit,
            &core_sim::components::rendering::SpriteEntityReference,
        ),
        (
            Or<(Changed<Position>, Changed<MilitaryUnit>)>,
            With<core_sim::components::rendering::SpriteEntityReference>,
        ),
    >,
) {
    let changed_unit_count = query.iter().count();
    if changed_unit_count == 0 {
        return;
    }

    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single() else {
        return;
    };

    for (_unit_entity, position, unit, sprite_ref) in query.iter() {
        if let Ok(mut transform) = transform_q.get_mut(sprite_ref.sprite_entity) {
            let tile_pos = TilePos {
                x: position.x as u32,
                y: position.y as u32,
            };
            let tile_center =
                tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

            transform.translation.x = tile_center.x;
            transform.translation.y = tile_center.y;

            apply_unit_facing_to_sprite_scale(&mut transform, unit.facing);

            crate::debug_println!(
                "Updated {:?} sprite position to world coordinates ({}, {}) facing {:?}",
                unit.unit_type,
                tile_center.x,
                tile_center.y,
                unit.facing
            );
        }
    }
}

pub fn apply_facing_to_new_sprites(
    mut transform_q: Query<&mut Transform>,
    query: Query<
        (
            &MilitaryUnit,
            &core_sim::components::rendering::SpriteEntityReference,
        ),
        Added<core_sim::components::rendering::SpriteEntityReference>,
    >,
) {
    for (unit, sprite_ref) in query.iter() {
        if let Ok(mut transform) = transform_q.get_mut(sprite_ref.sprite_entity) {
            apply_unit_facing_to_sprite_scale(&mut transform, unit.facing);
            crate::debug_println!(
                "Applied facing {:?} to newly created sprite for {:?}",
                unit.facing,
                unit.unit_type
            );
        }
    }
}

/// System to clean up all unit sprites when entering LoadingState
/// This ensures old unit sprites don't linger when loading a new game
pub fn cleanup_unit_sprites_on_load(
    mut commands: Commands,
    loading_state: Res<State<LoadingState>>,
    unit_sprites: Query<Entity, With<UnitSprite>>,
) {
    // Only cleanup when ENTERING LoadingState, not on any state change
    if *loading_state == LoadingState::Loading && loading_state.is_changed() {
        for sprite_entity in &unit_sprites {
            crate::debug_println!(
                "DEBUG: Explicitly despawning unit sprite {:?} on load",
                sprite_entity
            );
            commands.entity(sprite_entity).despawn();
        }
    }
}

use super::tilemap::spawn_entity_on_tile;
use crate::constants::rendering::z_layers;
use crate::debug_utils::DebugLogging;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::{military::MilitaryUnit, position::Position};
use core_sim::tile::tile_assets::TileAssets;

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
    // Also check for units without sprite references (e.g., loaded from save or spawned before tilemap was ready)
    units: Query<
        (Entity, &MilitaryUnit, &Position),
        Or<(
            Added<MilitaryUnit>,
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
            &debug_logging,
        );
    }
}

/// Recreate sprites for units that have invalid sprite references (e.g., after loading)
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
    sprite_entities: Query<&Transform>,
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

    for (unit_entity, unit, pos, sprite_ref) in units.iter() {
        let needs_new_sprite = if let Some(sprite_ref) = sprite_ref {
            // Check if the referenced sprite entity still exists
            sprite_entities.get(sprite_ref.sprite_entity).is_err()
        } else {
            // No sprite reference at all
            true
        };

        if needs_new_sprite {
            // Remove old invalid sprite reference if it exists
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
                &debug_logging,
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
    debug_logging: &DebugLogging,
) {
    let sprite_index = match unit.unit_type {
        core_sim::components::military::UnitType::Infantry => tile_assets.ancient_infantry_index,
        core_sim::components::military::UnitType::Archer => tile_assets.ancient_infantry_index, // TODO: Add archer sprite
        core_sim::components::military::UnitType::Cavalry => tile_assets.ancient_infantry_index, // TODO: Add cavalry sprite
        _ => tile_assets.ancient_infantry_index, // Default to infantry
    };

    crate::debug_println!(
        debug_logging,
        "DEBUG: Spawning unit sprite for {:?} at ({}, {}) with sprite index {}",
        unit.unit_type,
        pos.x,
        pos.y,
        sprite_index
    );

    if let Some(sprite_entity) = spawn_entity_on_tile(
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
        debug_logging,
    ) {
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
            Changed<Position>,
            With<core_sim::components::rendering::SpriteEntityReference>,
        ),
    >,
    debug_logging: Res<DebugLogging>,
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

            crate::debug_println!(
                debug_logging,
                "DEBUG: Updated {:?} sprite position to world coordinates ({}, {})",
                unit.unit_type,
                tile_center.x,
                tile_center.y
            );
        }
    }
}

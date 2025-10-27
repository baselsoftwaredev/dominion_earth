use crate::debug_println;
use crate::debug_utils::DebugLogging;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_tilemap::prelude::*;
use core_sim::{
    components::{Civilization, MilitaryUnit},
    Position,
};

/// Component to mark unit label entities
#[derive(Component)]
pub struct UnitLabel {
    pub unit_entity: Entity,
    pub unit_position: Position,
}

/// Constants for unit label styling
pub mod constants {
    pub const UNIT_LABEL_FONT_SIZE: f32 = 14.0;
    pub const UNIT_LABEL_Z_INDEX: f32 = 101.0; // Above capital labels
    pub const UNIT_LABEL_NORTH_OFFSET_TILES: f32 = 1.0;
    pub const UNIT_LABEL_VERTICAL_OFFSET_PIXELS: f32 = -40.0;
    pub const UNIT_LABEL_BACKGROUND_ALPHA: f32 = 0.7;
}

/// System to spawn unit labels using Text2d (world-space text that scales with camera)
pub fn spawn_unit_labels(
    mut commands: Commands,
    units_query: Query<(Entity, &Position, &MilitaryUnit), Added<MilitaryUnit>>,
    units_without_labels: Query<(Entity, &Position, &MilitaryUnit), Without<UnitLabel>>,
    existing_labels: Query<&UnitLabel>,
    civilizations_query: Query<&Civilization>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    debug_logging: Res<DebugLogging>,
) {
    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_query.single() else {
        return;
    };

    for (unit_entity, position, unit) in units_query.iter() {
        let civilization_name = get_civilization_name(&civilizations_query, unit.owner);

        let north_tile_world_position = calculate_north_tile_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_unit_label_text2d(
            &mut commands,
            unit_entity,
            *position,
            &unit.unit_type.name(),
            &civilization_name,
            north_tile_world_position,
        );

        debug_println!(
            debug_logging,
            "Spawned Text2d unit label for {} ({}) at world position ({:.1}, {:.1})",
            unit.unit_type.name(),
            civilization_name,
            north_tile_world_position.x,
            north_tile_world_position.y
        );
    }

    for (unit_entity, position, unit) in units_without_labels.iter() {
        if existing_labels
            .iter()
            .any(|label| label.unit_entity == unit_entity)
        {
            continue;
        }

        let civilization_name = get_civilization_name(&civilizations_query, unit.owner);

        let north_tile_world_position = calculate_north_tile_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_unit_label_text2d(
            &mut commands,
            unit_entity,
            *position,
            &unit.unit_type.name(),
            &civilization_name,
            north_tile_world_position,
        );

        debug_println!(
            debug_logging,
            "Spawned missing Text2d unit label for {} ({}) at world position ({:.1}, {:.1})",
            unit.unit_type.name(),
            civilization_name,
            north_tile_world_position.x,
            north_tile_world_position.y
        );
    }
}

/// System to update unit label positions when units move or are destroyed
pub fn update_unit_labels(
    mut commands: Commands,
    mut label_query: Query<(Entity, &mut Transform, &mut UnitLabel)>,
    units_query: Query<&Position, With<MilitaryUnit>>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
) {
    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_query.single() else {
        return;
    };

    for (label_entity, mut label_transform, mut unit_label) in label_query.iter_mut() {
        if let Ok(unit_position) = units_query.get(unit_label.unit_entity) {
            if *unit_position != unit_label.unit_position {
                let new_world_position = calculate_north_tile_world_position(
                    unit_position,
                    map_size,
                    tile_size,
                    grid_size,
                    map_type,
                    anchor,
                );

                label_transform.translation =
                    new_world_position.extend(constants::UNIT_LABEL_Z_INDEX);

                // Update cached position
                unit_label.unit_position = *unit_position;
            }
        } else {
            // Unit no longer exists, despawn the label
            commands.entity(label_entity).despawn();
        }
    }
}

/// Helper function to get civilization name from query
fn get_civilization_name(
    civilizations_query: &Query<&Civilization>,
    civ_id: core_sim::CivId,
) -> String {
    civilizations_query
        .iter()
        .find(|civ| civ.id == civ_id)
        .map(|civ| civ.name.clone())
        .unwrap_or_else(|| "Unknown".to_string())
}

/// Helper function to calculate world position of the north neighboring tile
fn calculate_north_tile_world_position(
    unit_position: &Position,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Vec2 {
    let north_tile_pos = TilePos {
        x: unit_position.x as u32,
        y: (unit_position.y as i32 + 1) as u32,
    };

    let mut world_pos =
        north_tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
    world_pos.y += constants::UNIT_LABEL_VERTICAL_OFFSET_PIXELS;
    world_pos
}

/// Helper function to spawn a Text2d unit label in world space
fn spawn_unit_label_text2d(
    commands: &mut Commands,
    unit_entity: Entity,
    unit_position: Position,
    unit_type_name: &str,
    civilization_name: &str,
    world_position: Vec2,
) {
    let label_text = format!("{}\n({})", unit_type_name, civilization_name);

    commands.spawn((
        Text2d::new(label_text),
        TextFont {
            font_size: constants::UNIT_LABEL_FONT_SIZE,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.9)), // Slightly yellowish tint to differentiate from capital labels
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_translation(world_position.extend(constants::UNIT_LABEL_Z_INDEX)),
        UnitLabel {
            unit_entity,
            unit_position,
        },
    ));
}

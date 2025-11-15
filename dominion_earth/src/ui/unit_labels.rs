use crate::debug_println;
use bevy::prelude::*;
use bevy::text::Justify;
use bevy::text::TextBackgroundColor;
use bevy_ecs_tilemap::prelude::*;
use core_sim::{
    components::{military::MilitaryUnit, Civilization},
    Position,
};

/// Component to mark unit label entities
#[derive(Component)]
pub struct UnitLabel {
    pub unit_entity: Entity,
    pub unit_position: Position,
}

/// Component to mark unit label container (parent) entities
#[derive(Component)]
pub struct UnitLabelContainer;

pub mod constants {
    pub const UNIT_LABEL_FONT_SIZE: f32 = 12.0;
    pub const UNIT_LABEL_Z_INDEX: f32 = 99.0;
    pub const UNIT_LABEL_OFFSET_Y: f32 = -25.0;
    pub const UNIT_LABEL_BACKGROUND_ALPHA: f32 = 0.8;
    pub const LABEL_TEXT_COLOR_RED: f32 = 1.0;
    pub const LABEL_TEXT_COLOR_GREEN: f32 = 1.0;
    pub const LABEL_TEXT_COLOR_BLUE: f32 = 1.0;
}

/// System to spawn unit labels using Text2d (world-space text that scales with camera)
/// NOTE: Labels are NOT rendered if a capital exists at the same position
pub fn spawn_unit_labels(
    mut commands: Commands,
    units_query: Query<(Entity, &Position, &MilitaryUnit), Added<MilitaryUnit>>,
    units_without_labels: Query<(Entity, &Position, &MilitaryUnit), Without<UnitLabel>>,
    existing_labels: Query<&UnitLabel>,
    capitals_query: Query<&Position, With<core_sim::components::city::Capital>>,
    civilizations_query: Query<&Civilization>,
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

    // Collect all capital positions
    let capital_positions: Vec<Position> = capitals_query.iter().copied().collect();

    for (unit_entity, position, unit) in units_query.iter() {
        // Skip if a capital exists at this position
        if capital_positions.contains(position) {
            debug_println!(
                "Skipping unit label for {} at ({}, {}) - capital present",
                unit.unit_type.name(),
                position.x,
                position.y
            );
            continue;
        }

        let civilization = civilizations_query.iter().find(|civ| civ.id == unit.owner);
        if civilization.is_none() {
            continue;
        }
        let civ_color = civilization.unwrap().color;

        let world_pos = calculate_unit_label_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_unit_label_text2d(
            &mut commands,
            unit_entity,
            *position,
            unit,
            civ_color,
            world_pos,
        );

        debug_println!(
            "Spawned Text2d unit label for {} at world position ({:.1}, {:.1})",
            unit.unit_type.name(),
            world_pos.x,
            world_pos.y
        );
    }

    for (unit_entity, position, unit) in units_without_labels.iter() {
        // Skip if a capital exists at this position
        if capital_positions.contains(position) {
            continue;
        }

        if existing_labels
            .iter()
            .any(|label| label.unit_entity == unit_entity)
        {
            continue;
        }

        let civilization = civilizations_query.iter().find(|civ| civ.id == unit.owner);
        if civilization.is_none() {
            continue;
        }
        let civ_color = civilization.unwrap().color;

        let world_pos = calculate_unit_label_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_unit_label_text2d(
            &mut commands,
            unit_entity,
            *position,
            unit,
            civ_color,
            world_pos,
        );

        debug_println!(
            "Spawned missing Text2d unit label for {} at world position ({:.1}, {:.1})",
            unit.unit_type.name(),
            world_pos.x,
            world_pos.y
        );
    }
}

/// System to update unit labels when units move
pub fn update_unit_labels(
    mut label_query: Query<(&mut Transform, &UnitLabel)>,
    unit_query: Query<&Position, (With<MilitaryUnit>, Changed<Position>)>,
    capitals_query: Query<&Position, With<core_sim::components::city::Capital>>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    mut commands: Commands,
) {
    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_query.single() else {
        return;
    };

    let capital_positions: Vec<Position> = capitals_query.iter().copied().collect();

    for (mut transform, label) in label_query.iter_mut() {
        if let Ok(new_position) = unit_query.get(label.unit_entity) {
            // If a capital now exists at this position, despawn the label
            if capital_positions.contains(new_position) {
                commands.entity(label.unit_entity).remove::<UnitLabel>();
                continue;
            }

            let world_pos = calculate_unit_label_world_position(
                new_position,
                &map_size,
                &tile_size,
                &grid_size,
                &map_type,
                &anchor,
            );

            transform.translation = world_pos;
        }
    }
}

fn calculate_unit_label_world_position(
    position: &Position,
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
    let mut world_pos = tile_center.extend(constants::UNIT_LABEL_Z_INDEX);
    world_pos.y += constants::UNIT_LABEL_OFFSET_Y;
    world_pos
}

fn spawn_unit_label_text2d(
    commands: &mut Commands,
    unit_entity: Entity,
    position: Position,
    unit: &MilitaryUnit,
    civ_color: [f32; 3],
    world_pos: Vec3,
) {
    let label_text = format!("{}", unit.id);

    let label_container = commands
        .spawn((
            UnitLabelContainer,
            Transform::from_translation(world_pos),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    // Spawn text as a child of the container
    commands.entity(label_container).with_children(|parent| {
        parent.spawn((
            Text2d::new(label_text),
            TextFont {
                font_size: constants::UNIT_LABEL_FONT_SIZE,
                ..default()
            },
            TextColor(Color::srgb(
                constants::LABEL_TEXT_COLOR_RED,
                constants::LABEL_TEXT_COLOR_GREEN,
                constants::LABEL_TEXT_COLOR_BLUE,
            )),
            TextBackgroundColor(
                Color::srgb(civ_color[0], civ_color[1], civ_color[2])
                    .with_alpha(constants::UNIT_LABEL_BACKGROUND_ALPHA),
            ),
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            GlobalTransform::default(),
        ));
    });

    // Add the label component to the unit entity
    commands.entity(unit_entity).insert(UnitLabel {
        unit_entity,
        unit_position: position,
    });
}

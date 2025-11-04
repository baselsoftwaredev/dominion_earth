use crate::debug_println;
use crate::screens::{LoadingState, Screen};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::TextBackgroundColor;
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

/// Component to mark label container (parent) entities
#[derive(Component)]
pub struct UnitLabelContainer;

/// Component to store label colors
#[derive(Component, Clone, Copy, Debug)]
pub struct LabelColors {
    pub background: Color,
    pub icon: Color,
    pub text: Color,
}

pub mod constants {
    pub const UNIT_LABEL_FONT_SIZE: f32 = 14.0;
    pub const UNIT_LABEL_Z_INDEX: f32 = 101.0;
    pub const UNIT_LABEL_NORTH_OFFSET_TILES: f32 = 1.0;
    pub const UNIT_LABEL_VERTICAL_OFFSET_PIXELS: f32 = -40.0;
    pub const UNIT_LABEL_BACKGROUND_ALPHA: f32 = 0.85;
    pub const LABEL_TEXT_COLOR_RED: f32 = 1.0;
    pub const LABEL_TEXT_COLOR_GREEN: f32 = 1.0;
    pub const LABEL_TEXT_COLOR_BLUE: f32 = 1.0;
    pub const UNKNOWN_CIVILIZATION_COLOR_RED: f32 = 0.5;
    pub const UNKNOWN_CIVILIZATION_COLOR_GREEN: f32 = 0.5;
    pub const UNKNOWN_CIVILIZATION_COLOR_BLUE: f32 = 0.5;
    pub const NORTH_TILE_OFFSET_Y: i32 = 1;
    pub const BACKGROUND_DARKNESS_FACTOR: f32 = 0.5;

    // Box and layout constants
    pub const LABEL_BOX_WIDTH: f32 = 120.0;
    pub const LABEL_BOX_HEIGHT: f32 = 80.0;
    pub const ICON_SIZE: f32 = 40.0;
    pub const ICON_PADDING: f32 = 5.0;
    pub const TEXT_PADDING: f32 = 5.0;
    pub const BOX_Z_INDEX: f32 = 100.0;
    pub const ICON_Z_INDEX: f32 = 101.0;
    pub const TEXT_Z_INDEX: f32 = 102.0;
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
) {
    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_query.single() else {
        return;
    };

    for (unit_entity, position, unit) in units_query.iter() {
        let (civilization_name, civilization_color) =
            get_civilization_info(&civilizations_query, unit.owner);

        let north_tile_world_position = calculate_north_tile_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_unit_label_text2d(
            &mut commands,
            unit_entity,
            *position,
            &unit.unit_type.name(),
            &civilization_name,
            civilization_color,
            north_tile_world_position,
        );

        debug_println!(
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

        let (civilization_name, civilization_color) =
            get_civilization_info(&civilizations_query, unit.owner);

        let north_tile_world_position = calculate_north_tile_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_unit_label_text2d(
            &mut commands,
            unit_entity,
            *position,
            &unit.unit_type.name(),
            &civilization_name,
            civilization_color,
            north_tile_world_position,
        );

        debug_println!(
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
    mut label_query: Query<(Entity, &mut Transform, &mut UnitLabel), With<UnitLabelContainer>>,
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

                label_transform.translation = new_world_position.extend(constants::BOX_Z_INDEX);

                // Update cached position
                unit_label.unit_position = *unit_position;
            }
        } else {
            // Unit no longer exists, despawn the label
            commands.entity(label_entity).despawn();
        }
    }
}

/// Helper function to get civilization name and color from query
fn get_civilization_info(
    civilizations_query: &Query<&Civilization>,
    civ_id: core_sim::CivId,
) -> (String, [f32; 3]) {
    civilizations_query
        .iter()
        .find(|civ| civ.id == civ_id)
        .map(|civ| (civ.name.clone(), civ.color))
        .unwrap_or_else(|| {
            (
                "Unknown".to_string(),
                [
                    constants::UNKNOWN_CIVILIZATION_COLOR_RED,
                    constants::UNKNOWN_CIVILIZATION_COLOR_GREEN,
                    constants::UNKNOWN_CIVILIZATION_COLOR_BLUE,
                ],
            )
        })
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
        y: (unit_position.y as i32 + constants::NORTH_TILE_OFFSET_Y) as u32,
    };

    let mut world_pos =
        north_tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
    world_pos.y += constants::UNIT_LABEL_VERTICAL_OFFSET_PIXELS;
    world_pos
}

/// Helper function to spawn a Text2d unit label in world space with box and icon
fn spawn_unit_label_text2d(
    commands: &mut Commands,
    unit_entity: Entity,
    unit_position: Position,
    unit_type_name: &str,
    civilization_name: &str,
    civilization_color: [f32; 3],
    world_position: Vec2,
) {
    let label_colors = create_label_colors(civilization_color);
    let text_color = create_label_text_color();

    // Spawn the container (parent)
    let container = commands
        .spawn((
            Transform::from_translation(world_position.extend(constants::BOX_Z_INDEX)),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            UnitLabelContainer,
            label_colors,
            DespawnOnExit(Screen::Gameplay),
            DespawnOnEnter(LoadingState::Loading),
        ))
        .id();

    // Spawn background box
    commands.entity(container).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: label_colors.background,
                custom_size: Some(Vec2::new(
                    constants::LABEL_BOX_WIDTH,
                    constants::LABEL_BOX_HEIGHT,
                )),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            GlobalTransform::default(),
        ));
    });

    // Spawn icon placeholder (left side)
    commands.entity(container).with_children(|parent| {
        let icon_x = -(constants::LABEL_BOX_WIDTH / 2.0)
            + constants::ICON_SIZE / 2.0
            + constants::ICON_PADDING;
        parent.spawn((
            Sprite {
                color: label_colors.icon,
                custom_size: Some(Vec2::new(constants::ICON_SIZE, constants::ICON_SIZE)),
                ..default()
            },
            Transform::from_translation(Vec3::new(icon_x, 0.0, constants::ICON_Z_INDEX)),
            GlobalTransform::default(),
        ));
    });

    // Spawn text (right side)
    let label_text = format!("{}\n{}", unit_type_name, civilization_name);
    let text_x = constants::ICON_SIZE + constants::ICON_PADDING + constants::TEXT_PADDING;

    commands.entity(container).with_children(|parent| {
        parent.spawn((
            Text2d::new(label_text),
            TextFont {
                font_size: constants::UNIT_LABEL_FONT_SIZE,
                ..default()
            },
            TextColor(text_color),
            TextLayout::new_with_justify(Justify::Left),
            Transform::from_translation(Vec3::new(
                text_x - constants::LABEL_BOX_WIDTH / 2.0,
                0.0,
                constants::TEXT_Z_INDEX,
            )),
            GlobalTransform::default(),
        ));
    });

    // Attach the label component to the container
    commands.entity(container).insert((UnitLabel {
        unit_entity,
        unit_position,
    },));
}

fn create_label_background_color(civilization_color: [f32; 3]) -> Color {
    Color::srgba(
        civilization_color[0] * constants::BACKGROUND_DARKNESS_FACTOR,
        civilization_color[1] * constants::BACKGROUND_DARKNESS_FACTOR,
        civilization_color[2] * constants::BACKGROUND_DARKNESS_FACTOR,
        constants::UNIT_LABEL_BACKGROUND_ALPHA,
    )
}

fn create_label_text_color() -> Color {
    Color::srgb(
        constants::LABEL_TEXT_COLOR_RED,
        constants::LABEL_TEXT_COLOR_GREEN,
        constants::LABEL_TEXT_COLOR_BLUE,
    )
}

fn create_label_colors(civilization_color: [f32; 3]) -> LabelColors {
    LabelColors {
        background: Color::srgba(
            civilization_color[0] * constants::BACKGROUND_DARKNESS_FACTOR,
            civilization_color[1] * constants::BACKGROUND_DARKNESS_FACTOR,
            civilization_color[2] * constants::BACKGROUND_DARKNESS_FACTOR,
            constants::UNIT_LABEL_BACKGROUND_ALPHA,
        ),
        icon: Color::srgba(
            civilization_color[0],
            civilization_color[1],
            civilization_color[2],
            1.0,
        ),
        text: Color::srgb(
            constants::LABEL_TEXT_COLOR_RED,
            constants::LABEL_TEXT_COLOR_GREEN,
            constants::LABEL_TEXT_COLOR_BLUE,
        ),
    }
}

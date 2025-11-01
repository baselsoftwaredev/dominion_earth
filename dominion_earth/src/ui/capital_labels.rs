use crate::debug_println;
use crate::screens::{LoadingState, Screen};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::TextBackgroundColor;
use bevy_ecs_tilemap::prelude::*;
use core_sim::{
    components::{Capital, City, Civilization},
    Position,
};

/// Component to mark capital label entities
#[derive(Component)]
pub struct CapitalLabel {
    pub capital_entity: Entity,
    pub capital_position: Position,
}

/// Component to mark label container (parent) entities
#[derive(Component)]
pub struct CapitalLabelContainer;

/// Component to store label colors
#[derive(Component, Clone, Copy, Debug)]
pub struct LabelColors {
    pub background: Color,
    pub icon: Color,
    pub text: Color,
}

pub mod constants {
    pub const CAPITAL_LABEL_FONT_SIZE: f32 = 16.0;
    pub const CAPITAL_LABEL_Z_INDEX: f32 = 100.0;
    pub const CAPITAL_LABEL_NORTH_OFFSET_TILES: f32 = 1.0;
    pub const CAPITAL_LABEL_VERTICAL_OFFSET_PIXELS: f32 = -40.0;
    pub const CAPITAL_LABEL_BACKGROUND_ALPHA: f32 = 0.85;
    pub const LABEL_TEXT_COLOR_RED: f32 = 1.0;
    pub const LABEL_TEXT_COLOR_GREEN: f32 = 1.0;
    pub const LABEL_TEXT_COLOR_BLUE: f32 = 1.0;
    pub const UNKNOWN_CIVILIZATION_COLOR_RED: f32 = 0.5;
    pub const UNKNOWN_CIVILIZATION_COLOR_GREEN: f32 = 0.5;
    pub const UNKNOWN_CIVILIZATION_COLOR_BLUE: f32 = 0.5;
    pub const NORTH_TILE_OFFSET_Y: i32 = 1;
    pub const BACKGROUND_DARKNESS_FACTOR: f32 = 0.5;

    // Box and layout constants
    pub const LABEL_BOX_WIDTH: f32 = 130.0;
    pub const LABEL_BOX_HEIGHT: f32 = 90.0;
    pub const ICON_SIZE: f32 = 45.0;
    pub const ICON_PADDING: f32 = 5.0;
    pub const TEXT_PADDING: f32 = 5.0;
    pub const BOX_Z_INDEX: f32 = 100.0;
    pub const ICON_Z_INDEX: f32 = 101.0;
    pub const TEXT_Z_INDEX: f32 = 102.0;
    pub const POPULATION_Z_INDEX: f32 = 103.0;

    // Population indicator constants
    pub const POPULATION_CIRCLE_SIZE: f32 = 20.0;
    pub const POPULATION_CIRCLE_COLOR_R: f32 = 0.0;
    pub const POPULATION_CIRCLE_COLOR_G: f32 = 1.0;
    pub const POPULATION_CIRCLE_COLOR_B: f32 = 0.0;
    pub const POPULATION_TEXT_COLOR_R: f32 = 1.0;
    pub const POPULATION_TEXT_COLOR_G: f32 = 1.0;
    pub const POPULATION_TEXT_COLOR_B: f32 = 1.0;
    pub const POPULATION_FONT_SIZE: f32 = 10.0;
    pub const POPULATION_INDICATOR_OFFSET_Y: f32 = 35.0; // Bottom of label
}
/// System to spawn capital labels using Text2d (world-space text that scales with camera)
pub fn spawn_capital_labels(
    mut commands: Commands,
    capitals_query: Query<(Entity, &Position, &Capital, &City), Added<Capital>>,
    capitals_without_labels: Query<(Entity, &Position, &Capital, &City), Without<CapitalLabel>>,
    existing_labels: Query<&CapitalLabel>,
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

    for (capital_entity, position, capital, city) in capitals_query.iter() {
        let (civilization_name, civilization_color) =
            get_civilization_info(&civilizations_query, capital.owner);

        let north_tile_world_position = calculate_north_tile_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_capital_label_text2d(
            &mut commands,
            capital_entity,
            *position,
            &city.name,
            &civilization_name,
            civilization_color,
            city.population,
            north_tile_world_position,
        );

        debug_println!(
            "Spawned Text2d capital label for {} ({}) with population {} at world position ({:.1}, {:.1})",
            city.name,
            civilization_name,
            city.population,
            north_tile_world_position.x,
            north_tile_world_position.y
        );
    }

    for (capital_entity, position, capital, city) in capitals_without_labels.iter() {
        if existing_labels
            .iter()
            .any(|label| label.capital_entity == capital_entity)
        {
            continue;
        }

        let (civilization_name, civilization_color) =
            get_civilization_info(&civilizations_query, capital.owner);

        let north_tile_world_position = calculate_north_tile_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_capital_label_text2d(
            &mut commands,
            capital_entity,
            *position,
            &city.name,
            &civilization_name,
            civilization_color,
            city.population,
            north_tile_world_position,
        );

        debug_println!(
            "Spawned missing Text2d capital label for {} ({}) with population {} at world position ({:.1}, {:.1})",
            city.name,
            civilization_name,
            city.population,
            north_tile_world_position.x,
            north_tile_world_position.y
        );
    }
}

/// System to update capital label positions when capitals move or are destroyed
pub fn update_capital_labels(
    mut commands: Commands,
    mut label_query: Query<(Entity, &mut Transform, &CapitalLabel), With<CapitalLabelContainer>>,
    capitals_query: Query<&Position, With<Capital>>,
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

    for (label_entity, mut label_transform, capital_label) in label_query.iter_mut() {
        if let Ok(capital_position) = capitals_query.get(capital_label.capital_entity) {
            if *capital_position != capital_label.capital_position {
                let new_world_position = calculate_north_tile_world_position(
                    capital_position,
                    map_size,
                    tile_size,
                    grid_size,
                    map_type,
                    anchor,
                );

                label_transform.translation = new_world_position.extend(constants::BOX_Z_INDEX);
            }
        } else {
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
    capital_position: &Position,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Vec2 {
    let north_tile_pos = TilePos {
        x: capital_position.x as u32,
        y: (capital_position.y as i32 + constants::NORTH_TILE_OFFSET_Y) as u32,
    };

    let mut world_pos =
        north_tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
    world_pos.y += constants::CAPITAL_LABEL_VERTICAL_OFFSET_PIXELS;
    world_pos
}

/// Helper function to spawn a Text2d capital label in world space with box and icon
fn spawn_capital_label_text2d(
    commands: &mut Commands,
    capital_entity: Entity,
    capital_position: Position,
    city_name: &str,
    civilization_name: &str,
    civilization_color: [f32; 3],
    population: u32,
    world_position: Vec2,
) {
    let label_colors = create_label_colors(civilization_color);
    let text_color = create_label_text_color();

    // Spawn the container (parent)
    let container = commands
        .spawn((
            Transform::from_translation(world_position.extend(constants::BOX_Z_INDEX)),
            GlobalTransform::default(),
            CapitalLabelContainer,
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
    let label_text = format!("{}\n{}", city_name, civilization_name);
    let text_x = constants::ICON_SIZE + constants::ICON_PADDING + constants::TEXT_PADDING;

    commands.entity(container).with_children(|parent| {
        parent.spawn((
            Text2d::new(label_text),
            TextFont {
                font_size: constants::CAPITAL_LABEL_FONT_SIZE,
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

    // Spawn population indicator (green circle at bottom)
    commands.entity(container).with_children(|parent| {
        let population_circle_color = Color::srgb(
            constants::POPULATION_CIRCLE_COLOR_R,
            constants::POPULATION_CIRCLE_COLOR_G,
            constants::POPULATION_CIRCLE_COLOR_B,
        );

        // Green circle background
        parent.spawn((
            Sprite {
                color: population_circle_color,
                custom_size: Some(Vec2::new(
                    constants::POPULATION_CIRCLE_SIZE,
                    constants::POPULATION_CIRCLE_SIZE,
                )),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                0.0,
                -constants::POPULATION_INDICATOR_OFFSET_Y,
                constants::POPULATION_Z_INDEX,
            )),
            GlobalTransform::default(),
        ));

        // Population text inside circle
        parent.spawn((
            Text2d::new(format!("{}", population / 1000)), // Display in thousands (e.g., "5" for 5000)
            TextFont {
                font_size: constants::POPULATION_FONT_SIZE,
                ..default()
            },
            TextColor(Color::srgb(
                constants::POPULATION_TEXT_COLOR_R,
                constants::POPULATION_TEXT_COLOR_G,
                constants::POPULATION_TEXT_COLOR_B,
            )),
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_translation(Vec3::new(
                0.0,
                -constants::POPULATION_INDICATOR_OFFSET_Y,
                constants::POPULATION_Z_INDEX + 1.0,
            )),
            GlobalTransform::default(),
        ));
    });

    // Attach the label component to the container
    commands.entity(container).insert((CapitalLabel {
        capital_entity,
        capital_position,
    },));
}

fn create_label_background_color(civilization_color: [f32; 3]) -> Color {
    Color::srgba(
        civilization_color[0] * constants::BACKGROUND_DARKNESS_FACTOR,
        civilization_color[1] * constants::BACKGROUND_DARKNESS_FACTOR,
        civilization_color[2] * constants::BACKGROUND_DARKNESS_FACTOR,
        constants::CAPITAL_LABEL_BACKGROUND_ALPHA,
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
            constants::CAPITAL_LABEL_BACKGROUND_ALPHA,
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

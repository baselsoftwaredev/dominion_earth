use crate::debug_println;
use crate::debug_utils::DebugLogging;
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

pub mod constants {
    pub const CAPITAL_LABEL_FONT_SIZE: f32 = 16.0;
    pub const CAPITAL_LABEL_Z_INDEX: f32 = 100.0;
    pub const CAPITAL_LABEL_NORTH_OFFSET_TILES: f32 = 1.0;
    pub const CAPITAL_LABEL_VERTICAL_OFFSET_PIXELS: f32 = -40.0;
    pub const CAPITAL_LABEL_BACKGROUND_ALPHA: f32 = 0.7;
    pub const LABEL_TEXT_COLOR_RED: f32 = 1.0;
    pub const LABEL_TEXT_COLOR_GREEN: f32 = 1.0;
    pub const LABEL_TEXT_COLOR_BLUE: f32 = 1.0;
    pub const UNKNOWN_CIVILIZATION_COLOR_RED: f32 = 0.5;
    pub const UNKNOWN_CIVILIZATION_COLOR_GREEN: f32 = 0.5;
    pub const UNKNOWN_CIVILIZATION_COLOR_BLUE: f32 = 0.5;
    pub const NORTH_TILE_OFFSET_Y: i32 = 1;
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
    debug_logging: Res<DebugLogging>,
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
            north_tile_world_position,
        );

        debug_println!(
            debug_logging,
            "Spawned Text2d capital label for {} ({}) at world position ({:.1}, {:.1})",
            city.name,
            civilization_name,
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
            north_tile_world_position,
        );

        debug_println!(
            debug_logging,
            "Spawned missing Text2d capital label for {} ({}) at world position ({:.1}, {:.1})",
            city.name,
            civilization_name,
            north_tile_world_position.x,
            north_tile_world_position.y
        );
    }
}

/// System to update capital label positions when capitals move or are destroyed
pub fn update_capital_labels(
    mut commands: Commands,
    mut label_query: Query<(Entity, &mut Transform, &CapitalLabel)>,
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

                label_transform.translation =
                    new_world_position.extend(constants::CAPITAL_LABEL_Z_INDEX);
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

/// Helper function to spawn a Text2d capital label in world space
fn spawn_capital_label_text2d(
    commands: &mut Commands,
    capital_entity: Entity,
    capital_position: Position,
    city_name: &str,
    civilization_name: &str,
    civilization_color: [f32; 3],
    world_position: Vec2,
) {
    let label_text = format!("{}\n({})", city_name, civilization_name);

    let background_color = create_label_background_color(civilization_color);
    let text_color = create_label_text_color();

    commands.spawn((
        Text2d::new(label_text),
        TextFont {
            font_size: constants::CAPITAL_LABEL_FONT_SIZE,
            ..default()
        },
        TextColor(text_color),
        TextBackgroundColor(background_color),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_translation(world_position.extend(constants::CAPITAL_LABEL_Z_INDEX)),
        CapitalLabel {
            capital_entity,
            capital_position,
        },
    ));
}

fn create_label_background_color(civilization_color: [f32; 3]) -> Color {
    Color::srgba(
        civilization_color[0],
        civilization_color[1],
        civilization_color[2],
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

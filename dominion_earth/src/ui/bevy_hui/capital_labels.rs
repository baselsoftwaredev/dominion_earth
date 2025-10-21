use crate::debug_println;
use crate::debug_utils::DebugLogging;
use bevy::prelude::*;
use bevy::sprite::Anchor;
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

/// Constants for capital label styling
pub mod constants {
    pub const CAPITAL_LABEL_FONT_SIZE: f32 = 16.0;
    pub const CAPITAL_LABEL_Z_INDEX: f32 = 100.0;
    pub const CAPITAL_LABEL_NORTH_OFFSET_TILES: f32 = 1.0;
    pub const CAPITAL_LABEL_VERTICAL_OFFSET_PIXELS: f32 = -40.0;
    pub const CAPITAL_LABEL_BACKGROUND_ALPHA: f32 = 0.7;
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
        let civilization_name = get_civilization_name(&civilizations_query, capital.owner);

        let north_tile_world_position = calculate_north_tile_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_capital_label_text2d(
            &mut commands,
            capital_entity,
            *position,
            &city.name,
            &civilization_name,
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

        let civilization_name = get_civilization_name(&civilizations_query, capital.owner);

        let north_tile_world_position = calculate_north_tile_world_position(
            position, map_size, tile_size, grid_size, map_type, anchor,
        );

        spawn_capital_label_text2d(
            &mut commands,
            capital_entity,
            *position,
            &city.name,
            &civilization_name,
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
    capital_position: &Position,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Vec2 {
    let north_tile_pos = TilePos {
        x: capital_position.x as u32,
        y: (capital_position.y as i32 + 1) as u32,
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
    world_position: Vec2,
) {
    let label_text = format!("{}\n({})", city_name, civilization_name);

    commands.spawn((
        Text2d::new(label_text),
        TextFont {
            font_size: constants::CAPITAL_LABEL_FONT_SIZE,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(world_position.extend(constants::CAPITAL_LABEL_Z_INDEX)),
        CapitalLabel {
            capital_entity,
            capital_position,
        },
    ));
}

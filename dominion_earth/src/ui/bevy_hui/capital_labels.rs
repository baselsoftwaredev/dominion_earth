use crate::debug_println;
use crate::debug_utils::DebugLogging;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_hui::prelude::*;
use core_sim::{
    components::{Capital, City},
    Position,
};

use super::constants;

/// Component to mark capital label entities
#[derive(Component)]
pub struct CapitalLabel {
    pub capital_entity: Entity,
}

struct TilemapCoordinateData {
    tilemap_size: TilemapSize,
    tile_size: TilemapTileSize,
    grid_size: TilemapGridSize,
    map_type: TilemapType,
    anchor: TilemapAnchor,
}

struct LabelWorldCoordinates {
    x: f32,
    y: f32,
}

/// System to spawn capital labels over capital tiles
pub fn spawn_capital_labels(
    mut commands_for_spawning: Commands,
    asset_server: Res<AssetServer>,
    capitals_requiring_labels: Query<
        (Entity, &Capital, &Position),
        (With<City>, Without<CapitalLabel>),
    >,
    cities_with_names: Query<&City>,
    tilemap_for_coordinate_conversion: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    existing_capital_labels: Query<&CapitalLabel>,
    debug_logging: Res<DebugLogging>,
) {
    debug_println!(debug_logging, "DEBUG: spawn_capital_labels system running");

    let tilemap_data = match extract_tilemap_information_for_coordinate_conversion(
        &tilemap_for_coordinate_conversion,
        &debug_logging,
    ) {
        Some(data) => data,
        None => return,
    };

    debug_println!(
        debug_logging,
        "DEBUG: Found {} capitals for label spawning",
        capitals_requiring_labels.iter().count()
    );

    for (capital_entity, _capital_component, capital_position) in capitals_requiring_labels.iter() {
        if capital_label_already_exists_for_entity(capital_entity, &existing_capital_labels) {
            continue;
        }

        debug_println!(
            debug_logging,
            "DEBUG: Processing NEW capital at position ({}, {})",
            capital_position.x,
            capital_position.y
        );

        let capital_display_name = extract_capital_display_name_from_city_component(
            capital_entity,
            &cities_with_names,
            &debug_logging,
        );

        let label_world_coordinates =
            calculate_capital_label_world_position(capital_position, &tilemap_data);

        spawn_single_capital_label_entity(
            &mut commands_for_spawning,
            &asset_server,
            capital_entity,
            &capital_display_name,
            &label_world_coordinates,
            &debug_logging,
        );
    }
}

/// System to update capital label positions when needed
pub fn update_capital_labels(
    mut capital_labels_requiring_updates: Query<
        (&CapitalLabel, &mut TemplateProperties),
        Changed<Position>,
    >,
    capital_positions: Query<&Position, With<Capital>>,
    capital_city_names: Query<&City, Changed<City>>,
    tilemap_for_coordinate_conversion: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
) {
    let tilemap_data = match extract_tilemap_information_for_coordinate_conversion(
        &tilemap_for_coordinate_conversion,
        &DebugLogging(false),
    ) {
        Some(data) => data,
        None => return,
    };

    for (capital_label, mut template_properties) in capital_labels_requiring_updates.iter_mut() {
        update_capital_label_position_if_changed(
            capital_label,
            &mut template_properties,
            &capital_positions,
            &tilemap_data,
        );

        update_capital_label_name_if_changed(
            capital_label,
            &mut template_properties,
            &capital_city_names,
        );
    }
}

fn extract_tilemap_information_for_coordinate_conversion(
    tilemap_query: &Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    debug_logging: &DebugLogging,
) -> Option<TilemapCoordinateData> {
    match tilemap_query.single() {
        Ok((tilemap_size, tile_size, grid_size, map_type, anchor)) => Some(TilemapCoordinateData {
            tilemap_size: *tilemap_size,
            tile_size: *tile_size,
            grid_size: *grid_size,
            map_type: *map_type,
            anchor: *anchor,
        }),
        Err(_) => {
            debug_println!(debug_logging, "DEBUG: No tilemap found for capital labels");
            None
        }
    }
}

fn capital_label_already_exists_for_entity(
    capital_entity: Entity,
    existing_labels: &Query<&CapitalLabel>,
) -> bool {
    existing_labels
        .iter()
        .any(|label| label.capital_entity == capital_entity)
}

fn extract_capital_display_name_from_city_component(
    capital_entity: Entity,
    cities_query: &Query<&City>,
    debug_logging: &DebugLogging,
) -> String {
    match cities_query.get(capital_entity) {
        Ok(city) => {
            debug_println!(debug_logging, "DEBUG: Found capital name: {}", city.name);
            city.name.clone()
        }
        Err(_) => {
            debug_println!(
                debug_logging,
                "DEBUG: No city component found, using default name"
            );
            constants::capital_labels::DEFAULT_CAPITAL_NAME.to_string()
        }
    }
}

fn calculate_capital_label_world_position(
    capital_position: &Position,
    tilemap_data: &TilemapCoordinateData,
) -> LabelWorldCoordinates {
    let tile_position = TilePos {
        x: capital_position.x as u32,
        y: capital_position.y as u32,
    };

    let world_position = tile_position.center_in_world(
        &tilemap_data.tilemap_size,
        &tilemap_data.grid_size,
        &tilemap_data.tile_size,
        &tilemap_data.map_type,
        &tilemap_data.anchor,
    );

    LabelWorldCoordinates {
        x: world_position.x,
        y: world_position.y + constants::capital_labels::LABEL_VERTICAL_OFFSET,
    }
}

fn spawn_single_capital_label_entity(
    commands: &mut Commands,
    asset_server: &AssetServer,
    capital_entity: Entity,
    capital_name: &str,
    world_coordinates: &LabelWorldCoordinates,
    debug_logging: &DebugLogging,
) {
    debug_println!(
        debug_logging,
        "DEBUG: Spawning capital label '{}' at world position ({}, {})",
        capital_name,
        world_coordinates.x,
        world_coordinates.y
    );

    commands.spawn((
        HtmlNode(asset_server.load(constants::capital_labels::CAPITAL_LABEL_TEMPLATE_PATH)),
        TemplateProperties::default()
            .with(
                constants::capital_labels::CAPITAL_NAME_PROPERTY_KEY,
                capital_name,
            )
            .with(
                constants::capital_labels::POSITION_X_PROPERTY_KEY,
                &world_coordinates.x.to_string(),
            )
            .with(
                constants::capital_labels::POSITION_Y_PROPERTY_KEY,
                &world_coordinates.y.to_string(),
            ),
        CapitalLabel { capital_entity },
        Name::new(constants::capital_labels::CAPITAL_LABEL_COMPONENT_NAME),
    ));
}

fn update_capital_label_position_if_changed(
    capital_label: &CapitalLabel,
    template_properties: &mut TemplateProperties,
    capital_positions: &Query<&Position, With<Capital>>,
    tilemap_data: &TilemapCoordinateData,
) {
    if let Ok(capital_position) = capital_positions.get(capital_label.capital_entity) {
        let label_world_coordinates =
            calculate_capital_label_world_position(capital_position, tilemap_data);

        template_properties.insert(
            constants::capital_labels::POSITION_X_PROPERTY_KEY.to_string(),
            label_world_coordinates.x.to_string(),
        );
        template_properties.insert(
            constants::capital_labels::POSITION_Y_PROPERTY_KEY.to_string(),
            label_world_coordinates.y.to_string(),
        );
    }
}

fn update_capital_label_name_if_changed(
    capital_label: &CapitalLabel,
    template_properties: &mut TemplateProperties,
    capital_city_names: &Query<&City, Changed<City>>,
) {
    if let Ok(city) = capital_city_names.get(capital_label.capital_entity) {
        template_properties.insert(
            constants::capital_labels::CAPITAL_NAME_PROPERTY_KEY.to_string(),
            city.name.clone(),
        );
    }
}

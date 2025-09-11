use crate::debug_println;
use crate::debug_utils::DebugLogging;
use crate::ui::constants::display_layout;
use bevy::prelude::*;
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
    pub const CAPITAL_LABEL_FONT_SIZE: f32 = 14.0;
    pub const CAPITAL_LABEL_Z_INDEX: i32 = 100; // High z-index to appear above other UI
    pub const CAPITAL_LABEL_NORTH_OFFSET_TILES: f32 = 1.0; // Position on north neighboring tile
    pub const CAPITAL_LABEL_VERTICAL_CENTER_OFFSET: f32 = 0.0; // Center vertically on the north tile
    pub const CAPITAL_LABEL_BACKGROUND_PADDING: f32 = 4.0;
    pub const CAPITAL_LABEL_HORIZONTAL_CENTER_OFFSET: f32 = 30.0; // Approximate text width for centering
    pub const LABEL_APPROXIMATE_WIDTH: f32 = 120.0; // Estimated label width for boundary checks
}/// System to spawn capital labels using traditional Bevy UI
pub fn spawn_capital_labels(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    capitals_query: Query<(Entity, &Position, &Capital, &City), Added<Capital>>,
    civilizations_query: Query<&Civilization>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    window_query: Query<&Window>,
    debug_logging: Res<DebugLogging>,
) {
    if capitals_query.is_empty() {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_query.single() else {
        return;
    };

    let Ok(window) = window_query.single() else {
        return;
    };

    for (capital_entity, position, capital, city) in capitals_query.iter() {
        // Get civilization name for the label
        let civilization_name = civilizations_query
            .iter()
            .find(|civ| civ.id == capital.owner)
            .map(|civ| civ.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Convert tile position to world coordinates
        let tile_pos = TilePos {
            x: position.x as u32,
            y: position.y as u32,
        };
        let world_pos = tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

        // Calculate the north neighboring tile position for label placement (like Civilization games)
        let north_tile_pos = TilePos {
            x: position.x as u32,
            y: (position.y + 1) as u32, // Move one tile north (positive Y direction)
        };
        let north_world_pos =
            north_tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

        // Convert north tile world coordinates to screen coordinates for label positioning
        if let Ok(screen_pos) =
            camera.world_to_viewport(camera_transform, north_world_pos.extend(0.0))
        {
            // Check if the label would be obscured by UI panels
            if !is_position_obscured_by_ui_panels(screen_pos, window.width()) {
                spawn_capital_label_ui_node(
                    &mut commands,
                    capital_entity,
                    *position,
                    &city.name,
                    &civilization_name,
                    screen_pos,
                );

                debug_println!(
                    debug_logging,
                    "Spawned capital label for {} ({}) at screen position ({:.1}, {:.1})",
                    city.name,
                    civilization_name,
                    screen_pos.x,
                    screen_pos.y
                );
            } else {
                debug_println!(
                    debug_logging,
                    "Skipped capital label for {} ({}) - would be obscured by UI panel",
                    city.name,
                    civilization_name
                );
            }
        }
    }
}

/// System to update capital label positions when camera moves
pub fn update_capital_labels(
    mut label_style_query: Query<(&mut Node, &mut Visibility, &CapitalLabel)>,
    camera_query: Query<(&Camera, &GlobalTransform), Changed<GlobalTransform>>,
    capitals_query: Query<(&Position, &Capital, &City)>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    window_query: Query<&Window>,
) {
    // Only update when camera transform changes
    if camera_query.is_empty() {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_query.single() else {
        return;
    };

    let Ok(window) = window_query.single() else {
        return;
    };

    // Update existing label positions and visibility instead of recreating them
    for (mut node_style, mut visibility, capital_label) in label_style_query.iter_mut() {
        if let Ok((position, _capital, _city)) = capitals_query.get(capital_label.capital_entity) {
            // Calculate the north neighboring tile position for label placement (like Civilization games)
            let north_tile_pos = TilePos {
                x: position.x as u32,
                y: (position.y + 1) as u32, // Move one tile north (positive Y direction)
            };
            let north_world_pos =
                north_tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

            // Convert north tile world coordinates to screen coordinates for label positioning
            if let Ok(screen_pos) =
                camera.world_to_viewport(camera_transform, north_world_pos.extend(0.0))
            {
                // Check if the label would be obscured by UI panels
                if !is_position_obscured_by_ui_panels(screen_pos, window.width()) {
                    // Update the existing label position and make it visible
                    node_style.left =
                        Val::Px(screen_pos.x - constants::CAPITAL_LABEL_HORIZONTAL_CENTER_OFFSET);
                    node_style.top =
                        Val::Px(screen_pos.y + constants::CAPITAL_LABEL_VERTICAL_CENTER_OFFSET);
                    *visibility = Visibility::Inherited;
                } else {
                    // Hide the label if it would be obscured by UI panels
                    *visibility = Visibility::Hidden;
                }
            } else {
                // Hide the label if it's not visible on screen
                *visibility = Visibility::Hidden;
            }
        }
    }
}

/// Helper function to check if a screen position would be obscured by UI panels
fn is_position_obscured_by_ui_panels(screen_position: Vec2, window_width: f32) -> bool {
    let label_left = screen_position.x - constants::CAPITAL_LABEL_HORIZONTAL_CENTER_OFFSET;
    let label_right = label_left + constants::LABEL_APPROXIMATE_WIDTH;
    let label_top = screen_position.y + constants::CAPITAL_LABEL_VERTICAL_CENTER_OFFSET;

    // Check if label overlaps with top panel
    if label_top < display_layout::HEADER_HEIGHT {
        return true;
    }

    // Check if label overlaps with left side panel
    if label_left < display_layout::LEFT_SIDEBAR_WIDTH {
        return true;
    }

    // Check if label overlaps with right side panel
    let right_panel_left = window_width - display_layout::RIGHT_SIDEBAR_WIDTH;
    if label_right > right_panel_left {
        return true;
    }

    false
}

/// Helper function to spawn a capital label UI node
fn spawn_capital_label_ui_node(
    commands: &mut Commands,
    capital_entity: Entity,
    capital_position: Position,
    city_name: &str,
    civilization_name: &str,
    screen_position: Vec2,
) {
    let label_text = format!("{}\n({})", city_name, civilization_name);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(
                    screen_position.x - constants::CAPITAL_LABEL_HORIZONTAL_CENTER_OFFSET,
                ), // Center horizontally on the north tile
                top: Val::Px(screen_position.y + constants::CAPITAL_LABEL_VERTICAL_CENTER_OFFSET), // Center vertically on north tile
                padding: UiRect::all(Val::Px(constants::CAPITAL_LABEL_BACKGROUND_PADDING)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // Semi-transparent black background
            ZIndex(constants::CAPITAL_LABEL_Z_INDEX),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label_text),
                TextFont {
                    font_size: constants::CAPITAL_LABEL_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        })
        .insert(CapitalLabel {
            capital_entity,
            capital_position,
        });
}

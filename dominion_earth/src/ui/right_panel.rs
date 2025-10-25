//! Native Bevy UI implementation for the right side panel.
//!
//! Displays world statistics, hovered tile information, civilizations list, and minimap.

use bevy::prelude::*;
use core_sim::{components::TerrainType, resources::CurrentTurn, Civilization, PlayerControlled};

use crate::ui::constants::display_layout;
use crate::ui::resources::{HoveredTile, TerrainCounts};

// ============================================================================
// Marker Components
// ============================================================================

/// Marker component for the right panel container
#[derive(Component)]
pub struct RightPanel;

// Statistics Panel Components
/// Marker component for the statistics panel section
#[derive(Component)]
pub struct StatisticsPanel;

#[derive(Component)]
pub struct StatisticsTurnText;

#[derive(Component)]
pub struct StatisticsLandText;

#[derive(Component)]
pub struct StatisticsWaterText;

#[derive(Component)]
pub struct StatisticsMountainText;

// Hovered Tile Info Components
/// Marker component for the hovered tile info section
#[derive(Component)]
pub struct HoveredTileInfoPanel;

#[derive(Component)]
pub struct HoveredPositionText;

#[derive(Component)]
pub struct HoveredTerrainText;

// Civilizations List Components
/// Marker component for the civilizations list section
#[derive(Component)]
pub struct CivilizationsListPanel;

#[derive(Component)]
pub struct CivilizationsListText;

// Minimap Components
/// Marker component for the minimap section
#[derive(Component)]
pub struct MinimapPanel;

// ============================================================================
// Setup System
// ============================================================================

/// Spawn the right side panel UI hierarchy
pub fn spawn_right_panel(mut commands: Commands) {
    commands
        .spawn((
            RightPanel,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(display_layout::HEADER_HEIGHT),
                width: Val::Px(display_layout::RIGHT_SIDEBAR_WIDTH),
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                overflow: Overflow::visible(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.102, 0.102, 0.102, 1.0)), // #1a1a1a
            Name::new("Right Panel"),
        ))
        .with_children(|parent| {
            // Statistics Panel
            parent
                .spawn((
                    StatisticsPanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(150.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                    BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    BorderRadius::all(Val::Px(8.0)),
                    Name::new("Statistics Panel"),
                ))
                .with_children(|stats_parent| {
                    // Panel title
                    stats_parent.spawn((
                        Text::new("World Statistics"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Node {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                        Name::new("Statistics Title"),
                    ));

                    // Turn info
                    stats_parent.spawn((
                        StatisticsTurnText,
                        Text::new("Turn: 1"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        Name::new("Turn Text"),
                    ));

                    // Land count
                    stats_parent.spawn((
                        StatisticsLandText,
                        Text::new("Land: 0"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                        Name::new("Land Count Text"),
                    ));

                    // Water count
                    stats_parent.spawn((
                        StatisticsWaterText,
                        Text::new("Water: 0"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                        Name::new("Water Count Text"),
                    ));

                    // Mountain count
                    stats_parent.spawn((
                        StatisticsMountainText,
                        Text::new("Mountains: 0"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Name::new("Mountain Count Text"),
                    ));
                });

            // Hovered Tile Info
            parent
                .spawn((
                    HoveredTileInfoPanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(120.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                    BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    BorderRadius::all(Val::Px(8.0)),
                    Name::new("Hovered Tile Info Panel"),
                ))
                .with_children(|tile_parent| {
                    // Panel title
                    tile_parent.spawn((
                        Text::new("Hovered Tile"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Node {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                        Name::new("Hovered Tile Title"),
                    ));

                    // Position
                    tile_parent.spawn((
                        HoveredPositionText,
                        Text::new("Position: None"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                        Name::new("Hovered Position Text"),
                    ));

                    // Terrain
                    tile_parent.spawn((
                        HoveredTerrainText,
                        Text::new("Terrain: None"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Name::new("Hovered Terrain Text"),
                    ));
                });

            // Civilizations List
            parent
                .spawn((
                    CivilizationsListPanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                    BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    BorderRadius::all(Val::Px(8.0)),
                    Name::new("Civilizations List Panel"),
                ))
                .with_children(|civs_parent| {
                    // Panel title
                    civs_parent.spawn((
                        Text::new("Civilizations"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Node {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                        Name::new("Civilizations Title"),
                    ));

                    // Civilizations text (dynamic)
                    civs_parent.spawn((
                        CivilizationsListText,
                        Text::new("Loading..."),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)), // #cccccc
                        Name::new("Civilizations Text"),
                    ));
                });

            // Minimap
            parent
                .spawn((
                    MinimapPanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(150.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                    BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    BorderRadius::all(Val::Px(8.0)),
                    Name::new("Minimap Panel"),
                ))
                .with_children(|minimap_parent| {
                    // Panel title
                    minimap_parent.spawn((
                        Text::new("Minimap"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Node {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                        Name::new("Minimap Title"),
                    ));

                    // Placeholder content
                    minimap_parent.spawn((
                        Text::new("Map view placeholder"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.533, 0.533, 0.533, 1.0)), // #888888
                        Name::new("Minimap Placeholder"),
                    ));
                });
        });
}

// ============================================================================
// Update Systems
// ============================================================================

/// Update statistics panel with current turn and terrain counts
pub fn update_statistics_panel(
    current_turn: Res<CurrentTurn>,
    terrain_counts: Res<TerrainCounts>,
    mut turn_text: Query<
        &mut Text,
        (
            With<StatisticsTurnText>,
            Without<StatisticsLandText>,
            Without<StatisticsWaterText>,
            Without<StatisticsMountainText>,
        ),
    >,
    mut land_text: Query<
        &mut Text,
        (
            With<StatisticsLandText>,
            Without<StatisticsTurnText>,
            Without<StatisticsWaterText>,
            Without<StatisticsMountainText>,
        ),
    >,
    mut water_text: Query<
        &mut Text,
        (
            With<StatisticsWaterText>,
            Without<StatisticsTurnText>,
            Without<StatisticsLandText>,
            Without<StatisticsMountainText>,
        ),
    >,
    mut mountain_text: Query<
        &mut Text,
        (
            With<StatisticsMountainText>,
            Without<StatisticsTurnText>,
            Without<StatisticsLandText>,
            Without<StatisticsWaterText>,
        ),
    >,
) {
    // Update turn text when it changes
    if current_turn.is_changed() {
        if let Some(mut text) = turn_text.iter_mut().next() {
            **text = format!("Turn: {}", current_turn.0);
        }
    }

    // Update terrain counts when they change
    if terrain_counts.is_changed() {
        let land_count = terrain_counts.plains
            + terrain_counts.hills
            + terrain_counts.forest
            + terrain_counts.desert;

        let water_count = terrain_counts.ocean + terrain_counts.coast + terrain_counts.river;

        if let Some(mut text) = land_text.iter_mut().next() {
            **text = format!("Land: {}", land_count);
        }

        if let Some(mut text) = water_text.iter_mut().next() {
            **text = format!("Water: {}", water_count);
        }

        if let Some(mut text) = mountain_text.iter_mut().next() {
            **text = format!("Mountains: {}", terrain_counts.mountains);
        }
    }
}

/// Update hovered tile information
pub fn update_hovered_tile_info(
    hovered_tile: Res<HoveredTile>,
    mut position_text: Query<&mut Text, (With<HoveredPositionText>, Without<HoveredTerrainText>)>,
    mut terrain_text: Query<&mut Text, (With<HoveredTerrainText>, Without<HoveredPositionText>)>,
) {
    if hovered_tile.is_changed() {
        match hovered_tile.position {
            Some(position) => {
                // Update position
                if let Some(mut text) = position_text.iter_mut().next() {
                    **text = format!("Position: ({}, {})", position.x, position.y);
                }

                // Update terrain
                if let Some(mut text) = terrain_text.iter_mut().next() {
                    let terrain_name = match &hovered_tile.terrain_type {
                        Some(terrain) => format_terrain_type(terrain),
                        None => "Unknown".to_string(),
                    };
                    **text = format!("Terrain: {}", terrain_name);
                }
            }
            None => {
                // No tile hovered
                if let Some(mut text) = position_text.iter_mut().next() {
                    **text = "Position: None".to_string();
                }

                if let Some(mut text) = terrain_text.iter_mut().next() {
                    **text = "Terrain: None".to_string();
                }
            }
        }
    }
}

/// Update civilizations list
pub fn update_civilizations_list(
    civs: Query<&Civilization>,
    player_civs: Query<&Civilization, With<PlayerControlled>>,
    mut civs_text: Query<&mut Text, With<CivilizationsListText>>,
) {
    // Only update if civilizations changed (we check via query changes)
    // For now, we'll update every frame when the query has results
    let all_civs: Vec<&Civilization> = civs.iter().collect();

    if all_civs.is_empty() {
        if let Some(mut text) = civs_text.iter_mut().next() {
            **text = "No civilizations yet".to_string();
        }
        return;
    }

    let civ_details: Vec<String> = all_civs
        .iter()
        .map(|civ| {
            let civ_type = if player_civs.iter().any(|pc| pc.id == civ.id) {
                "Player"
            } else {
                "AI"
            };
            format!(
                "{} - {} (Gold: {})",
                civ.name, civ_type, civ.economy.gold as i32
            )
        })
        .collect();

    if let Some(mut text) = civs_text.iter_mut().next() {
        **text = civ_details.join(", ");
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Format terrain type for display
fn format_terrain_type(terrain: &TerrainType) -> String {
    match terrain {
        TerrainType::Plains => "Plains".to_string(),
        TerrainType::Hills => "Hills".to_string(),
        TerrainType::Mountains => "Mountains".to_string(),
        TerrainType::Forest => "Forest".to_string(),
        TerrainType::Desert => "Desert".to_string(),
        TerrainType::Coast => "Coast".to_string(),
        TerrainType::ShallowCoast => "Shallow Coast".to_string(),
        TerrainType::Ocean => "Ocean".to_string(),
        TerrainType::River => "River".to_string(),
    }
}

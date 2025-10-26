pub mod civilizations_section;
pub mod constants;
pub mod hovered_tile_section;
pub mod statistics_section;

use bevy::prelude::*;

use crate::ui::constants::display_layout;
use constants::*;

// Re-export components from sub-modules
pub use civilizations_section::*;
pub use hovered_tile_section::*;
pub use statistics_section::*;

// ============================================================================
// Main Right Panel Components
// ============================================================================

/// Marker component for the right panel container
#[derive(Component)]
pub struct RightPanel;

// ============================================================================
// Setup System
// ============================================================================

/// Spawn the right side panel UI hierarchy with all sections
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
                        min_height: STATISTICS_PANEL_MIN_HEIGHT,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Statistics Panel"),
                ))
                .with_children(|stats_parent| {
                    // Panel title
                    stats_parent.spawn((
                        Text::new("World Statistics"),
                        TextFont {
                            font_size: TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Statistics Title"),
                    ));

                    // Turn info
                    stats_parent.spawn((
                        StatisticsTurnText,
                        Text::new("Turn: 1"),
                        TextFont {
                            font_size: SUBTITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Node {
                            margin: UiRect::bottom(SECTION_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Turn Text"),
                    ));

                    // Land count
                    stats_parent.spawn((
                        StatisticsLandText,
                        Text::new("Land: 0"),
                        TextFont {
                            font_size: SUBTITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Node {
                            margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Land Count Text"),
                    ));

                    // Water count
                    stats_parent.spawn((
                        StatisticsWaterText,
                        Text::new("Water: 0"),
                        TextFont {
                            font_size: SUBTITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Node {
                            margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Water Count Text"),
                    ));

                    // Mountain count
                    stats_parent.spawn((
                        StatisticsMountainText,
                        Text::new("Mountains: 0"),
                        TextFont {
                            font_size: SUBTITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Name::new("Mountain Count Text"),
                    ));
                });

            // Hovered Tile Info
            parent
                .spawn((
                    HoveredTileInfoPanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: HOVERED_TILE_PANEL_MIN_HEIGHT,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Hovered Tile Info Panel"),
                ))
                .with_children(|tile_parent| {
                    // Panel title
                    tile_parent.spawn((
                        Text::new("Hovered Tile"),
                        TextFont {
                            font_size: TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Hovered Tile Title"),
                    ));

                    // Position
                    tile_parent.spawn((
                        HoveredPositionText,
                        Text::new("Position: None"),
                        TextFont {
                            font_size: SUBTITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Node {
                            margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Hovered Position Text"),
                    ));

                    // Terrain
                    tile_parent.spawn((
                        HoveredTerrainText,
                        Text::new("Terrain: None"),
                        TextFont {
                            font_size: SUBTITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Name::new("Hovered Terrain Text"),
                    ));
                });

            // Civilizations List
            parent
                .spawn((
                    CivilizationsListPanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: CIVILIZATIONS_PANEL_MIN_HEIGHT,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Civilizations List Panel"),
                ))
                .with_children(|civs_parent| {
                    // Panel title
                    civs_parent.spawn((
                        Text::new("Civilizations"),
                        TextFont {
                            font_size: TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Civilizations Title"),
                    ));

                    // Civilizations text (dynamic)
                    civs_parent.spawn((
                        CivilizationsListText,
                        Text::new("Loading..."),
                        TextFont {
                            font_size: BODY_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_SECONDARY),
                        Name::new("Civilizations Text"),
                    ));
                });

            // Minimap
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        min_height: MINIMAP_PANEL_MIN_HEIGHT,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Minimap Panel"),
                ))
                .with_children(|minimap_parent| {
                    // Panel title
                    minimap_parent.spawn((
                        Text::new("Minimap"),
                        TextFont {
                            font_size: TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Minimap Title"),
                    ));

                    // Placeholder content
                    minimap_parent.spawn((
                        Text::new("Map view placeholder"),
                        TextFont {
                            font_size: SUBTITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_TERTIARY),
                        Name::new("Minimap Placeholder"),
                    ));
                });
        });
}

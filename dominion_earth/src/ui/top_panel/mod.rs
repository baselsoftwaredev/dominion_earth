pub mod constants;
pub mod resources_section;
pub mod turn_section;

use bevy::prelude::*;

use crate::ui::constants::display_layout;
use constants::*;

pub use resources_section::*;
pub use turn_section::*;

// ============================================================================
// Marker Components
// ============================================================================

/// Marker component for the top panel container
#[derive(Component)]
pub struct TopPanel;

/// Marker component for the game title text
#[derive(Component)]
pub struct GameTitleText;

// ============================================================================
// Setup System
// ============================================================================

/// Spawn the top panel UI hierarchy
pub fn spawn_top_panel(mut commands: Commands) {
    commands
        .spawn((
            TopPanel,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(display_layout::HEADER_HEIGHT),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(PANEL_BACKGROUND),
            Name::new("Top Panel"),
        ))
        .with_children(|parent| {
            // Game Title (left side)
            parent.spawn((
                GameTitleText,
                Text::new("Dominion Earth"),
                TextFont {
                    font_size: TITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(TITLE_COLOR),
                Node {
                    margin: UiRect::horizontal(Val::Px(20.0)),
                    ..default()
                },
                Name::new("Game Title"),
            ));

            // Stats Container (right side)
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        min_width: STATS_CONTAINER_MIN_WIDTH,
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        ..default()
                    },
                    BackgroundColor(STATS_CONTAINER_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Stats Container"),
                ))
                .with_children(|stats_parent| {
                    // Gold Display
                    stats_parent.spawn((
                        GoldDisplayText,
                        Text::new("Gold: 0"),
                        TextFont {
                            font_size: STATS_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Name::new("Gold Display"),
                    ));

                    // Production Display
                    stats_parent.spawn((
                        ProductionDisplayText,
                        Text::new("Production: 0"),
                        TextFont {
                            font_size: STATS_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Name::new("Production Display"),
                    ));

                    // Turn Display
                    stats_parent.spawn((
                        TurnDisplayText,
                        Text::new("Turn: 1"),
                        TextFont {
                            font_size: STATS_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Name::new("Turn Display"),
                    ));
                });
        });
}

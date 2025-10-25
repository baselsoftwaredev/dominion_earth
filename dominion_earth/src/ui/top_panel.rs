//! Native Bevy UI implementation for the top panel.
//!
//! Displays game title, player resources (gold, production), and current turn number.

use bevy::prelude::*;
use core_sim::{resources::CurrentTurn, Civilization, PlayerControlled};

use crate::theme::palette;
use crate::ui::constants::display_layout;

// ============================================================================
// Marker Components
// ============================================================================

/// Marker component for the top panel container
#[derive(Component)]
pub struct TopPanel;

/// Marker component for the game title text
#[derive(Component)]
pub struct GameTitleText;

/// Marker component for the gold display text
#[derive(Component)]
pub struct GoldDisplayText;

/// Marker component for the production display text
#[derive(Component)]
pub struct ProductionDisplayText;

/// Marker component for the turn display text
#[derive(Component)]
pub struct TurnDisplayText;

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
            BackgroundColor(Color::srgba(0.165, 0.165, 0.165, 1.0)), // #2a2a2a
            Name::new("Top Panel"),
        ))
        .with_children(|parent| {
            // Game Title (left side)
            parent.spawn((
                GameTitleText,
                Text::new("Dominion Earth"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
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
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        min_width: Val::Px(500.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                    BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    BorderRadius::all(Val::Px(8.0)),
                    Name::new("Stats Container"),
                ))
                .with_children(|stats_parent| {
                    // Gold Display
                    stats_parent.spawn((
                        GoldDisplayText,
                        Text::new("Gold: 0"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Name::new("Gold Display"),
                    ));

                    // Production Display
                    stats_parent.spawn((
                        ProductionDisplayText,
                        Text::new("Production: 0"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Name::new("Production Display"),
                    ));

                    // Turn Display
                    stats_parent.spawn((
                        TurnDisplayText,
                        Text::new("Turn: 1"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Name::new("Turn Display"),
                    ));
                });
        });
}

// ============================================================================
// Update Systems
// ============================================================================

/// Update gold and production displays from player civilization
pub fn update_player_resources(
    player_query: Query<&Civilization, With<PlayerControlled>>,
    mut gold_text: Query<&mut Text, (With<GoldDisplayText>, Without<ProductionDisplayText>)>,
    mut production_text: Query<&mut Text, With<ProductionDisplayText>>,
) {
    if let Some(player_civ) = player_query.iter().next() {
        // Update gold display
        if let Some(mut text) = gold_text.iter_mut().next() {
            **text = format!("Gold: {}", player_civ.economy.gold);
        }

        // Update production display
        if let Some(mut text) = production_text.iter_mut().next() {
            **text = format!("Production: {}", player_civ.economy.production);
        }
    }
}

/// Update turn display from CurrentTurn resource
pub fn update_turn_display(
    current_turn: Res<CurrentTurn>,
    mut turn_text: Query<&mut Text, With<TurnDisplayText>>,
) {
    if current_turn.is_changed() {
        if let Some(mut text) = turn_text.iter_mut().next() {
            **text = format!("Turn: {}", current_turn.0);
        }
    }
}

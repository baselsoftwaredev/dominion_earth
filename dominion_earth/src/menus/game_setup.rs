//! The game setup menu where players configure game settings before starting.

use bevy::prelude::*;

use crate::{menus::Menu, settings::GameSettings, theme::prelude::*};

/// Marker component for entities that belong to the game setup menu screen
#[derive(Component)]
struct OnGameSetupScreen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::GameSetup), setup_game_setup_menu)
        .add_systems(
            Update,
            (update_seed_label, update_ai_only_label).run_if(in_state(Menu::GameSetup)),
        );
}

fn setup_game_setup_menu(
    mut commands: Commands,
    settings: Res<GameSettings>,
) {
    crate::debug_println!("🎮 Spawning game setup menu");

    commands
        .spawn((
            widget::ui_root("Game Setup Menu"),
            GlobalZIndex(constants::z_index::MENU_OVERLAY_Z_INDEX),
            DespawnOnExit(Menu::GameSetup),
            OnGameSetupScreen,
        ))
        .with_children(|parent| {
            // Top section - Header (30% height)
            parent
                .spawn((
                    Name::new("Header Section"),
                    Node {
                        height: ui_palette::percent(30.0),
                        width: ui_palette::percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(widget::header("Game Setup"));
                });

            // Middle section - Settings (60% height)
            parent
                .spawn((
                    Name::new("Settings Section"),
                    Node {
                        height: ui_palette::percent(60.0),
                        width: ui_palette::percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Settings grid container
                    parent
                        .spawn((
                            Name::new("Settings Grid"),
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: ui_palette::px(40.0),
                                width: ui_palette::percent(80.0),
                                padding: UiRect::horizontal(ui_palette::px(40.0)),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            // Random Seed setting row
                            parent
                                .spawn((
                                    Name::new("Seed Row"),
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        column_gap: ui_palette::px(20.0),
                                        width: ui_palette::percent(100.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    // Label column (33% width)
                                    parent
                                        .spawn((
                                            Name::new("Label Column"),
                                            Node {
                                                width: ui_palette::percent(33.33),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(widget::label("Random Seed"));
                                        });

                                    // Value column (33% width)
                                    parent
                                        .spawn((
                                            Name::new("Value Column"),
                                            Node {
                                                width: ui_palette::percent(33.33),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Name::new("Seed Label"),
                                                Text::new(match settings.seed {
                                                    Some(seed) => format!("{}", seed),
                                                    None => "Random".to_string(),
                                                }),
                                                TextFont {
                                                    font_size: constants::font_sizes::LABEL_TEXT_SIZE,
                                                    ..default()
                                                },
                                                TextColor(ui_palette::TEXT_PRIMARY),
                                                SeedLabel,
                                            ));
                                        });

                                    // Buttons column (33% width)
                                    parent
                                        .spawn((
                                            Name::new("Seed Buttons"),
                                            Node {
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                column_gap: ui_palette::px(
                                                    crate::constants::ui::spacing::VOLUME_CONTROLS_GAP,
                                                ),
                                                width: ui_palette::percent(33.33),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(widget::button(
                                                "Random",
                                                widget::ButtonAction::SetRandomSeed,
                                            ));
                                        });
                                });

                            // AI-Only Mode setting row
                            parent
                                .spawn((
                                    Name::new("AI Only Row"),
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        column_gap: ui_palette::px(20.0),
                                        width: ui_palette::percent(100.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    // Label column (33% width)
                                    parent
                                        .spawn((
                                            Name::new("Label Column"),
                                            Node {
                                                width: ui_palette::percent(33.33),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(widget::label("AI-Only Mode"));
                                        });

                                    // Value column (33% width)
                                    parent
                                        .spawn((
                                            Name::new("Value Column"),
                                            Node {
                                                width: ui_palette::percent(33.33),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Name::new("AI Only Label"),
                                                Text::new(if settings.ai_only {
                                                    "Enabled"
                                                } else {
                                                    "Disabled"
                                                }),
                                                TextFont {
                                                    font_size: constants::font_sizes::LABEL_TEXT_SIZE,
                                                    ..default()
                                                },
                                                TextColor(ui_palette::TEXT_PRIMARY),
                                                AiOnlyLabel,
                                            ));
                                        });

                                    // Button column (33% width)
                                    parent
                                        .spawn((
                                            Name::new("Button Column"),
                                            Node {
                                                width: ui_palette::percent(33.33),
                                                ..default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(widget::button(
                                                "Toggle",
                                                widget::ButtonAction::ToggleAiOnly,
                                            ));
                                        });
                                });
                        });
                });

            // Bottom section - Buttons (10% height)
            parent
                .spawn((
                    Name::new("Buttons Section"),
                    Node {
                        height: ui_palette::percent(10.0),
                        width: ui_palette::percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Button row
                    parent
                        .spawn((
                            Name::new("Button Row"),
                            Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: ui_palette::px(20.0),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(widget::button(
                                "Start Game",
                                widget::ButtonAction::StartGame,
                            ));

                            parent.spawn(widget::button("Back", widget::ButtonAction::GoBack));
                        });
                });
        });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SeedLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct AiOnlyLabel;

fn update_seed_label(
    settings: Res<GameSettings>,
    mut label_query: Query<&mut Text, With<SeedLabel>>,
) {
    if settings.is_changed() {
        if let Some(mut text) = label_query.iter_mut().next() {
            **text = match settings.seed {
                Some(seed) => format!("{}", seed),
                None => "Random".to_string(),
            };
        }
    }
}

fn update_ai_only_label(
    settings: Res<GameSettings>,
    mut label_query: Query<&mut Text, With<AiOnlyLabel>>,
) {
    if settings.is_changed() {
        if let Some(mut text) = label_query.iter_mut().next() {
            **text = if settings.ai_only {
                "Enabled".to_string()
            } else {
                "Disabled".to_string()
            };
        }
    }
}

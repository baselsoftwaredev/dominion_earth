//! The game setup menu where players configure game settings before starting.

use bevy::prelude::*;

use crate::{debug_utils::DebugLogging, menus::Menu, settings::GameSettings, theme::prelude::*};

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
    debug_logging: Res<DebugLogging>,
    settings: Res<GameSettings>,
) {
    crate::debug_println!(debug_logging, "🎮 Spawning game setup menu");

    commands
        .spawn((
            widget::ui_root("Game Setup Menu"),
            GlobalZIndex(constants::z_index::MENU_OVERLAY_Z_INDEX),
            DespawnOnExit(Menu::GameSetup),
            OnGameSetupScreen,
        ))
        .with_children(|parent| {
            parent.spawn(widget::header("Game Setup"));

            parent
                .spawn((
                    Name::new("Seed Container"),
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: ui_palette::px(
                            crate::constants::ui::spacing::VOLUME_CONTROLS_GAP,
                        ),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(widget::label("Random Seed"));
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
                    parent.spawn(widget::button_small(
                        "Random",
                        widget::ButtonAction::SetRandomSeed,
                    ));
                    parent.spawn(widget::button_small(
                        "Clear",
                        widget::ButtonAction::ClearSeed,
                    ));
                });

            parent
                .spawn((
                    Name::new("AI Only Container"),
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: ui_palette::px(
                            crate::constants::ui::spacing::VOLUME_CONTROLS_GAP,
                        ),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(widget::label("AI-Only Mode"));
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
                    parent.spawn(widget::button_small(
                        "Toggle",
                        widget::ButtonAction::ToggleAiOnly,
                    ));
                });

            parent.spawn(widget::button(
                "Start Game",
                widget::ButtonAction::StartGame,
            ));

            parent.spawn(widget::button("Back", widget::ButtonAction::GoBack));
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

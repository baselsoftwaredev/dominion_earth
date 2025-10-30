pub mod constants;
pub mod production_section;
pub mod unit_info_section;

use bevy::prelude::*;
use core_sim::RequestTurnAdvance;
use moonshine_save::prelude::*;

use crate::ui::constants::display_layout;
use constants::*;

// Re-export components from sub-modules
pub use production_section::*;
pub use unit_info_section::*;

// Main left panel components
#[derive(Component)]
#[require(Unload)]
pub struct LeftPanel;

#[derive(Component)]
pub struct GamePanel;

#[derive(Component)]
pub struct NextTurnButton;

#[derive(Component)]
pub struct NextTurnButtonText;

/// Spawns the main left panel with all sections
pub fn spawn_left_panel(mut commands: Commands) {
    let production_panel = production_section::spawn_production_menu_panel(&mut commands);
    let unit_info_panel = unit_info_section::spawn_unit_info_panel(&mut commands);

    commands
        .spawn((
            LeftPanel,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(display_layout::HEADER_HEIGHT),
                width: Val::Px(display_layout::LEFT_SIDEBAR_WIDTH),
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            ScrollPosition::default(),
            BackgroundColor(Color::srgba(0.102, 0.102, 0.102, 1.0)),
            Name::new("Left Panel"),
        ))
        .with_children(|parent| {
            // Game Panel with Next Turn button
            parent
                .spawn((
                    GamePanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: GAME_PANEL_MIN_HEIGHT,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        flex_shrink: 0.0,
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Game Panel"),
                ))
                .with_children(|game_parent| {
                    game_parent.spawn((
                        Text::new("Your Empire"),
                        TextFont {
                            font_size: TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Game Panel Title"),
                    ));

                    game_parent
                        .spawn((
                            NextTurnButton,
                            Button,
                            Node {
                                height: NEXT_TURN_BUTTON_HEIGHT,
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(BUTTON_PADDING),
                                margin: UiRect::bottom(SECTION_MARGIN_BOTTOM),
                                border: UiRect::all(BUTTON_BORDER_WIDTH),
                                ..default()
                            },
                            BackgroundColor(BUTTON_BACKGROUND),
                            BorderColor::from(BUTTON_BORDER),
                            BorderRadius::all(BUTTON_BORDER_RADIUS),
                            Name::new("Next Turn Button"),
                        ))
                        .with_children(|button_parent| {
                            button_parent.spawn((
                                NextTurnButtonText,
                                Text::new("Next Turn"),
                                TextFont {
                                    font_size: SUBTITLE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_PRIMARY),
                            ));
                        });
                });
        })
        .add_children(&[production_panel, unit_info_panel]);
}

/// Handles Next Turn button interactions
pub fn handle_next_turn_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<NextTurnButton>),
    >,
    mut turn_advance_events: MessageWriter<RequestTurnAdvance>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut background, mut border) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(BUTTON_PRESSED_BACKGROUND);
                turn_advance_events.write(RequestTurnAdvance);
                info!("Player requested turn advancement");
                // TODO: Fix audio file corruption before re-enabling
                // crate::audio::play_sound_effect(&mut commands, &asset_server, "sounds/click.ogg");
            }
            Interaction::Hovered => {
                *background = BackgroundColor(BUTTON_HOVER_BACKGROUND);
                *border = BorderColor::all(BUTTON_HOVER_BORDER);
            }
            Interaction::None => {
                *background = BackgroundColor(BUTTON_BACKGROUND);
                *border = BorderColor::all(BUTTON_BORDER);
            }
        }
    }
}

pub fn update_next_turn_button_text(
    mut button_text_query: Query<&mut Text, With<NextTurnButtonText>>,
    turn_phase: Res<core_sim::TurnPhase>,
    turn_order: Res<core_sim::TurnOrder>,
    civilizations: Query<&core_sim::Civilization>,
    player_civs: Query<&core_sim::Civilization, With<core_sim::PlayerControlled>>,
) {
    if !turn_phase.is_changed() && !turn_order.is_changed() {
        return;
    }

    for mut text in button_text_query.iter_mut() {
        let new_text =
            determine_next_turn_button_text(turn_phase.as_ref(), &civilizations, &player_civs);

        info!("UI: Updating Next Turn button text to: '{}'", new_text);
        **text = new_text;
    }
}

fn determine_next_turn_button_text(
    turn_phase: &core_sim::TurnPhase,
    civilizations: &Query<&core_sim::Civilization>,
    player_civs: &Query<&core_sim::Civilization, With<core_sim::PlayerControlled>>,
) -> String {
    match turn_phase {
        core_sim::TurnPhase::CivilizationTurn { current_civ } => {
            determine_text_for_civilization_turn(*current_civ, civilizations, player_civs)
        }
        core_sim::TurnPhase::WaitingForNextTurn { next_civ } => {
            determine_text_for_waiting_turn(*next_civ, civilizations, player_civs)
        }
        core_sim::TurnPhase::TurnTransition => BUTTON_TEXT_PROCESSING.to_string(),
    }
}

fn determine_text_for_civilization_turn(
    current_civ: core_sim::CivId,
    civilizations: &Query<&core_sim::Civilization>,
    player_civs: &Query<&core_sim::Civilization, With<core_sim::PlayerControlled>>,
) -> String {
    if let Some(civ) = civilizations.iter().find(|c| c.id == current_civ) {
        let is_player = is_player_controlled_civilization(current_civ, player_civs);

        if is_player {
            BUTTON_TEXT_END_TURN.to_string()
        } else {
            format!("{}'s Turn (Processing...)", civ.name)
        }
    } else {
        BUTTON_TEXT_NEXT_TURN.to_string()
    }
}

fn determine_text_for_waiting_turn(
    next_civ: core_sim::CivId,
    civilizations: &Query<&core_sim::Civilization>,
    player_civs: &Query<&core_sim::Civilization, With<core_sim::PlayerControlled>>,
) -> String {
    if let Some(civ) = civilizations.iter().find(|c| c.id == next_civ) {
        let is_player = is_player_controlled_civilization(next_civ, player_civs);

        if is_player {
            BUTTON_TEXT_START_YOUR_TURN.to_string()
        } else {
            format!("Start {}'s Turn", civ.name)
        }
    } else {
        BUTTON_TEXT_NEXT_TURN.to_string()
    }
}

fn is_player_controlled_civilization(
    civ_id: core_sim::CivId,
    player_civs: &Query<&core_sim::Civilization, With<core_sim::PlayerControlled>>,
) -> bool {
    player_civs.iter().any(|c| c.id == civ_id)
}

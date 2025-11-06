use bevy::prelude::*;
use core_sim::components::events::{EventChoice, GameEvent, PendingEvent};
use core_sim::components::PlayerControlled;

use crate::debug_println;
use crate::screens::Screen;
use crate::theme::prelude::*;

/// Marker component for events that have been acknowledged by the player.
///
/// When added to an event entity, it signals that the event's effects should be applied
/// and the UI popup should be despawned.
#[derive(Component)]
pub struct EventAcknowledged {
    /// The index of the choice selected by the player (if any)
    pub selected_choice: Option<usize>,
}

/// Marker component for events that have had a popup shown.
///
/// This prevents duplicate popups from being created for the same event.
#[derive(Component)]
pub struct EventPopupShown;

/// Links a UI popup to its corresponding event entity.
///
/// This marker component ensures that when a player responds to an event,
/// only the correct popup is despawned, not all event popups.
#[derive(Component)]
pub struct EventPopup {
    pub event_entity: Entity,
}

/// Marker for event title text
#[derive(Component)]
pub struct EventTitleText;

/// Marker for event description text
#[derive(Component)]
pub struct EventDescriptionText;

/// Marker for event choice buttons
#[derive(Component)]
pub struct EventChoiceButton {
    pub choice_index: usize,
}

/// Spawns UI popups for player civilization events.
///
/// This system:
/// - Queries for events with the `PendingEvent` marker
/// - Filters to only show events for player-controlled civilizations
/// - For AI civs: Logs to console and removes `PendingEvent`
/// - For player civs: Creates a modal popup UI
/// - Removes `PendingEvent` immediately to prevent duplicate popups
///
/// The popup stays open until the player clicks a choice button, which
/// adds the `EventAcknowledged` marker to trigger cleanup.
pub fn spawn_event_popup(
    mut commands: Commands,
    pending_events: Query<(Entity, &GameEvent), (With<PendingEvent>, Without<EventPopupShown>)>,
    player_civs: Query<&core_sim::Civilization, With<PlayerControlled>>,
    game_state: Res<crate::game::GameState>,
) {
    // Only show events for player-controlled civilizations
    let player_civ_ids: Vec<_> = player_civs.iter().map(|civ| civ.id).collect();

    if !pending_events.is_empty() {
        debug_println!(
            "🎭 UI: Found {} pending events for {} player civs",
            pending_events.iter().count(),
            player_civ_ids.len()
        );
    }

    for (event_entity, event) in pending_events.iter() {
        debug_println!(
            "🎭 UI: Processing event '{}' for civ {:?}",
            event.title,
            event.affected_civ
        );
        if !player_civ_ids.contains(&event.affected_civ) {
            debug_println!("🎭 UI: Event is for AI civ, removing PendingEvent");
            if game_state.ai_only {
                debug_println!(
                    "🎭 EVENT [Civ {:?}]: {} - {}",
                    event.affected_civ,
                    event.title,
                    event.description
                );
            }
            commands.entity(event_entity).remove::<PendingEvent>();
            continue;
        }

        debug_println!(
            "🎭 UI: Creating popup UI for player event '{}'",
            event.title
        );
        create_event_popup_ui(&mut commands, event_entity, event);

        // Don't remove PendingEvent here - let the player's choice do that
        // Mark that we've shown this event to prevent duplicate popups
        commands.entity(event_entity).insert(EventPopupShown);
    }
}

fn create_event_popup_ui(commands: &mut Commands, event_entity: Entity, event: &GameEvent) {
    commands
        .spawn((
            Name::new("Event Popup Overlay"),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            GlobalZIndex(1000), // Very high z-index to appear above everything
            EventPopup { event_entity },
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|parent| {
            // Main popup container
            parent
                .spawn((
                    Name::new("Event Popup Container"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(30.0)),
                        row_gap: Val::Px(20.0),
                        width: Val::Px(600.0),
                        max_height: Val::Px(500.0),
                        ..default()
                    },
                    BackgroundColor(ui_palette::PANEL_BACKGROUND),
                    BorderColor::all(ui_palette::PANEL_BORDER),
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Name::new("Event Title"),
                        Text::new(&event.title),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(ui_palette::BUTTON_TEXT),
                        EventTitleText,
                    ));

                    // Description
                    parent.spawn((
                        Name::new("Event Description"),
                        Text::new(&event.description),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(ui_palette::TEXT_PRIMARY),
                        Node {
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        EventDescriptionText,
                    ));

                    // Effects preview
                    if !event.effects.is_empty() {
                        parent
                            .spawn((
                                Name::new("Effects Section"),
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(8.0),
                                    margin: UiRect::vertical(Val::Px(10.0)),
                                    ..default()
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Effects:"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(ui_palette::BUTTON_TEXT),
                                ));

                                for effect in &event.effects {
                                    let effect_text = format_effect(effect);
                                    parent.spawn((
                                        Text::new(effect_text),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(ui_palette::TEXT_PRIMARY),
                                    ));
                                }
                            });
                    }

                    // Choices/Buttons
                    parent
                        .spawn((
                            Name::new("Choice Buttons"),
                            Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(15.0),
                                justify_content: JustifyContent::Center,
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            if event.choices.is_empty() {
                                // Simple "Acknowledge" button if no choices
                                let btn_text = "Acknowledge";
                                parent
                                    .spawn((
                                        Name::new(format!("Event Button: {}", btn_text)),
                                        Button,
                                        Node {
                                            width: Val::Px(150.0),
                                            height: Val::Px(50.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(ui_palette::BUTTON_BACKGROUND),
                                        EventChoiceButton { choice_index: 0 },
                                    ))
                                    .observe(
                                        move |_trigger: On<Pointer<Click>>,
                                              mut commands: Commands| {
                                            acknowledge_event(&mut commands, event_entity);
                                        },
                                    )
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new(btn_text),
                                            TextFont {
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(ui_palette::BUTTON_TEXT),
                                        ));
                                    });
                            } else {
                                // Multiple choice buttons
                                for (index, choice) in event.choices.iter().enumerate() {
                                    let btn_text = choice.text.clone();
                                    parent
                                        .spawn((
                                            Name::new(format!("Event Button: {}", btn_text)),
                                            Button,
                                            Node {
                                                width: Val::Px(150.0),
                                                height: Val::Px(50.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(ui_palette::BUTTON_BACKGROUND),
                                            EventChoiceButton {
                                                choice_index: index,
                                            },
                                        ))
                                        .observe(
                                            move |_trigger: On<Pointer<Click>>,
                                                  mut commands: Commands| {
                                                choose_event_option(
                                                    &mut commands,
                                                    event_entity,
                                                    index,
                                                );
                                            },
                                        )
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new(btn_text.clone()),
                                                TextFont {
                                                    font_size: 18.0,
                                                    ..default()
                                                },
                                                TextColor(ui_palette::BUTTON_TEXT),
                                            ));
                                        });
                                }
                            }
                        });
                });
        });
}

fn format_effect(effect: &core_sim::components::events::EventEffect) -> String {
    use core_sim::components::events::EventEffect;

    match effect {
        EventEffect::GoldChange(amount) => {
            if *amount >= 0.0 {
                format!("+ {} Gold", amount)
            } else {
                format!("- {} Gold", amount.abs())
            }
        }
        EventEffect::HappinessChange(amount) => {
            if *amount >= 0.0 {
                format!("+ {} Happiness", amount)
            } else {
                format!("- {} Happiness", amount.abs())
            }
        }
        EventEffect::GrantUnit { unit_type } => format!("Gain 1 {}", unit_type),
        EventEffect::GrantTechnology { tech_name } => format!("Research: {}", tech_name),
        EventEffect::ProductionModifier {
            multiplier,
            duration_turns,
        } => {
            format!("{}x Production for {} turns", multiplier, duration_turns)
        }
        EventEffect::TemporaryTraitModifier {
            trait_name,
            modifier,
            duration_turns,
        } => {
            format!(
                "{} {:+.1} for {} turns",
                trait_name, modifier, duration_turns
            )
        }
    }
}

fn acknowledge_event(commands: &mut Commands, event_entity: Entity) {
    // Mark event as shown so apply_event_effects will process it
    commands
        .entity(event_entity)
        .remove::<PendingEvent>()
        .insert(EventAcknowledged {
            selected_choice: None,
        });
}

fn choose_event_option(commands: &mut Commands, event_entity: Entity, choice_index: usize) {
    // Queue a command to update the GameEvent and remove PendingEvent
    commands.queue(move |world: &mut World| {
        if let Some(mut event) = world.get_mut::<GameEvent>(event_entity) {
            event.selected_choice = Some(choice_index);
            info!(
                "Player selected choice {} for event '{}'",
                choice_index, event.title
            );
        }

        // Remove PendingEvent marker to trigger effect application
        world
            .entity_mut(event_entity)
            .remove::<PendingEvent>()
            .insert(EventAcknowledged {
                selected_choice: Some(choice_index),
            });
    });
}

/// Despawns event popups after their events have been acknowledged.
///
/// This system queries all popups and checks if their linked event entities
/// no longer have PendingEvent (meaning they've been acknowledged and processed).
/// Despawns popups when the event entity is missing (despawned) or no longer pending.
pub fn despawn_completed_event_popups(
    mut commands: Commands,
    popups: Query<(Entity, &EventPopup)>,
    pending_events: Query<(), With<PendingEvent>>,
) {
    for (popup_entity, popup) in popups.iter() {
        // Despawn popup if the event entity doesn't exist or doesn't have PendingEvent
        let should_despawn = !pending_events.contains(popup.event_entity);

        if should_despawn {
            debug_println!("🎭 UI: Despawning popup for processed event");
            commands.entity(popup_entity).despawn();
        }
    }
}

use crate::components::{
    civilization::{CivId, Civilization, Economy},
    events::{
        ActiveEventModifiers, EventEffect, EventHistory, EventTriggerType, GameEvent, PendingEvent,
        ResourceType, ThresholdComparison,
    },
};
use crate::constants::events::{
    DEFAULT_FOOD_RESOURCE_VALUE, DEFAULT_PRODUCTION_RESOURCE_VALUE, RESOURCE_EQUALITY_TOLERANCE,
};
use crate::resources::{CurrentTurn, EventDefinitions, GameRng};
use bevy_ecs::prelude::*;
use rand::Rng;
use tracing::info;

/// Checks for event triggers and spawns event entities when conditions are met.
///
/// This system evaluates all event definitions against all civilizations each turn.
/// It uses a `Local<u32>` to ensure it only runs once per turn, not every frame.
/// When an event triggers, it spawns a `GameEvent` entity with the `PendingEvent` marker.
pub fn check_event_triggers(
    mut commands: Commands,
    event_defs: Res<EventDefinitions>,
    mut rng: ResMut<GameRng>,
    turn_count: Res<CurrentTurn>,
    civs: Query<(Entity, &CivId, &Civilization, Option<&EventHistory>)>,
    mut last_turn_checked: Local<u32>,
) {
    if *last_turn_checked == turn_count.0 {
        return;
    }
    *last_turn_checked = turn_count.0;

    info!(
        "🎭 Checking event triggers for turn {} with {} event definitions and {} civilizations",
        turn_count.0,
        event_defs.events.len(),
        civs.iter().count()
    );

    for (entity, civ_id, civilization, history) in civs.iter() {
        let history_ref = history.cloned().unwrap_or_default();

        for event_def in &event_defs.events {
            if history_ref.has_triggered_event(&event_def.id) {
                info!(
                    "Skipping event '{}' for civ {:?} - already triggered",
                    event_def.id, civ_id
                );
                continue;
            }

            let should_trigger = match &event_def.trigger {
                EventTriggerType::Random { probability } => {
                    let random_value: f32 = rng.0.gen();
                    let result = random_value < *probability;
                    info!(
                        "Random event '{}' for civ {:?}: roll={:.2}, probability={:.2}, trigger={}",
                        event_def.id, civ_id, random_value, probability, result
                    );
                    result
                }
                EventTriggerType::ResourceThreshold {
                    resource,
                    threshold,
                    comparison,
                } => {
                    let value = match resource {
                        ResourceType::Gold => civilization.economy.gold,
                        ResourceType::Food => DEFAULT_FOOD_RESOURCE_VALUE,
                        ResourceType::Production => DEFAULT_PRODUCTION_RESOURCE_VALUE,
                        ResourceType::Science => civilization.technologies.research_points,
                    };

                    match comparison {
                        ThresholdComparison::GreaterThan => value > *threshold,
                        ThresholdComparison::LessThan => value < *threshold,
                        ThresholdComparison::Equal => {
                            (value - threshold).abs() < RESOURCE_EQUALITY_TOLERANCE
                        }
                    }
                }
                EventTriggerType::FirstUnitCreated { unit_type } => {
                    history_ref.has_created_unit(unit_type)
                }
                EventTriggerType::FirstCivContact { other_civ: _ } => false,
                EventTriggerType::TechnologyResearched { tech_name } => {
                    history_ref.has_researched_tech(tech_name)
                }
                EventTriggerType::CityFounded { city_count: _ } => false,
                EventTriggerType::SpecificTurn { turn } => {
                    let result = turn_count.0 == *turn;
                    if result {
                        info!(
                            "Turn-based event '{}' triggered for civ {:?} on turn {}",
                            event_def.id, civ_id, turn
                        );
                    }
                    result
                }
            };

            if should_trigger {
                info!(
                    "🎉 Triggering event '{}' for civ {:?} ({})",
                    event_def.id, civ_id, civilization.name
                );
                spawn_event(&mut commands, entity, *civ_id, event_def, turn_count.0);
            }
        }
    }
}

fn spawn_event(
    commands: &mut Commands,
    civ_entity: Entity,
    civ_id: CivId,
    event_def: &crate::resources::EventDefinition,
    current_turn: u32,
) {
    let event_entity = commands
        .spawn((
            GameEvent {
                event_id: event_def.id.clone(),
                affected_civ: civ_id,
                title: event_def.title.clone(),
                description: event_def.description.clone(),
                effects: event_def.effects.clone(),
                choices: event_def.choices.clone(),
                has_been_shown: false,
                turn_triggered: current_turn,
            },
            PendingEvent,
        ))
        .id();

    commands.entity(civ_entity).insert({
        let mut history = EventHistory::default();
        history.record_event_triggered(event_def.id.clone());
        history
    });

    info!(
        "Event triggered: '{}' for civilization {:?}",
        event_def.title, civ_id
    );
}

/// Applies event effects to civilizations after events have been acknowledged.
///
/// This system queries for events without the `PendingEvent` marker (meaning they've been
/// acknowledged by the player or AI). It applies the effects to the affected civilization
/// and then despawns the event entity.
pub fn apply_event_effects(
    mut commands: Commands,
    mut civs: Query<(
        &CivId,
        &mut Civilization,
        &mut Economy,
        Option<&mut ActiveEventModifiers>,
    )>,
    events: Query<(Entity, &GameEvent), Without<PendingEvent>>,
) {
    for (event_entity, event) in events.iter() {
        if let Some((_, mut civ, mut economy, mut modifiers)) = civs
            .iter_mut()
            .find(|(civ_id, _, _, _)| **civ_id == event.affected_civ)
        {
            for effect in &event.effects {
                apply_effect(
                    effect,
                    &mut civ,
                    &mut economy,
                    &mut modifiers,
                    &mut commands,
                );
            }

            commands.entity(event_entity).despawn();
        }
    }
}

fn apply_effect(
    effect: &EventEffect,
    _civilization: &mut Civilization,
    economy: &mut Economy,
    modifiers: &mut Option<Mut<ActiveEventModifiers>>,
    _commands: &mut Commands,
) {
    match effect {
        EventEffect::GoldChange(amount) => {
            economy.gold += amount;
            info!("Applied gold change: {}", amount);
        }
        EventEffect::HappinessChange(_amount) => {
            info!("Happiness change not yet implemented");
        }
        EventEffect::GrantUnit { unit_type } => {
            info!("Grant unit not yet implemented: {}", unit_type);
        }
        EventEffect::GrantTechnology { tech_name } => {
            info!("Grant technology not yet implemented: {}", tech_name);
        }
        EventEffect::ProductionModifier {
            multiplier,
            duration_turns,
        } => {
            if let Some(ref mut mods) = modifiers {
                mods.add_production_multiplier(*multiplier, *duration_turns);
                info!(
                    "Applied production multiplier: {}x for {} turns",
                    multiplier, duration_turns
                );
            }
        }
        EventEffect::TemporaryTraitModifier {
            trait_name,
            modifier,
            duration_turns,
        } => {
            if let Some(ref mut mods) = modifiers {
                mods.add_trait_modifier(trait_name.clone(), *modifier, *duration_turns);
                info!(
                    "Applied trait modifier to {}: {} for {} turns",
                    trait_name, modifier, duration_turns
                );
            }
        }
    }
}

/// Updates active event modifiers each turn by decrementing their duration.
///
/// This system should be run with `.run_if(resource_changed::<CurrentTurn>())` to ensure
/// it only processes modifiers when the turn actually changes.
/// Expired modifiers (duration <= 0) are automatically removed.
pub fn update_event_modifiers(mut civs: Query<&mut ActiveEventModifiers>) {
    for mut modifiers in civs.iter_mut() {
        modifiers.update_turn();
    }
}

/// Placeholder system for recording unit creation in event history.
///
/// This will be called when units are created to track "first unit" type events.
/// TODO: Hook this up to actual unit creation events
pub fn record_unit_creation(mut civs: Query<&mut EventHistory>) {
    let _ = civs;
}

/// Placeholder system for recording technology research in event history.
///
/// Checks for newly researched technologies and updates event history.
/// TODO: Complete implementation when tech system is fully integrated
pub fn record_tech_research(mut civs: Query<(&Civilization, &mut EventHistory)>) {
    for (civ, mut history) in civs.iter_mut() {
        if let Some(ref current_tech) = civ.technologies.current_research {
            let _ = (current_tech, history);
        }
    }
}

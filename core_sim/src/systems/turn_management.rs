use crate::{
    components::{
        city::City,
        position::MovementOrder,
        production::{ProductionItem, ProductionQueue},
        turn_phases::{
            AITurnComplete, AllAITurnsComplete, ProcessAITurn, StartPlayerTurn, TurnOrder,
            TurnPhase,
        },
        Capital, Civilization, MilitaryUnit, PlayerActionsComplete, PlayerControlled,
    },
    constants::civilization_management::{PLAYER_CIVILIZATION_ID, STARTING_UNIT_ID_COUNTER},
    pathfinding::Pathfinder,
    resources::{CurrentTurn, GameConfig},
    CivId, Position, WorldMap,
};
use bevy_ecs::prelude::*;

#[derive(Message)]
pub struct ProductionUpdated {
    pub capital_entity: Entity,
}

pub fn handle_turn_advance_requests(
    mut turn_requests: MessageReader<RequestTurnAdvance>,
    mut turn_phase: ResMut<TurnPhase>,
    mut turn_order: ResMut<TurnOrder>,
    mut ai_turn_events: MessageWriter<ProcessAITurn>,
    civilizations: Query<&Civilization>,
) {
    for _request in turn_requests.read() {
        tracing::info!("Turn advancement requested");

        let current_phase = turn_phase.clone();

        match current_phase {
            TurnPhase::CivilizationTurn { current_civ } => {
                tracing::info!("Ending turn for civilization {}", current_civ.0);

                let completed_round = turn_order.advance();

                if completed_round {
                    tracing::info!(
                        "All civilizations have taken their turn - advancing to turn transition"
                    );
                    *turn_phase = TurnPhase::TurnTransition;
                } else {
                    if let Some(next_civ) = turn_order.current_civ() {
                        let is_player = turn_order.is_player_civ(next_civ);

                        if is_player {
                            tracing::info!("Starting player turn (Civ {})", next_civ.0);
                            *turn_phase = TurnPhase::CivilizationTurn {
                                current_civ: next_civ,
                            };
                        } else {
                            tracing::info!(
                                "Next up: AI civilization {} (waiting for user to press Next Turn)",
                                next_civ.0
                            );
                            *turn_phase = TurnPhase::WaitingForNextTurn { next_civ };
                        }
                    }
                }
            }
            TurnPhase::WaitingForNextTurn { next_civ } => {
                let is_player = turn_order.is_player_civ(next_civ);

                if is_player {
                    tracing::info!("Starting player turn (Civ {})", next_civ.0);
                    *turn_phase = TurnPhase::CivilizationTurn {
                        current_civ: next_civ,
                    };
                } else {
                    tracing::info!("Starting AI turn for civilization {}", next_civ.0);
                    *turn_phase = TurnPhase::CivilizationTurn {
                        current_civ: next_civ,
                    };
                    ai_turn_events.write(ProcessAITurn { civ_id: next_civ });
                }
            }
            TurnPhase::TurnTransition => {
                tracing::debug!("Ignoring turn advance request during turn transition");
            }
        }
    }
}

pub fn handle_ai_turn_processing(
    mut ai_turn_events: MessageReader<ProcessAITurn>,
    mut ai_complete_events: MessageWriter<AITurnComplete>,
    civilizations: Query<&Civilization>,
    mut commands: Commands,
    mut units_query: Query<(Entity, &mut MilitaryUnit, &mut Position)>,
    world_map: Res<WorldMap>,
) {
    for ai_event in ai_turn_events.read() {
        tracing::info!("Processing AI turn for civilization {:?}", ai_event.civ_id);

        process_ai_civilization_turn(
            ai_event.civ_id,
            &civilizations,
            &mut commands,
            &mut units_query,
            &world_map,
        );

        ai_complete_events.write(AITurnComplete {
            civ_id: ai_event.civ_id,
        });
    }
}

pub fn handle_ai_turn_completion(
    mut ai_complete_events: MessageReader<AITurnComplete>,
    mut turn_phase: ResMut<TurnPhase>,
    mut turn_order: ResMut<TurnOrder>,
) {
    for ai_event in ai_complete_events.read() {
        tracing::info!("AI civilization {} completed their turn", ai_event.civ_id.0);

        let completed_round = turn_order.advance();

        if completed_round {
            tracing::info!(
                "All civilizations have completed their turns - transitioning to new turn"
            );
            *turn_phase = TurnPhase::TurnTransition;
        } else {
            if let Some(next_civ) = turn_order.current_civ() {
                let is_player = turn_order.is_player_civ(next_civ);

                if is_player {
                    tracing::info!("Next up: Player turn (Civ {})", next_civ.0);
                    *turn_phase = TurnPhase::CivilizationTurn {
                        current_civ: next_civ,
                    };
                } else {
                    tracing::info!(
                        "Next up: AI civilization {} (waiting for user to press Next Turn)",
                        next_civ.0
                    );
                    *turn_phase = TurnPhase::WaitingForNextTurn { next_civ };
                }
            }
        }
    }
}

fn process_ai_civilization_turn(
    civ_id: CivId,
    civilizations: &Query<&Civilization>,
    commands: &mut Commands,
    units_query: &mut Query<(Entity, &mut MilitaryUnit, &mut Position)>,
    world_map: &WorldMap,
) {
    if let Some(civ) = civilizations.iter().find(|civ| civ.id == civ_id) {
        tracing::info!("AI {} ({}) is taking their turn", civ.name, civ_id.0);

        let mut unit_count = 0;
        let mut moved_units = 0;

        for (entity, mut unit, mut position) in units_query.iter_mut() {
            if unit.owner == civ_id {
                unit_count += 1;
                tracing::debug!("Found AI unit {} for civilization {}", unit.id, civ_id.0);

                if unit.can_move() {
                    move_ai_unit_simple(entity, &mut unit, &mut position, commands, world_map);
                    moved_units += 1;
                }
            }
        }

        tracing::info!(
            "AI {} completed their turn (processed {} units, {} could move)",
            civ.name,
            unit_count,
            moved_units
        );
    }
}

/// Simple AI unit movement - moves to the first valid adjacent tile
fn move_ai_unit_simple(
    entity: Entity,
    unit: &mut MilitaryUnit,
    position: &mut Position,
    commands: &mut Commands,
    world_map: &WorldMap,
) {
    let current_pos = *position;
    let adjacent_positions = current_pos.adjacent_positions();

    tracing::debug!(
        "AI unit {} at ({}, {}) checking {} adjacent positions",
        unit.id,
        current_pos.x,
        current_pos.y,
        adjacent_positions.len()
    );

    // Try to move to the first valid adjacent position
    for (i, target_pos) in adjacent_positions.iter().enumerate() {
        tracing::debug!(
            "AI unit {} checking position {} ({}, {})",
            unit.id,
            i,
            target_pos.x,
            target_pos.y
        );

        if is_valid_move_target(current_pos, *target_pos, world_map) {
            commands
                .entity(entity)
                .insert(MovementOrder::new(vec![*target_pos], *target_pos));

            tracing::info!(
                "AI unit {} planned movement from ({}, {}) to ({}, {})",
                unit.id,
                current_pos.x,
                current_pos.y,
                target_pos.x,
                target_pos.y
            );

            return;
        } else {
            tracing::debug!(
                "AI unit {} invalid move to ({}, {})",
                unit.id,
                target_pos.x,
                target_pos.y
            );
        }
    }

    tracing::debug!("AI unit {} found no valid moves", unit.id);
}

fn is_valid_move_target(from: Position, to: Position, world_map: &WorldMap) -> bool {
    if let Some(tile) = world_map.get_tile(to) {
        match tile.terrain {
            crate::TerrainType::Ocean => false,
            _ => from.manhattan_distance_to(&to) == 1,
        }
    } else {
        false
    }
}

pub fn handle_turn_transition_complete(
    mut turn_phase: ResMut<TurnPhase>,
    mut current_turn: ResMut<CurrentTurn>,
    mut player_actions: ResMut<PlayerActionsComplete>,
    mut units: Query<&mut MilitaryUnit>,
    mut production_query: Query<(Entity, &mut ProductionQueue, &mut City, &Capital, &Position)>,
    mut commands: Commands,
    mut unit_id_counter: Local<u32>,
    world_map: Res<WorldMap>,
    mut production_events: MessageWriter<ProductionUpdated>,
    mut start_player_events: MessageWriter<StartPlayerTurn>,
    mut turn_order: ResMut<TurnOrder>,
) {
    if !matches!(*turn_phase, TurnPhase::TurnTransition) {
        return;
    }

    tracing::info!("Processing turn transition");

    let next_turn_number = calculate_next_turn_number(current_turn.0);

    process_all_city_production_for_turn(
        &mut production_query,
        &mut commands,
        &mut unit_id_counter,
        &world_map,
        next_turn_number,
        &mut production_events,
    );

    advance_current_turn(&mut current_turn);
    reset_all_unit_movement_points(&mut units);
    reset_player_action_tracking(&mut player_actions);

    turn_order.current_index = 0;

    if let Some(first_civ) = turn_order.current_civ() {
        tracing::info!(
            "Advanced to turn {} - Starting with civilization {}",
            current_turn.0,
            first_civ.0
        );
        *turn_phase = TurnPhase::CivilizationTurn {
            current_civ: first_civ,
        };

        if turn_order.is_player_civ(first_civ) {
            start_player_events.write(StartPlayerTurn);
        }
    }
}

fn calculate_next_turn_number(current_turn: u32) -> u32 {
    current_turn + 1
}

fn advance_current_turn(current_turn: &mut ResMut<CurrentTurn>) {
    current_turn.0 += 1;
}

fn reset_all_unit_movement_points(units: &mut Query<&mut MilitaryUnit>) {
    for mut unit in units.iter_mut() {
        unit.reset_movement();
    }
}

fn reset_player_action_tracking(player_actions: &mut ResMut<PlayerActionsComplete>) {
    player_actions.reset();
}

fn process_all_city_production_for_turn(
    production_query: &mut Query<(Entity, &mut ProductionQueue, &mut City, &Capital, &Position)>,
    commands: &mut Commands,
    unit_id_counter: &mut Local<u32>,
    world_map: &WorldMap,
    turn_number: u32,
    production_events: &mut MessageWriter<ProductionUpdated>,
) {
    for (entity, mut production_queue, mut city, capital, position) in production_query.iter_mut() {
        let had_production_before =
            production_queue.current_production.is_some() || !production_queue.queue.is_empty();

        if let Some(completed_item) = production_queue.add_production(city.production) {
            spawn_completed_production_item(
                commands,
                &completed_item,
                &capital.owner,
                position,
                unit_id_counter,
                &mut city,
                world_map,
                turn_number,
            );
        }

        let has_production_after =
            production_queue.current_production.is_some() || !production_queue.queue.is_empty();

        if had_production_before || has_production_after {
            production_events.write(ProductionUpdated {
                capital_entity: entity,
            });
        }
    }
}

fn process_production_for_turn(
    production_query: &mut Query<(&mut ProductionQueue, &mut City, &Capital, &Position)>,
    commands: &mut Commands,
    unit_id_counter: &mut Local<u32>,
    world_map: &WorldMap,
    turn_number: u32,
) {
    for (mut production_queue, mut city, capital, position) in production_query.iter_mut() {
        if let Some(completed_item) = production_queue.add_production(city.production) {
            spawn_completed_production_item(
                commands,
                &completed_item,
                &capital.owner,
                position,
                unit_id_counter,
                &mut city,
                world_map,
                turn_number,
            );
        }
    }
}

fn spawn_completed_production_item(
    commands: &mut Commands,
    item: &ProductionItem,
    owner: &crate::components::CivId,
    position: &Position,
    unit_id_counter: &mut Local<u32>,
    city: &mut City,
    _world_map: &WorldMap,
    _current_turn: u32,
) {
    match item {
        ProductionItem::Unit(unit_type) => {
            spawn_military_unit_at_position(commands, unit_type, owner, position, unit_id_counter);
        }
        ProductionItem::Building(building_type) => {
            add_building_to_city(city, building_type);
        }
    }
}

fn spawn_military_unit_at_position(
    commands: &mut Commands,
    unit_type: &crate::components::military::UnitType,
    owner: &crate::components::CivId,
    position: &Position,
    unit_id_counter: &mut Local<u32>,
) {
    let unit = crate::MilitaryUnit::new(**unit_id_counter, *owner, *unit_type, *position);
    **unit_id_counter += 1;

    let mut entity_commands = commands.spawn((unit, *position));

    if is_player_controlled_civilization(owner) {
        entity_commands.insert(crate::PlayerControlled);
    }
}

fn add_building_to_city(city: &mut City, building_type: &crate::components::city::BuildingType) {
    city.add_building(building_type.clone());
}

fn is_player_controlled_civilization(owner: &crate::components::CivId) -> bool {
    owner.0 == PLAYER_CIVILIZATION_ID
}

pub fn auto_advance_turn_system(
    player_civs: Query<Entity, With<PlayerControlled>>,
    mut turn_advance: MessageWriter<RequestTurnAdvance>,
    game_config: Res<GameConfig>,
) {
    if should_auto_advance_turn_for_ai_only_game(&player_civs, &game_config) {
        turn_advance.write(RequestTurnAdvance);
    }
}

fn should_auto_advance_turn_for_ai_only_game(
    player_civs: &Query<Entity, With<PlayerControlled>>,
    game_config: &GameConfig,
) -> bool {
    // Only auto-advance if there are no player civs AND we're not in manual AI-only mode
    player_civs.is_empty() && !game_config.ai_only
}

#[derive(Message)]
pub struct RequestTurnAdvance;

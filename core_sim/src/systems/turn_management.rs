use crate::{
    components::{
        city::City,
        position::MovementOrder,
        production::{ProductionItem, ProductionQueue},
        turn_phases::{
            AITurnComplete, AllAITurnsComplete, ProcessAITurn, StartPlayerTurn, TurnPhase,
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
    civilizations: Query<(Entity, &Civilization), Without<PlayerControlled>>,
    mut ai_turn_events: MessageWriter<ProcessAITurn>,
    game_config: Res<GameConfig>,
) {
    for _request in turn_requests.read() {
        tracing::info!("Player requested turn advancement");

        // Start AI turn sequence
        let ai_civs: Vec<CivId> = civilizations.iter().map(|(_, civ)| civ.id).collect();

        if ai_civs.is_empty() {
            tracing::info!("No AI civilizations found, advancing turn immediately");
            *turn_phase = TurnPhase::TurnTransition;
        } else {
            if game_config.ai_only {
                // In AI-only mode, check current phase to determine what to do
                let current_phase = turn_phase.clone();
                match current_phase {
                    TurnPhase::PlayerTurn => {
                        // First press: Queue up AI turns but don't start them yet
                        tracing::info!(
                            "AI-only mode: Queuing {} AI civilizations for next turn",
                            ai_civs.len()
                        );
                        *turn_phase = TurnPhase::AITurnPending {
                            pending_ais: ai_civs,
                        };
                    }
                    TurnPhase::AITurnPending { pending_ais } => {
                        // Second press: Actually start the AI turns
                        tracing::info!(
                            "AI-only mode: Starting AI turn sequence for {} civilizations",
                            pending_ais.len()
                        );
                        if let Some(first_ai) = pending_ais.first() {
                            let remaining_ais = pending_ais.iter().skip(1).cloned().collect();
                            *turn_phase = TurnPhase::AITurn {
                                current_ai: *first_ai,
                                remaining_ais,
                            };
                            ai_turn_events.write(ProcessAITurn { civ_id: *first_ai });
                        }
                    }
                    TurnPhase::AITurnWaiting {
                        next_ai,
                        remaining_ais,
                    } => {
                        // Continue to next AI in sequence
                        tracing::info!(
                            "AI-only mode: Continuing to next AI {} in sequence",
                            next_ai.0
                        );
                        *turn_phase = TurnPhase::AITurn {
                            current_ai: next_ai,
                            remaining_ais,
                        };
                        ai_turn_events.write(ProcessAITurn { civ_id: next_ai });
                    }
                    _ => {
                        // In other phases, ignore additional requests
                        tracing::debug!(
                            "AI-only mode: Ignoring turn advance request during {:?}",
                            current_phase
                        );
                    }
                }
            } else {
                // Normal mode: start AI turns immediately
                tracing::info!(
                    "Starting AI turn sequence for {} civilizations",
                    ai_civs.len()
                );
                if let Some(first_ai) = ai_civs.first() {
                    let remaining_ais = ai_civs.iter().skip(1).cloned().collect();
                    *turn_phase = TurnPhase::AITurn {
                        current_ai: *first_ai,
                        remaining_ais,
                    };
                    ai_turn_events.write(ProcessAITurn { civ_id: *first_ai });
                }
            }
        }
    }
}

/// Handles AI turn processing
pub fn handle_ai_turn_processing(
    mut ai_turn_events: MessageReader<ProcessAITurn>,
    mut ai_complete_events: MessageWriter<AITurnComplete>,
    civilizations: Query<&Civilization>,
    mut commands: Commands,
    mut units_query: Query<(Entity, &mut MilitaryUnit, &mut Position), Without<PlayerControlled>>,
    world_map: Res<WorldMap>,
) {
    for ai_event in ai_turn_events.read() {
        tracing::info!("Processing AI turn for civilization {:?}", ai_event.civ_id);

        // Process the AI's turn (movement, production, etc.)
        process_ai_civilization_turn(
            ai_event.civ_id,
            &civilizations,
            &mut commands,
            &mut units_query,
            &world_map,
        );

        // Signal that this AI has completed their turn
        ai_complete_events.write(AITurnComplete {
            civ_id: ai_event.civ_id,
        });
    }
}

/// System to handle AI turn completion and advance to next AI or complete all turns
pub fn handle_ai_turn_completion(
    mut ai_complete_events: MessageReader<AITurnComplete>,
    mut turn_phase: ResMut<TurnPhase>,
    mut next_ai_events: MessageWriter<ProcessAITurn>,
    mut all_ai_complete_events: MessageWriter<AllAITurnsComplete>,
    game_config: Res<GameConfig>,
) {
    for _ai_event in ai_complete_events.read() {
        // Check if there are more AIs to process
        if let TurnPhase::AITurn {
            current_ai: _,
            remaining_ais,
        } = turn_phase.as_ref()
        {
            if remaining_ais.is_empty() {
                // All AIs have completed their turns
                tracing::info!("All AI civilizations have completed their turns");
                all_ai_complete_events.write(AllAITurnsComplete);
                *turn_phase = TurnPhase::TurnTransition;
            } else {
                // Move to next AI
                let next_ai = remaining_ais[0];
                let new_remaining: Vec<CivId> = remaining_ais.iter().skip(1).cloned().collect();

                if game_config.ai_only {
                    // In AI-only mode, pause and wait for user input before next AI
                    tracing::info!(
                        "AI-only mode: Waiting for next turn input before AI {} takes their turn",
                        next_ai.0
                    );
                    *turn_phase = TurnPhase::AITurnWaiting {
                        next_ai,
                        remaining_ais: new_remaining,
                    };
                } else {
                    // Normal mode: immediately continue to next AI
                    *turn_phase = TurnPhase::AITurn {
                        current_ai: next_ai,
                        remaining_ais: new_remaining,
                    };
                    next_ai_events.write(ProcessAITurn { civ_id: next_ai });
                }
            }
        }
    }
}

/// Process a single AI civilization's turn
fn process_ai_civilization_turn(
    civ_id: CivId,
    civilizations: &Query<&Civilization>,
    commands: &mut Commands,
    units_query: &mut Query<(Entity, &mut MilitaryUnit, &mut Position), Without<PlayerControlled>>,
    world_map: &WorldMap,
) {
    // Find the AI civilization
    if let Some(civ) = civilizations.iter().find(|civ| civ.id == civ_id) {
        tracing::info!("AI {} ({}) is taking their turn", civ.name, civ_id.0);

        // Count and move all AI units for this civilization
        let mut unit_count = 0;
        let mut moved_units = 0;

        for (entity, mut unit, mut position) in units_query.iter_mut() {
            if unit.owner == civ_id {
                unit_count += 1;
                tracing::debug!("Found AI unit {} for civilization {}", unit.id, civ_id.0);

                if unit.can_move() {
                    // Simple AI movement: try to move to an adjacent valid tile
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
            // Add a movement order for this AI unit
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

            // Only move to one position per turn
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

/// Check if a move from one position to another is valid
fn is_valid_move_target(from: Position, to: Position, world_map: &WorldMap) -> bool {
    // Check if target is within map bounds
    if let Some(tile) = world_map.get_tile(to) {
        // Check if the tile is walkable (not ocean)
        match tile.terrain {
            crate::TerrainType::Ocean => false,
            _ => {
                // Check if it's adjacent (Manhattan distance of 1)
                from.manhattan_distance_to(&to) == 1
            }
        }
    } else {
        false
    }
}

/// System to handle completion of all AI turns and advance to next player turn
pub fn handle_all_ai_turns_complete(
    mut ai_complete_events: MessageReader<AllAITurnsComplete>,
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
) {
    for _event in ai_complete_events.read() {
        tracing::info!("All AI turns complete, advancing to next turn");

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

        // Start next player turn
        *turn_phase = TurnPhase::PlayerTurn;
        start_player_events.write(StartPlayerTurn);

        tracing::info!("Advanced to turn {} - Player turn begins", current_turn.0);
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

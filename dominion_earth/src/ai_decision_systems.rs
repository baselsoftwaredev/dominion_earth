//! AI decision generation systems for Dominion Earth
//!
//! This module contains systems responsible for generating and processing
//! AI decisions during the game's AI turn phases.

use crate::{debug_utils::DebugLogging, game::GameState};
use ai_planner::ai_coordinator::AICoordinatorSystem;
use bevy::prelude::*;
use core_sim::{
    resources::CurrentTurn, AIAction, ActionQueue, CivId, Civilization, CivilizationData,
    GameState as CoreGameState, PlayerControlled, ProcessAITurn,
};

/// System to generate AI decisions only when it's an AI turn
pub fn generate_ai_decisions_on_ai_turn(
    mut ai_turn_events: EventReader<ProcessAITurn>,
    mut game_state: ResMut<GameState>,
    mut action_queues: Query<(&mut ActionQueue, &CivId)>,
    civs: Query<&Civilization, Without<PlayerControlled>>,
    current_turn: Res<CurrentTurn>,
    debug_logging: Res<DebugLogging>,
) {
    // Only generate AI decisions when we receive a ProcessAITurn event
    for _ai_turn_event in ai_turn_events.read() {
        let civilization_data = collect_civilization_data(&civs);
        let ai_game_state = create_ai_game_state(&current_turn, civilization_data);

        let ai_decisions = game_state
            ._ai_coordinator
            .generate_turn_decisions(&ai_game_state);

        log_ai_decision_generation(&debug_logging, &ai_decisions, &current_turn, &civs);
        populate_action_queues(ai_decisions, action_queues, current_turn);

        break; // Only process the first event to avoid duplicates
    }
}

/// Collect civilization data for AI coordinator
fn collect_civilization_data(
    civs: &Query<&Civilization, Without<PlayerControlled>>,
) -> std::collections::HashMap<CivId, CivilizationData> {
    let mut civilization_data = std::collections::HashMap::new();

    for civilization in civs.iter() {
        let civ_data = CivilizationData {
            civilization: civilization.clone(),
            cities: Vec::new(), // TODO: populate with actual city data when available
            territories: Vec::new(), // TODO: populate with actual territory data when available
            diplomatic_relations: Vec::new(), // TODO: populate with actual diplomatic relations when available
        };
        civilization_data.insert(civilization.id, civ_data);
    }

    civilization_data
}

/// Create game state for AI coordinator
fn create_ai_game_state(
    current_turn: &CurrentTurn,
    civilization_data: std::collections::HashMap<CivId, CivilizationData>,
) -> CoreGameState {
    CoreGameState {
        turn: current_turn.0,
        civilizations: civilization_data,
        current_player: None,
    }
}

/// Log AI decision generation results
fn log_ai_decision_generation(
    debug_logging: &DebugLogging,
    ai_decisions: &std::collections::HashMap<CivId, Vec<AIAction>>,
    current_turn: &CurrentTurn,
    civs: &Query<&Civilization, Without<PlayerControlled>>,
) {
    use crate::debug_utils::DebugUtils;

    DebugUtils::log_info(
        debug_logging,
        &format!(
            "Generated {} AI decisions for AI turn {} (civs found: {})",
            ai_decisions.len(),
            current_turn.0,
            civs.iter().count()
        ),
    );
}

/// Populate action queues with AI decisions
fn populate_action_queues(
    ai_decisions: std::collections::HashMap<CivId, Vec<AIAction>>,
    action_queues: Query<(&mut ActionQueue, &CivId)>,
    current_turn: Res<CurrentTurn>,
) {
    let ai_decisions_vec: Vec<(CivId, Vec<AIAction>)> = ai_decisions.into_iter().collect();

    core_sim::populate_action_queues_from_ai_decisions(
        ai_decisions_vec,
        action_queues,
        current_turn,
    );
}

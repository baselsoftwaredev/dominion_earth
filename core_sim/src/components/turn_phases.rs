use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::CivId;

/// Represents the current phase of a turn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnPhase {
    PlayerTurn,
    AITurn { current_ai: CivId, remaining_ais: Vec<CivId> },
    TurnTransition,
}

impl Default for TurnPhase {
    fn default() -> Self {
        TurnPhase::PlayerTurn
    }
}

// Manual Resource implementation
impl bevy_ecs::resource::Resource for TurnPhase {}

/// Event to signal that AI should take their turn
#[derive(Event)]
pub struct ProcessAITurn {
    pub civ_id: CivId,
}

/// Event to signal that an AI has completed their turn
#[derive(Event)]
pub struct AITurnComplete {
    pub civ_id: CivId,
}

/// Event to signal that all AI turns are complete
#[derive(Event)]
pub struct AllAITurnsComplete;

/// Event to signal start of player turn
#[derive(Event)]
pub struct StartPlayerTurn;
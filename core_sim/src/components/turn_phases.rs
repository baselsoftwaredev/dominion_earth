use crate::CivId;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the current phase of a turn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnPhase {
    PlayerTurn,
    AITurn {
        current_ai: CivId,
        remaining_ais: Vec<CivId>,
    },
    AITurnPending {
        pending_ais: Vec<CivId>,
    }, // Waiting to start AI turns
    AITurnWaiting {
        next_ai: CivId,
        remaining_ais: Vec<CivId>,
    }, // Waiting between AI turns
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

use crate::{constants::civilization_management::PLAYER_CIVILIZATION_ID, CivId};
use bevy::prelude::Reflect;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect, Resource)]
#[reflect(Resource)]
pub enum TurnPhase {
    CivilizationTurn { current_civ: CivId },
    WaitingForNextTurn { next_civ: CivId },
    TurnTransition,
}

impl Default for TurnPhase {
    fn default() -> Self {
        TurnPhase::CivilizationTurn {
            current_civ: CivId(PLAYER_CIVILIZATION_ID),
        }
    }
}

#[derive(Debug, Clone, Resource, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct TurnOrder {
    pub civilizations: Vec<CivId>,
    pub current_index: usize,
}

impl TurnOrder {
    pub fn new(civilizations: Vec<CivId>) -> Self {
        Self {
            civilizations,
            current_index: 0,
        }
    }

    pub fn current_civ(&self) -> Option<CivId> {
        self.civilizations.get(self.current_index).copied()
    }

    pub fn advance(&mut self) -> bool {
        if self.civilizations.is_empty() {
            return false;
        }

        self.current_index += 1;
        if self.current_index >= self.civilizations.len() {
            self.current_index = 0;
            true
        } else {
            false
        }
    }

    pub fn peek_next(&self) -> Option<CivId> {
        if self.civilizations.is_empty() {
            return None;
        }

        let next_index = (self.current_index + 1) % self.civilizations.len();
        self.civilizations.get(next_index).copied()
    }

    pub fn is_player_civ(&self, civ_id: CivId) -> bool {
        civ_id.0 == PLAYER_CIVILIZATION_ID
    }
}

impl Default for TurnOrder {
    fn default() -> Self {
        Self {
            civilizations: vec![CivId(PLAYER_CIVILIZATION_ID)],
            current_index: 0,
        }
    }
}

/// Event to signal that AI should take their turn
#[derive(Message)]
pub struct ProcessAITurn {
    pub civ_id: CivId,
}

/// Event to signal that an AI has completed their turn
#[derive(Message)]
pub struct AITurnComplete {
    pub civ_id: CivId,
}

/// Event to signal that all AI turns are complete
#[derive(Message)]
pub struct AllAITurnsComplete;

/// Event to signal start of player turn
#[derive(Message)]
pub struct StartPlayerTurn;

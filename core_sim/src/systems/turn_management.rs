use bevy_ecs::prelude::*;
use crate::{GameState, CivId};

/// System for managing turn progression
pub struct TurnManagementSystem;

impl TurnManagementSystem {
    /// Advance to the next turn
    pub fn advance_turn(
        mut game_state: ResMut<GameState>,
    ) {
        let previous_turn = game_state.turn;
        game_state.turn += 1;
        
        tracing::info!("Advanced from turn {} to turn {}", previous_turn, game_state.turn);
    }

    /// Check if turn should advance
    pub fn should_advance_turn(
        _game_state: Res<GameState>,
    ) -> bool {
        // In this simple implementation, advance every update
        // In a real game, this would check if all civilizations have finished their turns
        true
    }

    /// Initialize turn for all civilizations
    pub fn initialize_turn(
        civs: Query<&CivId>,
        game_state: Res<GameState>,
    ) {
        for civ_id in civs.iter() {
            // Reset turn-based state for each civilization
            tracing::debug!("Initializing turn {} for civ {:?}", game_state.turn, civ_id);
        }
    }

    /// Finalize turn cleanup
    pub fn finalize_turn(
        game_state: Res<GameState>,
    ) {
        tracing::debug!("Finalizing turn {}", game_state.turn);
        // Cleanup any temporary turn state
    }
}

use bevy_ecs::prelude::*;
use crate::{GameState, CivId, SimError};

/// System for managing turn progression
pub struct TurnManagementSystem;

impl TurnManagementSystem {
    /// Advance to the next turn
    pub fn advance_turn(
        mut game_state: ResMut<GameState>,
        mut commands: Commands,
    ) {
        game_state.current_turn += 1;
        
        // Emit turn changed event
        commands.add(|world: &mut World| {
            tracing::info!("Advanced to turn {}", world.resource::<GameState>().current_turn);
        });
    }

    /// Check if turn should advance
    pub fn should_advance_turn(
        game_state: Res<GameState>,
    ) -> bool {
        // In this simple implementation, advance every update
        // In a real game, this would check if all civilizations have finished their turns
        true
    }

    /// Initialize turn for all civilizations
    pub fn initialize_turn(
        mut civs: Query<&mut CivId>,
        game_state: Res<GameState>,
    ) {
        for mut civ_id in civs.iter_mut() {
            // Reset turn-based state for each civilization
            tracing::debug!("Initializing turn {} for civ {:?}", game_state.current_turn, civ_id);
        }
    }

    /// Finalize turn cleanup
    pub fn finalize_turn(
        game_state: Res<GameState>,
    ) {
        tracing::debug!("Finalizing turn {}", game_state.current_turn);
        // Cleanup any temporary turn state
    }
}

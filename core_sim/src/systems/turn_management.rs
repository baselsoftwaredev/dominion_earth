use bevy_ecs::prelude::*;
use crate::{
    components::{MilitaryUnit, PlayerActionsComplete, PlayerControlled},
    resources::CurrentTurn,
};

/// System to handle turn advancement requests
pub fn handle_turn_advance_requests(
    mut turn_requests: EventReader<RequestTurnAdvance>,
    mut current_turn: ResMut<CurrentTurn>,
    mut player_actions: ResMut<PlayerActionsComplete>,
    mut units: Query<&mut MilitaryUnit>,
) {
    for _request in turn_requests.read() {
        // Advance the turn
        current_turn.0 += 1;
        
        // Reset unit movement for all units
        for mut unit in units.iter_mut() {
            unit.reset_movement();
        }
        
        // Reset player actions tracking
        player_actions.reset();
        
        tracing::info!("Advanced to turn {}", current_turn.0);
    }
}

/// System to automatically process AI turns and request turn advance when appropriate
pub fn auto_advance_turn_system(
    player_civs: Query<Entity, With<PlayerControlled>>,
    mut turn_advance: EventWriter<RequestTurnAdvance>,
) {
    // If no player civilizations, auto-advance immediately
    if player_civs.is_empty() {
        turn_advance.write(RequestTurnAdvance);
    }
    // Note: We don't auto-advance when player civs exist - the player must manually end their turn
    // via the handle_end_turn_input system in the frontend
}

/// Event to request turn advancement
#[derive(Event)]
pub struct RequestTurnAdvance;

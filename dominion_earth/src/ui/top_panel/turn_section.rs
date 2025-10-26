use bevy::prelude::*;
use core_sim::resources::CurrentTurn;

// ============================================================================
// Marker Components
// ============================================================================

/// Marker component for the turn display text
#[derive(Component)]
pub struct TurnDisplayText;

// ============================================================================
// Update Systems
// ============================================================================

/// Update turn display from CurrentTurn resource
pub fn update_turn_display(
    current_turn: Res<CurrentTurn>,
    mut turn_text: Query<&mut Text, With<TurnDisplayText>>,
) {
    if current_turn.is_changed() {
        if let Some(mut text) = turn_text.iter_mut().next() {
            **text = format!("Turn: {}", current_turn.0);
        }
    }
}

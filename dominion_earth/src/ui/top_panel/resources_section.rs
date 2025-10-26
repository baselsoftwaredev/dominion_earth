use bevy::prelude::*;
use core_sim::{Civilization, PlayerControlled};

// ============================================================================
// Marker Components
// ============================================================================

/// Marker component for the gold display text
#[derive(Component)]
pub struct GoldDisplayText;

/// Marker component for the production display text
#[derive(Component)]
pub struct ProductionDisplayText;

// ============================================================================
// Update Systems
// ============================================================================

/// Update gold and production displays from player civilization
pub fn update_player_resources(
    player_query: Query<&Civilization, With<PlayerControlled>>,
    mut gold_text: Query<&mut Text, (With<GoldDisplayText>, Without<ProductionDisplayText>)>,
    mut production_text: Query<&mut Text, With<ProductionDisplayText>>,
) {
    if let Some(player_civ) = player_query.iter().next() {
        // Update gold display
        if let Some(mut text) = gold_text.iter_mut().next() {
            **text = format!("Gold: {}", player_civ.economy.gold);
        }

        // Update production display
        if let Some(mut text) = production_text.iter_mut().next() {
            **text = format!("Production: {}", player_civ.economy.production);
        }
    }
}

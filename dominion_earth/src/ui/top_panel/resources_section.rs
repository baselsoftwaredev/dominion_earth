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
    mut gold_text: Query<&mut Text, With<GoldDisplayText>>,
    mut production_text: Query<&mut Text, (With<ProductionDisplayText>, Without<GoldDisplayText>)>,
) {
    let Ok(player_civ) = player_query.single() else {
        return;
    };

    for mut text in gold_text.iter_mut() {
        let new_gold_text = format!("Gold: {:.0}", player_civ.economy.gold);
        if **text != new_gold_text {
            info!("Gold text changed from '{}' to '{}'", **text, new_gold_text);
            **text = new_gold_text;
        }
    }

    for mut text in production_text.iter_mut() {
        let new_production_text = format!("Production: {:.0}", player_civ.economy.production);
        if **text != new_production_text {
            info!(
                "Production text changed from '{}' to '{}'",
                **text, new_production_text
            );
            **text = new_production_text;
        }
    }
}

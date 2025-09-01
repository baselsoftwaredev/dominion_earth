use bevy_ecs::prelude::*;
use crate::{CivId, Civilization, CivPersonality, Position, MilitaryUnit, City, Economy, GameState, AIAction};

/// System for processing AI decisions
pub struct AIDecisionSystem;

impl AIDecisionSystem {
    pub fn new() -> Self {
        Self
    }

    /// Process AI decisions for all civilizations
    /// This will be called with decisions provided by the ai_planner
    pub fn process_ai_decisions(
        ai_decisions: Vec<(CivId, Vec<AIAction>)>,
        mut civs: Query<(&mut Civilization, &CivPersonality, &CivId)>,
        mut commands: Commands,
    ) {
        for (civ_id, decisions) in ai_decisions {
            // Find the civilization
            if let Some((mut civ, _personality, _)) = civs.iter_mut().find(|(_, _, id)| **id == civ_id) {
                for decision in decisions {
                    Self::execute_ai_action(&decision, &mut civ, &mut commands);
                    tracing::debug!("AI decision for {:?}: {:?}", civ_id, decision);
                }
            }
        }
    }

    /// Execute an AI action from the GOAP planner
    fn execute_ai_action(
        action: &AIAction,
        civ: &mut Civilization,
        commands: &mut Commands,
    ) {
        match action {
            AIAction::BuildUnit { unit_type, position, .. } => {
                tracing::info!("AI action: Build {:?} at {:?}", unit_type, position);
                // Implementation would spawn unit entity
            },
            AIAction::Research { technology, .. } => {
                tracing::info!("AI action: Research {}", technology);
                // Update research progress - this might need proper research queue
                // civ.technologies.current_research = Some(technology.clone());
            },
            AIAction::Expand { target_position, .. } => {
                tracing::info!("AI action: Expand to {:?}", target_position);
                // Implementation would spawn territory entity
            },
            AIAction::BuildBuilding { building_type, position, .. } => {
                tracing::info!("AI action: Build {:?} at {:?}", building_type, position);
                // Implementation would add building to city
            },
            AIAction::Trade { partner, resource, .. } => {
                tracing::info!("AI action: Trade {:?} with {:?}", resource, partner);
                // Implementation would create trade agreement
            },
            AIAction::Attack { target, target_position, .. } => {
                tracing::info!("AI action: Attack {:?} at {:?}", target, target_position);
                // Implementation would issue attack orders
            },
            AIAction::Diplomacy { target, action, .. } => {
                tracing::info!("AI action: Diplomatic {:?} with {:?}", action, target);
                // Implementation would handle diplomatic actions
            },
            AIAction::Defend { position, .. } => {
                tracing::info!("AI action: Defend position {:?}", position);
                // Implementation would position defensive units
            },
        }
    }
}

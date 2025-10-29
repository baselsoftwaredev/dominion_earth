use crate::{
    AIAction, City, CivId, CivPersonality, Civilization, Economy, GameState, MilitaryUnit, Position,
};
use bevy_ecs::prelude::*;

/// System for processing AI decisions
pub struct AIDecisionSystem;

impl AIDecisionSystem {
    pub fn new() -> Self {
        Self
    }

    /// Process AI decisions for all civilizations
    pub fn process_ai_decisions(
        ai_decisions: Vec<(CivId, Vec<AIAction>)>,
        mut civs: Query<(&mut Civilization, &CivPersonality, &CivId)>,
        mut commands: Commands,
    ) {
        for (civ_id, decisions) in ai_decisions {
            if let Some((mut civ, _personality, _)) =
                civs.iter_mut().find(|(_, _, id)| **id == civ_id)
            {
                for decision in decisions {
                    Self::execute_ai_action(&decision, &mut civ, &mut commands);
                    tracing::debug!("AI decision for {:?}: {:?}", civ_id, decision);
                }
            }
        }
    }

    /// Execute an AI action from the GOAP planner
    fn execute_ai_action(action: &AIAction, civ: &mut Civilization, commands: &mut Commands) {
        match action {
            AIAction::BuildUnit {
                unit_type,
                position,
                ..
            } => {
                tracing::info!("AI action: Build {:?} at {:?}", unit_type, position);
            }
            AIAction::Research { technology, .. } => {
                tracing::info!("AI action: Research {}", technology);
            }
            AIAction::Expand {
                target_position, ..
            } => {
                tracing::info!("AI action: Expand to {:?}", target_position);
            }
            AIAction::BuildBuilding {
                building_type,
                position,
                ..
            } => {
                tracing::info!("AI action: Build {:?} at {:?}", building_type, position);
            }
            AIAction::Trade {
                partner, resource, ..
            } => {
                tracing::info!("AI action: Trade {:?} with {:?}", resource, partner);
            }
            AIAction::Attack {
                target,
                target_position,
                ..
            } => {
                tracing::info!("AI action: Attack {:?} at {:?}", target, target_position);
            }
            AIAction::Diplomacy { target, action, .. } => {
                tracing::info!("AI action: Diplomatic {:?} with {:?}", action, target);
            }
            AIAction::Defend { position, .. } => {
                tracing::info!("AI action: Defend position {:?}", position);
            }
            AIAction::Explore {
                target_position, ..
            } => {
                tracing::info!("AI action: Explore towards {:?}", target_position);
            }
        }
    }
}

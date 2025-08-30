pub mod constants;
pub mod utility_ai;
pub mod goap;
pub mod htn_planner;
pub mod ai_coordinator;

use core_sim::{CivId, CivPersonality, GameState, AIAction};
use std::collections::HashMap;

/// Main AI coordinator that combines different AI approaches
#[derive(Debug)]
pub struct AICoordinator {
    pub utility_ai: utility_ai::UtilityAI,
    pub goap_planner: goap::GOAPPlanner,
    pub htn_planner: htn_planner::HTNPlanner,
    pub decision_cache: HashMap<CivId, Vec<AIAction>>,
}

impl Default for AICoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl AICoordinator {
    pub fn new() -> Self {
        Self {
            utility_ai: utility_ai::UtilityAI::new(),
            goap_planner: goap::GOAPPlanner::new(),
            htn_planner: htn_planner::HTNPlanner::new(),
            decision_cache: HashMap::new(),
        }
    }

    /// Generate AI decisions for all civilizations
    pub fn generate_decisions(&mut self, game_state: &GameState) -> HashMap<CivId, Vec<AIAction>> {
        let mut all_decisions = HashMap::new();

        for (civ_id, civ_data) in &game_state.civilizations {
            let decisions = self.generate_civ_decisions(*civ_id, civ_data, game_state);
            all_decisions.insert(*civ_id, decisions);
        }

        self.decision_cache = all_decisions.clone();
        all_decisions
    }

    fn generate_civ_decisions(
        &mut self,
        civ_id: CivId,
        civ_data: &core_sim::CivilizationData,
        game_state: &GameState,
    ) -> Vec<AIAction> {
        let personality = &civ_data.civilization.personality;
        let mut decisions = Vec::new();

        // Use Utility AI for immediate tactical decisions
        let utility_actions = self.utility_ai.evaluate_actions(civ_id, civ_data, game_state);
        decisions.extend(utility_actions);

        // Use GOAP for strategic planning
        let strategic_goals = self.determine_strategic_goals(personality, game_state);
        for goal in strategic_goals {
            if let Some(plan) = self.goap_planner.plan_for_goal(civ_id, &goal, game_state) {
                decisions.extend(plan);
            }
        }

        // Use HTN for complex multi-turn strategies
        let htn_tasks = self.determine_htn_tasks(personality, game_state);
        for task in htn_tasks {
            if let Some(plan) = self.htn_planner.decompose_task(civ_id, &task, game_state) {
                decisions.extend(plan);
            }
        }

        // Prioritize and filter decisions
        self.prioritize_decisions(&mut decisions, personality);
        decisions
    }

    fn determine_strategic_goals(&self, personality: &CivPersonality, _game_state: &GameState) -> Vec<StrategicGoal> {
        let mut goals = Vec::new();

        if personality.land_hunger > 0.6 {
            goals.push(StrategicGoal::ExpandTerritory);
        }

        if personality.tech_focus > 0.6 {
            goals.push(StrategicGoal::AdvanceTechnology);
        }

        if personality.industry_focus > 0.6 {
            goals.push(StrategicGoal::DevelopEconomy);
        }

        if personality.militarism > 0.6 {
            goals.push(StrategicGoal::BuildMilitary);
        }

        goals
    }

    fn determine_htn_tasks(&self, personality: &CivPersonality, _game_state: &GameState) -> Vec<HTNTask> {
        let mut tasks = Vec::new();

        if personality.interventionism > 0.5 {
            tasks.push(HTNTask::DiplomaticCampaign);
        }

        if personality.land_hunger > 0.7 && personality.militarism > 0.5 {
            tasks.push(HTNTask::ConquestCampaign);
        }

        if personality.industry_focus > 0.7 {
            tasks.push(HTNTask::EconomicDevelopment);
        }

        tasks
    }

    fn prioritize_decisions(&self, decisions: &mut Vec<AIAction>, personality: &CivPersonality) {
        decisions.sort_by(|a, b| {
            let priority_a = self.calculate_action_priority(a, personality);
            let priority_b = self.calculate_action_priority(b, personality);
            priority_b.partial_cmp(&priority_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit to top actions to prevent decision paralysis
        decisions.truncate(5);
    }

    fn calculate_action_priority(&self, action: &AIAction, personality: &CivPersonality) -> f32 {
        match action {
            AIAction::Expand { .. } => personality.land_hunger * 1.2,
            AIAction::Research { .. } => personality.tech_focus * 1.1,
            AIAction::BuildUnit { .. } => personality.militarism * 1.0,
            AIAction::BuildBuilding { .. } => personality.industry_focus * 0.9,
            AIAction::Trade { .. } => personality.industry_focus * 0.8,
            AIAction::Attack { .. } => personality.militarism * personality.risk_tolerance * 1.3,
            AIAction::Diplomacy { .. } => (1.0 - personality.isolationism) * 0.7,
            AIAction::Defend { .. } => 1.5, // Defense is always high priority
        }
    }

    /// Clear decision cache (called after decisions are processed)
    pub fn clear_cache(&mut self) {
        self.decision_cache.clear();
    }
}

/// Strategic goals for GOAP planning
#[derive(Debug, Clone, PartialEq)]
pub enum StrategicGoal {
    ExpandTerritory,
    AdvanceTechnology,
    DevelopEconomy,
    BuildMilitary,
    EstablishDiplomacy,
    DefendTerritory,
}

/// High-level tasks for HTN planning
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HTNTask {
    ConquestCampaign,
    DiplomaticCampaign,
    EconomicDevelopment,
    TechnologicalAdvancement,
    DefensivePreparation,
}

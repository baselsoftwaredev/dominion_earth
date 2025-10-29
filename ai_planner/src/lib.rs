pub mod ai_coordinator;
pub mod constants;
pub mod goap;
pub mod htn_planner;
pub mod utility_ai;

use constants::coordinator::{decision, htn};
use core_sim::{AIAction, CivId, CivPersonality, GameState};
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

        let utility_actions = self
            .utility_ai
            .evaluate_actions(civ_id, civ_data, game_state);
        decisions.extend(utility_actions);

        let strategic_goals = self.determine_strategic_goals(personality, game_state);
        for goal in strategic_goals {
            if let Some(plan) = self.goap_planner.plan_for_goal(civ_id, &goal, game_state) {
                decisions.extend(plan);
            }
        }

        let htn_tasks = self.determine_htn_tasks(personality, game_state);
        for task in htn_tasks {
            if let Some(plan) = self.htn_planner.decompose_task(civ_id, &task, game_state) {
                decisions.extend(plan);
            }
        }

        self.prioritize_decisions(&mut decisions, personality);
        decisions
    }

    fn determine_strategic_goals(
        &self,
        personality: &CivPersonality,
        game_state: &GameState,
    ) -> Vec<StrategicGoal> {
        let mut goals = Vec::new();

        if personality.militarism > decision::PERSONALITY_THRESHOLD_MODERATE {
            goals.push(StrategicGoal::BuildMilitary);
        }

        if personality.industry_focus > decision::PERSONALITY_THRESHOLD_HIGH {
            goals.push(StrategicGoal::DevelopEconomy);
        }

        if personality.exploration_drive > decision::EXPLORATION_PERSONALITY_THRESHOLD
            && game_state.turn < decision::EARLY_GAME_EXPLORATION_TURN_LIMIT
        {
            goals.push(StrategicGoal::ExploreTerritory);
        }

        goals
    }

    fn determine_htn_tasks(
        &self,
        personality: &CivPersonality,
        _game_state: &GameState,
    ) -> Vec<HTNTask> {
        let mut tasks = Vec::new();

        if personality.interventionism > decision::PERSONALITY_THRESHOLD_MODERATE {
            tasks.push(HTNTask::DiplomaticCampaign);
        }

        if personality.land_hunger > htn::LAND_HUNGER_CONQUEST_THRESHOLD
            && personality.militarism > decision::PERSONALITY_THRESHOLD_MODERATE
        {
            tasks.push(HTNTask::ConquestCampaign);
        }

        if personality.industry_focus > htn::INDUSTRY_FOCUS_ECONOMY_THRESHOLD {
            tasks.push(HTNTask::EconomicDevelopment);
        }

        tasks
    }

    fn prioritize_decisions(&self, decisions: &mut Vec<AIAction>, personality: &CivPersonality) {
        decisions.sort_by(|a, b| {
            let priority_a = self.calculate_action_priority(a, personality);
            let priority_b = self.calculate_action_priority(b, personality);
            priority_b
                .partial_cmp(&priority_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        decisions.truncate(decision::MAX_DECISIONS_PER_TURN);
    }

    fn calculate_action_priority(&self, action: &AIAction, personality: &CivPersonality) -> f32 {
        match action {
            AIAction::Expand { .. } => personality.land_hunger * decision::PRIORITY_WEIGHT_EXPAND,
            AIAction::Research { .. } => {
                personality.tech_focus * decision::PRIORITY_WEIGHT_RESEARCH
            }
            AIAction::BuildUnit { .. } => {
                personality.militarism * decision::PRIORITY_WEIGHT_BUILD_UNIT
            }
            AIAction::BuildBuilding { .. } => {
                personality.industry_focus * decision::PRIORITY_WEIGHT_BUILD_BUILDING
            }
            AIAction::Trade { .. } => personality.industry_focus * decision::PRIORITY_WEIGHT_TRADE,
            AIAction::Attack { .. } => {
                personality.militarism
                    * personality.risk_tolerance
                    * decision::PRIORITY_WEIGHT_ATTACK
            }
            AIAction::Diplomacy { .. } => {
                (1.0 - personality.isolationism) * decision::PRIORITY_WEIGHT_DIPLOMACY
            }
            AIAction::Defend { .. } => decision::PRIORITY_BASE_DEFEND,
            AIAction::Explore { .. } => {
                personality.exploration_drive * decision::PRIORITY_WEIGHT_EXPLORE
            }
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
    ExploreTerritory,
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

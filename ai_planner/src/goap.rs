use crate::constants::goap::{defaults, goals, planning};
use crate::{AIAction, StrategicGoal};
use core_sim::{
    BuildingType, CivId, DiplomaticAction, GameResource as Resource, GameState, Position, UnitType,
};
use std::collections::{HashMap, HashSet, VecDeque};

/// Goal-Oriented Action Planning (GOAP) system
#[derive(Debug, Clone)]
pub struct GOAPPlanner {
    actions: Vec<GOAPAction>,
    _max_planning_depth: usize, // Reserved for future depth limiting
}

impl GOAPPlanner {
    pub fn new() -> Self {
        Self {
            actions: Self::create_goap_actions(),
            _max_planning_depth: planning::MAX_PLANNING_DEPTH,
        }
    }

    /// Plan a sequence of actions to achieve a goal
    pub fn plan_for_goal(
        &self,
        civ_id: CivId,
        goal: &StrategicGoal,
        game_state: &GameState,
    ) -> Option<Vec<AIAction>> {
        let current_state = self.extract_world_state(civ_id, game_state);
        let goal_state = self.create_goal_state(goal, &current_state);

        self.a_star_plan(&current_state, &goal_state, civ_id, game_state)
    }

    fn extract_world_state(&self, civ_id: CivId, game_state: &GameState) -> WorldState {
        let mut state = WorldState::new();

        if let Some(civ_data) = game_state.civilizations.get(&civ_id) {
            // Territory control
            let territory_count = civ_data.territories.len();
            state.set("territory_count", territory_count as f32);

            // Military strength
            state.set(
                "military_strength",
                civ_data.civilization.military.total_strength,
            );

            // Economic state
            state.set("gold", civ_data.civilization.economy.gold);
            state.set("income", civ_data.civilization.economy.income);

            // Technology level
            let tech_count = civ_data
                .civilization
                .technologies
                .known
                .values()
                .filter(|&&v| v)
                .count();
            state.set("technology_level", tech_count as f32);

            // City count
            state.set("city_count", civ_data.cities.len() as f32);

            // Has capital
            state.set(
                "has_capital",
                if civ_data.civilization.capital.is_some() {
                    1.0
                } else {
                    0.0
                },
            );

            // Trade routes
            state.set(
                "trade_routes",
                civ_data.civilization.economy.trade_routes.len() as f32,
            );
        }

        state
    }

    fn create_goal_state(&self, goal: &StrategicGoal, current_state: &WorldState) -> WorldState {
        let mut goal_state = current_state.clone();

        match goal {
            StrategicGoal::ExpandTerritory => {
                let current_territory = current_state.get("territory_count").unwrap_or(defaults::DEFAULT_STATE_VALUE);
                goal_state.set("territory_count", current_territory + goals::TERRITORY_EXPANSION_TARGET);
            }
            StrategicGoal::AdvanceTechnology => {
                let current_tech = current_state.get("technology_level").unwrap_or(defaults::DEFAULT_STATE_VALUE);
                goal_state.set("technology_level", current_tech + goals::TECHNOLOGY_ADVANCEMENT_TARGET);
            }
            StrategicGoal::DevelopEconomy => {
                let current_income = current_state.get("income").unwrap_or(defaults::DEFAULT_STATE_VALUE);
                goal_state.set("income", current_income * goals::INCOME_MULTIPLIER);
                let current_trade = current_state.get("trade_routes").unwrap_or(defaults::DEFAULT_STATE_VALUE);
                goal_state.set("trade_routes", current_trade + goals::TRADE_ROUTES_TARGET);
            }
            StrategicGoal::BuildMilitary => {
                let current_military = current_state.get("military_strength").unwrap_or(0.0);
                goal_state.set("military_strength", current_military * 1.5);
            }
            StrategicGoal::EstablishDiplomacy => {
                goal_state.set("diplomatic_relations", 3.0);
            }
            StrategicGoal::DefendTerritory => {
                let current_military = current_state.get("military_strength").unwrap_or(0.0);
                goal_state.set("military_strength", current_military * 1.3);
                goal_state.set("fortifications", 2.0);
            }
        }

        goal_state
    }

    fn a_star_plan(
        &self,
        start_state: &WorldState,
        goal_state: &WorldState,
        civ_id: CivId,
        game_state: &GameState,
    ) -> Option<Vec<AIAction>> {
        let mut open_set = VecDeque::new();
        let mut closed_set = HashSet::new();
        let mut came_from: HashMap<WorldState, (WorldState, GOAPAction)> = HashMap::new();
        let mut g_score: HashMap<WorldState, f32> = HashMap::new();

        open_set.push_back(start_state.clone());
        g_score.insert(start_state.clone(), 0.0);

        let mut iterations = 0;
        let max_iterations = 1000;

        while let Some(current_state) = open_set.pop_front() {
            iterations += 1;
            if iterations > max_iterations {
                break;
            }

            if self.is_goal_satisfied(&current_state, goal_state) {
                return Some(self.reconstruct_plan(&came_from, &current_state, civ_id, game_state));
            }

            closed_set.insert(current_state.clone());

            for action in &self.actions {
                if !action.preconditions_met(&current_state, civ_id, game_state) {
                    continue;
                }

                let new_state = action.apply_effects(&current_state);

                if closed_set.contains(&new_state) {
                    continue;
                }

                let tentative_g_score =
                    g_score.get(&current_state).unwrap_or(&f32::INFINITY) + action.cost;

                if tentative_g_score < *g_score.get(&new_state).unwrap_or(&f32::INFINITY) {
                    came_from.insert(new_state.clone(), (current_state.clone(), action.clone()));
                    g_score.insert(new_state.clone(), tentative_g_score);

                    if !open_set.iter().any(|state| *state == new_state) {
                        open_set.push_back(new_state);
                    }
                }
            }
        }

        None
    }

    fn is_goal_satisfied(&self, current_state: &WorldState, goal_state: &WorldState) -> bool {
        for (key, &target_value) in &goal_state.values {
            let current_value = current_state.get(key).unwrap_or(0.0);
            if current_value < target_value as f32 {
                return false;
            }
        }
        true
    }

    fn reconstruct_plan(
        &self,
        came_from: &HashMap<WorldState, (WorldState, GOAPAction)>,
        final_state: &WorldState,
        civ_id: CivId,
        game_state: &GameState,
    ) -> Vec<AIAction> {
        let mut plan = Vec::new();
        let mut current_state = final_state.clone();

        while let Some((previous_state, action)) = came_from.get(&current_state) {
            if let Some(ai_action) = action.to_ai_action(civ_id, game_state) {
                plan.push(ai_action);
            }
            current_state = previous_state.clone();
        }

        plan.reverse();
        plan
    }

    fn create_goap_actions() -> Vec<GOAPAction> {
        vec![
            // Expand territory action
            GOAPAction {
                name: "expand_territory".to_string(),
                cost: 2.0,
                preconditions: vec![("has_capital".to_string(), 1.0), ("gold".to_string(), 10.0)],
                effects: vec![
                    ("territory_count".to_string(), 1.0), // Adds 1 territory
                ],
                action_type: GOAPActionType::Expand,
            },
            // Research technology action
            GOAPAction {
                name: "research_technology".to_string(),
                cost: 3.0,
                preconditions: vec![("gold".to_string(), 50.0)],
                effects: vec![("technology_level".to_string(), 1.0)],
                action_type: GOAPActionType::Research,
            },
            // Build military unit action
            GOAPAction {
                name: "build_military_unit".to_string(),
                cost: 2.5,
                preconditions: vec![("gold".to_string(), 30.0), ("city_count".to_string(), 1.0)],
                effects: vec![("military_strength".to_string(), 10.0)],
                action_type: GOAPActionType::BuildMilitary,
            },
            // Establish trade route action
            GOAPAction {
                name: "establish_trade".to_string(),
                cost: 1.5,
                preconditions: vec![("city_count".to_string(), 1.0)],
                effects: vec![
                    ("trade_routes".to_string(), 1.0),
                    ("income".to_string(), 5.0),
                ],
                action_type: GOAPActionType::Trade,
            },
            // Build economic building action
            GOAPAction {
                name: "build_economic_building".to_string(),
                cost: 2.0,
                preconditions: vec![("gold".to_string(), 25.0), ("city_count".to_string(), 1.0)],
                effects: vec![("income".to_string(), 3.0)],
                action_type: GOAPActionType::BuildEconomic,
            },
        ]
    }
}

impl Default for GOAPPlanner {
    fn default() -> Self {
        Self::new()
    }
}

/// World state representation for GOAP planning
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldState {
    values: HashMap<String, i32>, // Using i32 for hash compatibility
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: f32) {
        self.values.insert(key.to_string(), (value * 100.0) as i32); // Scale for precision
    }

    pub fn get(&self, key: &str) -> Option<f32> {
        self.values.get(key).map(|&v| v as f32 / 100.0)
    }

    pub fn add(&mut self, key: &str, delta: f32) {
        let current = self.get(key).unwrap_or(0.0);
        self.set(key, current + delta);
    }
}

impl std::hash::Hash for WorldState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Sort keys for consistent hashing
        let mut sorted_keys: Vec<_> = self.values.keys().collect();
        sorted_keys.sort();

        for key in sorted_keys {
            key.hash(state);
            self.values[key].hash(state);
        }
    }
}

/// GOAP action definition
#[derive(Debug, Clone)]
pub struct GOAPAction {
    pub name: String,
    pub cost: f32,
    pub preconditions: Vec<(String, f32)>,
    pub effects: Vec<(String, f32)>,
    pub action_type: GOAPActionType,
}

#[derive(Debug, Clone)]
pub enum GOAPActionType {
    Expand,
    Research,
    BuildMilitary,
    Trade,
    BuildEconomic,
    Diplomacy,
}

impl GOAPAction {
    pub fn preconditions_met(
        &self,
        state: &WorldState,
        _civ_id: CivId,
        _game_state: &GameState,
    ) -> bool {
        for (key, required_value) in &self.preconditions {
            let current_value = state.get(key).unwrap_or(0.0);
            if current_value < *required_value {
                return false;
            }
        }
        true
    }

    pub fn apply_effects(&self, state: &WorldState) -> WorldState {
        let mut new_state = state.clone();

        for (key, effect_value) in &self.effects {
            new_state.add(key, *effect_value);
        }

        // Apply costs (reduce gold)
        new_state.add("gold", -self.cost * 5.0); // Cost scaling

        new_state
    }

    pub fn to_ai_action(&self, civ_id: CivId, game_state: &GameState) -> Option<AIAction> {
        let civ_data = game_state.civilizations.get(&civ_id)?;
        let capital = civ_data
            .civilization
            .capital
            .unwrap_or(Position::new(50, 25));

        match self.action_type {
            GOAPActionType::Expand => {
                // Find suitable expansion target
                // TODO: Add world_map back to GameState
                // let neighbors = game_state.world_map.neighbors(capital);
                // For now, use a simple adjacent position
                let target_position = Position::new(capital.x + 1, capital.y);
                Some(AIAction::Expand {
                    target_position,
                    priority: 1.0 - self.cost / 10.0,
                })
            }
            GOAPActionType::Research => {
                Some(AIAction::Research {
                    technology: "Agriculture".to_string(), // Simplified
                    priority: 1.0 - self.cost / 10.0,
                })
            }
            GOAPActionType::BuildMilitary => Some(AIAction::BuildUnit {
                unit_type: UnitType::Infantry,
                position: capital,
                priority: 1.0 - self.cost / 10.0,
            }),
            GOAPActionType::Trade => {
                // Find trade partner
                for other_civ in game_state.civilizations.values() {
                    if other_civ.civilization.id != civ_id {
                        return Some(AIAction::Trade {
                            partner: other_civ.civilization.id,
                            resource: Resource::Gold,
                            priority: 1.0 - self.cost / 10.0,
                        });
                    }
                }
                None
            }
            GOAPActionType::BuildEconomic => Some(AIAction::BuildBuilding {
                building_type: BuildingType::Market,
                position: capital,
                priority: 1.0 - self.cost / 10.0,
            }),
            GOAPActionType::Diplomacy => {
                // Find diplomatic target
                for other_civ in game_state.civilizations.values() {
                    if other_civ.civilization.id != civ_id {
                        return Some(AIAction::Diplomacy {
                            target: other_civ.civilization.id,
                            action: DiplomaticAction::ProposeTradePact,
                            priority: 1.0 - self.cost / 10.0,
                        });
                    }
                }
                None
            }
        }
    }
}

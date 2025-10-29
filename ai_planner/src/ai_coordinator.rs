use crate::constants::coordinator::{cooldowns, costs, defense, diplomacy, territory, trade};
use crate::{AIAction, AICoordinator};
use core_sim::{
    BuildingType, CivId, DiplomaticAction, GameResource as Resource, GameState, UnitType,
};
use std::collections::HashMap;

/// Coordinates AI decision making for all civilizations
pub struct AICoordinatorSystem {
    coordinator: AICoordinator,
    turn_cooldown: HashMap<CivId, u32>,
}

impl AICoordinatorSystem {
    pub fn new() -> Self {
        Self {
            coordinator: AICoordinator::new(),
            turn_cooldown: HashMap::new(),
        }
    }

    pub fn generate_turn_decisions(
        &mut self,
        game_state: &GameState,
    ) -> HashMap<CivId, Vec<AIAction>> {
        for cooldown in self.turn_cooldown.values_mut() {
            *cooldown = cooldown.saturating_sub(cooldowns::COOLDOWN_DECREMENT);
        }

        let mut decisions = HashMap::new();

        for &civ_id in game_state.civilizations.keys() {
            if self
                .turn_cooldown
                .get(&civ_id)
                .unwrap_or(&cooldowns::NO_COOLDOWN)
                > &cooldowns::NO_COOLDOWN
            {
                continue;
            }

            let civ_decisions = self.coordinator.generate_decisions(game_state);
            if let Some(actions) = civ_decisions.get(&civ_id) {
                if !actions.is_empty() {
                    decisions.insert(civ_id, actions.clone());

                    let cooldown = match actions.len() {
                        cooldowns::MIN_ACTIONS_NO_COOLDOWN..=cooldowns::MAX_ACTIONS_NO_COOLDOWN => {
                            cooldowns::NO_COOLDOWN
                        }
                        cooldowns::MIN_ACTIONS_SHORT_COOLDOWN
                            ..=cooldowns::MAX_ACTIONS_SHORT_COOLDOWN => {
                            cooldowns::SHORT_COOLDOWN_DURATION
                        }
                        _ => cooldowns::LONG_COOLDOWN_DURATION,
                    };
                    self.turn_cooldown.insert(civ_id, cooldown);
                }
            }
        }

        decisions
    }

    pub fn execute_decisions(
        &self,
        decisions: &HashMap<CivId, Vec<AIAction>>,
        game_state: &mut GameState,
    ) -> Vec<ExecutionResult> {
        let mut results = Vec::new();

        for (civ_id, actions) in decisions {
            for action in actions {
                let result = self.execute_single_action(*civ_id, action, game_state);
                results.push(result);
            }
        }

        results
    }

    fn execute_single_action(
        &self,
        civ_id: CivId,
        action: &AIAction,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        match action {
            AIAction::Expand {
                target_position, ..
            } => self.execute_expand(civ_id, *target_position, game_state),
            AIAction::Research { technology, .. } => {
                self.execute_research(civ_id, technology, game_state)
            }
            AIAction::BuildUnit {
                unit_type,
                position,
                ..
            } => self.execute_build_unit(civ_id, unit_type.clone(), *position, game_state),
            AIAction::BuildBuilding {
                building_type,
                position,
                ..
            } => self.execute_build_building(civ_id, building_type.clone(), *position, game_state),
            AIAction::Trade {
                partner, resource, ..
            } => self.execute_trade(civ_id, *partner, resource.clone(), game_state),
            AIAction::Attack {
                target,
                target_position,
                ..
            } => self.execute_attack(civ_id, *target, *target_position, game_state),
            AIAction::Diplomacy { target, action, .. } => {
                self.execute_diplomacy(civ_id, *target, action, game_state)
            }
            AIAction::Defend { position, .. } => self.execute_defend(civ_id, *position, game_state),
            AIAction::Explore {
                target_position, ..
            } => self.execute_explore(civ_id, *target_position, game_state),
        }
    }

    fn execute_expand(
        &self,
        civ_id: CivId,
        target_position: core_sim::Position,
        _game_state: &mut GameState,
    ) -> ExecutionResult {
        ExecutionResult::Success {
            civ_id,
            action_description: format!("Attempted expansion to {:?}", target_position),
        }
    }

    fn execute_research(
        &self,
        civ_id: CivId,
        technology: &str,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
            let cost = costs::BASE_RESEARCH_COST;

            if civ_data.civilization.economy.gold >= cost {
                civ_data.civilization.economy.gold -= cost;
                civ_data
                    .civilization
                    .technologies
                    .known
                    .insert(technology.to_string(), true);

                ExecutionResult::Success {
                    civ_id,
                    action_description: format!("Researched {}", technology),
                }
            } else {
                ExecutionResult::Failed {
                    civ_id,
                    reason: "Insufficient gold for research".to_string(),
                }
            }
        } else {
            ExecutionResult::Failed {
                civ_id,
                reason: "Civilization not found".to_string(),
            }
        }
    }

    fn execute_build_unit(
        &self,
        civ_id: CivId,
        unit_type: UnitType,
        position: core_sim::Position,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
            let cost = costs::BASE_UNIT_COST;

            if civ_data.civilization.economy.gold >= cost {
                civ_data.civilization.economy.gold -= cost;

                let unit = core_sim::MilitaryUnit::new(
                    civ_data.civilization.military.units.len() as u32,
                    civ_id,
                    unit_type.clone(),
                    position,
                );

                civ_data.civilization.military.units.push(unit.clone());
                civ_data.civilization.military.total_strength +=
                    unit.effective_attack() + unit.effective_defense();

                ExecutionResult::Success {
                    civ_id,
                    action_description: format!("Built {:?} at {:?}", unit_type, position),
                }
            } else {
                ExecutionResult::Failed {
                    civ_id,
                    reason: "Insufficient gold for unit".to_string(),
                }
            }
        } else {
            ExecutionResult::Failed {
                civ_id,
                reason: "Civilization not found".to_string(),
            }
        }
    }

    fn execute_build_building(
        &self,
        civ_id: CivId,
        building_type: BuildingType,
        _position: core_sim::Position,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
            let cost = costs::BASE_BUILDING_COST;

            if civ_data.civilization.economy.gold >= cost {
                civ_data.civilization.economy.gold -= cost;

                for city in &mut civ_data.cities {
                    city.buildings.push(core_sim::Building {
                        building_type: building_type.clone(),
                        level: 1,
                    });

                    return ExecutionResult::Success {
                        civ_id,
                        action_description: format!("Built {:?} in {}", building_type, city.name),
                    };
                }

                ExecutionResult::Failed {
                    civ_id,
                    reason: "No city found for building".to_string(),
                }
            } else {
                ExecutionResult::Failed {
                    civ_id,
                    reason: "Insufficient gold for building".to_string(),
                }
            }
        } else {
            ExecutionResult::Failed {
                civ_id,
                reason: "Civilization not found".to_string(),
            }
        }
    }

    fn execute_trade(
        &self,
        civ_id: CivId,
        partner: CivId,
        _resource: Resource,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        let partner_capital = game_state
            .civilizations
            .get(&partner)
            .and_then(|p| p.civilization.capital)
            .unwrap_or(core_sim::Position::new(60, 25));

        if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
            let trade_route = core_sim::TradeRoute {
                from: civ_data
                    .civilization
                    .capital
                    .unwrap_or(core_sim::Position::new(50, 25)),
                to: partner_capital,
                value: trade::DEFAULT_TRADE_ROUTE_VALUE,
                security: trade::DEFAULT_TRADE_ROUTE_SECURITY,
            };

            civ_data.civilization.economy.trade_routes.push(trade_route);
            civ_data.civilization.economy.income += trade::TRADE_INCOME_BONUS;

            ExecutionResult::Success {
                civ_id,
                action_description: format!("Established trade with {:?}", partner),
            }
        } else {
            ExecutionResult::Failed {
                civ_id,
                reason: "Civilization not found".to_string(),
            }
        }
    }

    fn execute_attack(
        &self,
        civ_id: CivId,
        target: CivId,
        _target_position: core_sim::Position,
        _game_state: &mut GameState,
    ) -> ExecutionResult {
        let _relation_key = (civ_id, target);

        ExecutionResult::Success {
            civ_id,
            action_description: format!("Declared war on {:?}", target),
        }
    }

    fn execute_diplomacy(
        &self,
        civ_id: CivId,
        target: CivId,
        _action: &DiplomaticAction,
        _game_state: &mut GameState,
    ) -> ExecutionResult {
        ExecutionResult::Success {
            civ_id,
            action_description: format!("Initiated diplomacy with {:?}", target),
        }
    }

    fn execute_defend(
        &self,
        civ_id: CivId,
        position: core_sim::Position,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
            for unit in &mut civ_data.civilization.military.units {
                if unit.position.distance_to(&position) < defense::DEFENSIVE_POSITIONING_DISTANCE {
                    unit.position = position;
                }
            }

            ExecutionResult::Success {
                civ_id,
                action_description: format!("Defensive positions at {:?}", position),
            }
        } else {
            ExecutionResult::Failed {
                civ_id,
                reason: "Civilization not found".to_string(),
            }
        }
    }

    fn execute_explore(
        &self,
        civ_id: CivId,
        target_position: core_sim::Position,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        if let Some(civ_data) = game_state.civilizations.get(&civ_id) {
            let has_units = !civ_data.civilization.military.units.is_empty();

            if has_units {
                ExecutionResult::Success {
                    civ_id,
                    action_description: format!("Exploring towards {:?}", target_position),
                }
            } else {
                ExecutionResult::Failed {
                    civ_id,
                    reason: "No units available for exploration".to_string(),
                }
            }
        } else {
            ExecutionResult::Failed {
                civ_id,
                reason: "Civilization not found".to_string(),
            }
        }
    }
}

impl Default for AICoordinatorSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of executing an AI action
#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Success {
        civ_id: CivId,
        action_description: String,
    },
    Failed {
        civ_id: CivId,
        reason: String,
    },
}

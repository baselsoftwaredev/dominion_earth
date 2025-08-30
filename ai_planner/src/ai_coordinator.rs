use crate::constants::coordinator::{cooldowns, costs, military, territory};
use crate::{AICoordinator, AIAction};
use core_sim::{
    CivId, GameState, 
    UnitType, BuildingType, DiplomaticAction,
    GameResource as Resource,
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

    /// Generate decisions for all civilizations
    pub fn generate_turn_decisions(&mut self, game_state: &GameState) -> HashMap<CivId, Vec<AIAction>> {
        // Update cooldowns
        for cooldown in self.turn_cooldown.values_mut() {
            *cooldown = cooldown.saturating_sub(cooldowns::COOLDOWN_DECREMENT);
        }

        let mut decisions = HashMap::new();

        for &civ_id in game_state.civilizations.keys() {
            // Check if this civ can make decisions this turn
            if self.turn_cooldown.get(&civ_id).unwrap_or(&cooldowns::NO_COOLDOWN) > &cooldowns::NO_COOLDOWN {
                continue;
            }

            // Generate decisions for this civilization
            let civ_decisions = self.coordinator.generate_decisions(game_state);
            if let Some(actions) = civ_decisions.get(&civ_id) {
                if !actions.is_empty() {
                    decisions.insert(civ_id, actions.clone());
                    
                    // Set cooldown based on number of actions taken
                    let cooldown = match actions.len() {
                        cooldowns::MIN_ACTIONS_NO_COOLDOWN..=cooldowns::MAX_ACTIONS_NO_COOLDOWN => cooldowns::NO_COOLDOWN,
                        cooldowns::MIN_ACTIONS_SHORT_COOLDOWN..=cooldowns::MAX_ACTIONS_SHORT_COOLDOWN => cooldowns::SHORT_COOLDOWN_DURATION,
                        _ => cooldowns::LONG_COOLDOWN_DURATION,
                    };
                    self.turn_cooldown.insert(civ_id, cooldown);
                }
            }
        }

        decisions
    }

    /// Execute AI decisions in the game world
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
            AIAction::Expand { target_position, .. } => {
                self.execute_expand(civ_id, *target_position, game_state)
            }
            AIAction::Research { technology, .. } => {
                self.execute_research(civ_id, technology, game_state)
            }
            AIAction::BuildUnit { unit_type, position, .. } => {
                self.execute_build_unit(civ_id, unit_type.clone(), *position, game_state)
            }
            AIAction::BuildBuilding { building_type, position, .. } => {
                self.execute_build_building(civ_id, building_type.clone(), *position, game_state)
            }
            AIAction::Trade { partner, resource, .. } => {
                self.execute_trade(civ_id, *partner, resource.clone(), game_state)
            }
            AIAction::Attack { target, target_position, .. } => {
                self.execute_attack(civ_id, *target, *target_position, game_state)
            }
            AIAction::Diplomacy { target, action, .. } => {
                self.execute_diplomacy(civ_id, *target, action, game_state)
            }
            AIAction::Defend { position, .. } => {
                self.execute_defend(civ_id, *position, game_state)
            }
        }
    }

    fn execute_expand(
        &self,
        civ_id: CivId,
        target_position: core_sim::Position,
        _game_state: &mut GameState,
    ) -> ExecutionResult {
        // TODO: Replace with actual world_map integration
        /*
        if let Some(tile) = game_state.world_map.get_tile_mut(target_position) {
            if tile.owner.is_none() {
                tile.owner = Some(civ_id);
                
                // Add territory to civilization data
                if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
                    let territory = core_sim::Territory {
                        owner: civ_id,
                        control_strength: territory::DEFAULT_CONTROL_STRENGTH,
                        terrain_type: tile.terrain.clone(),
                    };
                    civ_data.territories.push((target_position, territory));
                }

                ExecutionResult::Success {
                    civ_id,
                    action_description: format!("Expanded to {:?}", target_position),
                }
            } else {
                ExecutionResult::Failed {
                    civ_id,
                    reason: "Territory already owned".to_string(),
                }
            }
        } else {
            ExecutionResult::Failed {
                civ_id,
                reason: "Invalid position".to_string(),
            }
        }
        */
        
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
            let cost = costs::BASE_RESEARCH_COST; // Base research cost
            
            if civ_data.civilization.economy.gold >= cost {
                civ_data.civilization.economy.gold -= cost;
                civ_data.civilization.technologies.known.insert(technology.to_string(), true);
                
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
            let cost = costs::BASE_UNIT_COST; // Base unit cost
            
            if civ_data.civilization.economy.gold >= cost {
                civ_data.civilization.economy.gold -= cost;
                
                let unit = core_sim::MilitaryUnit {
                    id: civ_data.civilization.military.units.len() as u32,
                    owner: civ_id,
                    unit_type: unit_type.clone(),
                    position,
                    strength: military::DEFAULT_UNIT_STRENGTH,
                    movement_remaining: military::DEFAULT_UNIT_MOVEMENT,
                    experience: military::DEFAULT_UNIT_EXPERIENCE,
                };
                
                civ_data.civilization.military.units.push(unit);
                civ_data.civilization.military.total_strength += military::DEFAULT_UNIT_STRENGTH;
                
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
            let cost = 25.0; // Base building cost
            
            if civ_data.civilization.economy.gold >= cost {
                civ_data.civilization.economy.gold -= cost;
                
                // Find city at position and add building
                for city in &mut civ_data.cities {
                    // Simplified: add building to first city
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
        // Get partner's capital before mutable borrow
        let partner_capital = game_state.civilizations.get(&partner)
            .and_then(|p| p.civilization.capital)
            .unwrap_or(core_sim::Position::new(60, 25));
            
        if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
            // Simplified trade implementation
            let trade_route = core_sim::TradeRoute {
                from: civ_data.civilization.capital.unwrap_or(core_sim::Position::new(50, 25)),
                to: partner_capital,
                value: 10.0,
                security: 0.8,
            };
            
            civ_data.civilization.economy.trade_routes.push(trade_route);
            civ_data.civilization.economy.income += 5.0;
            
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
        // Simplified attack implementation
        let _relation_key = (civ_id, target);
        
        // TODO: Replace with actual diplomatic_state integration
        /*
        if let Some(relation) = game_state.diplomatic_state.relations.get_mut(&relation_key) {
            relation.relation_value -= 50.0;
            relation.treaties.push(core_sim::Treaty::War { started_turn: game_state.turn });
        }
        */
        
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
        // TODO: Replace with actual diplomatic system integration
        /*
        // Simplified diplomacy implementation
        let proposal = match action {
            core_sim::diplomacy::DiplomaticAction::ProposeTradePact => {
                core_sim::DiplomaticProposal::TradePact
            }
            core_sim::diplomacy::DiplomaticAction::ProposeAlliance => {
                core_sim::DiplomaticProposal::Alliance
            }
            core_sim::diplomacy::DiplomaticAction::ProposeNonAggression => {
                core_sim::DiplomaticProposal::NonAggressionPact
            }
            _ => core_sim::DiplomaticProposal::TradePact,
        };
        
        let negotiation = core_sim::Negotiation {
            initiator: civ_id,
            target,
            proposal,
            turns_remaining: 3,
        };
        
        game_state.diplomatic_state.ongoing_negotiations.push(negotiation);
        */
        
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
            // Move units to defensive position
            for unit in &mut civ_data.civilization.military.units {
                if unit.position.distance_to(&position) < 5.0 {
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

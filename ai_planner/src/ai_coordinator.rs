use crate::{AICoordinator, AIAction};
use core_sim::{
    CivId, GameState, CivilizationData, Position, 
    UnitType, BuildingType, DiplomaticAction,
    GameResource as Resource,
    resources::{DiplomaticProposal, Negotiation}
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
            *cooldown = cooldown.saturating_sub(1);
        }

        let mut decisions = HashMap::new();

        for &civ_id in game_state.civilizations.keys() {
            // Check if this civ can make decisions this turn
            if self.turn_cooldown.get(&civ_id).unwrap_or(&0) > &0 {
                continue;
            }

            // Generate decisions for this civilization
            let civ_decisions = self.coordinator.generate_decisions(game_state);
            if let Some(actions) = civ_decisions.get(&civ_id) {
                if !actions.is_empty() {
                    decisions.insert(civ_id, actions.clone());
                    
                    // Set cooldown based on number of actions taken
                    let cooldown = match actions.len() {
                        0..=1 => 0,
                        2..=3 => 1,
                        _ => 2,
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
        game_state: &mut GameState,
    ) -> ExecutionResult {
        if let Some(tile) = game_state.world_map.get_tile_mut(target_position) {
            if tile.owner.is_none() {
                tile.owner = Some(civ_id);
                
                // Add territory to civilization data
                if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
                    let territory = core_sim::Territory {
                        owner: civ_id,
                        control_strength: 1.0,
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
    }

    fn execute_research(
        &self,
        civ_id: CivId,
        technology: &str,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        if let Some(civ_data) = game_state.civilizations.get_mut(&civ_id) {
            let cost = 50.0; // Base research cost
            
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
            let cost = 30.0; // Base unit cost
            
            if civ_data.civilization.economy.gold >= cost {
                civ_data.civilization.economy.gold -= cost;
                
                let unit = core_sim::MilitaryUnit {
                    id: civ_data.civilization.military.units.len() as u32,
                    unit_type: unit_type.clone(),
                    position,
                    strength: 10.0,
                    movement_remaining: 2,
                    experience: 0.0,
                };
                
                civ_data.civilization.military.units.push(unit);
                civ_data.civilization.military.total_strength += 10.0;
                
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
        position: core_sim::Position,
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
        if let (Some(civ_data), Some(_partner_data)) = (
            game_state.civilizations.get_mut(&civ_id),
            game_state.civilizations.get(&partner),
        ) {
            // Simplified trade implementation
            let trade_route = core_sim::TradeRoute {
                from: civ_data.civilization.capital.unwrap_or(core_sim::Position::new(50, 25)),
                to: game_state.civilizations.get(&partner).unwrap().civilization.capital.unwrap_or(core_sim::Position::new(60, 25)),
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
                reason: "Trade partner not found".to_string(),
            }
        }
    }

    fn execute_attack(
        &self,
        civ_id: CivId,
        target: CivId,
        _target_position: core_sim::Position,
        game_state: &mut GameState,
    ) -> ExecutionResult {
        // Simplified attack implementation
        let relation_key = (civ_id, target);
        
        if let Some(relation) = game_state.diplomatic_state.relations.get_mut(&relation_key) {
            relation.relation_value -= 50.0;
            relation.treaties.push(core_sim::Treaty::War { started_turn: game_state.turn });
        }
        
        ExecutionResult::Success {
            civ_id,
            action_description: format!("Declared war on {:?}", target),
        }
    }

    fn execute_diplomacy(
        &self,
        civ_id: CivId,
        target: CivId,
        action: &DiplomaticAction,
        game_state: &mut GameState,
    ) -> ExecutionResult {
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

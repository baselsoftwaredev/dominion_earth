use core_sim::{CivId, CivilizationData, GameState, Position, CivPersonality};
use crate::{AIAction, StrategicGoal};
use std::collections::HashMap;
use rand::Rng;

/// Utility-based AI for immediate decision making
#[derive(Debug, Clone)]
pub struct UtilityAI {
    utility_functions: Vec<UtilityFunction>,
}

impl UtilityAI {
    pub fn new() -> Self {
        Self {
            utility_functions: Self::create_utility_functions(),
        }
    }

    /// Evaluate all possible actions and return the best ones
    pub fn evaluate_actions(
        &self,
        civ_id: CivId,
        civ_data: &CivilizationData,
        game_state: &GameState,
    ) -> Vec<AIAction> {
        let mut evaluated_actions = Vec::new();

        for utility_function in &self.utility_functions {
            let utility_score = utility_function.evaluate(civ_id, civ_data, game_state);
            
            if utility_score > 0.3 { // Threshold for considering an action
                if let Some(action) = utility_function.create_action(civ_id, civ_data, game_state, utility_score) {
                    evaluated_actions.push(action);
                }
            }
        }

        // Sort by utility score (priority)
        evaluated_actions.sort_by(|a, b| {
            let priority_a = self.get_action_priority(a);
            let priority_b = self.get_action_priority(b);
            priority_b.partial_cmp(&priority_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        evaluated_actions
    }

    fn get_action_priority(&self, action: &AIAction) -> f32 {
        match action {
            AIAction::Expand { priority, .. } => *priority,
            AIAction::Research { priority, .. } => *priority,
            AIAction::BuildUnit { priority, .. } => *priority,
            AIAction::BuildBuilding { priority, .. } => *priority,
            AIAction::Trade { priority, .. } => *priority,
            AIAction::Attack { priority, .. } => *priority,
            AIAction::Diplomacy { priority, .. } => *priority,
            AIAction::Defend { priority, .. } => *priority,
        }
    }

    fn create_utility_functions() -> Vec<UtilityFunction> {
        vec![
            UtilityFunction::new(
                "expand_territory",
                Box::new(|_civ_id, civ_data, game_state| {
                    let personality = &civ_data.civilization.personality;
                    let land_hunger = personality.land_hunger;
                    
                    // Check if there are available adjacent territories
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(50, 25));
                    let available_tiles = game_state.world_map.neighbors(capital)
                        .into_iter()
                        .filter(|pos| {
                            game_state.world_map.get_tile(*pos)
                                .map(|tile| tile.owner.is_none())
                                .unwrap_or(false)
                        })
                        .count();
                    
                    land_hunger * (available_tiles as f32 / 8.0).min(1.0)
                }),
                Box::new(|civ_id, civ_data, game_state, utility| {
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(50, 25));
                    let available_positions: Vec<_> = game_state.world_map.neighbors(capital)
                        .into_iter()
                        .filter(|pos| {
                            game_state.world_map.get_tile(*pos)
                                .map(|tile| tile.owner.is_none() && !matches!(tile.terrain, core_sim::TerrainType::Ocean))
                                .unwrap_or(false)
                        })
                        .collect();
                    
                    if let Some(&target) = available_positions.first() {
                        Some(AIAction::Expand {
                            target_position: target,
                            priority: utility,
                        })
                    } else {
                        None
                    }
                }),
            ),
            
            UtilityFunction::new(
                "research_technology",
                Box::new(|_civ_id, civ_data, _game_state| {
                    let personality = &civ_data.civilization.personality;
                    let tech_focus = personality.tech_focus;
                    let economy = &civ_data.civilization.economy;
                    
                    // Higher utility if we have research capacity
                    let research_capacity = (economy.gold / 100.0).min(1.0);
                    tech_focus * research_capacity
                }),
                Box::new(|_civ_id, civ_data, _game_state, utility| {
                    let technologies = &civ_data.civilization.technologies;
                    let available_techs = ["Agriculture", "Bronze Working", "Writing", "Mathematics", "Iron Working"];
                    
                    for tech in &available_techs {
                        if !technologies.known.get(*tech).unwrap_or(&false) {
                            return Some(AIAction::Research {
                                technology: tech.to_string(),
                                priority: utility,
                            });
                        }
                    }
                    None
                }),
            ),
            
            UtilityFunction::new(
                "build_military",
                Box::new(|_civ_id, civ_data, game_state| {
                    let personality = &civ_data.civilization.personality;
                    let militarism = personality.militarism;
                    let military = &civ_data.civilization.military;
                    
                    // Calculate threat level from nearby civilizations
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(50, 25));
                    let nearby_threat = game_state.civilizations.values()
                        .filter(|other_civ| other_civ.civilization.id != civ_data.civilization.id)
                        .map(|other_civ| {
                            if let Some(other_capital) = other_civ.civilization.capital {
                                let distance = capital.distance_to(&other_capital);
                                if distance < 20.0 {
                                    other_civ.civilization.military.total_strength / (distance + 1.0)
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            }
                        })
                        .sum::<f32>();
                    
                    let threat_factor = (nearby_threat / (military.total_strength + 1.0)).min(2.0);
                    militarism * (0.5 + threat_factor * 0.5)
                }),
                Box::new(|_civ_id, civ_data, _game_state, utility| {
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(50, 25));
                    
                    // Choose unit type based on what we need
                    let unit_type = if civ_data.civilization.military.units.len() < 2 {
                        core_sim::UnitType::Infantry
                    } else {
                        core_sim::UnitType::Archer
                    };
                    
                    Some(AIAction::BuildUnit {
                        unit_type,
                        position: capital,
                        priority: utility,
                    })
                }),
            ),
            
            UtilityFunction::new(
                "develop_economy",
                Box::new(|_civ_id, civ_data, _game_state| {
                    let personality = &civ_data.civilization.personality;
                    let industry_focus = personality.industry_focus;
                    let economy = &civ_data.civilization.economy;
                    
                    // Higher utility if income is low relative to expenses
                    let economic_pressure = if economy.income > 0.0 {
                        (economy.expenses / economy.income).min(2.0)
                    } else {
                        2.0
                    };
                    
                    industry_focus * (0.3 + economic_pressure * 0.7)
                }),
                Box::new(|_civ_id, civ_data, _game_state, utility| {
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(50, 25));
                    
                    // Choose building type based on current needs
                    let building_type = if civ_data.cities.iter().any(|city| {
                        city.buildings.iter().any(|b| matches!(b.building_type, core_sim::BuildingType::Market))
                    }) {
                        core_sim::BuildingType::Workshop
                    } else {
                        core_sim::BuildingType::Market
                    };
                    
                    Some(AIAction::BuildBuilding {
                        building_type,
                        position: capital,
                        priority: utility,
                    })
                }),
            ),
            
            UtilityFunction::new(
                "establish_trade",
                Box::new(|civ_id, civ_data, game_state| {
                    let personality = &civ_data.civilization.personality;
                    let industry_focus = personality.industry_focus;
                    let current_trade_routes = civ_data.civilization.economy.trade_routes.len();
                    
                    // Lower utility if we already have many trade routes
                    let trade_saturation = (current_trade_routes as f32 / 5.0).min(1.0);
                    
                    // Check for potential trade partners
                    let potential_partners = game_state.civilizations.values()
                        .filter(|other_civ| other_civ.civilization.id != *civ_id)
                        .count();
                    
                    if potential_partners > 0 {
                        industry_focus * (1.0 - trade_saturation) * 0.8
                    } else {
                        0.0
                    }
                }),
                Box::new(|civ_id, civ_data, game_state, utility| {
                    // Find the best trade partner
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(50, 25));
                    
                    let mut best_partner = None;
                    let mut best_distance = f32::INFINITY;
                    
                    for other_civ in game_state.civilizations.values() {
                        if other_civ.civilization.id != *civ_id {
                            if let Some(other_capital) = other_civ.civilization.capital {
                                let distance = capital.distance_to(&other_capital);
                                if distance < best_distance && distance < 30.0 {
                                    best_distance = distance;
                                    best_partner = Some(other_civ.civilization.id);
                                }
                            }
                        }
                    }
                    
                    if let Some(partner) = best_partner {
                        Some(AIAction::Trade {
                            partner,
                            resource: core_sim::Resource::Gold, // Simplified for now
                            priority: utility,
                        })
                    } else {
                        None
                    }
                }),
            ),
        ]
    }
}

impl Default for UtilityAI {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual utility function that evaluates a specific action type
#[derive(Clone)]
pub struct UtilityFunction {
    pub name: String,
    pub evaluator: Box<dyn Fn(CivId, &CivilizationData, &GameState) -> f32 + Send + Sync>,
    pub action_creator: Box<dyn Fn(CivId, &CivilizationData, &GameState, f32) -> Option<AIAction> + Send + Sync>,
}

impl std::fmt::Debug for UtilityFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UtilityFunction")
            .field("name", &self.name)
            .finish()
    }
}

impl UtilityFunction {
    pub fn new(
        name: &str,
        evaluator: Box<dyn Fn(CivId, &CivilizationData, &GameState) -> f32 + Send + Sync>,
        action_creator: Box<dyn Fn(CivId, &CivilizationData, &GameState, f32) -> Option<AIAction> + Send + Sync>,
    ) -> Self {
        Self {
            name: name.to_string(),
            evaluator,
            action_creator,
        }
    }

    pub fn evaluate(&self, civ_id: CivId, civ_data: &CivilizationData, game_state: &GameState) -> f32 {
        (self.evaluator)(civ_id, civ_data, game_state)
    }

    pub fn create_action(
        &self,
        civ_id: CivId,
        civ_data: &CivilizationData,
        game_state: &GameState,
        utility_score: f32,
    ) -> Option<AIAction> {
        (self.action_creator)(civ_id, civ_data, game_state, utility_score)
    }
}

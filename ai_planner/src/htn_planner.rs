use core_sim::{CivId, GameState, Position, UnitType, BuildingType, DiplomaticAction, GameResource as Resource};
use crate::constants::htn::{defaults, diplomacy, military, priorities};
use crate::{AIAction, HTNTask};
use std::collections::HashMap;

/// Hierarchical Task Network (HTN) planner for complex multi-turn strategies
#[derive(Debug, Clone)]
pub struct HTNPlanner {
    task_network: HashMap<HTNTask, TaskNetwork>,
}

impl HTNPlanner {
    pub fn new() -> Self {
        Self {
            task_network: Self::create_task_networks(),
        }
    }

    /// Decompose a high-level task into concrete actions
    pub fn decompose_task(
        &self,
        civ_id: CivId,
        task: &HTNTask,
        game_state: &GameState,
    ) -> Option<Vec<AIAction>> {
        let network = self.task_network.get(task)?;
        self.execute_network(network, civ_id, game_state)
    }

    fn execute_network(
        &self,
        network: &TaskNetwork,
        civ_id: CivId,
        game_state: &GameState,
    ) -> Option<Vec<AIAction>> {
        for method in &network.methods {
            if self.method_applicable(method, civ_id, game_state) {
                return self.decompose_method(method, civ_id, game_state);
            }
        }
        None
    }

    fn method_applicable(&self, method: &HTNMethod, civ_id: CivId, game_state: &GameState) -> bool {
        let Some(civ_data) = game_state.civilizations.get(&civ_id) else {
            return false;
        };
        
        for condition in &method.preconditions {
            if !self.evaluate_condition(condition, civ_data, game_state) {
                return false;
            }
        }
        true
    }

    fn evaluate_condition(
        &self,
        condition: &TaskCondition,
        civ_data: &core_sim::CivilizationData,
        game_state: &GameState,
    ) -> bool {
        match condition {
            TaskCondition::HasGold(amount) => civ_data.civilization.economy.gold >= *amount,
            TaskCondition::HasMilitaryStrength(strength) => {
                civ_data.civilization.military.total_strength >= *strength
            }
            TaskCondition::HasCities(count) => civ_data.cities.len() >= *count,
            TaskCondition::HasTechnology(tech) => {
                civ_data.civilization.technologies.known.get(tech).unwrap_or(&false) == &true
            }
            TaskCondition::HasEnemies => {
                // TODO: Replace with actual diplomatic_state integration
                // Check if there are hostile civilizations
                false // Stub value
            }
            TaskCondition::HasAllies => {
                // TODO: Replace with actual diplomatic_state integration
                // Check if there are friendly civilizations
                false // Stub value
            }
            TaskCondition::TurnGreaterThan(turn) => game_state.turn > *turn,
        }
    }

    fn decompose_method(
        &self,
        method: &HTNMethod,
        civ_id: CivId,
        game_state: &GameState,
    ) -> Option<Vec<AIAction>> {
        let mut actions = Vec::new();

        for subtask in &method.subtasks {
            match subtask {
                HTNSubtask::PrimitiveAction(action_type) => {
                    if let Some(action) = self.create_primitive_action(action_type, civ_id, game_state) {
                        actions.push(action);
                    }
                }
                HTNSubtask::CompoundTask(task) => {
                    if let Some(mut subtask_actions) = self.decompose_task(civ_id, task, game_state) {
                        actions.append(&mut subtask_actions);
                    }
                }
            }
        }

        if actions.is_empty() {
            None
        } else {
            Some(actions)
        }
    }

    fn create_primitive_action(
        &self,
        action_type: &PrimitiveActionType,
        civ_id: CivId,
        game_state: &GameState,
    ) -> Option<AIAction> {
        let civ_data = game_state.civilizations.get(&civ_id)?;
        let capital = civ_data.civilization.capital.unwrap_or(Position::new(defaults::DEFAULT_CAPITAL_X, defaults::DEFAULT_CAPITAL_Y));

        match action_type {
            PrimitiveActionType::BuildArmy => {
                Some(AIAction::BuildUnit {
                    unit_type: UnitType::Infantry,
                    position: capital,
                    priority: priorities::ESTABLISH_CITY_PRIORITY,
                })
            }
            PrimitiveActionType::ExpandTerritory => {
                // TODO: Replace with actual world_map integration
                // Find expansion target - using stub implementation
                Some(AIAction::Expand {
                    target_position: Position::new(capital.x + defaults::EXPANSION_X_OFFSET, capital.y),
                    priority: priorities::BUILD_UNIT_PRIORITY,
                })
            }
            PrimitiveActionType::ResearchTechnology => {
                Some(AIAction::Research {
                    technology: "Iron Working".to_string(),
                    priority: priorities::RESEARCH_TECH_PRIORITY,
                })
            }
            PrimitiveActionType::EstablishTrade => {
                // Find trade partner
                for other_civ in game_state.civilizations.values() {
                    if other_civ.civilization.id != civ_id {
                        return Some(AIAction::Trade {
                            partner: other_civ.civilization.id,
                            resource: Resource::Gold,
                            priority: 0.5,
                        });
                    }
                }
                None
            }
            PrimitiveActionType::BuildInfrastructure => {
                Some(AIAction::BuildBuilding {
                    building_type: BuildingType::Workshop,
                    position: capital,
                    priority: 0.6,
                })
            }
            PrimitiveActionType::FormAlliance => {
                // Find potential ally
                let best_candidate = None;
                let _best_relation = diplomacy::INITIAL_WORST_RELATION;

                // TODO: Replace with actual diplomatic_state integration
                /*
                for relation in game_state.diplomatic_state.relations.values() {
                    let other_civ = if relation.civ_a == civ_id {
                        Some(relation.civ_b)
                    } else if relation.civ_b == civ_id {
                        Some(relation.civ_a)
                    } else {
                        None
                    };

                    if let Some(other) = other_civ {
                        if relation.relation_value > best_relation && relation.relation_value > diplomacy::ALLIANCE_THRESHOLD {
                            best_relation = relation.relation_value;
                            best_candidate = Some(other);
                        }
                    }
                }
                */

                if let Some(ally) = best_candidate {
                    Some(AIAction::Diplomacy {
                        target: ally,
                        action: DiplomaticAction::ProposeAlliance,
                        priority: priorities::DIPLOMACY_PRIORITY,
                    })
                } else {
                    None
                }
            }
            PrimitiveActionType::DeclareWar => {
                // Find war target
                let mut weakest_enemy = None;
                let mut weakest_strength = f32::INFINITY;

                for other_civ in game_state.civilizations.values() {
                    if other_civ.civilization.id != civ_id {
                        let military_strength = other_civ.civilization.military.total_strength;
                        if military_strength < weakest_strength && military_strength < civ_data.civilization.military.total_strength * military::STRENGTH_WEAKNESS_THRESHOLD {
                            weakest_strength = military_strength;
                            weakest_enemy = Some(other_civ.civilization.id);
                        }
                    }
                }

                if let Some(target) = weakest_enemy {
                    if let Some(target_capital) = game_state.civilizations.get(&target)?.civilization.capital {
                        Some(AIAction::Attack {
                            target,
                            target_position: target_capital,
                            priority: 0.9,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            PrimitiveActionType::DefendTerritory => {
                Some(AIAction::Defend {
                    position: capital,
                    priority: 1.0,
                })
            }
        }
    }

    fn create_task_networks() -> HashMap<HTNTask, TaskNetwork> {
        let mut networks = HashMap::new();

        // Conquest Campaign
        networks.insert(
            HTNTask::ConquestCampaign,
            TaskNetwork {
                methods: vec![
                    HTNMethod {
                        name: "aggressive_conquest".to_string(),
                        preconditions: vec![
                            TaskCondition::HasMilitaryStrength(50.0),
                            TaskCondition::HasGold(100.0),
                        ],
                        subtasks: vec![
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::BuildArmy),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::ResearchTechnology),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::DeclareWar),
                        ],
                    },
                    HTNMethod {
                        name: "preparation_phase".to_string(),
                        preconditions: vec![
                            TaskCondition::HasCities(1),
                        ],
                        subtasks: vec![
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::BuildArmy),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::BuildInfrastructure),
                            HTNSubtask::CompoundTask(HTNTask::EconomicDevelopment),
                        ],
                    },
                ],
            },
        );

        // Diplomatic Campaign
        networks.insert(
            HTNTask::DiplomaticCampaign,
            TaskNetwork {
                methods: vec![
                    HTNMethod {
                        name: "alliance_building".to_string(),
                        preconditions: vec![
                            TaskCondition::TurnGreaterThan(10),
                        ],
                        subtasks: vec![
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::EstablishTrade),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::FormAlliance),
                        ],
                    },
                ],
            },
        );

        // Economic Development
        networks.insert(
            HTNTask::EconomicDevelopment,
            TaskNetwork {
                methods: vec![
                    HTNMethod {
                        name: "infrastructure_focus".to_string(),
                        preconditions: vec![
                            TaskCondition::HasCities(1),
                        ],
                        subtasks: vec![
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::BuildInfrastructure),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::EstablishTrade),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::ExpandTerritory),
                        ],
                    },
                ],
            },
        );

        // Technological Advancement
        networks.insert(
            HTNTask::TechnologicalAdvancement,
            TaskNetwork {
                methods: vec![
                    HTNMethod {
                        name: "research_focus".to_string(),
                        preconditions: vec![
                            TaskCondition::HasGold(50.0),
                        ],
                        subtasks: vec![
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::ResearchTechnology),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::BuildInfrastructure),
                        ],
                    },
                ],
            },
        );

        // Defensive Preparation
        networks.insert(
            HTNTask::DefensivePreparation,
            TaskNetwork {
                methods: vec![
                    HTNMethod {
                        name: "defensive_buildup".to_string(),
                        preconditions: vec![
                            TaskCondition::HasEnemies,
                        ],
                        subtasks: vec![
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::BuildArmy),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::DefendTerritory),
                            HTNSubtask::PrimitiveAction(PrimitiveActionType::FormAlliance),
                        ],
                    },
                ],
            },
        );

        networks
    }
}

impl Default for HTNPlanner {
    fn default() -> Self {
        Self::new()
    }
}

/// HTN task network definition
#[derive(Debug, Clone)]
pub struct TaskNetwork {
    pub methods: Vec<HTNMethod>,
}

/// HTN method (way to accomplish a task)
#[derive(Debug, Clone)]
pub struct HTNMethod {
    pub name: String,
    pub preconditions: Vec<TaskCondition>,
    pub subtasks: Vec<HTNSubtask>,
}

/// HTN subtask types
#[derive(Debug, Clone)]
pub enum HTNSubtask {
    PrimitiveAction(PrimitiveActionType),
    CompoundTask(HTNTask),
}

/// Primitive action types that can be directly executed
#[derive(Debug, Clone)]
pub enum PrimitiveActionType {
    BuildArmy,
    ExpandTerritory,
    ResearchTechnology,
    EstablishTrade,
    BuildInfrastructure,
    FormAlliance,
    DeclareWar,
    DefendTerritory,
}

/// Task preconditions
#[derive(Debug, Clone)]
pub enum TaskCondition {
    HasGold(f32),
    HasMilitaryStrength(f32),
    HasCities(usize),
    HasTechnology(String),
    HasEnemies,
    HasAllies,
    TurnGreaterThan(u32),
}

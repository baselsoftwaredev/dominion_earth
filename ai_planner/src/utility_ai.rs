use crate::constants::utility::{defaults, economy, expansion, exploration, military, thresholds};
use crate::AIAction;
use core_sim::{
    BuildingType, CivId, CivilizationData, GameResource as Resource, GameState, Position, UnitType,
};

/// Utility-based AI for immediate decision making
#[derive(Debug)]
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

            if utility_score > thresholds::ACTION_CONSIDERATION_THRESHOLD {
                // Threshold for considering an action
                if let Some(action) =
                    utility_function.create_action(civ_id, civ_data, game_state, utility_score)
                {
                    evaluated_actions.push(action);
                }
            }
        }

        // Sort by utility score (priority)
        evaluated_actions.sort_by(|a, b| {
            let priority_a = self.get_action_priority(a);
            let priority_b = self.get_action_priority(b);
            priority_b
                .partial_cmp(&priority_a)
                .unwrap_or(std::cmp::Ordering::Equal)
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
            AIAction::Explore { priority, .. } => *priority,
        }
    }

    fn create_utility_functions() -> Vec<UtilityFunction> {
        vec![
            UtilityFunction::new(
                "expand_territory",
                Box::new(|_civ_id, civ_data, _game_state| {
                    let personality = &civ_data.civilization.personality;
                    let land_hunger = personality.land_hunger;

                    let _capital = civ_data.civilization.capital.unwrap_or(Position::new(
                        defaults::DEFAULT_CAPITAL_X,
                        defaults::DEFAULT_CAPITAL_Y,
                    ));
                    let available_tiles = expansion::AVAILABLE_TILES_STUB;

                    land_hunger
                        * (available_tiles as f32 / expansion::MAX_EXPANSION_FACTOR)
                            .min(thresholds::MAX_UTILITY_SCORE)
                }),
                Box::new(|_civ_id, civ_data, _game_state, utility| {
                    let _capital = civ_data.civilization.capital.unwrap_or(Position::new(
                        defaults::DEFAULT_CAPITAL_X,
                        defaults::DEFAULT_CAPITAL_Y,
                    ));
                    let available_positions = vec![Position::new(
                        defaults::DEFAULT_CAPITAL_X,
                        defaults::DEFAULT_CAPITAL_Y + defaults::DEFAULT_EXPANSION_Y_OFFSET,
                    )];

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

                    let research_capacity = (economy.gold / economy::GOLD_TO_RESEARCH_DIVISOR)
                        .min(thresholds::MAX_UTILITY_SCORE);
                    tech_focus * research_capacity
                }),
                Box::new(|_civ_id, civ_data, _game_state, utility| {
                    let technologies = &civ_data.civilization.technologies;
                    let available_techs = [
                        "Agriculture",
                        "Bronze Working",
                        "Writing",
                        "Mathematics",
                        "Iron Working",
                    ];

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

                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(
                        defaults::DEFAULT_CAPITAL_X,
                        defaults::DEFAULT_CAPITAL_Y,
                    ));
                    let nearby_threat = game_state
                        .civilizations
                        .values()
                        .filter(|other_civ| other_civ.civilization.id != civ_data.civilization.id)
                        .map(|other_civ| {
                            if let Some(other_capital) = other_civ.civilization.capital {
                                let distance = capital.distance_to(&other_capital);
                                if distance < expansion::PROXIMITY_THRESHOLD {
                                    other_civ.civilization.military.total_strength
                                        / (distance + 1.0)
                                } else {
                                    thresholds::MIN_UTILITY_SCORE
                                }
                            } else {
                                thresholds::MIN_UTILITY_SCORE
                            }
                        })
                        .sum::<f32>();

                    let threat_factor = (nearby_threat
                        / (military.total_strength + military::THREAT_FACTOR_DEFENSE_OFFSET))
                        .min(military::THREAT_FACTOR_MAX);
                    militarism
                        * (military::BASE_MILITARISM_WEIGHT
                            + threat_factor * military::BASE_MILITARISM_WEIGHT)
                }),
                Box::new(|_civ_id, civ_data, _game_state, utility| {
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(
                        defaults::DEFAULT_CAPITAL_X,
                        defaults::DEFAULT_CAPITAL_Y,
                    ));

                    let unit_type = if civ_data.civilization.military.units.len()
                        < military::INITIAL_UNIT_COUNT_THRESHOLD
                    {
                        UnitType::Infantry
                    } else {
                        UnitType::Archer
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

                    let economic_pressure = if economy.income > thresholds::MIN_UTILITY_SCORE {
                        (economy.expenses / economy.income).min(military::ECONOMIC_PRESSURE_MAX)
                    } else {
                        military::ECONOMIC_PRESSURE_MAX
                    };

                    industry_focus
                        * (military::ECONOMIC_PRESSURE_BASE_WEIGHT
                            + economic_pressure * military::ECONOMIC_PRESSURE_VARIABLE_WEIGHT)
                }),
                Box::new(|_civ_id, civ_data, _game_state, utility| {
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(
                        defaults::DEFAULT_CAPITAL_X,
                        defaults::DEFAULT_CAPITAL_Y,
                    ));

                    let building_type = if civ_data.cities.iter().any(|city| {
                        city.buildings
                            .iter()
                            .any(|b| matches!(b.building_type, BuildingType::Market))
                    }) {
                        BuildingType::Workshop
                    } else {
                        BuildingType::Market
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

                    let trade_saturation = (current_trade_routes as f32
                        / military::TRADE_ROUTE_SATURATION_DIVISOR)
                        .min(thresholds::MAX_UTILITY_SCORE);

                    let potential_partners = game_state
                        .civilizations
                        .values()
                        .filter(|other_civ| other_civ.civilization.id != civ_id)
                        .count();

                    if potential_partners > military::MINIMUM_POTENTIAL_PARTNERS {
                        industry_focus
                            * (thresholds::MAX_UTILITY_SCORE - trade_saturation)
                            * military::TRADE_UTILITY_MULTIPLIER
                    } else {
                        thresholds::MIN_UTILITY_SCORE
                    }
                }),
                Box::new(|civ_id, civ_data, game_state, utility| {
                    let capital = civ_data.civilization.capital.unwrap_or(Position::new(
                        defaults::DEFAULT_CAPITAL_X,
                        defaults::DEFAULT_CAPITAL_Y,
                    ));

                    let mut best_partner = None;
                    let mut best_distance = f32::INFINITY;

                    for other_civ in game_state.civilizations.values() {
                        if other_civ.civilization.id != civ_id {
                            if let Some(other_capital) = other_civ.civilization.capital {
                                let distance = capital.distance_to(&other_capital);
                                if distance < best_distance
                                    && distance < military::MAX_TRADE_DISTANCE
                                {
                                    best_distance = distance;
                                    best_partner = Some(other_civ.civilization.id);
                                }
                            }
                        }
                    }

                    if let Some(partner) = best_partner {
                        Some(AIAction::Trade {
                            partner,
                            resource: Resource::Gold,
                            priority: utility,
                        })
                    } else {
                        None
                    }
                }),
            ),
            UtilityFunction::new(
                "explore_territory",
                Box::new(|_civ_id, civ_data, _game_state| {
                    let personality = &civ_data.civilization.personality;
                    let exploration_drive = personality.exploration_drive;

                    let turn_factor = if _game_state.turn < exploration::EARLY_GAME_TURN_THRESHOLD {
                        exploration::EARLY_GAME_EXPLORATION_MULTIPLIER
                    } else if _game_state.turn < exploration::MID_GAME_TURN_THRESHOLD {
                        exploration::MID_GAME_EXPLORATION_MULTIPLIER
                    } else {
                        exploration::LATE_GAME_EXPLORATION_MULTIPLIER
                    };

                    let territory_factor =
                        if civ_data.territories.len() < exploration::FEW_TERRITORIES_THRESHOLD {
                            exploration::FEW_TERRITORIES_MULTIPLIER
                        } else if civ_data.territories.len()
                            < exploration::MODERATE_TERRITORIES_THRESHOLD
                        {
                            exploration::MODERATE_TERRITORIES_MULTIPLIER
                        } else {
                            exploration::MANY_TERRITORIES_MULTIPLIER
                        };

                    exploration_drive * turn_factor * territory_factor
                }),
                Box::new(|_civ_id, civ_data, _game_state, utility| {
                    let exploration_base = if let Some(capital) = civ_data.civilization.capital {
                        capital
                    } else if let Some((pos, _)) = civ_data.territories.first() {
                        *pos
                    } else {
                        return None;
                    };

                    let directions = [
                        (exploration::EXPLORATION_DISTANCE_NEAR, 0),
                        (-exploration::EXPLORATION_DISTANCE_NEAR, 0),
                        (0, exploration::EXPLORATION_DISTANCE_NEAR),
                        (0, -exploration::EXPLORATION_DISTANCE_NEAR),
                        (
                            exploration::EXPLORATION_DISTANCE_DIAGONAL,
                            exploration::EXPLORATION_DISTANCE_DIAGONAL,
                        ),
                        (
                            -exploration::EXPLORATION_DISTANCE_DIAGONAL,
                            exploration::EXPLORATION_DISTANCE_DIAGONAL,
                        ),
                        (
                            exploration::EXPLORATION_DISTANCE_DIAGONAL,
                            -exploration::EXPLORATION_DISTANCE_DIAGONAL,
                        ),
                        (
                            -exploration::EXPLORATION_DISTANCE_DIAGONAL,
                            -exploration::EXPLORATION_DISTANCE_DIAGONAL,
                        ),
                    ];

                    let direction_idx = (_game_state.turn as usize) % directions.len();
                    let (dx, dy) = directions[direction_idx];

                    let target = Position::new(exploration_base.x + dx, exploration_base.y + dy);

                    Some(AIAction::Explore {
                        target_position: target,
                        priority: utility,
                    })
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
pub struct UtilityFunction {
    pub name: String,
    pub evaluator: Box<dyn Fn(CivId, &CivilizationData, &GameState) -> f32 + Send + Sync>,
    pub action_creator:
        Box<dyn Fn(CivId, &CivilizationData, &GameState, f32) -> Option<AIAction> + Send + Sync>,
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
        action_creator: Box<
            dyn Fn(CivId, &CivilizationData, &GameState, f32) -> Option<AIAction> + Send + Sync,
        >,
    ) -> Self {
        Self {
            name: name.to_string(),
            evaluator,
            action_creator,
        }
    }

    pub fn evaluate(
        &self,
        civ_id: CivId,
        civ_data: &CivilizationData,
        game_state: &GameState,
    ) -> f32 {
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

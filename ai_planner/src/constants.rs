// Constants for the AI Planner module
// This centralizes configuration values to eliminate magic numbers

/// Utility AI scoring and evaluation constants
pub mod utility {
    /// Thresholds for action consideration and evaluation
    pub mod thresholds {
        pub const ACTION_CONSIDERATION_THRESHOLD: f32 = 0.3; // Minimum utility score to consider an action
        pub const MIN_UTILITY_SCORE: f32 = 0.0;
        pub const MAX_UTILITY_SCORE: f32 = 1.0;
    }

    /// Expansion and territory evaluation constants
    pub mod expansion {
        pub const AVAILABLE_TILES_STUB: i32 = 4; // Placeholder for available expansion tiles
        pub const MAX_EXPANSION_FACTOR: f32 = 8.0; // Divisor for land hunger calculation
        pub const PROXIMITY_THRESHOLD: f32 = 20.0; // Distance threshold for nearby civilizations
    }

    /// Economic evaluation constants
    pub mod economy {
        pub const GOLD_TO_RESEARCH_DIVISOR: f32 = 100.0;
    }

    /// Military evaluation constants
    pub mod military {
        pub const THREAT_FACTOR_MAX: f32 = 2.0;
        pub const THREAT_FACTOR_DEFENSE_OFFSET: f32 = 1.0;
        pub const BASE_MILITARISM_WEIGHT: f32 = 0.5;
        pub const INITIAL_UNIT_COUNT_THRESHOLD: usize = 2;
        pub const TRADE_ROUTE_SATURATION_DIVISOR: f32 = 5.0;
        pub const MAX_TRADE_DISTANCE: f32 = 30.0;
        pub const ECONOMIC_PRESSURE_MAX: f32 = 2.0;
        pub const ECONOMIC_PRESSURE_BASE_WEIGHT: f32 = 0.3;
        pub const ECONOMIC_PRESSURE_VARIABLE_WEIGHT: f32 = 0.7;
        pub const TRADE_UTILITY_MULTIPLIER: f32 = 0.8;
        pub const MINIMUM_POTENTIAL_PARTNERS: usize = 0;
    }

    /// Default positions and fallback values
    pub mod defaults {
        pub const DEFAULT_CAPITAL_X: i32 = 50;
        pub const DEFAULT_CAPITAL_Y: i32 = 25;
        pub const DEFAULT_EXPANSION_X_OFFSET: i32 = 0;
        pub const DEFAULT_EXPANSION_Y_OFFSET: i32 = 1;
    }

    /// Exploration behavior constants
    pub mod exploration {
        pub const EARLY_GAME_TURN_THRESHOLD: u32 = 20;
        pub const MID_GAME_TURN_THRESHOLD: u32 = 50;
        pub const EARLY_GAME_EXPLORATION_MULTIPLIER: f32 = 1.5;
        pub const MID_GAME_EXPLORATION_MULTIPLIER: f32 = 1.0;
        pub const LATE_GAME_EXPLORATION_MULTIPLIER: f32 = 0.5;
        pub const FEW_TERRITORIES_THRESHOLD: usize = 3;
        pub const MODERATE_TERRITORIES_THRESHOLD: usize = 6;
        pub const FEW_TERRITORIES_MULTIPLIER: f32 = 1.2;
        pub const MODERATE_TERRITORIES_MULTIPLIER: f32 = 1.0;
        pub const MANY_TERRITORIES_MULTIPLIER: f32 = 0.7;
        pub const EXPLORATION_DISTANCE_NEAR: i32 = 5;
        pub const EXPLORATION_DISTANCE_DIAGONAL: i32 = 3;
    }
}

/// GOAP (Goal-Oriented Action Planning) constants
pub mod goap {
    /// Planning and search parameters
    pub mod planning {
        pub const MAX_PLANNING_DEPTH: usize = 10; // Maximum depth for planning search
    }

    /// Goal state configuration values
    pub mod goals {
        pub const TERRITORY_EXPANSION_TARGET: f32 = 3.0;
        pub const TECHNOLOGY_ADVANCEMENT_TARGET: f32 = 2.0;
        pub const INCOME_MULTIPLIER: f32 = 1.5;
        pub const TRADE_ROUTES_TARGET: f32 = 2.0;
        pub const EXPLORATION_TARGET: f32 = 10.0;
    }

    /// GOAP action costs and effects
    pub mod actions {
        pub const EXPAND_ACTION_COST: f32 = 2.0;
        pub const EXPAND_GOLD_REQUIREMENT: f32 = 10.0;
        pub const EXPAND_TERRITORY_EFFECT: f32 = 1.0;

        pub const RESEARCH_ACTION_COST: f32 = 3.0;
        pub const RESEARCH_GOLD_REQUIREMENT: f32 = 50.0;
        pub const RESEARCH_TECH_LEVEL_EFFECT: f32 = 1.0;

        pub const BUILD_MILITARY_ACTION_COST: f32 = 2.5;
        pub const BUILD_MILITARY_GOLD_REQUIREMENT: f32 = 30.0;
        pub const BUILD_MILITARY_CITY_REQUIREMENT: f32 = 1.0;
        pub const BUILD_MILITARY_STRENGTH_EFFECT: f32 = 10.0;

        pub const TRADE_ACTION_COST: f32 = 1.5;
        pub const TRADE_CITY_REQUIREMENT: f32 = 1.0;
        pub const TRADE_ROUTE_EFFECT: f32 = 1.0;
        pub const TRADE_INCOME_EFFECT: f32 = 5.0;

        pub const BUILD_ECONOMIC_ACTION_COST: f32 = 2.0;
        pub const BUILD_ECONOMIC_GOLD_REQUIREMENT: f32 = 25.0;
        pub const BUILD_ECONOMIC_CITY_REQUIREMENT: f32 = 1.0;
        pub const BUILD_ECONOMIC_INCOME_EFFECT: f32 = 3.0;

        pub const EXPLORE_ACTION_COST: f32 = 1.0;
        pub const EXPLORE_CAPITAL_REQUIREMENT: f32 = 1.0;
        pub const EXPLORE_TILES_EFFECT: f32 = 5.0;
    }

    /// Default state values
    pub mod defaults {
        pub const DEFAULT_STATE_VALUE: f32 = 0.0; // Default value for uninitialized state variables
    }
}

/// HTN (Hierarchical Task Network) planning constants
pub mod htn {
    /// Action priorities for different strategic decisions
    pub mod priorities {
        pub const ESTABLISH_CITY_PRIORITY: f32 = 0.8;
        pub const BUILD_UNIT_PRIORITY: f32 = 0.7;
        pub const RESEARCH_TECH_PRIORITY: f32 = 0.6;
        pub const EXPLORE_PRIORITY: f32 = 0.5;
        pub const DIPLOMACY_PRIORITY: f32 = 0.7;
        pub const MILITARY_ACTION_PRIORITY: f32 = 0.6;
    }

    /// Diplomatic relations and thresholds
    pub mod diplomacy {
        pub const INITIAL_WORST_RELATION: f32 = -100.0; // Starting value for finding worst relations
        pub const ALLIANCE_THRESHOLD: f32 = 20.0; // Minimum relation value for alliance consideration
    }

    /// Military strength evaluation
    pub mod military {
        pub const STRENGTH_WEAKNESS_THRESHOLD: f32 = 0.7; // Percentage threshold for considering weakness
    }

    /// Default positions
    pub mod defaults {
        pub const DEFAULT_CAPITAL_X: i32 = 50;
        pub const DEFAULT_CAPITAL_Y: i32 = 25;
        pub const EXPANSION_X_OFFSET: i32 = 1;
        pub const EXPANSION_Y_OFFSET: i32 = 0;
    }
}

/// AI Coordinator system constants
pub mod coordinator {
    /// Turn-based cooldown management
    pub mod cooldowns {
        pub const COOLDOWN_DECREMENT: u32 = 1; // Amount to decrease cooldown each turn
        pub const NO_COOLDOWN: u32 = 0; // No cooldown value

        // Cooldown durations based on number of actions
        pub const MIN_ACTIONS_NO_COOLDOWN: usize = 0;
        pub const MAX_ACTIONS_NO_COOLDOWN: usize = 1;
        pub const MIN_ACTIONS_SHORT_COOLDOWN: usize = 2;
        pub const MAX_ACTIONS_SHORT_COOLDOWN: usize = 3;
        pub const SHORT_COOLDOWN_DURATION: u32 = 1;
        pub const LONG_COOLDOWN_DURATION: u32 = 2;
    }

    /// Economic costs for various actions
    pub mod costs {
        pub const BASE_RESEARCH_COST: f32 = 50.0;
        pub const BASE_UNIT_COST: f32 = 30.0;
        pub const BASE_BUILDING_COST: f32 = 25.0;
    }

    /// Military unit default statistics
    pub mod military {
        pub const DEFAULT_UNIT_STRENGTH: f32 = 10.0;
        pub const DEFAULT_UNIT_MOVEMENT: u32 = 2;
        pub const DEFAULT_UNIT_EXPERIENCE: f32 = 0.0;
    }

    /// Territory control values
    pub mod territory {
        pub const DEFAULT_CONTROL_STRENGTH: f32 = 1.0;
    }

    /// Trade and economy defaults
    pub mod trade {
        pub const DEFAULT_TRADE_ROUTE_VALUE: f32 = 10.0;
        pub const DEFAULT_TRADE_ROUTE_SECURITY: f32 = 0.8;
        pub const TRADE_INCOME_BONUS: f32 = 5.0;
    }

    /// Diplomacy impact values
    pub mod diplomacy {
        pub const WAR_DECLARATION_RELATION_PENALTY: f32 = 50.0;
        pub const NEGOTIATION_TURNS_REMAINING: u32 = 3;
    }

    /// Defense and positioning
    pub mod defense {
        pub const DEFENSIVE_POSITIONING_DISTANCE: f32 = 5.0;
    }

    /// AI decision-making thresholds and weights
    pub mod decision {
        pub const PERSONALITY_THRESHOLD_MODERATE: f32 = 0.5;
        pub const PERSONALITY_THRESHOLD_HIGH: f32 = 0.7;
        pub const EXPLORATION_PERSONALITY_THRESHOLD: f32 = 0.4;
        pub const EARLY_GAME_EXPLORATION_TURN_LIMIT: u32 = 30;
        pub const MAX_DECISIONS_PER_TURN: usize = 8;

        pub const PRIORITY_WEIGHT_EXPAND: f32 = 1.3;
        pub const PRIORITY_WEIGHT_RESEARCH: f32 = 1.2;
        pub const PRIORITY_WEIGHT_BUILD_UNIT: f32 = 1.1;
        pub const PRIORITY_WEIGHT_BUILD_BUILDING: f32 = 1.0;
        pub const PRIORITY_WEIGHT_TRADE: f32 = 0.9;
        pub const PRIORITY_WEIGHT_ATTACK: f32 = 1.4;
        pub const PRIORITY_WEIGHT_DIPLOMACY: f32 = 0.8;
        pub const PRIORITY_BASE_DEFEND: f32 = 1.5;
        pub const PRIORITY_WEIGHT_EXPLORE: f32 = 1.15;
    }

    pub mod htn {
        pub const LAND_HUNGER_CONQUEST_THRESHOLD: f32 = 0.7;
        pub const INDUSTRY_FOCUS_ECONOMY_THRESHOLD: f32 = 0.7;
    }
}

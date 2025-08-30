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
        pub const GOLD_TO_RESEARCH_DIVISOR: f32 = 100.0; // Gold amount needed per research capacity
    }

    /// Default positions and fallback values
    pub mod defaults {
        pub const DEFAULT_CAPITAL_X: i32 = 50;
        pub const DEFAULT_CAPITAL_Y: i32 = 25;
        pub const DEFAULT_EXPANSION_X_OFFSET: i32 = 0;
        pub const DEFAULT_EXPANSION_Y_OFFSET: i32 = 1;
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
        pub const TERRITORY_EXPANSION_TARGET: f32 = 3.0; // Additional territories to acquire
        pub const TECHNOLOGY_ADVANCEMENT_TARGET: f32 = 2.0; // Technology levels to advance
        pub const INCOME_MULTIPLIER: f32 = 1.5; // Income improvement target multiplier
        pub const TRADE_ROUTES_TARGET: f32 = 2.0; // Additional trade routes to establish
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
        pub const BASE_RESEARCH_COST: f32 = 50.0; // Base cost for research actions
        pub const BASE_UNIT_COST: f32 = 30.0; // Base cost for unit creation
    }

    /// Military unit default statistics
    pub mod military {
        pub const DEFAULT_UNIT_STRENGTH: f32 = 10.0;
        pub const DEFAULT_UNIT_MOVEMENT: u32 = 2;
        pub const DEFAULT_UNIT_EXPERIENCE: f32 = 0.0;
    }

    /// Territory control values
    pub mod territory {
        pub const DEFAULT_CONTROL_STRENGTH: f32 = 1.0; // Base control strength for territories
    }
}

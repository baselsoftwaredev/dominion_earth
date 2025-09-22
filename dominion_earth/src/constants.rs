// Constants for the Dominion Earth frontend module
// This centralizes configuration values to eliminate magic numbers

/// Rendering and visual constants
pub mod rendering {
    /// Tile and grid size constants
    pub mod tile_size {
        pub const TILE_WIDTH: f32 = 64.0;
        pub const TILE_HEIGHT: f32 = 64.0;
        pub const GRID_WIDTH: f32 = 64.0;
        pub const GRID_HEIGHT: f32 = 64.0;
    }

    /// Z-layer offsets for different entity types
    pub mod z_layers {
        pub const TERRAIN_Z: f32 = 0.0;
        pub const CAPITAL_Z: f32 = 10.0; // Capitals render above terrain but below units
        pub const UNIT_Z: f32 = 15.0; // Units render above capitals
    }

    /// Transform constants
    pub mod transform {
        pub const DEFAULT_X: f32 = 0.0;
        pub const DEFAULT_Y: f32 = 0.0;
        pub const DEFAULT_Z: f32 = 0.0;
    }

    /// Camera positioning
    pub mod camera {
        pub const INITIAL_CAMERA_X: f32 = 1600.0;
        pub const INITIAL_CAMERA_Y: f32 = 800.0;
        pub const INITIAL_CAMERA_Z: f32 = 0.0;
    }

    /// Animation constants
    pub mod animation {
        pub const ANCIENT_CAPITAL_START_FRAME: u32 = 3;
        pub const ANCIENT_CAPITAL_END_FRAME: u32 = 7;
        pub const ANCIENT_CAPITAL_ANIMATION_SPEED: f32 = 0.5;
        pub const ANIMATION_TIMER_RESET_VALUE: f32 = 0.0;
    }

    /// Border rendering constants
    pub mod borders {
        pub const UNIT_BORDER_HALF_WIDTH_FACTOR: f32 = 0.45;
        pub const UNIT_BORDER_HALF_HEIGHT_FACTOR: f32 = 0.45;
        pub const CAPITAL_OUTER_BORDER_HALF_WIDTH_FACTOR: f32 = 0.5;
        pub const CAPITAL_OUTER_BORDER_HALF_HEIGHT_FACTOR: f32 = 0.5;
        pub const CAPITAL_INNER_BORDER_HALF_WIDTH_FACTOR: f32 = 0.4;
        pub const CAPITAL_INNER_BORDER_HALF_HEIGHT_FACTOR: f32 = 0.4;
    }
}

/// Input and camera control constants
pub mod input {
    /// Camera movement and control
    pub mod camera {
        pub const MOVEMENT_SPEED: f32 = 200.0;
        pub const ZOOM_RATE: f32 = 1.0;
    }

    /// Tile coordinate conversion
    pub mod coordinates {
        pub const TILE_SIZE_FOR_INPUT: f32 = 64.0;
    }

    /// Simulation speed controls
    pub mod simulation {
        pub const SPEED_MULTIPLIER: f32 = 1.5;
        pub const MAX_SPEED: f32 = 5.0;
        pub const MIN_SPEED: f32 = 0.2;
        pub const BASE_TURN_DURATION: f32 = 1.0; // seconds - halved for faster AI turns
    }
}

/// Game initialization and setup constants
pub mod game {
    /// Map generation parameters
    pub mod map {
        pub const DEFAULT_WIDTH: u32 = 50;
        pub const DEFAULT_HEIGHT: u32 = 25;
    }

    /// Simulation timing
    pub mod timing {
        pub const DEFAULT_SIMULATION_SPEED: f32 = 1.0;
        pub const BASE_TURN_TIMER_SECONDS: f32 = 1.0; // seconds - halved for faster AI turns
    }

    /// Civilization spawning
    pub mod civilizations {
        pub const MAX_STARTING_CIVS: usize = 20;
        pub const INITIAL_SPAWNED_COUNT: usize = 0;
    }

    /// Personality trait ranges for AI
    pub mod personality {
        pub const TRAIT_MIN: f32 = 0.2;
        pub const TRAIT_MAX: f32 = 0.8;

        // Specific ranges for certain traits
        pub const INTERVENTIONISM_MIN: f32 = 0.1;
        pub const INTERVENTIONISM_MAX: f32 = 0.7;
        pub const HONOR_TREATIES_MIN: f32 = 0.3;
        pub const HONOR_TREATIES_MAX: f32 = 0.9;
        pub const ISOLATIONISM_MIN: f32 = 0.1;
        pub const ISOLATIONISM_MAX: f32 = 0.6;
    }

    /// Random number generation
    pub mod rng {
        pub const PCG64_SEED_CONVERSION: u64 = 64; // For Pcg64::seed_from_u64
    }
}

/// Window and application constants
pub mod window {
    pub const DEFAULT_WIDTH: f32 = 1400.0;
    pub const DEFAULT_HEIGHT: f32 = 1000.0;
    pub const TITLE: &str = "Dominion Earth";
}

/// Network and remote protocol constants
pub mod network {
    pub const DEFAULT_REMOTE_PORT: u16 = 15702;
}

/// Debug and logging constants
pub mod debug {
    pub const LOG_PRECISION: usize = 2; // Decimal places for debug coordinate logging
}

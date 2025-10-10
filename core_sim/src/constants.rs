//! Game Constants
//!
//! This module contains all the magic numbers, strings, and configuration values
//! used throughout the core simulation. Organizing these in one place makes
//! the codebase more maintainable and easier to balance.

// ============================================================================
// TEXTURE ATLAS CONFIGURATION
// ============================================================================

/// Sprite sheet texture atlas configuration
pub mod texture_atlas {
    /// Path to the main sprite sheet texture
    pub const SPRITE_SHEET_PATH: &str = "tiles/sprite-sheet.png";

    /// Size of each individual tile sprite in pixels
    pub const TILE_SIZE_PIXELS: u32 = 128;

    /// Number of columns in the texture atlas
    pub const ATLAS_COLUMNS: u32 = 8;

    /// Number of rows in the texture atlas
    pub const ATLAS_ROWS: u32 = 3;

    /// Total number of sprites in the atlas (columns * rows)
    pub const TOTAL_SPRITES: u32 = ATLAS_COLUMNS * ATLAS_ROWS; // 24
}

// ============================================================================
// SPRITE INDICES
// ============================================================================

/// Sprite indices for different terrain types and entities
pub mod sprite_indices {
    // Terrain sprites
    pub const PLAINS: usize = 0;
    pub const HILLS: usize = 0; // TODO: Set actual index
    pub const MOUNTAINS: usize = 0; // TODO: Set actual index
    pub const FOREST: usize = 0; // TODO: Set actual index
    pub const DESERT: usize = 0; // TODO: Set actual index
    pub const RIVER: usize = 0; // TODO: Set actual index

    // Water sprites
    pub const OCEAN: usize = 16;
    pub const SHALLOW_COAST: usize = 17;
    pub const COAST_FALLBACK: usize = 8;

    // Coast variations (rotated/flipped as needed)
    pub const COAST_1_SIDE: usize = 8; // 1 side coast (ocean to south)
    pub const COAST_2_SIDE: usize = 9; // 2 side coast (ocean to east and south)
    pub const COAST_3_SIDE: usize = 1; // 3 side coast (ocean to north, east, south)
    pub const ISLAND: usize = 2; // Island (ocean on all 4 sides)

    // Entity sprites
    pub const CAPITAL_ANCIENT: usize = 3;
    pub const ANCIENT_INFANTRY: usize = 10;
}

// ============================================================================
// MAP GENERATION
// ============================================================================

/// Map generation constants
pub mod map_generation {
    /// Default map width in tiles
    pub const DEFAULT_MAP_WIDTH: u32 = 100;

    /// Default map height in tiles
    pub const DEFAULT_MAP_HEIGHT: u32 = 50;

    /// Range for number of major islands
    pub const MAJOR_ISLANDS_MIN: u32 = 3;
    pub const MAJOR_ISLANDS_MAX: u32 = 5;

    /// Range for number of small islands
    pub const SMALL_ISLANDS_MIN: u32 = 8;
    pub const SMALL_ISLANDS_MAX: u32 = 15;

    /// Island generation bounds (fraction of map size)
    pub const ISLAND_CENTER_MARGIN: u32 = 6; // center in 1/6 to 5/6 of map

    /// Major island radius range
    pub const MAJOR_ISLAND_RADIUS_MIN: u32 = 8;
    pub const MAJOR_ISLAND_RADIUS_MAX: u32 = 15;

    /// Range for satellite islands around major islands
    pub const SATELLITE_ISLANDS_MIN: u32 = 2;
    pub const SATELLITE_ISLANDS_MAX: u32 = 4;
}

// ============================================================================
// MOVEMENT AND TERRAIN STATS
// ============================================================================

/// Movement and terrain configuration
pub mod terrain_stats {
    /// Base movement cost for most terrain
    pub const BASE_MOVEMENT_COST: f32 = 1.0;

    /// Movement cost for ocean terrain
    pub const OCEAN_MOVEMENT_COST: f32 = 3.0;

    /// Base defense bonus for most terrain
    pub const BASE_DEFENSE_BONUS: f32 = 0.0;

    /// Ocean defense bonus
    pub const OCEAN_DEFENSE_BONUS: f32 = 0.0;
}

/// Movement validation constants
pub mod movement_validation {
    /// Maximum distance for adjacent tile movement
    pub const ADJACENT_TILE_DISTANCE: u32 = 1;

    /// Default movement cost when terrain cost is zero
    pub const DEFAULT_MOVEMENT_COST_WHEN_ZERO: u32 = 1;
}

/// Movement directions (4-directional)
pub mod movement_directions {
    /// North direction vector (x, y)
    pub const NORTH: (i32, i32) = (0, 1);

    /// South direction vector (x, y)
    pub const SOUTH: (i32, i32) = (0, -1);

    /// East direction vector (x, y)
    pub const EAST: (i32, i32) = (1, 0);

    /// West direction vector (x, y)
    pub const WEST: (i32, i32) = (-1, 0);

    /// All cardinal directions
    pub const ALL_DIRECTIONS: [(i32, i32); 4] = [NORTH, SOUTH, EAST, WEST];
}

// ============================================================================
// ECONOMIC SYSTEM
// ============================================================================

/// Resource prices and economic constants
pub mod economy {
    /// Base resource prices
    pub const IRON_BASE_PRICE: f32 = 10.0;
    pub const GOLD_BASE_PRICE: f32 = 50.0;
    pub const HORSES_BASE_PRICE: f32 = 20.0;
    pub const WHEAT_BASE_PRICE: f32 = 5.0;
    pub const FISH_BASE_PRICE: f32 = 3.0;
    pub const STONE_BASE_PRICE: f32 = 8.0;
    pub const WOOD_BASE_PRICE: f32 = 6.0;
    pub const SPICES_BASE_PRICE: f32 = 25.0;

    /// Base production amounts
    pub const BASE_GOLD_PRODUCTION: f32 = 10.0;
    pub const BASE_WHEAT_PRODUCTION: f32 = 5.0;

    /// Default amount when resource not found
    pub const DEFAULT_RESOURCE_AMOUNT: f32 = 0.0;
}

// ============================================================================
// GAME FLOW
// ============================================================================

/// Game turn and timing constants
pub mod game_flow {
    /// Starting turn number
    pub const STARTING_TURN: u32 = 1;

    /// Starting civilization index
    pub const STARTING_CIV_INDEX: usize = 0;

    /// Default maximum turns
    pub const DEFAULT_MAX_TURNS: u32 = 500;

    /// Default random seed for debugging
    pub const DEFAULT_DEBUG_SEED: u64 = 42;
}

// ============================================================================
// COORDINATE BOUNDS
// ============================================================================

/// Coordinate system constants
pub mod coordinates {
    /// Minimum coordinate value
    pub const MIN_COORDINATE: i32 = 0;
}

// ============================================================================
// TILE PASS PROCESSING
// ============================================================================

/// Constants for tile processing passes
pub mod tile_passes {
    /// Pass 1: Spawn tiles
    pub const SPAWN_PASS: u8 = 1;

    /// Pass 2: Link neighbors
    pub const NEIGHBOR_PASS: u8 = 2;

    /// Pass 3: Coast conversion
    pub const COAST_PASS: u8 = 3;

    /// Neighbor offset for checking adjacent tiles
    pub const NEIGHBOR_OFFSET: u32 = 1;
}

// ============================================================================
// CIVILIZATION MANAGEMENT
// ============================================================================

/// Civilization identification and management constants
pub mod civilization_management {
    /// The civilization ID reserved for player-controlled civilizations
    pub const PLAYER_CIVILIZATION_ID: u32 = 0;

    /// Starting unit ID counter value
    pub const STARTING_UNIT_ID_COUNTER: u32 = 0;
}

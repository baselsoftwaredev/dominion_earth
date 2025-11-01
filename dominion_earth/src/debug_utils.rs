use bevy::prelude::*;
use core_sim::{
    components::{Capital, MilitaryUnit, TerrainType},
    Position,
};

/// Macro for debug printing with format args
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

/// Debug utility functions for common ECS queries and data
#[cfg(debug_assertions)]
pub struct DebugUtils;

#[cfg(debug_assertions)]
impl DebugUtils {
    /// Log all entities from a generic query with custom formatter
    pub fn log_query_entities<T>(
        query_name: &str,
        entities: &[T],
        formatter: impl Fn(&T) -> String,
    ) {
        debug_println!(
            "UI DEBUG: Found {} {} in query:",
            entities.len(),
            query_name
        );
        for entity in entities {
            debug_println!("  {}", formatter(entity));
        }
    }

    /// Log basic informational message
    pub fn log_info(message: &str) {
        debug_println!("{}", message);
    }

    /// Log tile click events
    pub fn log_tile_click(x: i32, y: i32) {
        debug_println!("Tile clicked: ({}, {})", x, y);
    }

    /// Log game state changes
    pub fn log_game_state_change(state: &str, enabled: bool) {
        debug_println!(
            "Game {}",
            if enabled {
                format!("{} enabled", state)
            } else {
                format!("{} disabled", state)
            }
        );
    }

    /// Log simulation speed changes
    pub fn log_simulation_speed(speed: f32) {
        debug_println!("Simulation speed: {:.1}x", speed);
    }

    /// Log world generation messages
    pub fn log_world_generation(seed: u64) {
        debug_println!("Generating world with random seed: {}", seed);
    }

    /// Log civilization spawning
    pub fn log_civilization_spawn(count: usize) {
        debug_println!("Spawned {} civilizations on buildable terrain", count);
    }

    /// Log capital spawning details
    pub fn log_capital_spawn_skip(name: &str, x: i32, y: i32) {
        debug_println!(
            "DEBUG: Skipping {} capital spawn - position ({}, {}) is not on buildable terrain",
            name,
            x,
            y
        );
    }

    /// Log successful capital spawning
    pub fn log_capital_spawn_success(name: &str, pos: &Position, sprite_index: usize) {
        debug_println!(
            "DEBUG: Spawning capital for {} at {:?} with sprite index {} (buildable terrain)",
            name,
            pos,
            sprite_index
        );
    }

    /// Log world initialization message
    pub fn log_world_initialization(width: u32, height: u32) {
        debug_println!(
            "Game world initialized with {} x {} map (reduced size for performance)",
            width,
            height
        );
    }

    /// Log neighbor debugging info
    pub fn log_neighbors_header(x: i32, y: i32, terrain: &str) {
        debug_println!("=== DEBUG LOGGING: Tile ({}, {}) Neighbors ===", x, y);
        debug_println!("DEBUG LOG - Center tile: {}", terrain);
    }

    /// Log single neighbor info
    pub fn log_single_neighbor(
        direction: &str,
        terrain: Option<&str>,
        x: Option<i32>,
        y: Option<i32>,
    ) {
        match (terrain, x, y) {
            (Some(terrain), Some(x), Some(y)) => {
                debug_println!("{}: {} at ({}, {})", direction, terrain, x, y);
            }
            _ => {
                debug_println!("{}: OutOfBounds", direction);
            }
        }
    }

    /// Log neighbors footer
    pub fn log_neighbors_footer() {
        debug_println!("===============================");
    }

    /// Log capital entities specifically
    pub fn log_capitals(capitals: &[(&Capital, &Position)]) {
        Self::log_query_entities("capitals", capitals, |(capital, pos)| {
            format!(
                "Capital at ({}, {}) for Civ {}",
                pos.x, pos.y, capital.owner.0
            )
        });
    }

    /// Log military unit entities specifically
    pub fn log_units(units: &[(&MilitaryUnit, &Position)]) {
        Self::log_query_entities("units", units, |(unit, pos)| {
            format!(
                "{:?} at ({}, {}) for Civ {}",
                unit.unit_type, pos.x, pos.y, unit.owner.0
            )
        });
    }

    /// Log tile information
    pub fn log_tile_check(pos: &Position) {
        debug_println!(
            "UI DEBUG: Checking for structures at tile ({}, {})",
            pos.x,
            pos.y
        );
    }

    /// Log structure matches on a specific tile
    pub fn log_structure_matches(
        pos: &Position,
        capitals: &[(&Capital, &Position)],
        units: &[(&MilitaryUnit, &Position)],
    ) {
        let matching_capitals: Vec<_> = capitals
            .iter()
            .filter(|(_, capital_pos)| capital_pos.x == pos.x && capital_pos.y == pos.y)
            .collect();

        let matching_units: Vec<_> = units
            .iter()
            .filter(|(_, unit_pos)| unit_pos.x == pos.x && unit_pos.y == pos.y)
            .collect();

        if !matching_capitals.is_empty() {
            debug_println!(
                "UI DEBUG: Found {} capital(s) at tile ({}, {}):",
                matching_capitals.len(),
                pos.x,
                pos.y
            );
            for (capital, _) in &matching_capitals {
                debug_println!("  🏛️ Capital (Civ {})", capital.owner.0);
            }
        }

        if !matching_units.is_empty() {
            debug_println!(
                "UI DEBUG: Found {} unit(s) at tile ({}, {}):",
                matching_units.len(),
                pos.x,
                pos.y
            );
            for (unit, _) in &matching_units {
                debug_println!("  ⚔️ {:?} (Civ {})", unit.unit_type, unit.owner.0);
            }
        }

        if matching_capitals.is_empty() && matching_units.is_empty() {
            debug_println!(
                "UI DEBUG: No structures found at tile ({}, {})",
                pos.x,
                pos.y
            );
        }
    }

    /// Generic position-based entity matcher
    pub fn find_entities_at_position<'a, T>(
        entities: &'a [T],
        target_pos: &Position,
        pos_extractor: impl Fn(&T) -> &Position,
    ) -> Vec<&'a T> {
        entities
            .iter()
            .filter(|entity| {
                let pos = pos_extractor(entity);
                pos.x == target_pos.x && pos.y == target_pos.y
            })
            .collect()
    }
}

/// Extension trait for easier debugging of common query results
#[cfg(debug_assertions)]
pub trait DebugQueryExt {
    fn debug_count(&self, name: &str);
}

#[cfg(debug_assertions)]
impl<T> DebugQueryExt for Vec<T> {
    fn debug_count(&self, name: &str) {
        debug_println!("DEBUG: {} count: {}", name, self.len());
    }
}

#[cfg(debug_assertions)]
impl DebugUtils {
    /// Log terrain comparison between ECS and WorldMap
    pub fn log_terrain_comparison(
        pos: &Position,
        ecs_terrain: Option<&TerrainType>,
        worldmap_terrain: Option<&TerrainType>,
        neighbors_info: &[(String, String)],
    ) {
        println!("=== UI DISPLAY: Tile ({}, {}) Data ===", pos.x, pos.y);
        if let Some(terrain) = ecs_terrain {
            println!("UI DISPLAY - ECS Terrain: {:?}", terrain);
        }

        // Print neighbor terrain types
        println!("UI DISPLAY - Neighbors:");
        for (direction, terrain) in neighbors_info {
            println!("  {}: {}", direction, terrain);
        }

        if let Some(wm_terrain) = worldmap_terrain {
            println!("UI DISPLAY - WorldMap Terrain: {:?}", wm_terrain);
            if let Some(ecs_terrain) = ecs_terrain {
                if ecs_terrain != wm_terrain {
                    println!(
                        "⚠️  TERRAIN MISMATCH: ECS={:?} vs WorldMap={:?}",
                        ecs_terrain, wm_terrain
                    );
                }
            }
        }
        println!("=====================================");
    }
}

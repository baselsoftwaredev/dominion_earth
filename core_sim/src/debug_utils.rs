use crate::TerrainType;

/// Debug utilities for the core simulation
///
/// This module provides centralized debug logging that respects both RUST_LOG and DEBUG_LOGGING environment variables.
/// These utilities can be used across all packages in the workspace.
pub struct CoreDebugUtils;

impl CoreDebugUtils {
    /// Checks if debug logging is enabled by checking RUST_LOG or DEBUG_LOGGING environment variables
    pub fn is_debug_enabled() -> bool {
        std::env::var("RUST_LOG").is_ok() || std::env::var("DEBUG_LOGGING").is_ok()
    }

    /// General debug info logging
    pub fn log_info(message: &str) {
        if Self::is_debug_enabled() {
            println!("{}", message);
        }
    }

    /// Log tile conversion operations
    pub fn log_tile_conversion(
        x: u32,
        y: u32,
        from_terrain: &TerrainType,
        to_terrain: &TerrainType,
    ) {
        if Self::is_debug_enabled() {
            println!(
                "Converting tile at ({}, {}) from {:?} to {:?}",
                x, y, from_terrain, to_terrain
            );
        }
    }

    /// Log ocean to shallow coast conversion
    pub fn log_shallow_coast_conversion(x: u32, y: u32) {
        if Self::is_debug_enabled() {
            println!("Converting ocean tile at ({}, {}) to ShallowCoast", x, y);
        }
    }

    /// Log land to coast conversion with ocean neighbors
    pub fn log_coast_conversion(
        x: u32,
        y: u32,
        original_terrain: &TerrainType,
        ocean_directions: &[&str],
    ) {
        if Self::is_debug_enabled() {
            println!(
                "Converting land tile at ({}, {}) from {:?} to Coast. Ocean neighbors: {}",
                x,
                y,
                original_terrain,
                ocean_directions.join(", ")
            );
        }
    }

    /// Log coast sprite selection
    pub fn log_coast_sprite_selection(sprite_index: u32) {
        if Self::is_debug_enabled() {
            println!("  -> Using coast tile index: {}", sprite_index);
        }
    }

    /// Log tile flip operations
    pub fn log_tile_flip(x: bool, y: bool, d: bool) {
        if Self::is_debug_enabled() && (x || y || d) {
            println!("  -> Flipping tile: x={}, y={}, d={}", x, y, d);
        }
    }

    /// Log neighbor analysis for debugging
    pub fn log_neighbor_analysis(
        x: u32,
        y: u32,
        terrain: &TerrainType,
        neighbors: &[(&str, Option<TerrainType>)],
    ) {
        if Self::is_debug_enabled() {
            println!("=== TILE NEIGHBOR ANALYSIS ===");
            println!("Center tile at ({}, {}): {:?}", x, y, terrain);
            for (direction, neighbor_terrain) in neighbors {
                match neighbor_terrain {
                    Some(terrain) => println!("  {}: {:?}", direction, terrain),
                    None => println!("  {}: OutOfBounds", direction),
                }
            }
            println!("===============================");
        }
    }

    /// Log world generation progress
    pub fn log_world_generation(seed: u64) {
        if Self::is_debug_enabled() {
            println!("Generating world with random seed: {}", seed);
        }
    }

    /// Log civilization spawning
    pub fn log_civilization_spawn(count: usize) {
        if Self::is_debug_enabled() {
            println!("Spawned {} civilizations on buildable terrain", count);
        }
    }

    /// Log turn order initialization
    pub fn log_turn_order_init(civ_count: usize) {
        if Self::is_debug_enabled() {
            println!("Initialized turn order with {} civilizations", civ_count);
        }
    }

    /// Log civilization turn activation
    pub fn log_civ_turn_active(turn: u32, civ_name: &str, civ_id: &str) {
        if Self::is_debug_enabled() {
            println!("Turn {}: {} ({}) is now active", turn, civ_name, civ_id);
        }
    }

    /// Log unit movement
    pub fn log_unit_movement(unit_id: u32, civ_id: u32, x: i32, y: i32) {
        if Self::is_debug_enabled() {
            println!(
                "  Unit {} owned by Civ {} moved to ({}, {})",
                unit_id, civ_id, x, y
            );
        }
    }

    /// Log turn completion
    pub fn log_turn_complete(completed_turn: u32, starting_turn: u32) {
        if Self::is_debug_enabled() {
            println!("=== Turn {} Complete ===", completed_turn);
            println!("=== Starting Turn {} ===", starting_turn);
        }
    }

    /// Log turn advancement (legacy system)
    pub fn log_turn_advance(turn: u32) {
        if Self::is_debug_enabled() {
            println!("Advanced to turn {}", turn);
        }
    }

    /// Log capital evolution
    pub fn log_capital_evolution(civ_name: &str, from_age: &str, to_age: &str, turn: u32) {
        if Self::is_debug_enabled() {
            println!(
                "Capital of {} evolving from {} to {} at turn {}",
                civ_name, from_age, to_age, turn
            );
        }
    }
}

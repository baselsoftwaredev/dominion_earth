//! World Generation Module
//!
//! Orchestrates the multi-step procedural world generation pipeline following Bevy ECS patterns.
//!
//! The generation process is composed of discrete, testable steps:
//! 1. **Ocean initialization** (landmass_gen) - Initialize all tiles as ocean
//! 2. **Plains landmass generation** (landmass_gen) - Create archipelago-style islands
//! 3. **Forest placement** (forest_gen) - Add forests with natural clustering to plains
//! 4. **Resource placement** (resource_gen) - Distribute resources by terrain type
//! 5. **Coast conversion** (tile_passes.rs, external) - Convert edge tiles to coast
//!
//! Each module can be tested independently and represents a coherent generation phase.
//! This approach mirrors Bevy's system scheduling model where each system has a clear purpose
//! and operates on specific data types.

mod forest_gen;
mod landmass_gen;
mod resource_gen;

use crate::WorldMap;
use rand::Rng;

/// Generates a complete randomized archipelago map with terrain and resources.
///
/// This is the main entry point for world generation. It coordinates all generation phases
/// to produce a fully populated world map ready for gameplay.
///
/// # Arguments
///
/// * `width` - Width of the map in tiles
/// * `height` - Height of the map in tiles
/// * `rng` - Random number generator for procedural generation
///
/// # Returns
///
/// A fully initialized `WorldMap` with oceans, landmasses, forests, and resources.
pub fn generate_island_map(width: u32, height: u32, rng: &mut impl Rng) -> WorldMap {
    let mut map = WorldMap::new(width, height);

    // STEP 1 & 2: Initialize ocean and create plain landmasses
    landmass_gen::generate_landmasses(&mut map, width, height, rng);

    // STEP 3: Add forests to suitable plains tiles
    forest_gen::generate_forests(&mut map, rng);

    // NOTE: Coast tile conversion will happen later in tile_passes.rs
    // The coast conversion logic is handled in the three-pass system:
    // - spawn_tiles_pass: creates initial terrain
    // - assign_neighbors_pass: links neighbors
    // - convert_to_coast_pass: processes land->coast conversion with flipping

    // STEP 4: Place resources on all terrain types
    resource_gen::place_resources(&mut map, rng);

    map
}

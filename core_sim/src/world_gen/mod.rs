//! World Generation Module
//!
//! Orchestrates the multi-step procedural world generation pipeline following Bevy ECS patterns.
//!
//! The generation process is composed of discrete, testable steps:
//! 1. **Ocean initialization** (landmass_gen) - Initialize all tiles as ocean
//! 2. **Plains landmass generation** (landmass_gen) - Create archipelago-style islands
//! 3. **Mountain range placement** (mountain_gen) - Add linear mountain chains to plains
//! 4. **Forest placement** (forest_gen) - Add forests with natural clustering to plains
//! 5. **Resource placement** (resource_gen) - Distribute resources by terrain type
//! 6. **Coast conversion** (tile_passes.rs, external) - Convert edge tiles to coast
//!
//! Each module can be tested independently and represents a coherent generation phase.
//! This approach mirrors Bevy's system scheduling model where each system has a clear purpose
//! and operates on specific data types.

mod forest_gen;
mod landmass_gen;
mod mountain_gen;
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
/// A fully initialized `WorldMap` with oceans, landmasses, forests, mountains, and resources.
pub fn generate_island_map(width: u32, height: u32, rng: &mut impl Rng) -> WorldMap {
    let mut map = WorldMap::new(width, height);

    landmass_gen::generate_landmasses(&mut map, width, height, rng);
    mountain_gen::generate_mountains(&mut map, rng);
    forest_gen::generate_forests(&mut map, rng);
    resource_gen::place_resources(&mut map, rng);

    map
}

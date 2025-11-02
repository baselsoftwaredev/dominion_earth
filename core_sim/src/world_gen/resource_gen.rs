//! Resource Placement
//!
//! Handles placement of strategic and luxury resources on terrain tiles.
//!
//! Resources are distributed by terrain type with different probability distributions.
//! This module follows Bevy's data-driven design by using only constants from the
//! constants module for configuration.

use crate::{
    constants::resource_generation, resources::Resource as GameResource, Position, TerrainType,
    WorldMap,
};
use rand::Rng;

/// Place resources on land tiles based on terrain type.
///
/// Resources are distributed probabilistically based on terrain:
/// - Mountains: Iron or Stone
/// - Hills: Iron, Gold, or Stone
/// - Plains: Wheat or Horses
/// - Forests: Wood
/// - Deserts: Gold or Spices
/// - Coasts: Fish
/// - Rivers: Fish
///
/// The placement is random but limited by resource density to maintain balance.
///
/// # Arguments
///
/// * `map` - Mutable reference to the world map to populate with resources
/// * `rng` - Random number generator for probabilistic placement
pub fn place_resources(map: &mut WorldMap, rng: &mut impl Rng) {
    let total_tiles = map.width * map.height;
    let target_resources = (total_tiles as f32 * resource_generation::RESOURCE_DENSITY) as u32;

    let mut placed = 0;
    while placed < target_resources {
        let x = rng.gen_range(0..map.width);
        let y = rng.gen_range(0..map.height);
        let pos = Position::new(x as i32, y as i32);

        if let Some(tile) = map.get_tile_mut(pos) {
            if tile.resource.is_none() && !matches!(tile.terrain, TerrainType::Ocean) {
                tile.resource = Some(select_resource_for_terrain(tile.terrain.clone(), rng));
                placed += 1;
            }
        }
    }
}

/// Selects an appropriate resource for the given terrain type.
///
/// Uses probability distributions to match resources to terrain realistically.
/// For example, mountains are more likely to have stone than horses.
fn select_resource_for_terrain(terrain: TerrainType, rng: &mut impl Rng) -> GameResource {
    match terrain {
        TerrainType::Mountains => {
            if rng.gen_bool(resource_generation::MOUNTAIN_IRON_PROBABILITY as f64) {
                GameResource::Iron
            } else {
                GameResource::Stone
            }
        }
        TerrainType::Hills => {
            match rng.gen_range(0..resource_generation::HILL_RESOURCE_DENOMINATOR) {
                i if i < resource_generation::HILL_RESOURCE_IRON_NUMERATOR => GameResource::Iron,
                i if i < resource_generation::HILL_RESOURCE_IRON_NUMERATOR
                    + resource_generation::HILL_RESOURCE_GOLD_NUMERATOR =>
                {
                    GameResource::Gold
                }
                _ => GameResource::Stone,
            }
        }
        TerrainType::Plains => {
            if rng.gen_bool(resource_generation::PLAINS_WHEAT_PROBABILITY as f64) {
                GameResource::Wheat
            } else {
                GameResource::Horses
            }
        }
        TerrainType::Forest => GameResource::Wood,
        TerrainType::Desert => {
            if rng.gen_bool(resource_generation::DESERT_GOLD_PROBABILITY as f64) {
                GameResource::Gold
            } else {
                GameResource::Spices
            }
        }
        TerrainType::Coast => GameResource::Fish,
        TerrainType::ShallowCoast => GameResource::Fish,
        TerrainType::River => GameResource::Fish,
        TerrainType::Ocean => unreachable!("Ocean tiles should not receive resources"),
    }
}

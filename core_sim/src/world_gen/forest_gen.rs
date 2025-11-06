//! Forest Generation
//!
//! Handles forest placement on suitable plains tiles using noise-based distribution.
//!
//! Forests are placed with natural clustering behavior - new forests have a base chance
//! of appearing, but are more likely to appear adjacent to existing forests. This creates
//! realistic forest patches and groves rather than scattered individual trees.

use crate::{
    constants::{map_generation, terrain_stats},
    resources::MapTile,
    Position, TerrainType, WorldMap,
};
use rand::Rng;

/// Generate forests on suitable plains tiles using noise-based, clustered placement.
///
/// Scans all plains tiles and converts them to forests based on noise values and
/// proximity to existing forests, creating natural-looking forest clusters.
///
/// # Arguments
///
/// * `map` - Mutable reference to the world map to populate with forests
/// * `rng` - Random number generator for procedural generation
pub fn generate_forests(map: &mut WorldMap, rng: &mut impl Rng) {
    let mut plains_positions = Vec::new();
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = Position::new(x as i32, y as i32);
            if let Some(tile) = map.get_tile(pos) {
                if matches!(tile.terrain, TerrainType::Plains) {
                    plains_positions.push(pos);
                }
            }
        }
    }

    for pos in plains_positions {
        if should_place_forest(map, pos, rng) {
            if let Some(tile) = map.get_tile_mut(pos) {
                *tile = create_forest_tile();
            }
        }
    }
}

/// Creates a default forest tile with appropriate stats.
fn create_forest_tile() -> MapTile {
    MapTile {
        terrain: TerrainType::Forest,
        owner: None,
        city: None,
        resource: None,
    }
}

/// Determines whether a forest should be placed at the given position.
///
/// Uses a two-tier system:
/// - Base chance for new forest patches (lower probability)
/// - Higher chance for forests adjacent to existing forests (clustering)
/// - Additionally checks overall density threshold
fn should_place_forest(map: &WorldMap, pos: Position, rng: &mut impl Rng) -> bool {
    let noise_value = rng.gen::<f32>();

    if noise_value >= map_generation::FOREST_DENSITY {
        return false;
    }

    if has_adjacent_forest(map, pos) {
        rng.gen::<f32>() < map_generation::FOREST_CLUSTER_CHANCE
    } else {
        rng.gen::<f32>() < map_generation::FOREST_BASE_CHANCE
    }
}

/// Checks if any adjacent tiles (4-directional cardinal neighbors) contain forests.
///
/// Used to determine if a plains tile should be more likely to convert to forest
/// based on proximity to existing forests (clustering behavior).
///
/// # Arguments
///
/// * `map` - Reference to the world map
/// * `pos` - Position to check neighbors for
///
/// # Returns
///
/// `true` if any of the four cardinal neighbors is a forest, `false` otherwise.
fn has_adjacent_forest(map: &WorldMap, pos: Position) -> bool {
    let neighbors = [
        Position::new(pos.x - 1, pos.y), // West
        Position::new(pos.x + 1, pos.y), // East
        Position::new(pos.x, pos.y - 1), // North
        Position::new(pos.x, pos.y + 1), // South
    ];

    neighbors.iter().any(|neighbor_pos| {
        map.get_tile(*neighbor_pos)
            .is_some_and(|tile| matches!(tile.terrain, TerrainType::Forest))
    })
}

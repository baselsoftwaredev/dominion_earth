//! Mountain Range Generation
//!
//! Handles mountain range placement on suitable plains tiles.
//!
//! Mountains are placed in linear formations (ranges) rather than scattered clusters.
//! Each range has a starting point, direction, and length, creating realistic
//! mountain chain formations similar to real-world mountain systems.

use crate::{
    constants::{map_generation, terrain_stats},
    resources::MapTile,
    Position, TerrainType, WorldMap,
};
use rand::Rng;

const DIRECTION_NORTH: (i32, i32) = (0, 1);
const DIRECTION_NORTHEAST: (i32, i32) = (1, 1);
const DIRECTION_EAST: (i32, i32) = (1, 0);
const DIRECTION_SOUTHEAST: (i32, i32) = (1, -1);
const DIRECTION_SOUTH: (i32, i32) = (0, -1);
const DIRECTION_SOUTHWEST: (i32, i32) = (-1, -1);
const DIRECTION_WEST: (i32, i32) = (-1, 0);
const DIRECTION_NORTHWEST: (i32, i32) = (-1, 1);

const MOUNTAIN_RANGE_DIRECTIONS: [(i32, i32); 8] = [
    DIRECTION_NORTH,
    DIRECTION_NORTHEAST,
    DIRECTION_EAST,
    DIRECTION_SOUTHEAST,
    DIRECTION_SOUTH,
    DIRECTION_SOUTHWEST,
    DIRECTION_WEST,
    DIRECTION_NORTHWEST,
];

/// Generate mountain ranges on suitable plains tiles.
///
/// Creates several mountain ranges across the map, each with its own direction
/// and length. Mountains are placed in linear chains to simulate natural
/// geological formations.
///
/// # Arguments
///
/// * `map` - Mutable reference to the world map to populate with mountains
/// * `rng` - Random number generator for procedural generation
pub fn generate_mountains(map: &mut WorldMap, rng: &mut impl Rng) {
    let num_ranges =
        rng.gen_range(map_generation::MOUNTAIN_RANGES_MIN..=map_generation::MOUNTAIN_RANGES_MAX);

    for _ in 0..num_ranges {
        generate_mountain_range(map, rng);
    }
}

/// Generates a single mountain range on the map.
///
/// Creates a linear chain of mountain tiles starting from a random position
/// on a plains tile, extending in a semi-random direction with some wandering.
fn generate_mountain_range(map: &mut WorldMap, rng: &mut impl Rng) {
    let start_pos = match find_suitable_mountain_start(map, rng) {
        Some(pos) => pos,
        None => return,
    };

    let range_length = rng.gen_range(
        map_generation::MOUNTAIN_RANGE_LENGTH_MIN..=map_generation::MOUNTAIN_RANGE_LENGTH_MAX,
    );

    let mut direction_idx = rng.gen_range(0..MOUNTAIN_RANGE_DIRECTIONS.len());
    let mut current_pos = start_pos;

    for _ in 0..range_length {
        if is_valid_mountain_position(map, current_pos) {
            if let Some(tile) = map.get_tile_mut(current_pos) {
                *tile = create_mountain_tile();
            }
        }

        let (dx, dy) = MOUNTAIN_RANGE_DIRECTIONS[direction_idx];
        current_pos = Position::new(current_pos.x + dx, current_pos.y + dy);

        if should_change_mountain_range_direction(rng) {
            direction_idx = calculate_new_direction_index(direction_idx, rng);
        }

        if !is_within_bounds(map, current_pos) {
            break;
        }
    }
}

/// Finds a suitable starting position for a mountain range.
///
/// Searches for a plains tile that isn't too close to the map edge to allow
/// the range to extend properly.
fn find_suitable_mountain_start(map: &WorldMap, rng: &mut impl Rng) -> Option<Position> {
    let margin = map_generation::MOUNTAIN_EDGE_MARGIN;

    for _ in 0..map_generation::MOUNTAIN_START_POSITION_ATTEMPTS {
        let x = rng.gen_range(margin..(map.width - margin)) as i32;
        let y = rng.gen_range(margin..(map.height - margin)) as i32;
        let pos = Position::new(x, y);

        if is_plains_terrain(map, pos) {
            return Some(pos);
        }
    }

    None
}

fn is_plains_terrain(map: &WorldMap, pos: Position) -> bool {
    map.get_tile(pos)
        .is_some_and(|tile| matches!(tile.terrain, TerrainType::Plains))
}

/// Creates a default mountain tile with appropriate stats.
fn create_mountain_tile() -> MapTile {
    MapTile {
        terrain: TerrainType::Mountains,
        owner: None,
        city: None,
        resource: None,
        movement_cost: terrain_stats::MOUNTAIN_MOVEMENT_COST,
        defense_bonus: terrain_stats::MOUNTAIN_DEFENSE_BONUS,
    }
}

/// Checks if a position is valid for mountain placement.
///
/// A position is valid if it's within map bounds and the tile is plains.
fn is_valid_mountain_position(map: &WorldMap, pos: Position) -> bool {
    if !is_within_bounds(map, pos) {
        return false;
    }

    is_plains_terrain(map, pos)
}

/// Checks if a position is within map bounds.
fn is_within_bounds(map: &WorldMap, pos: Position) -> bool {
    pos.x >= 0 && pos.x < map.width as i32 && pos.y >= 0 && pos.y < map.height as i32
}

fn should_change_mountain_range_direction(rng: &mut impl Rng) -> bool {
    rng.gen::<f32>() < map_generation::MOUNTAIN_DIRECTION_CHANGE_CHANCE
}

fn calculate_new_direction_index(current_index: usize, rng: &mut impl Rng) -> usize {
    let direction_change = rng.gen_range(-1..=1);
    let total_directions = MOUNTAIN_RANGE_DIRECTIONS.len() as i32;
    ((current_index as i32 + direction_change + total_directions) % total_directions) as usize
}

//! Landmass Generation
//!
//! Handles the initial ocean and plains generation for the world map.
//!
//! This module follows the Bevy ECS pattern with pure functions for procedural generation.
//! The generation process is composed of discrete steps that can be tested independently.

use crate::{
    constants::{map_generation, terrain_stats},
    resources::MapTile,
    Position, TerrainType, WorldMap,
};
use rand::Rng;

/// Initializes the map with ocean tiles, then generates plain landmasses.
///
/// # Arguments
///
/// * `map` - Mutable reference to the world map to be populated
/// * `width` - Width of the map in tiles
/// * `height` - Height of the map in tiles
/// * `rng` - Random number generator for procedural generation
pub fn generate_landmasses(map: &mut WorldMap, width: u32, height: u32, rng: &mut impl Rng) {
    initialize_ocean_map(map);
    generate_plain_landmasses(map, width, height, rng);
}

/// Initializes the entire map with ocean tiles.
///
/// This is the foundational step of world generation. All tiles are set to ocean
/// before subsequent generation passes add landmasses.
fn initialize_ocean_map(map: &mut WorldMap) {
    let ocean_tile = create_ocean_tile();

    for x in 0..map.width {
        for y in 0..map.height {
            let pos = Position::new(x as i32, y as i32);
            if let Some(tile) = map.get_tile_mut(pos) {
                *tile = ocean_tile.clone();
            }
        }
    }
}

/// Creates a default ocean tile with appropriate stats.
fn create_ocean_tile() -> MapTile {
    MapTile {
        terrain: TerrainType::Ocean,
        owner: None,
        city: None,
        resource: None,
    }
}

/// Creates a default plains tile with appropriate stats.
fn create_plains_tile() -> MapTile {
    MapTile {
        terrain: TerrainType::Plains,
        owner: None,
        city: None,
        resource: None,
    }
}

/// Generates plain landmasses on the initialized ocean map.
///
/// This creates both major island clusters (with satellite islands) and small scattered islands.
/// Only plains terrain is created in this pass; other terrain types are added in later passes.
fn generate_plain_landmasses(map: &mut WorldMap, width: u32, height: u32, rng: &mut impl Rng) {
    let num_major_islands =
        rng.gen_range(map_generation::MAJOR_ISLANDS_MIN..=map_generation::MAJOR_ISLANDS_MAX);

    for _ in 0..num_major_islands {
        generate_plain_island_cluster(map, width, height, rng);
    }

    let num_small_islands =
        rng.gen_range(map_generation::SMALL_ISLANDS_MIN..map_generation::SMALL_ISLANDS_MAX);
    for _ in 0..num_small_islands {
        generate_small_plain_island(map, width, height, rng);
    }

    smooth_plain_landmasses(map);
}

/// Generates a cluster of islands: one major island with satellite islands around it.
fn generate_plain_island_cluster(map: &mut WorldMap, width: u32, height: u32, rng: &mut impl Rng) {
    let center_x = rng.gen_range(
        width / map_generation::ISLAND_CENTER_MARGIN
            ..(map_generation::ISLAND_CENTER_MARGIN - 1) * width
                / map_generation::ISLAND_CENTER_MARGIN,
    );
    let center_y = rng.gen_range(
        height / map_generation::ISLAND_CENTER_MARGIN
            ..(map_generation::ISLAND_CENTER_MARGIN - 1) * height
                / map_generation::ISLAND_CENTER_MARGIN,
    );

    let main_radius = rng.gen_range(
        map_generation::MAJOR_ISLAND_RADIUS_MIN..map_generation::MAJOR_ISLAND_RADIUS_MAX,
    );
    generate_plain_island_at(map, center_x, center_y, main_radius, rng);

    let num_satellites = rng
        .gen_range(map_generation::SATELLITE_ISLANDS_MIN..=map_generation::SATELLITE_ISLANDS_MAX);
    for _ in 0..num_satellites {
        let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let distance = rng.gen_range(
            map_generation::SATELLITE_MINIMUM_DISTANCE..map_generation::SATELLITE_MAXIMUM_DISTANCE,
        ) as f32;

        let sat_x = center_x as f32 + angle.cos() * distance;
        let sat_y = center_y as f32 + angle.sin() * distance;

        if sat_x >= 0.0 && sat_x < width as f32 && sat_y >= 0.0 && sat_y < height as f32 {
            let sat_radius = rng.gen_range(
                map_generation::SATELLITE_ISLAND_RADIUS_MIN
                    ..map_generation::SATELLITE_ISLAND_RADIUS_MAX,
            );
            generate_plain_island_at(map, sat_x as u32, sat_y as u32, sat_radius, rng);
        }
    }
}

/// Generates a small island at a random location on the map.
fn generate_small_plain_island(map: &mut WorldMap, width: u32, height: u32, rng: &mut impl Rng) {
    let center_x = rng.gen_range(0..width);
    let center_y = rng.gen_range(0..height);
    let radius = rng.gen_range(
        map_generation::SMALL_ISLAND_RADIUS_MIN..map_generation::SMALL_ISLAND_RADIUS_MAX,
    );

    generate_plain_island_at(map, center_x, center_y, radius, rng);
}

/// Generates a circular island of plains terrain at the specified location.
///
/// Uses noise-based distance checking to create natural-looking coastlines.
///
/// # Arguments
///
/// * `map` - Mutable reference to the world map
/// * `center_x` - X coordinate of the island center
/// * `center_y` - Y coordinate of the island center
/// * `radius` - Base radius of the island
/// * `rng` - Random number generator
fn generate_plain_island_at(
    map: &mut WorldMap,
    center_x: u32,
    center_y: u32,
    radius: u32,
    rng: &mut impl Rng,
) {
    let radius_f = radius as f32;
    let plains_tile = create_plains_tile();

    for dx in -(radius as i32)..=(radius as i32) {
        for dy in -(radius as i32)..=(radius as i32) {
            let x = center_x as i32 + dx;
            let y = center_y as i32 + dy;

            if x < 0 || x >= map.width as i32 || y < 0 || y >= map.height as i32 {
                continue;
            }

            let distance = ((dx * dx + dy * dy) as f32).sqrt();
            let noise = rng.gen::<f32>() * map_generation::ISLAND_EDGE_NOISE_AMPLITUDE
                - map_generation::ISLAND_EDGE_NOISE_OFFSET;
            let adjusted_radius = radius_f + noise;

            if distance <= adjusted_radius {
                let pos = Position::new(x, y);
                if let Some(tile) = map.get_tile_mut(pos) {
                    *tile = plains_tile.clone();
                }
            }
        }
    }
}

/// Smooths landmasses to improve natural appearance.
///
/// Currently a placeholder that applies no changes. Future implementations
/// could apply cellular automata or erosion simulation for more natural coastlines.
fn smooth_plain_landmasses(_map: &mut WorldMap) {
    // Placeholder: currently no changes applied
    // Could implement cellular automata or erosion simulation here
}

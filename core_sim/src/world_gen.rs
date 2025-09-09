use crate::{
    constants::{map_generation, terrain_stats},
    resources::MapTile, resources::Resource as GameResource, Position, TerrainType, WorldMap,
};
use rand::Rng;

/// Generate a randomized archipelago map following clean generation steps:
/// 1. Start with whole map as ocean
/// 2. Add plain land masses only (no hills/other terrain yet)
/// 3. Process land tiles to convert to appropriate coast tiles based on ocean neighbors
pub fn generate_island_map(width: u32, height: u32, rng: &mut impl Rng) -> WorldMap {
    let mut map = WorldMap::new(width, height);

    // STEP 1: Start with whole map as ocean tiles
    initialize_ocean_map(&mut map);

    // STEP 2: Create plain land masses only (no hills/other terrain)
    generate_plain_landmasses(&mut map, width, height, rng);

    // NOTE: Coast tile conversion will happen later in tile_passes.rs
    // The coast conversion logic is handled in the three-pass system:
    // - spawn_tiles_pass: creates initial terrain
    // - assign_neighbors_pass: links neighbors
    // - convert_to_coast_pass: processes land->coast conversion with flipping

    // Place resources on land (this can stay for now)
    place_resources(&mut map, rng);

    map
}

/// STEP 1: Initialize entire map with ocean tiles
fn initialize_ocean_map(map: &mut WorldMap) {
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = Position::new(x as i32, y as i32);
            if let Some(tile) = map.get_tile_mut(pos) {
                *tile = MapTile {
                    terrain: TerrainType::Ocean,
                    owner: None,
                    city: None,
                    resource: None,
                    movement_cost: terrain_stats::OCEAN_MOVEMENT_COST,
                    defense_bonus: terrain_stats::OCEAN_DEFENSE_BONUS,
                };
            }
        }
    }
}

/// STEP 2: Generate plain land masses only (no hills or other terrain types)
fn generate_plain_landmasses(map: &mut WorldMap, width: u32, height: u32, rng: &mut impl Rng) {
    // Generate major islands with smaller satellite islands
    let num_major_islands = rng.gen_range(map_generation::MAJOR_ISLANDS_MIN..=map_generation::MAJOR_ISLANDS_MAX);

    for _ in 0..num_major_islands {
        generate_plain_island_cluster(map, width, height, rng);
    }

    // Add scattered small islands
    let num_small_islands = rng.gen_range(map_generation::SMALL_ISLANDS_MIN..map_generation::SMALL_ISLANDS_MAX);
    for _ in 0..num_small_islands {
        generate_small_plain_island(map, width, height, rng);
    }

    // Smooth and refine the landmasses
    smooth_plain_landmasses(map);
}

fn generate_plain_island_cluster(map: &mut WorldMap, width: u32, height: u32, rng: &mut impl Rng) {
    // Pick a random center for the main island
    let center_x = rng.gen_range(width / map_generation::ISLAND_CENTER_MARGIN..(map_generation::ISLAND_CENTER_MARGIN - 1) * width / map_generation::ISLAND_CENTER_MARGIN);
    let center_y = rng.gen_range(height / map_generation::ISLAND_CENTER_MARGIN..(map_generation::ISLAND_CENTER_MARGIN - 1) * height / map_generation::ISLAND_CENTER_MARGIN);

    // Generate main island
    let main_radius = rng.gen_range(map_generation::MAJOR_ISLAND_RADIUS_MIN..map_generation::MAJOR_ISLAND_RADIUS_MAX);
    generate_plain_island_at(map, center_x, center_y, main_radius, rng);

    // Generate satellite islands around the main one
    let num_satellites = rng.gen_range(map_generation::SATELLITE_ISLANDS_MIN..=map_generation::SATELLITE_ISLANDS_MAX);
    for _ in 0..num_satellites {
        let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let distance = rng.gen_range(15..25) as f32;

        let sat_x = center_x as f32 + angle.cos() * distance;
        let sat_y = center_y as f32 + angle.sin() * distance;

        if sat_x >= 0.0 && sat_x < width as f32 && sat_y >= 0.0 && sat_y < height as f32 {
            let sat_radius = rng.gen_range(4..8);
            generate_plain_island_at(map, sat_x as u32, sat_y as u32, sat_radius, rng);
        }
    }
}

fn generate_small_plain_island(map: &mut WorldMap, width: u32, height: u32, rng: &mut impl Rng) {
    let center_x = rng.gen_range(0..width);
    let center_y = rng.gen_range(0..height);
    let radius = rng.gen_range(2..5);

    generate_plain_island_at(map, center_x, center_y, radius, rng);
}

fn generate_plain_island_at(
    map: &mut WorldMap,
    center_x: u32,
    center_y: u32,
    radius: u32,
    rng: &mut impl Rng,
) {
    let radius_f = radius as f32;

    for dx in -(radius as i32)..=(radius as i32) {
        for dy in -(radius as i32)..=(radius as i32) {
            let x = center_x as i32 + dx;
            let y = center_y as i32 + dy;

            if x < 0 || x >= map.width as i32 || y < 0 || y >= map.height as i32 {
                continue;
            }

            let distance = ((dx * dx + dy * dy) as f32).sqrt();
            let noise = rng.gen::<f32>() * 0.7 - 0.35; // Random variation
            let adjusted_radius = radius_f + noise;

            if distance <= adjusted_radius {
                let pos = Position::new(x, y);
                if let Some(tile) = map.get_tile_mut(pos) {
                    // Create ONLY plains - no hills or other terrain types yet
                    // Hills and other terrain will be added in later generation passes
                    *tile = MapTile {
                        terrain: TerrainType::Plains,
                        owner: None,
                        city: None,
                        resource: None,
                        movement_cost: 1.0,
                        defense_bonus: 0.0,
                    };
                }
            }
        }
    }
}

fn smooth_plain_landmasses(map: &mut WorldMap) {
    let changes = Vec::new();
    // Apply changes
    for (pos, new_terrain) in changes {
        if let Some(tile) = map.get_tile_mut(pos) {
            let (movement_cost, defense_bonus) = match new_terrain {
                TerrainType::Ocean => (terrain_stats::OCEAN_MOVEMENT_COST, terrain_stats::OCEAN_DEFENSE_BONUS),
                TerrainType::Plains => (terrain_stats::BASE_MOVEMENT_COST, terrain_stats::BASE_DEFENSE_BONUS),
                _ => (terrain_stats::BASE_MOVEMENT_COST, terrain_stats::BASE_DEFENSE_BONUS),
            };

            tile.terrain = new_terrain;
            tile.movement_cost = movement_cost;
            tile.defense_bonus = defense_bonus;
        }
    }
}

fn place_resources(map: &mut WorldMap, rng: &mut impl Rng) {
    let total_tiles = map.width * map.height;
    let resource_density = 0.15; // 15% of land tiles get resources
    let target_resources = (total_tiles as f32 * resource_density) as u32;

    let mut placed = 0;
    while placed < target_resources {
        let x = rng.gen_range(0..map.width);
        let y = rng.gen_range(0..map.height);
        let pos = Position::new(x as i32, y as i32);

        if let Some(tile) = map.get_tile_mut(pos) {
            if tile.resource.is_none() && !matches!(tile.terrain, TerrainType::Ocean) {
                tile.resource = Some(match tile.terrain {
                    TerrainType::Mountains => {
                        if rng.gen_bool(0.5) {
                            GameResource::Iron
                        } else {
                            GameResource::Stone
                        }
                    }
                    TerrainType::Hills => match rng.gen_range(0..3) {
                        0 => GameResource::Iron,
                        1 => GameResource::Gold,
                        _ => GameResource::Stone,
                    },
                    TerrainType::Plains => {
                        if rng.gen_bool(0.7) {
                            GameResource::Wheat
                        } else {
                            GameResource::Horses
                        }
                    }
                    TerrainType::Forest => GameResource::Wood,
                    TerrainType::Desert => {
                        if rng.gen_bool(0.3) {
                            GameResource::Gold
                        } else {
                            GameResource::Spices
                        }
                    }
                    TerrainType::Coast => GameResource::Fish,
                    TerrainType::ShallowCoast => GameResource::Fish,
                    TerrainType::River => GameResource::Fish,
                    TerrainType::Ocean => continue,
                });
                placed += 1;
            }
        }
    }
}
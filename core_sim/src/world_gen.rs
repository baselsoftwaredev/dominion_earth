use crate::{Position, WorldMap, TerrainType, resources::MapTile, resources::Resource as GameResource};
use rand::Rng;

/// Generate a basic Earth-like world map
pub fn generate_earth_map(width: u32, height: u32, rng: &mut impl Rng) -> WorldMap {
    let mut map = WorldMap::new(width, height);

    // Generate continents with simple noise
    for x in 0..width {
        for y in 0..height {
            let pos = Position::new(x as i32, y as i32);
            if let Some(tile) = map.get_tile_mut(pos) {
                *tile = generate_tile(x, y, width, height, rng);
            }
        }
    }

    // Add rivers connecting to coasts
    add_rivers(&mut map, rng);

    // Place strategic resources
    place_resources(&mut map, rng);

    map
}

fn generate_tile(x: u32, y: u32, width: u32, height: u32, rng: &mut impl Rng) -> MapTile {
    let fx = x as f32 / width as f32;
    let fy = y as f32 / height as f32;

    // Simple continent generation - create landmasses
    let distance_from_edge = {
        let dx = (fx - 0.5).abs();
        let dy = (fy - 0.5).abs();
        1.0 - (dx * dx + dy * dy).sqrt() * 2.0
    };

    let noise = rng.gen::<f32>() * 0.4 - 0.2; // -0.2 to 0.2
    let land_value = distance_from_edge + noise;

    let terrain = if land_value < 0.1 {
        TerrainType::Ocean
    } else if land_value < 0.15 {
        TerrainType::Coast
    } else {
        // Generate varied terrain for land
        let terrain_roll = rng.gen::<f32>();
        match terrain_roll {
            f if f < 0.4 => TerrainType::Plains,
            f if f < 0.55 => TerrainType::Hills,
            f if f < 0.65 => TerrainType::Forest,
            f if f < 0.75 => TerrainType::Mountains,
            _ => TerrainType::Desert,
        }
    };

    let (movement_cost, defense_bonus) = match terrain {
        TerrainType::Plains => (1.0, 0.0),
        TerrainType::Hills => (1.5, 0.25),
        TerrainType::Mountains => (2.0, 0.5),
        TerrainType::Forest => (1.5, 0.25),
        TerrainType::Desert => (1.5, 0.0),
        TerrainType::Coast => (1.0, 0.0),
        TerrainType::Ocean => (3.0, 0.0),
        TerrainType::River => (1.0, -0.25),
    };

    MapTile {
        terrain,
        owner: None,
        city: None,
        resource: None,
        movement_cost,
        defense_bonus,
    }
}

fn add_rivers(map: &mut WorldMap, rng: &mut impl Rng) {
    let river_count = (map.width * map.height / 500).max(3);

    for _ in 0..river_count {
        let start_x = rng.gen_range(map.width / 4..3 * map.width / 4);
        let start_y = rng.gen_range(map.height / 4..3 * map.height / 4);

        let mut current = Position::new(start_x as i32, start_y as i32);
        let river_length = rng.gen_range(5..15);

        for _ in 0..river_length {
            if let Some(tile) = map.get_tile_mut(current) {
                if !matches!(tile.terrain, TerrainType::Ocean | TerrainType::Coast) {
                    tile.terrain = TerrainType::River;
                    tile.movement_cost = 1.0;
                    tile.defense_bonus = -0.25;
                }
            }

            // Move river towards coast
            let neighbors = map.neighbors(current);
            if let Some(next) = neighbors.into_iter().find(|&pos| {
                map.get_tile(pos)
                    .map(|t| matches!(t.terrain, TerrainType::Coast | TerrainType::Ocean))
                    .unwrap_or(false)
            }) {
                current = next;
                break;
            } else if let Some(next) = map.neighbors(current).into_iter().next() {
                current = next;
            } else {
                break;
            }
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
                        if rng.gen_bool(0.5) { GameResource::Iron } else { GameResource::Stone }
                    }
                    TerrainType::Hills => {
                        match rng.gen_range(0..3) {
                            0 => GameResource::Iron,
                            1 => GameResource::Gold,
                            _ => GameResource::Stone,
                        }
                    }
                    TerrainType::Plains => {
                        if rng.gen_bool(0.7) { GameResource::Wheat } else { GameResource::Horses }
                    }
                    TerrainType::Forest => GameResource::Wood,
                    TerrainType::Desert => {
                        if rng.gen_bool(0.3) { GameResource::Gold } else { GameResource::Spices }
                    }
                    TerrainType::Coast => GameResource::Fish,
                    TerrainType::River => GameResource::Fish,
                    TerrainType::Ocean => continue,
                });
                placed += 1;
            }
        }
    }
}

/// Starting positions for civilizations based on real-world locations
pub fn get_starting_positions() -> Vec<(String, Position, [f32; 3])> {
    vec![
        ("Egypt".to_string(), Position::new(52, 25), [1.0, 0.8, 0.0]),
        ("Babylon".to_string(), Position::new(55, 23), [0.6, 0.4, 0.8]),
        ("Greece".to_string(), Position::new(50, 20), [0.0, 0.5, 1.0]),
        ("Rome".to_string(), Position::new(48, 19), [0.8, 0.2, 0.2]),
        ("Persia".to_string(), Position::new(58, 22), [0.5, 0.0, 0.5]),
        ("India".to_string(), Position::new(65, 25), [1.0, 0.5, 0.0]),
        ("China".to_string(), Position::new(75, 22), [1.0, 0.0, 0.0]),
        ("Japan".to_string(), Position::new(82, 20), [1.0, 1.0, 1.0]),
        ("Vikings".to_string(), Position::new(48, 12), [0.0, 0.8, 1.0]),
        ("England".to_string(), Position::new(45, 15), [0.0, 0.3, 0.6]),
        ("France".to_string(), Position::new(46, 17), [0.0, 0.0, 1.0]),
        ("Germany".to_string(), Position::new(48, 16), [0.3, 0.3, 0.3]),
        ("Russia".to_string(), Position::new(58, 14), [0.0, 0.5, 0.0]),
        ("Mongolia".to_string(), Position::new(70, 16), [0.6, 0.3, 0.0]),
        ("Aztecs".to_string(), Position::new(20, 24), [0.8, 0.8, 0.0]),
        ("Incas".to_string(), Position::new(25, 35), [0.5, 0.8, 0.3]),
        ("Maya".to_string(), Position::new(18, 26), [0.0, 0.8, 0.0]),
        ("Iroquois".to_string(), Position::new(30, 18), [0.4, 0.2, 0.0]),
        ("Mali".to_string(), Position::new(45, 30), [0.8, 0.5, 0.0]),
        ("Ethiopia".to_string(), Position::new(54, 32), [0.6, 0.8, 0.2]),
        ("Zulu".to_string(), Position::new(52, 42), [0.2, 0.0, 0.0]),
        ("Arabs".to_string(), Position::new(56, 26), [0.0, 0.6, 0.0]),
        ("Ottomans".to_string(), Position::new(52, 20), [0.8, 0.0, 0.0]),
        ("Korea".to_string(), Position::new(80, 19), [0.8, 0.8, 0.8]),
        ("Siam".to_string(), Position::new(72, 28), [1.0, 0.0, 0.5]),
        ("Khmer".to_string(), Position::new(73, 29), [0.5, 0.0, 0.5]),
        ("Indonesia".to_string(), Position::new(76, 33), [0.0, 0.5, 0.0]),
        ("Australia".to_string(), Position::new(85, 42), [0.0, 0.0, 0.5]),
        ("Polynesia".to_string(), Position::new(95, 35), [0.0, 0.8, 0.8]),
        ("Portugal".to_string(), Position::new(43, 19), [0.0, 0.5, 0.0]),
        ("Spain".to_string(), Position::new(44, 20), [0.8, 0.8, 0.0]),
        ("Netherlands".to_string(), Position::new(47, 15), [1.0, 0.5, 0.0]),
        ("Poland".to_string(), Position::new(50, 15), [1.0, 1.0, 1.0]),
        ("Sweden".to_string(), Position::new(49, 11), [0.0, 0.3, 0.8]),
        ("Brazil".to_string(), Position::new(28, 38), [0.0, 1.0, 0.0]),
        ("Argentina".to_string(), Position::new(26, 45), [0.5, 0.8, 1.0]),
        ("Canada".to_string(), Position::new(28, 10), [1.0, 0.0, 0.0]),
        ("America".to_string(), Position::new(25, 20), [0.0, 0.0, 1.0]),
        ("Mexico".to_string(), Position::new(22, 24), [0.0, 0.5, 0.3]),
        ("Morocco".to_string(), Position::new(44, 24), [0.8, 0.0, 0.0]),
    ]
}

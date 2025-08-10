use bevy::prelude::*;
use core_sim::*;

#[derive(Resource)]
pub struct TileAssets {
    pub plains: Handle<Image>,
    pub ocean: Handle<Image>,
    pub capital_ancient: Handle<Image>,
    // Add more tile types as you create them
}

#[derive(Component)]
pub struct WorldTile {
    pub grid_pos: Position,
    pub terrain_type: TerrainType,
}

/// Load tile assets
pub fn setup_tile_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_assets = TileAssets {
        plains: asset_server.load("tiles/land/plains_tile.png"),
        ocean: asset_server.load("tiles/land/ocean_tile.png"),
        capital_ancient: asset_server.load("tiles/settlement/capital_ancient_age.png"),
        // Add more tiles here as you create them
    };
    commands.insert_resource(tile_assets);
}

/// Spawn world tiles as sprite entities
pub fn spawn_world_tiles(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    tile_assets: Res<TileAssets>,
) {
    let tile_size = 64.0; // Match your 64x64 tile images
    let map_offset = Vec2::new(-3200.0, -1600.0); // Adjusted for 64px tiles

    // Spawn tile sprites (render every tile for no gaps)
    for x in 0..world_map.width {
        for y in 0..world_map.height {
            let world_pos = Position::new(x as i32, y as i32);
            if let Some(tile) = world_map.get_tile(world_pos) {
                let screen_pos = map_offset + Vec2::new(x as f32 * tile_size, y as f32 * tile_size);

                // Choose texture based on terrain type
                let texture = match tile.terrain {
                    TerrainType::Plains => tile_assets.plains.clone(),
                    TerrainType::Hills | TerrainType::Forest => tile_assets.plains.clone(), // Use plains as placeholder
                    TerrainType::Ocean | TerrainType::Coast => tile_assets.ocean.clone(),
                    // Use plains as fallback for other types (Mountains, Desert, River)
                    _ => tile_assets.plains.clone(),
                };

                commands.spawn((
                    Sprite::from_image(texture),
                    Transform::from_translation(screen_pos.extend(0.0))
                        .with_scale(Vec3::splat(1.0)),
                    WorldTile {
                        grid_pos: world_pos,
                        terrain_type: tile.terrain.clone(),
                    },
                ));
            }
        }
    }
}

/// Generate a unique color for each civilization based on their ID
fn get_civ_color(civ_id: &CivId) -> Color {
    // Simple hash-based color generation for consistent colors per civilization
    let hash = civ_id.0.wrapping_mul(31);

    // Convert hash to HSV for better color distribution
    let hue = (hash % 360) as f32;
    let saturation = 0.7;
    let value = 0.9;

    // Convert HSV to RGB
    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = match hue as u32 / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color::srgb(r + m, g + m, b + m)
}

/// Simple 2D rendering system for civilizations and units (keeping this for overlays)
pub fn render_world_overlays(
    mut commands: Commands,
    mut gizmos: Gizmos,
    world_map: Res<WorldMap>,
    tile_assets: Res<TileAssets>,
    civs: Query<(&Civilization, &Position)>,
    cities: Query<(&City, &Position)>,
    units: Query<(&MilitaryUnit, &Position)>,
    camera: Query<&Transform, With<Camera>>,
) {
    // Get camera position for world-to-screen calculations
    let camera_transform = camera.single();
    let _camera_pos = camera_transform.translation.truncate();

    // Define rendering parameters
    let tile_size = 64.0; // Match your 64x64 tile images
    let map_offset = Vec2::new(-3200.0, -1600.0); // Adjusted for 64px tiles

    // Render civilization capitals
    for (civilization, position) in civs.iter() {
        let screen_pos =
            map_offset + Vec2::new(position.x as f32 * tile_size, position.y as f32 * tile_size);

        // Spawn a sprite for the capital city tile art
        commands.spawn((
            Sprite::from_image(tile_assets.capital_ancient.clone()),
            Transform::from_translation(screen_pos.extend(10.0)) // Z=10 to render above terrain
                .with_scale(Vec3::splat(1.0)),
        ));

        // Draw a colored circle around the capital for this civilization
        let civ_color = get_civ_color(&civilization.id);
        gizmos.circle_2d(screen_pos, tile_size * 0.8, civ_color);
    }

    // Render cities
    for (city, position) in cities.iter() {
        let screen_pos =
            map_offset + Vec2::new(position.x as f32 * tile_size, position.y as f32 * tile_size);

        // City population indicator (circle to match capitals)
        let pop_size = (city.population as f32 / 5000.0).clamp(0.5, 2.0);
        gizmos.circle_2d(
            screen_pos,
            tile_size * pop_size,
            Color::srgb(1.0, 1.0, 0.0),
        ); // Yellow circle for cities
    }

    // Render military units
    for (unit, position) in units.iter() {
        let screen_pos =
            map_offset + Vec2::new(position.x as f32 * tile_size, position.y as f32 * tile_size);

        let unit_color = match unit.unit_type {
            UnitType::Infantry => Color::srgb(0.5, 0.0, 0.0), // Maroon
            UnitType::Cavalry => Color::srgb(0.5, 0.0, 0.5),  // Purple
            UnitType::Archer => Color::srgb(1.0, 0.27, 0.0),  // Orange Red
            UnitType::Siege => Color::srgb(0.33, 0.33, 0.33), // Dark Gray
            UnitType::Naval => Color::srgb(0.0, 0.0, 0.5),    // Navy
        };

        // Draw unit as a small diamond
        let diamond_size = tile_size * 0.8;
        let diamond_points = [
            screen_pos + Vec2::new(0.0, diamond_size),
            screen_pos + Vec2::new(diamond_size, 0.0),
            screen_pos + Vec2::new(0.0, -diamond_size),
            screen_pos + Vec2::new(-diamond_size, 0.0),
        ];

        for i in 0..4 {
            let next_i = (i + 1) % 4;
            gizmos.line_2d(diamond_points[i], diamond_points[next_i], unit_color);
        }

        // Unit strength indicator
        let strength_radius = (unit.strength / 20.0).clamp(1.0, 5.0);
        gizmos.circle_2d(screen_pos, strength_radius, unit_color);
    }

    // Render grid lines (optional, for debugging)
    if false {
        // Set to true to show grid
        for x in (0..world_map.width).step_by(10) {
            let start = map_offset + Vec2::new(x as f32 * tile_size, 0.0);
            let end =
                map_offset + Vec2::new(x as f32 * tile_size, world_map.height as f32 * tile_size);
            gizmos.line_2d(start, end, Color::srgba(1.0, 1.0, 1.0, 0.2));
        }

        for y in (0..world_map.height).step_by(5) {
            let start = map_offset + Vec2::new(0.0, y as f32 * tile_size);
            let end =
                map_offset + Vec2::new(world_map.width as f32 * tile_size, y as f32 * tile_size);
            gizmos.line_2d(start, end, Color::srgba(1.0, 1.0, 1.0, 0.2));
        }
    }
}

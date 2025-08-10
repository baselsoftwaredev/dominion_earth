use bevy::prelude::*;
use core_sim::*;

/// Simple 2D rendering system for the world map
pub fn render_world(
    mut gizmos: Gizmos,
    world_map: Res<WorldMap>,
    civs: Query<(&Civilization, &Position)>,
    cities: Query<(&City, &Position)>,
    units: Query<(&MilitaryUnit, &Position)>,
    camera: Query<&Transform, With<Camera>>,
) {
    // Get camera position for world-to-screen calculations
    let camera_transform = camera.single();
    let camera_pos = camera_transform.translation.truncate();

    // Define rendering parameters
    let tile_size = 8.0;
    let map_offset = Vec2::new(-400.0, -200.0); // Center the map

    // Render world tiles (simplified - only show major features)
    for x in (0..world_map.width).step_by(2) {
        for y in (0..world_map.height).step_by(2) {
            let world_pos = Position::new(x as i32, y as i32);
            if let Some(tile) = world_map.get_tile(world_pos) {
                let screen_pos = map_offset + Vec2::new(x as f32 * tile_size, y as f32 * tile_size);
                
                let color = match tile.terrain {
                    TerrainType::Ocean => Color::srgb(0.0, 0.0, 1.0),        // Blue
                    TerrainType::Plains => Color::srgb(0.0, 1.0, 0.0),       // Green
                    TerrainType::Hills => Color::srgb(0.6, 0.8, 0.4),        // Light Green
                    TerrainType::Mountains => Color::srgb(0.5, 0.5, 0.5),    // Gray
                    TerrainType::Forest => Color::srgb(0.0, 0.5, 0.0),       // Dark Green
                    TerrainType::Desert => Color::srgb(1.0, 1.0, 0.0),       // Yellow
                    TerrainType::Coast => Color::srgb(0.0, 1.0, 1.0),        // Cyan
                    TerrainType::River => Color::srgb(0.0, 1.0, 1.0),        // Cyan
                };

                // Draw tile as a small rectangle
                gizmos.rect_2d(screen_pos, Vec2::splat(tile_size * 0.8), color);

                // Show ownership
                if let Some(_owner) = tile.owner {
                    gizmos.rect_2d(screen_pos, Vec2::splat(tile_size * 0.6), Color::srgb(1.0, 0.0, 0.0)); // Red
                }
            }
        }
    }

    // Render civilizations (capitals)
    for (civilization, position) in civs.iter() {
        let screen_pos = map_offset + Vec2::new(position.x as f32 * tile_size, position.y as f32 * tile_size);
        
        let civ_color = Color::srgb(
            civilization.color[0],
            civilization.color[1],
            civilization.color[2],
        );

        // Draw capital as a larger circle
        gizmos.circle_2d(screen_pos, tile_size * 2.0, civ_color);
        gizmos.circle_2d(screen_pos, tile_size * 1.5, Color::WHITE);
    }

    // Render cities
    for (city, position) in cities.iter() {
        let screen_pos = map_offset + Vec2::new(position.x as f32 * tile_size, position.y as f32 * tile_size);
        
        // Draw city as a square
        gizmos.rect_2d(screen_pos, Vec2::splat(tile_size * 1.5), Color::srgb(1.0, 0.5, 0.0)); // Orange
        
        // City population indicator
        let pop_size = (city.population as f32 / 5000.0).clamp(0.5, 3.0);
        gizmos.rect_2d(screen_pos, Vec2::splat(tile_size * pop_size), Color::srgb(1.0, 1.0, 0.0)); // Yellow
    }

    // Render military units
    for (unit, position) in units.iter() {
        let screen_pos = map_offset + Vec2::new(position.x as f32 * tile_size, position.y as f32 * tile_size);
        
        let unit_color = match unit.unit_type {
            UnitType::Infantry => Color::srgb(0.5, 0.0, 0.0),    // Maroon
            UnitType::Cavalry => Color::srgb(0.5, 0.0, 0.5),     // Purple
            UnitType::Archer => Color::srgb(1.0, 0.27, 0.0),     // Orange Red
            UnitType::Siege => Color::srgb(0.33, 0.33, 0.33),    // Dark Gray
            UnitType::Naval => Color::srgb(0.0, 0.0, 0.5),       // Navy
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
    if false { // Set to true to show grid
        for x in (0..world_map.width).step_by(10) {
            let start = map_offset + Vec2::new(x as f32 * tile_size, 0.0);
            let end = map_offset + Vec2::new(x as f32 * tile_size, world_map.height as f32 * tile_size);
            gizmos.line_2d(start, end, Color::srgba(1.0, 1.0, 1.0, 0.2));
        }
        
        for y in (0..world_map.height).step_by(5) {
            let start = map_offset + Vec2::new(0.0, y as f32 * tile_size);
            let end = map_offset + Vec2::new(world_map.width as f32 * tile_size, y as f32 * tile_size);
            gizmos.line_2d(start, end, Color::srgba(1.0, 1.0, 1.0, 0.2));
        }
    }
}

use core_sim::tile::tile_components::{WorldTile, DefaultViewPoint};
/// System to rotate coast tile sprites based on their facing direction
pub fn rotate_coast_tiles(
    mut query: Query<(&WorldTile, &mut Transform, &TileTextureIndex)>,
) {
    for (world_tile, mut transform, texture_index) in query.iter_mut() {
        // Only rotate coast tiles
        if world_tile.terrain_type == core_sim::TerrainType::Coast {
            let angle = match world_tile.default_view_point {
                DefaultViewPoint::North => 0.0,
                DefaultViewPoint::East => -std::f32::consts::FRAC_PI_2,
                DefaultViewPoint::South => std::f32::consts::PI,
                DefaultViewPoint::West => std::f32::consts::FRAC_PI_2,
                DefaultViewPoint::NorthEast => -std::f32::consts::FRAC_PI_4,
                DefaultViewPoint::SouthEast => -3.0 * std::f32::consts::FRAC_PI_4,
                DefaultViewPoint::SouthWest => 3.0 * std::f32::consts::FRAC_PI_4,
                DefaultViewPoint::NorthWest => std::f32::consts::FRAC_PI_4,
            };
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::tile::tile_assets::{setup_tile_assets, TileAssets};
use core_sim::*;

#[derive(Resource, Clone)]
pub struct TilemapIdResource(pub TilemapId);

/// Setup isometric diamond tilemap using bevy_ecs_tilemap
pub fn setup_tilemap(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    world_map: Res<WorldMap>,
) {
    // Create tilemap entity early - we need its ID for tile references
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    // Store the tilemap ID as a resource for other systems to access
    commands.insert_resource(TilemapIdResource(tilemap_id));

    // Use core_sim's setup_world_tiles to create tile entities and neighbors
    let tile_storage = core_sim::tile::tile_components::setup_world_tiles(
        &mut commands,
        tilemap_id,
        &*tile_assets,
        &*world_map,
    );

    // Configure tilemap for square rendering (can switch to isometric later)
    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = TilemapGridSize { x: 64.0, y: 64.0 };
    let map_type = TilemapType::Square;

    // Add the tilemap bundle to the tilemap entity
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: TilemapSize {
            x: world_map.width,
            y: world_map.height,
        },
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_assets.sprite_sheet.clone()),
        tile_size,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

/// Spawn a unit/capital/city as a child of the correct tile entity in the tilemap
pub fn spawn_entity_on_tile(
    mut commands: Commands,
    tilemap_query: Query<&TileStorage>,
    tilemap_id: TilemapId,
    tile_assets: &TileAssets,
    sprite_index: usize,
    position: Position,
) {
    let tile_pos = TilePos {
        x: position.x as u32,
        y: position.y as u32,
    };
    if let Ok(tile_storage) = tilemap_query.get(tilemap_id.0) {
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            commands.entity(tile_entity).with_children(|parent| {
                parent.spawn((
                    Sprite::from_atlas_image(
                        tile_assets.sprite_sheet.clone(),
                        TextureAtlas {
                            layout: tile_assets.texture_atlas_layout.clone(),
                            index: sprite_index,
                        },
                    ),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // Slightly above the tile
                ));
            });
        }
    }
}

/// System to spawn all world tiles using the new tilemap setup
pub fn spawn_world_tiles(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    world_map: Res<WorldMap>,
) {
    setup_tilemap(commands, tile_assets, world_map);
}

/// System to spawn all unit sprites on their respective tiles
pub fn spawn_unit_sprites(
    mut commands: Commands,
    tilemap_query: Query<&TileStorage>,
    tilemap_id: Res<TilemapIdResource>,
    tile_assets: Res<TileAssets>,
    units: Query<(Entity, &Position), With<core_sim::components::MilitaryUnit>>,
) {
    for (_, position) in units.iter() {
        spawn_entity_on_tile(
            commands.reborrow(),
            tilemap_query,
            tilemap_id.0,
            &tile_assets,
            tile_assets.ancient_infantry_index,
            *position,
        );
    }
}

/// System to spawn all capital sprites on their respective tiles
pub fn spawn_capital_sprites(
    mut commands: Commands,
    tilemap_query: Query<&TileStorage>,
    tilemap_id: Res<TilemapIdResource>,
    tile_assets: Res<TileAssets>,
    civs: Query<(Entity, &core_sim::components::Civilization)>,
) {
    for (_, civ) in civs.iter() {
        if let Some(pos) = civ.capital {
            spawn_entity_on_tile(
                commands.reborrow(),
                tilemap_query,
                tilemap_id.0,
                &tile_assets,
                tile_assets.capital_ancient_index,
                pos,
            );
        }
    }
}

/// System to update unit sprites (stub for future logic)
pub fn update_unit_sprites() {
    // Implement logic to update unit sprites if needed
}

/// System to render overlays (stub for future logic)
pub fn render_world_overlays() {
    // Implement logic to render overlays if needed
}

// Example usage for units, capitals, cities:
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, unit_assets.ancient_infantry.clone(), unit_position, 1.0);
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, tile_assets.capital_ancient.clone(), capital_position, 2.0);
// spawn_entity_on_tile(commands, tilemap_query, tilemap_id, city_asset, city_position, 3.0);

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

    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::srgb(r + m, g + m, b + m)
}

//! Settlement sprite rendering system
//!
//! This module handles loading and rendering sprites for settlements (capitals, cities, etc.)

use super::common::calculate_world_position_for_gizmo;
use crate::screens::Screen;
use bevy::prelude::*;
use crate::debug_println;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::city::Capital;
use core_sim::constants::{sprite_indices, texture_atlas};
use core_sim::Position;

/// Component that links a sprite to its settlement
#[derive(Component, Debug)]
pub struct SettlementSpriteLink {
    pub settlement_entity: Entity,
}

/// Resource that holds the sprite sheet texture atlas layout
#[derive(Resource)]
pub struct SettlementSpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

/// Plugin that sets up settlement sprite sheet loading and rendering
pub struct SettlementSpritePlugin;

impl Plugin for SettlementSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_settlement_sprite_sheet)
            .add_systems(
                Update,
                (
                    spawn_settlement_sprites,
                    update_settlement_sprite_positions,
                    despawn_settlement_sprites,
                ),
            );
    }
}

/// System that loads the sprite sheet on startup
fn load_settlement_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    debug_println!("Loading settlement sprite sheet from: {}", texture_atlas::SPRITE_SHEET_PATH);

    // Load the texture
    let texture = asset_server.load(texture_atlas::SPRITE_SHEET_PATH);

    // Create the texture atlas layout
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(texture_atlas::TILE_SIZE_PIXELS),
        texture_atlas::ATLAS_COLUMNS,
        texture_atlas::ATLAS_ROWS,
        None,
        None,
    );

    let layout_handle = texture_atlas_layouts.add(layout);

    // Store as a resource
    commands.insert_resource(SettlementSpriteSheet {
        texture,
        layout: layout_handle,
    });

    debug_println!("Settlement sprite sheet loaded successfully");
}

/// System that spawns sprites for newly added capitals
fn spawn_settlement_sprites(
    mut commands: Commands,
    sprite_sheet: Option<Res<SettlementSpriteSheet>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    // Query for capitals that don't have sprites yet
    capital_query: Query<(Entity, &Capital, &Position), Without<SettlementSpriteLink>>,
) {
    // Wait until sprite sheet is loaded
    let Some(sprite_sheet) = sprite_sheet else {
        debug_println!("Settlement sprite sheet not loaded yet, skipping spawn");
        return;
    };

    // Wait until tilemap is loaded
    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single() else {
        debug_println!("Tilemap not ready for settlement sprites, skipping spawn");
        return;
    };

    let count = capital_query.iter().count();
    if count > 0 {
        debug_println!("Found {} capitals to spawn sprites for", count);
    }

    for (settlement_entity, capital, position) in capital_query.iter() {
        debug_println!(
            "Spawning capital sprite for settlement at position ({}, {})",
            position.x, position.y
        );

        // Calculate world position using the same method as gizmos
        let world_pos = calculate_world_position_for_gizmo(
            *position, map_size, tile_size, grid_size, map_type, anchor,
        );

        // Spawn the sprite entity as a standalone entity
        let sprite_entity = commands
            .spawn((
                Sprite::from_atlas_image(
                    sprite_sheet.texture.clone(),
                    TextureAtlas {
                        layout: sprite_sheet.layout.clone(),
                        index: sprite_indices::CAPITAL_ANCIENT,
                    },
                ),
                Transform::from_translation(world_pos).with_scale(Vec3::splat(0.5)),
                GlobalTransform::default(),
                Visibility::Visible,
                DespawnOnExit(Screen::Gameplay),
            ))
            .id();

        // Link sprite to settlement with a marker component
        commands
            .entity(settlement_entity)
            .insert(SettlementSpriteLink {
                settlement_entity: sprite_entity,
            });

        debug_println!("Capital sprite spawned for settlement entity");
        debug_println!("Sprite entity ID: {:?}", sprite_entity);
        debug_println!("Settlement entity ID: {:?}", settlement_entity);
        debug_println!("Sprite world position: {:?}", world_pos);
        debug_println!(
            "Sprite texture: {:?}, Layout: {:?}, Index: {}",
            sprite_sheet.texture,
            sprite_sheet.layout,
            sprite_indices::CAPITAL_ANCIENT
        );
    }
}

/// System that updates sprite positions when settlements move
fn update_settlement_sprite_positions(
    mut sprite_transforms: Query<&mut Transform>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    // Query for capitals that have moved
    changed_settlements: Query<(&Position, &SettlementSpriteLink), Changed<Position>>,
) {
    // Wait until tilemap is loaded
    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single() else {
        return;
    };

    for (position, sprite_link) in changed_settlements.iter() {
        debug_println!(
            "Settlement at position ({}, {}), updating sprite",
            position.x, position.y
        );

        // Calculate new world position
        let world_pos = calculate_world_position_for_gizmo(
            *position, map_size, tile_size, grid_size, map_type, anchor,
        );

        // Update sprite transform
        if let Ok(mut transform) = sprite_transforms.get_mut(sprite_link.settlement_entity) {
            let old_pos = transform.translation;
            transform.translation = world_pos;
            debug_println!(
                "Updated settlement sprite position from {:?} to {:?}",
                old_pos, world_pos
            );
        } else {
            debug_println!(
                "Could not find sprite entity {:?} for update",
                sprite_link.settlement_entity
            );
        }
    }
}

/// System that despawns sprites when settlements are removed
fn despawn_settlement_sprites(
    mut commands: Commands,
    mut removed_settlements: RemovedComponents<Capital>,
    sprite_links: Query<&SettlementSpriteLink>,
) {
    for settlement_entity in removed_settlements.read() {
        // Check if this settlement had a linked sprite
        if let Ok(sprite_link) = sprite_links.get(settlement_entity) {
            debug_println!(
                "Despawning sprite for removed settlement entity: {:?}",
                settlement_entity
            );
            // Despawn the sprite entity
            commands.entity(sprite_link.settlement_entity).despawn();
        }
    }
}

//! Unit sprite rendering system
//!
//! This module handles loading the sprite sheet and rendering sprites for military units.

use super::common::calculate_world_position_for_gizmo;
use crate::screens::Screen;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::military::{MilitaryUnit, UnitType};
use core_sim::constants::{sprite_indices, texture_atlas};
use core_sim::Position;

/// Component that links a sprite to its unit
#[derive(Component, Debug)]
pub struct UnitSpriteLink {
    pub unit_entity: Entity,
}

/// Resource that holds the sprite sheet texture atlas layout
#[derive(Resource)]
pub struct UnitSpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

/// Plugin that sets up sprite sheet loading and unit sprite spawning
pub struct UnitSpritePlugin;

impl Plugin for UnitSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sprite_sheet).add_systems(
            Update,
            (
                spawn_infantry_sprites.run_if(in_state(Screen::Gameplay)),
                update_infantry_sprite_positions.run_if(in_state(Screen::Gameplay)),
                despawn_unit_sprites.run_if(in_state(Screen::Gameplay)),
            ),
        );
    }
}

/// System that loads the sprite sheet on startup
fn load_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!(
        "Loading sprite sheet from: {}",
        texture_atlas::SPRITE_SHEET_PATH
    );

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
    commands.insert_resource(UnitSpriteSheet {
        texture,
        layout: layout_handle,
    });

    info!("Sprite sheet loaded successfully");
}

/// System that spawns sprites for newly added infantry units
fn spawn_infantry_sprites(
    mut commands: Commands,
    sprite_sheet: Option<Res<UnitSpriteSheet>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    // Query for infantry units that don't have sprites yet
    infantry_query: Query<(Entity, &MilitaryUnit), Without<UnitSpriteLink>>,
) {
    // Wait until sprite sheet is loaded
    let Some(sprite_sheet) = sprite_sheet else {
        info!("Sprite sheet not loaded yet, skipping spawn");
        return;
    };

    // Wait until tilemap is loaded
    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single() else {
        info!("Tilemap not ready, skipping spawn");
        return;
    };

    let count = infantry_query.iter().count();
    if count > 0 {
        info!("Found {} infantry units to spawn sprites for", count);
    }

    for (unit_entity, military_unit) in infantry_query.iter() {
        // Only spawn sprites for infantry units
        if military_unit.unit_type != UnitType::Infantry {
            continue;
        }

        info!(
            "Spawning infantry sprite for unit {} at position ({}, {})",
            military_unit.id, military_unit.position.x, military_unit.position.y
        );

        // Calculate world position using the same method as gizmos
        let world_pos = calculate_world_position_for_gizmo(
            military_unit.position,
            map_size,
            tile_size,
            grid_size,
            map_type,
            anchor,
        );

        // Spawn the sprite entity as a standalone child of the unit
        let sprite_entity = commands
            .spawn((
                Sprite::from_atlas_image(
                    sprite_sheet.texture.clone(),
                    TextureAtlas {
                        layout: sprite_sheet.layout.clone(),
                        index: sprite_indices::ANCIENT_INFANTRY,
                    },
                ),
                Transform::from_translation(world_pos).with_scale(Vec3::splat(0.5)),
                GlobalTransform::default(),
                Visibility::Visible,
                DespawnOnExit(Screen::Gameplay),
            ))
            .id();

        // Link sprite to unit with a marker component
        commands.entity(unit_entity).insert(UnitSpriteLink {
            unit_entity: sprite_entity,
        });

        info!("Infantry sprite spawned for unit entity");
        info!("Sprite entity ID: {:?}", sprite_entity);
        info!("Unit entity ID: {:?}", unit_entity);
        info!("Sprite world position: {:?}", world_pos);
        info!(
            "Sprite texture: {:?}, Layout: {:?}, Index: {}",
            sprite_sheet.texture,
            sprite_sheet.layout,
            sprite_indices::ANCIENT_INFANTRY
        );
    }
}

/// System that updates sprite positions when infantry units move
fn update_infantry_sprite_positions(
    mut sprite_transforms: Query<&mut Transform>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    // Query for infantry units that have moved - check BOTH Position and MilitaryUnit changes
    changed_infantry: Query<
        (&MilitaryUnit, &Position, &UnitSpriteLink),
        Or<(Changed<MilitaryUnit>, Changed<Position>)>,
    >,
) {
    // Wait until tilemap is loaded
    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single() else {
        return;
    };

    for (military_unit, position, sprite_link) in changed_infantry.iter() {
        // Only update sprites for infantry units
        if military_unit.unit_type != UnitType::Infantry {
            continue;
        }

        info!(
            "Infantry unit {} at position ({}, {}), updating sprite",
            military_unit.id, position.x, position.y
        );

        // Calculate new world position
        let world_pos = calculate_world_position_for_gizmo(
            *position, map_size, tile_size, grid_size, map_type, anchor,
        );

        // Update sprite transform
        if let Ok(mut transform) = sprite_transforms.get_mut(sprite_link.unit_entity) {
            let old_pos = transform.translation;
            transform.translation = world_pos;
            info!(
                "Updated sprite position from {:?} to {:?}",
                old_pos, world_pos
            );
        } else {
            info!(
                "Could not find sprite entity {:?} for update",
                sprite_link.unit_entity
            );
        }
    }
}

/// System that despawns sprites when their linked units are removed
fn despawn_unit_sprites(
    mut commands: Commands,
    mut removed_units: RemovedComponents<MilitaryUnit>,
    sprite_links: Query<&UnitSpriteLink>,
) {
    for unit_entity in removed_units.read() {
        // Check if this unit had a linked sprite
        if let Ok(sprite_link) = sprite_links.get(unit_entity) {
            info!(
                "Despawning sprite for removed unit entity: {:?}",
                unit_entity
            );
            // Despawn the sprite entity
            commands.entity(sprite_link.unit_entity).despawn();
        }
    }
}

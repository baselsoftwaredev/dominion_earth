//! Unit sprite rendering system
//!
//! This module handles loading the sprite sheet and rendering sprites for military units.

use bevy::prelude::*;
use core_sim::components::military::{MilitaryUnit, UnitType};
use core_sim::components::rendering::SpriteEntityReference;
use core_sim::constants::{sprite_indices, texture_atlas};

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
        app.add_systems(Startup, load_sprite_sheet)
            .add_systems(Update, (spawn_infantry_sprites, despawn_unit_sprites));
    }
}

/// System that loads the sprite sheet on startup
fn load_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading sprite sheet from: {}", texture_atlas::SPRITE_SHEET_PATH);
    
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
    // Query for infantry units that don't have a sprite reference yet
    infantry_query: Query<(Entity, &MilitaryUnit), (Without<SpriteEntityReference>, Added<MilitaryUnit>)>,
) {
    // Wait until sprite sheet is loaded
    let Some(sprite_sheet) = sprite_sheet else {
        return;
    };
    
    for (unit_entity, military_unit) in infantry_query.iter() {
        // Only spawn sprites for infantry units
        if military_unit.unit_type != UnitType::Infantry {
            continue;
        }
        
        info!(
            "Spawning infantry sprite for unit {} at position ({}, {})",
            military_unit.id, military_unit.position.x, military_unit.position.y
        );
        
        // Spawn the sprite entity
        let sprite_entity = commands.spawn((
            Sprite::from_atlas_image(
                sprite_sheet.texture.clone(),
                TextureAtlas {
                    layout: sprite_sheet.layout.clone(),
                    index: sprite_indices::ANCIENT_INFANTRY,
                },
            ),
            Transform::from_xyz(
                military_unit.position.x as f32 * texture_atlas::TILE_SIZE_PIXELS as f32,
                military_unit.position.y as f32 * texture_atlas::TILE_SIZE_PIXELS as f32,
                1.0, // Z-layer for units
            ),
        )).id();
        
        // Link the sprite to the unit
        commands.entity(unit_entity).insert(SpriteEntityReference::create_new_reference(sprite_entity));
        
        info!("Infantry sprite spawned and linked to unit entity");
    }
}

/// System that despawns sprites when their linked units are removed
fn despawn_unit_sprites(
    mut commands: Commands,
    mut removed_units: RemovedComponents<MilitaryUnit>,
    sprite_refs: Query<&SpriteEntityReference>,
) {
    for unit_entity in removed_units.read() {
        // Check if this entity had a sprite reference
        if let Ok(sprite_ref) = sprite_refs.get(unit_entity) {
            info!("Despawning sprite for removed unit entity: {:?}", unit_entity);
            
            // Despawn the sprite entity
            commands.entity(sprite_ref.sprite_entity).despawn();
        }
    }
}

//! Settlement sprite rendering system
//!
//! This module handles loading and rendering sprites for settlements (capitals, cities, etc.)

use super::common::calculate_world_position_for_gizmo;
use crate::debug_println;
use crate::screens::Screen;
use crate::theme::constants::font_sizes;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::components::city::Capital;
use core_sim::components::city::City;
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

/// Component that links a label entity to its settlement
#[derive(Component, Debug)]
pub struct SettlementLabelLink {
    pub label_entity: Entity,
}

/// Container marker for settlement label UI in world space
#[derive(Component, Debug)]
pub struct SettlementLabelContainer;

/// Colors for the label box / icon / text
#[derive(Component, Clone, Copy, Debug)]
pub struct SettlementLabelColors {
    pub background: Color,
    pub icon: Color,
    pub text: Color,
}

pub mod label_constants {
    pub const LABEL_FONT_SIZE: f32 = 14.0;
    pub const LABEL_Z_INDEX: f32 = 50.0;
    pub const LABEL_VERTICAL_OFFSET_PIXELS: f32 = 8.0;
    pub const LABEL_BOX_WIDTH: f32 = 120.0;
    pub const LABEL_BOX_HEIGHT: f32 = 28.0;
    pub const ICON_SIZE: f32 = 16.0;
    pub const ICON_PADDING: f32 = 4.0;
    pub const TEXT_PADDING: f32 = 6.0;
}

/// Logical label component containing the label text. Rendering is done by the
/// frontend; we keep the transform on a simple entity so other systems can
/// pick it up and actually draw text if desired.
#[derive(Component, Debug, Clone)]
pub struct SettlementLabel {
    pub text: String,
}

/// Plugin that sets up settlement sprite sheet loading and rendering
pub struct SettlementSpritePlugin;

impl Plugin for SettlementSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_settlement_sprite_sheet)
            .add_systems(
                Update,
                (
                    spawn_settlement_sprites.run_if(in_state(Screen::Gameplay)),
                    spawn_settlement_labels.run_if(in_state(Screen::Gameplay)),
                    update_settlement_sprite_positions.run_if(in_state(Screen::Gameplay)),
                    despawn_settlement_sprites.run_if(in_state(Screen::Gameplay)),
                ),
            );
    }
}

fn create_label_text_color() -> Color {
    Color::WHITE
}

fn create_label_colors(civilization_color: [f32; 3]) -> SettlementLabelColors {
    SettlementLabelColors {
        background: Color::srgba(
            civilization_color[0] * 0.5,
            civilization_color[1] * 0.5,
            civilization_color[2] * 0.5,
            0.85,
        ),
        icon: Color::srgba(
            civilization_color[0],
            civilization_color[1],
            civilization_color[2],
            1.0,
        ),
        text: create_label_text_color(),
    }
}

/// System that loads the sprite sheet on startup
fn load_settlement_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    debug_println!(
        "Loading settlement sprite sheet from: {}",
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
            position.x,
            position.y
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

        // NOTE: label creation is handled in a separate system `spawn_settlement_labels` so we
        // keep sprite spawning focused. That system will create a text label entity and link it
        // back to the settlement.

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
            position.x,
            position.y
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
                old_pos,
                world_pos
            );
        } else {
            debug_println!(
                "Could not find sprite entity {:?} for update",
                sprite_link.settlement_entity
            );
        }
    }
}

/// System that spawns text labels for settlements (center-top of tile)
fn spawn_settlement_labels(
    mut commands: Commands,
    sprite_sheet: Option<Res<SettlementSpriteSheet>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    // Query for capitals that don't have labels yet; City may be absent so use Option
    capital_query: Query<
        (Entity, &Capital, &Position, Option<&City>),
        Without<SettlementLabelLink>,
    >,
    civilizations_query: Query<&core_sim::components::civilization::Civilization>,
) {
    let Some(sprite_sheet) = sprite_sheet else {
        debug_println!("Settlement sprite sheet (and font) not loaded yet, skipping label spawn");
        return;
    };

    let Ok((map_size, tile_size, grid_size, map_type, anchor)) = tilemap_q.single() else {
        debug_println!("Tilemap not ready for settlement labels, skipping spawn");
        return;
    };

    for (settlement_entity, _capital, position, city_opt) in capital_query.iter() {
        // Determine civilization color and civ name if available (used as fallback label)
        let (civ_color, civ_name) = civilizations_query
            .iter()
            .find(|civ| civ.id == _capital.owner)
            .map(|civ| (civ.color, civ.name.clone()))
            .unwrap_or(([1.0, 1.0, 1.0], "Unknown".to_string()));

        // Decide label text: prefer city's name when present
        let label_text = if let Some(city) = city_opt {
            city.name.clone()
        } else {
            civ_name.clone()
        };

        debug_println!(
            "Spawning settlement label '{}' at ({}, {})",
            label_text,
            position.x,
            position.y
        );

        let world_pos = calculate_world_position_for_gizmo(
            *position, map_size, tile_size, grid_size, map_type, anchor,
        );

        // Position label at center-top of the tile
        let mut label_pos = world_pos;
        label_pos.y += tile_size.y * 0.5 + 8.0; // small margin above tile top
        label_pos.z += 1.0; // render above the sprite

        // Spawn a container for the label (background box + icon + text)
        let colors = create_label_colors(civ_color);

        let container = commands
            .spawn((
                Name::new(format!("SettlementLabelContainer: {}", label_text)),
                Transform::from_translation(label_pos),
                GlobalTransform::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
                SettlementLabelContainer,
                colors,
                DespawnOnExit(Screen::Gameplay),
            ))
            .id();

        // Background box
        commands.entity(container).with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: colors.background,
                    custom_size: Some(Vec2::new(
                        label_constants::LABEL_BOX_WIDTH,
                        label_constants::LABEL_BOX_HEIGHT,
                    )),
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                GlobalTransform::default(),
            ));
        });

        // Icon placeholder
        commands.entity(container).with_children(|parent| {
            let icon_x = -label_constants::LABEL_BOX_WIDTH / 2.0
                + label_constants::ICON_SIZE / 2.0
                + label_constants::ICON_PADDING;
            parent.spawn((
                Sprite {
                    color: colors.icon,
                    custom_size: Some(Vec2::new(
                        label_constants::ICON_SIZE,
                        label_constants::ICON_SIZE,
                    )),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    icon_x,
                    0.0,
                    label_constants::LABEL_Z_INDEX + 1.0,
                )),
                GlobalTransform::default(),
            ));
        });

        // Text (use city's name when present, otherwise fall back to civilization/capital name)
        let label_text = if let Some(city) = city_opt {
            city.name.clone()
        } else {
            civ_name.clone()
        };
        let text_x = label_constants::ICON_SIZE
            + label_constants::ICON_PADDING
            + label_constants::TEXT_PADDING;
        commands.entity(container).with_children(|parent| {
            parent.spawn((
                Text2d::new(label_text),
                // TextFont and TextColor are used throughout the project UI
                TextFont {
                    font_size: label_constants::LABEL_FONT_SIZE,
                    ..default()
                },
                TextColor(colors.text),
                TextLayout::new_with_justify(Justify::Left),
                Transform::from_translation(Vec3::new(
                    text_x - label_constants::LABEL_BOX_WIDTH / 2.0,
                    0.0,
                    label_constants::LABEL_Z_INDEX + 2.0,
                )),
                GlobalTransform::default(),
            ));
        });

        commands
            .entity(settlement_entity)
            .insert(SettlementLabelLink {
                label_entity: container,
            });
    }
}

/// System that despawns sprites when settlements are removed
fn despawn_settlement_sprites(
    mut commands: Commands,
    mut removed_settlements: RemovedComponents<Capital>,
    sprite_links: Query<&SettlementSpriteLink>,
    label_links: Query<&SettlementLabelLink>,
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
        // Also despawn any linked label
        if let Ok(label_link) = label_links.get(settlement_entity) {
            debug_println!(
                "Despawning label for removed settlement entity: {:?}",
                settlement_entity
            );
            commands.entity(label_link.label_entity).despawn();
        }
    }
}

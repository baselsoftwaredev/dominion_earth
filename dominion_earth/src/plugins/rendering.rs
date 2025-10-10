use crate::rendering;
use crate::screens::Screen;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

/// Plugin for all rendering systems and setup
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            // External rendering plugin
            .add_plugins(TilemapPlugin)
            // Load tile assets - runs repeatedly in Update until assets are loaded and resource is inserted
            .add_systems(Update, core_sim::tile::tile_assets::load_tile_assets)
            // Tilemap and Asset Setup Systems - run when TileAssets becomes available
            .add_systems(
                Update,
                (
                    rendering::tilemap::setup_tilemap.after(crate::game::setup_game),
                    rendering::tilemap::spawn_world_tiles.after(rendering::tilemap::setup_tilemap),
                    rendering::tilemap::attach_tile_sprite_components
                        .after(rendering::tilemap::setup_tilemap),
                    // Sprite Spawning Systems - must be in Update to run after tilemap setup
                    rendering::units::spawn_unit_sprites
                        .after(rendering::tilemap::spawn_world_tiles),
                    rendering::capitals::spawn_animated_capital_tiles
                        .after(rendering::tilemap::spawn_world_tiles),
                )
                    .run_if(in_state(Screen::Gameplay)),
            )
            // Runtime Rendering Update Systems
            .add_systems(
                Update,
                (
                    rendering::units::recreate_missing_unit_sprites,
                    rendering::units::update_unit_sprites,
                    rendering::capitals::update_capital_sprites,
                    rendering::capitals::update_animated_capital_sprites,
                    rendering::borders::render_civilization_borders,
                    // Fog of War rendering
                    rendering::fog_of_war::apply_fog_of_war_to_tiles,
                    rendering::fog_of_war::hide_entities_in_fog,
                )
                    .run_if(in_state(Screen::Gameplay)),
            );
    }
}

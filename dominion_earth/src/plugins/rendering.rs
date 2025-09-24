use crate::rendering;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

/// Plugin for all rendering systems and setup
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            // External rendering plugin
            .add_plugins(TilemapPlugin)
            // Tilemap and Asset Setup Systems
            .add_systems(
                Startup,
                (
                    core_sim::tile::tile_assets::setup_tile_assets,
                    rendering::tilemap::setup_tilemap
                        .after(core_sim::tile::tile_assets::setup_tile_assets)
                        .after(crate::game::setup_game),
                    rendering::tilemap::spawn_world_tiles.after(rendering::tilemap::setup_tilemap),
                ),
            )
            // Sprite Spawning Systems
            .add_systems(
                Startup,
                (
                    rendering::units::spawn_unit_sprites
                        .after(rendering::tilemap::spawn_world_tiles),
                    rendering::capitals::spawn_animated_capital_tiles
                        .after(rendering::tilemap::spawn_world_tiles),
                ),
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
                ),
            );
    }
}

use crate::rendering;
use crate::screens::Screen;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use core_sim::ChunkManager;

/// Plugin for all rendering systems and setup
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default())
            .add_plugins(TilemapPlugin)
            .add_systems(Update, core_sim::tile::tile_assets::load_tile_assets)
            .add_systems(
                Update,
                (
                    rendering::tilemap::setup_tilemap.after(crate::game::setup_game),
                    rendering::tilemap::spawn_world_tiles.after(rendering::tilemap::setup_tilemap),
                    rendering::tilemap::attach_tile_sprite_components
                        .after(rendering::tilemap::setup_tilemap),
                    rendering::chunks::update_chunk_manager_from_camera
                        .after(rendering::tilemap::setup_tilemap),
                )
                    .run_if(in_state(Screen::Gameplay)),
            )
            .add_systems(
                Update,
                (
                    rendering::chunks::debug_chunk_info,
                    rendering::borders::render_civilization_borders,
                )
                    .run_if(in_state(Screen::Gameplay)),
            );
    }
}

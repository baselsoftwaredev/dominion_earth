use crate::rendering;
use crate::screens::Screen;
use bevy::prelude::*;
use core_sim::ChunkManager;

/// Plugin for all rendering systems and setup
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default())
            .add_systems(Update, core_sim::tile::tile_assets::load_tile_assets)
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

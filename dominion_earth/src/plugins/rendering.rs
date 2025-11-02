use crate::rendering;
use crate::screens::Screen;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use core_sim::ChunkManager;

fn should_render_sprites_not_loading_from_save(
    save_state: Option<Res<crate::plugins::save_load::SaveLoadState>>,
) -> bool {
    save_state.map_or(true, |state| !state.is_loading_from_save)
}

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
                    rendering::units::spawn_unit_sprites
                        .after(rendering::tilemap::spawn_world_tiles)
                        .after(crate::plugins::save_load::handle_load_requests),
                    rendering::capitals::spawn_animated_capital_tiles
                        .after(rendering::tilemap::spawn_world_tiles)
                        .after(crate::plugins::save_load::handle_load_requests),
                )
                    .run_if(in_state(Screen::Gameplay))
                    .run_if(should_render_sprites_not_loading_from_save),
            )
            .add_systems(
                Update,
                (
                    rendering::chunks::debug_chunk_info,
                    rendering::units::recreate_missing_unit_sprites
                        .after(crate::plugins::save_load::handle_load_requests),
                    rendering::units::apply_facing_to_new_sprites,
                    rendering::units::update_unit_sprites,
                    rendering::capitals::update_capital_sprites,
                    rendering::capitals::update_animated_capital_sprites,
                    rendering::borders::render_civilization_borders,
                    rendering::fog_of_war::apply_fog_of_war_to_tiles,
                    rendering::fog_of_war::hide_entities_in_fog,
                    rendering::fog_of_war::hide_capital_labels_in_fog,
                    rendering::fog_of_war::hide_unit_labels_in_fog,
                )
                    .run_if(in_state(Screen::Gameplay))
                    .run_if(should_render_sprites_not_loading_from_save),
            );
    }
}

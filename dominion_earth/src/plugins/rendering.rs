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
                    rendering::tilemap::setup_tilemap
                        .after(crate::game::setup_game)
                        .after(crate::plugins::save_load::rebuild_tilemap_after_modifications),
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
                    .run_if(in_state(Screen::Gameplay)),
            )
            .add_systems(
                Update,
                (
                    rendering::chunks::debug_chunk_info,
                    rendering::units::cleanup_unit_sprites_on_load, // Cleanup old unit sprites on load
                    rendering::units::recreate_missing_unit_sprites
                        .after(crate::plugins::save_load::handle_load_requests),
                    rendering::capitals::recreate_missing_capital_sprites
                        .after(crate::plugins::save_load::handle_load_requests),
                    rendering::capitals::validate_and_recreate_capital_sprites
                        .after(crate::plugins::save_load::handle_load_requests),
                    rendering::units::apply_facing_to_new_sprites,
                    rendering::units::update_unit_sprites,
                    rendering::capitals::cleanup_capital_sprites_on_load, // Cleanup old capital sprites on load
                    rendering::capitals::update_capital_sprites,
                    rendering::capitals::update_animated_capital_sprites,
                    rendering::borders::render_civilization_borders,
                    // Apply fog of war AFTER all sprite recreation to ensure new sprites have correct visibility
                    rendering::fog_of_war::apply_fog_of_war_to_tiles
                        .after(rendering::units::recreate_missing_unit_sprites)
                        .after(rendering::capitals::validate_and_recreate_capital_sprites),
                    rendering::fog_of_war::hide_entities_in_fog
                        .after(rendering::units::recreate_missing_unit_sprites)
                        .after(rendering::capitals::validate_and_recreate_capital_sprites),
                    rendering::fog_of_war::hide_capital_labels_in_fog
                        .after(rendering::capitals::validate_and_recreate_capital_sprites),
                    rendering::fog_of_war::hide_unit_labels_in_fog
                        .after(rendering::units::recreate_missing_unit_sprites),
                    // Despawn hidden sprites to save memory
                    rendering::fog_of_war::despawn_hidden_entity_sprites
                        .after(rendering::fog_of_war::hide_entities_in_fog),
                )
                    .run_if(in_state(Screen::Gameplay)),
            );
    }
}

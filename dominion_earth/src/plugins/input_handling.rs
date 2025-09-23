use crate::{input, production_input};
use bevy::prelude::*;

/// Plugin for handling all input systems
pub struct InputHandlingPlugin;

impl Plugin for InputHandlingPlugin {
    fn build(&self, app: &mut App) {
        app
            // General Input Systems
            .add_systems(
                Update,
                (
                    input::handle_input,
                    input::handle_mouse_input,
                    input::handle_tile_selection_on_mouse_click,
                    input::handle_tile_hover_on_mouse_move,
                    input::handle_player_unit_interaction,
                ),
            )
            // Production Input Systems
            .add_systems(
                Update,
                (
                    production_input::handle_production_input,
                    production_input::handle_end_turn_input,
                ),
            );
    }
}

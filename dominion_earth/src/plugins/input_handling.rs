use crate::{input, production_input, screens::Screen};
use bevy::prelude::*;

/// Plugin for handling all input systems
pub struct InputHandlingPlugin;

impl Plugin for InputHandlingPlugin {
    fn build(&self, app: &mut App) {
        app
            // General Input Systems - only run during gameplay
            .add_systems(
                Update,
                (
                    input::handle_input,
                    input::handle_mouse_input,
                    input::handle_tile_selection_on_mouse_click,
                    input::handle_tile_hover_on_mouse_move,
                    input::handle_player_unit_interaction,
                )
                    .run_if(in_state(Screen::Gameplay)),
            )
            // Production Input Systems - only run during gameplay
            .add_systems(
                Update,
                (
                    production_input::handle_production_input,
                    production_input::handle_end_turn_input,
                )
                    .run_if(in_state(Screen::Gameplay)),
            );
    }
}

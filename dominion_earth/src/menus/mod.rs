//! The game's menus and transitions between them.

mod credits;
mod game_setup;
mod main;
mod pause;
mod settings;
pub mod ui_visibility;

use crate::debug_utils::DebugLogging;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Menu>();

    app.add_plugins((
        main::plugin,
        pause::plugin,
        settings::plugin,
        credits::plugin,
        game_setup::plugin,
    ));

    // Debug menu state changes
    app.add_systems(Update, log_menu_transitions);
}

fn log_menu_transitions(menu: Res<State<Menu>>, debug_logging: Res<DebugLogging>) {
    if menu.is_changed() {
        crate::debug_println!(debug_logging, "📜 Menu state changed to: {:?}", menu.get());
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
    GameSetup,
}

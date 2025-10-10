//! The game's main screen states and transitions between them.

mod gameplay;
mod main_menu;
mod splash;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((splash::plugin, main_menu::plugin, gameplay::plugin));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    MainMenu,
    Gameplay,
}

//! The game's main screen states and transitions between them.

mod gameplay;
mod main_menu;
mod splash;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((splash::plugin, main_menu::plugin, gameplay::plugin));

    // Add debug logging for screen transitions
    app.add_systems(Update, log_screen_transitions);
}

fn log_screen_transitions(screen: Res<State<Screen>>) {
    if screen.is_changed() {
        println!("🖥️  Screen changed to: {:?}", screen.get());
    }
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    MainMenu,
    Gameplay,
}

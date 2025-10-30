//! The game's main screen states and transitions between them.

mod game_setup;
mod gameplay;
mod main_menu;
mod splash;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.init_state::<LoadingState>();

    app.add_plugins((
        splash::plugin,
        main_menu::plugin,
        game_setup::plugin,
        gameplay::plugin,
    ));

    // Add debug logging for screen transitions
    app.add_systems(Update, log_screen_transitions);
    app.add_systems(Update, log_loading_state_transitions);
}

fn log_screen_transitions(screen: Res<State<Screen>>) {
    if screen.is_changed() {
        println!("🖥️  Screen changed to: {:?}", screen.get());
    }
}

fn log_loading_state_transitions(loading: Res<State<LoadingState>>) {
    if loading.is_changed() {
        println!("📦 Loading state changed to: {:?}", loading.get());
    }
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    MainMenu,
    GameSetup,
    Gameplay,
}

/// Loading state for managing entity lifecycle during save/load operations.
/// Sprites and labels are marked with DespawnOnEnter(LoadingState::Loading)
/// so they automatically clean up when a load begins.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading,
}

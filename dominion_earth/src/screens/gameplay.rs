//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub fn plugin(app: &mut App) {
    // Toggle pause menu on Escape
    app.add_systems(
        Update,
        open_pause_menu.run_if(
            in_state(Screen::Gameplay)
                .and(in_state(Menu::None))
                .and(input_just_pressed(KeyCode::Escape)),
        ),
    );

    app.add_systems(
        Update,
        close_menu.run_if(
            in_state(Screen::Gameplay)
                .and(not(in_state(Menu::None)))
                .and(input_just_pressed(KeyCode::Escape)),
        ),
    );

    app.add_systems(OnExit(Screen::Gameplay), close_menu);
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn input_just_pressed(key: KeyCode) -> impl Condition<()> {
    IntoSystem::into_system(move |input: Res<ButtonInput<KeyCode>>| input.just_pressed(key))
}

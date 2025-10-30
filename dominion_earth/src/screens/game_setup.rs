//! The game setup screen where players configure their game before starting.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameSetup), setup)
        .add_systems(OnExit(Screen::GameSetup), cleanup);
}

fn setup(mut next_menu: ResMut<NextState<Menu>>) {
    println!("🎮 Opening game setup menu");
    next_menu.set(Menu::GameSetup);
}

fn cleanup(mut next_menu: ResMut<NextState<Menu>>) {
    println!("📋 Closing menu from GameSetup screen");
    next_menu.set(Menu::None);
}

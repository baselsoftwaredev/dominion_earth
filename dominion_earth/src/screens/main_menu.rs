//! The main menu screen.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::MainMenu), open_main_menu);
    app.add_systems(OnExit(Screen::MainMenu), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    println!("📋 Opening main menu");
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    println!("📋 Closing menu from MainMenu screen");
    next_menu.set(Menu::None);
}

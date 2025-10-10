//! The main menu (seen on the main menu screen).

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            widget::ui_root("Main Menu"),
            GlobalZIndex(2),
            StateScoped(Menu::Main),
        ))
        .with_children(|parent| {
            parent.spawn(widget::header("Dominion Earth"));
            parent.spawn(widget::button("Play", widget::ButtonAction::EnterGameplay));
            parent.spawn(widget::button(
                "Settings",
                widget::ButtonAction::OpenSettings,
            ));
            parent.spawn(widget::button("Credits", widget::ButtonAction::OpenCredits));

            #[cfg(not(target_family = "wasm"))]
            parent.spawn(widget::button("Exit", widget::ButtonAction::ExitApp));
        });
}

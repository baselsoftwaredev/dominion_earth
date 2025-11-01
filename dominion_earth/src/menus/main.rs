//! The main menu (seen on the main menu screen).

use bevy::prelude::*;

use crate::{menus::Menu, theme::prelude::*};

/// Marker component for entities that belong to the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), setup_main_menu);
}

fn setup_main_menu(mut commands: Commands) {
    crate::debug_println!("📋 Spawning main menu");
    commands
        .spawn((
            widget::ui_root("Main Menu"),
            GlobalZIndex(constants::z_index::MENU_OVERLAY_Z_INDEX),
            DespawnOnExit(Menu::Main),
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(widget::header("Dominion Earth"));
            parent.spawn(widget::button("Play", widget::ButtonAction::OpenGameSetup));
            parent.spawn(widget::button(
                "Settings",
                widget::ButtonAction::OpenSettings,
            ));
            parent.spawn(widget::button("Credits", widget::ButtonAction::OpenCredits));

            #[cfg(not(target_family = "wasm"))]
            parent.spawn(widget::button("Exit", widget::ButtonAction::ExitApp));
        });
}

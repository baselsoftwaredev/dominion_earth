//! The main menu (seen on the main menu screen).

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::prelude::*};

#[derive(Component)]
struct MainMenuRoot;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_systems(OnExit(Menu::Main), cleanup_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    println!("📋 Spawning main menu");
    commands
        .spawn((
            widget::ui_root("Main Menu"),
            GlobalZIndex(100),
            DespawnOnExit(Menu::Main),
            MainMenuRoot, // Marker component
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

fn cleanup_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuRoot>>) {
    println!(
        "🧹 Cleaning up main menu - found {} entities",
        menu_query.iter().count()
    );
    for entity in &menu_query {
        commands.entity(entity).despawn();
    }
}

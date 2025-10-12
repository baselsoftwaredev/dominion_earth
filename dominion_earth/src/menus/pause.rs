//! The pause menu.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::prelude::*};

#[derive(Component)]
struct PauseMenuRoot;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(OnExit(Menu::Pause), cleanup_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands) {
    println!("📋 Spawning pause menu");
    commands
        .spawn((
            widget::ui_root("Pause Menu"),
            GlobalZIndex(100),
            StateScoped(Menu::Pause),
            PauseMenuRoot, // Marker component
        ))
        .with_children(|parent| {
            parent.spawn(widget::header("Game Paused"));
            parent.spawn(widget::button("Continue", widget::ButtonAction::CloseMenu));
            parent.spawn(widget::button(
                "Settings",
                widget::ButtonAction::OpenSettings,
            ));
            parent.spawn(widget::button(
                "Quit to Menu",
                widget::ButtonAction::QuitToMenu,
            ));
        });
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn input_just_pressed(key: KeyCode) -> impl Condition<()> {
    IntoSystem::into_system(move |input: Res<ButtonInput<KeyCode>>| input.just_pressed(key))
}

fn cleanup_pause_menu(mut commands: Commands, menu_query: Query<Entity, With<PauseMenuRoot>>) {
    println!(
        "🧹 Cleaning up pause menu - found {} entities",
        menu_query.iter().count()
    );
    for entity in &menu_query {
        commands.entity(entity).despawn();
    }
}

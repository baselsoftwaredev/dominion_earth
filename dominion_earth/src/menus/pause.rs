use bevy::prelude::*;

use crate::{
    debug_utils::DebugLogging,
    menus::{ui_visibility, Menu},
    screens::Screen,
    theme::prelude::*,
};

#[derive(Component)]
struct PauseMenuRoot;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Menu::Pause),
        (spawn_pause_menu, ui_visibility::hide_gameplay_ui_panels),
    );
    app.add_systems(OnExit(Menu::Pause), ui_visibility::show_gameplay_ui_panels);
    app.add_systems(
        Update,
        close_pause_menu_on_escape
            .run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands, debug_logging: Res<DebugLogging>) {
    crate::debug_println!(debug_logging, "📋 Spawning pause menu");

    commands
        .spawn((
            widget::ui_root("Pause Menu"),
            GlobalZIndex(constants::z_index::MENU_OVERLAY_Z_INDEX),
            DespawnOnExit(Menu::Pause),
            PauseMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn(widget::header("Game Paused"));
            parent.spawn(widget::button("Continue", widget::ButtonAction::CloseMenu));
            parent.spawn(widget::button(
                "Save Game (F5)",
                widget::ButtonAction::SaveGame,
            ));
            parent.spawn(widget::button(
                "Load Game (F9)",
                widget::ButtonAction::LoadGame,
            ));
            parent.spawn(widget::button(
                "Settings",
                widget::ButtonAction::OpenSettings,
            ));
            parent.spawn(widget::button(
                "Main Menu",
                widget::ButtonAction::QuitToMenu,
            ));
        });
}

fn close_pause_menu_on_escape(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn input_just_pressed(key: KeyCode) -> impl SystemCondition<()> {
    IntoSystem::into_system(move |input: Res<ButtonInput<KeyCode>>| input.just_pressed(key))
}

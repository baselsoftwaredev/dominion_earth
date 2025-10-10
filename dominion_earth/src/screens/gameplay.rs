//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::widget::ButtonAction};

pub fn plugin(app: &mut App) {
    // Force cleanup any menu UI when entering gameplay
    app.add_systems(OnEnter(Screen::Gameplay), cleanup_all_menu_entities);

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

/// Force cleanup any leftover menu UI entities when entering gameplay
fn cleanup_all_menu_entities(
    mut commands: Commands,
    menu_query: Query<Entity, (With<Node>, With<GlobalZIndex>)>,
    z_index_query: Query<&GlobalZIndex>,
) {
    println!("🧹 Cleaning up menu UI on entering Gameplay");
    // Despawn any UI with GlobalZIndex (menus use this, game UI doesn't)
    for entity in &menu_query {
        if let Ok(z_index) = z_index_query.get(entity) {
            if z_index.0 >= 100 {
                println!("  Despawning menu entity with z-index {}", z_index.0);
                commands.entity(entity).despawn();
            }
        }
    }
}

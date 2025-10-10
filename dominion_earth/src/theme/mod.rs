//! Reusable UI widgets & theming.

pub mod interaction;
pub mod palette;
pub mod widget;

pub mod prelude {
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
}

use crate::{menus::Menu, screens::Screen};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
    // Only handle button interactions in the appropriate screens
    app.add_systems(
        Update,
        handle_button_interactions.run_if(
            in_state(Screen::Splash)
                .or(in_state(Screen::MainMenu))
                .or(in_state(Screen::Gameplay)),
        ),
    );
    app.add_systems(Update, spawn_button_text);
}

/// System to spawn text children for buttons that have ButtonText component
fn spawn_button_text(
    mut commands: Commands,
    button_query: Query<(Entity, &widget::ButtonText), (With<Button>, Without<Children>)>,
) {
    for (entity, button_text) in &button_query {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Name::new("Button Text"),
                Text::new(button_text.0.clone()),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(palette::BUTTON_TEXT),
            ));
        });
    }
}

/// System to handle button interactions based on ButtonAction component
fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &widget::ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut global_volume: ResMut<GlobalVolume>,
    mut app_exit: EventWriter<AppExit>,
    screen: Res<State<Screen>>,
) {
    use bevy::audio::{GlobalVolume, Volume};

    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!(
                "🎮 Button pressed: {:?} (current screen: {:?})",
                action,
                screen.get()
            );
            match action {
                widget::ButtonAction::EnterGameplay => {
                    // Only allow entering gameplay from MainMenu
                    if **screen != Screen::MainMenu {
                        println!("⚠️  Ignoring EnterGameplay button - not in MainMenu!");
                        continue;
                    }
                    println!("🎮 Transitioning to Gameplay screen");
                    next_screen.set(Screen::Gameplay);
                }
                widget::ButtonAction::OpenSettings => {
                    next_menu.set(Menu::Settings);
                }
                widget::ButtonAction::OpenCredits => {
                    next_menu.set(Menu::Credits);
                }
                widget::ButtonAction::ExitApp => {
                    #[cfg(not(target_family = "wasm"))]
                    app_exit.write(AppExit::Success);
                }
                widget::ButtonAction::CloseMenu => {
                    next_menu.set(Menu::None);
                }
                widget::ButtonAction::OpenPauseMenu => {
                    next_menu.set(Menu::Pause);
                }
                widget::ButtonAction::QuitToMenu => {
                    next_screen.set(Screen::MainMenu);
                }
                widget::ButtonAction::GoBack => {
                    next_menu.set(if **screen == Screen::MainMenu {
                        Menu::Main
                    } else {
                        Menu::Pause
                    });
                }
                widget::ButtonAction::LowerVolume => {
                    let linear = (global_volume.volume.to_linear() - 0.1).max(0.0);
                    global_volume.volume = Volume::Linear(linear);
                }
                widget::ButtonAction::RaiseVolume => {
                    let linear = (global_volume.volume.to_linear() + 0.1).min(3.0);
                    global_volume.volume = Volume::Linear(linear);
                }
            }
        }
    }
}

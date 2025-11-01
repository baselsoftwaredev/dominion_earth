//! Reusable UI widgets & theming.

pub mod constants;
pub mod interaction;
pub mod palette;
pub mod widget;

pub mod prelude {
    pub use super::{constants, interaction::InteractionPalette, palette as ui_palette, widget};
}

use crate::{menus::Menu, screens::Screen};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
    app.add_systems(
        Update,
        handle_button_interactions.run_if(
            in_state(Screen::Splash)
                .or(in_state(Screen::MainMenu))
                .or(in_state(Screen::GameSetup))
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
                    font_size: constants::font_sizes::BUTTON_TEXT_SIZE,
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
    mut settings: ResMut<crate::settings::GameSettings>,
    mut app_exit: MessageWriter<AppExit>,
    screen: Res<State<Screen>>,
) {
    use bevy::audio::{GlobalVolume, Volume};

    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            crate::debug_println!(
                "🎮 Button pressed: {:?} (current screen: {:?})",
                action,
                screen.get()
            );
            match action {
                widget::ButtonAction::OpenGameSetup => {
                    if **screen != Screen::MainMenu {
                        crate::debug_println!(
                            "⚠️  Ignoring OpenGameSetup button - not in MainMenu!"
                        );
                        continue;
                    }
                    crate::debug_println!("🎮 Transitioning to GameSetup screen");
                    next_screen.set(Screen::GameSetup);
                }
                widget::ButtonAction::StartGame => {
                    if **screen != Screen::GameSetup {
                        crate::debug_println!(
                            "⚠️  Ignoring StartGame button - not in GameSetup!"
                        );
                        continue;
                    }
                    crate::debug_println!(
                        "🎮 Starting game - transitioning to Gameplay screen"
                    );
                    next_screen.set(Screen::Gameplay);
                }
                widget::ButtonAction::EnterGameplay => {
                    if **screen != Screen::MainMenu {
                        crate::debug_println!(
                            "⚠️  Ignoring EnterGameplay button - not in MainMenu!"
                        );
                        continue;
                    }
                    crate::debug_println!("🎮 Transitioning to Gameplay screen");
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
                    // Special handling for GameSetup screen - return to MainMenu
                    if **screen == Screen::GameSetup {
                        crate::debug_println!(
                            "🎮 Going back from GameSetup to MainMenu"
                        );
                        next_screen.set(Screen::MainMenu);
                    } else {
                        next_menu.set(determine_parent_menu_from_screen(**screen));
                    }
                }
                widget::ButtonAction::LowerVolume => {
                    apply_volume_adjustment(
                        &mut global_volume,
                        -constants::audio::VOLUME_ADJUSTMENT_STEP,
                    );
                }
                widget::ButtonAction::RaiseVolume => {
                    apply_volume_adjustment(
                        &mut global_volume,
                        constants::audio::VOLUME_ADJUSTMENT_STEP,
                    );
                }
                widget::ButtonAction::SaveSettings => {
                    crate::debug_println!("💾 Saving settings to file...");
                    if let Err(e) = settings.save() {
                        error!("❌ Failed to save settings: {}", e);
                    } else {
                        info!("✅ Settings saved successfully");
                    }
                }
                widget::ButtonAction::ToggleAiOnly => {
                    settings.ai_only = !settings.ai_only;
                    crate::debug_println!(
                        "🤖 AI-only mode toggled: {}",
                        if settings.ai_only {
                            "enabled"
                        } else {
                            "disabled"
                        }
                    );
                }
                widget::ButtonAction::ClearSeed => {
                    settings.seed = None;
                    crate::debug_println!("🎲 Seed cleared");
                }
                widget::ButtonAction::SetRandomSeed => {
                    use rand::Rng;
                    let new_seed = rand::thread_rng().gen::<u64>();
                    settings.seed = Some(new_seed);
                    crate::debug_println!("🎲 Random seed set: {}", new_seed);
                }
            }
        }
    }
}

fn determine_parent_menu_from_screen(screen: Screen) -> Menu {
    match screen {
        Screen::MainMenu => Menu::Main,
        Screen::GameSetup => Menu::GameSetup,
        _ => Menu::Pause,
    }
}

fn apply_volume_adjustment(global_volume: &mut ResMut<GlobalVolume>, adjustment: f32) {
    use bevy::audio::Volume;
    let current_linear_volume = global_volume.volume.to_linear();
    let adjusted_volume = (current_linear_volume + adjustment)
        .max(constants::audio::MINIMUM_VOLUME_LEVEL)
        .min(constants::audio::MAXIMUM_VOLUME_LEVEL);
    global_volume.volume = Volume::Linear(adjusted_volume);
}

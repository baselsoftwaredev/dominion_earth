use bevy::audio::{GlobalVolume, Volume};
use bevy::prelude::*;

use crate::{
    debug_utils::DebugLogging,
    menus::{ui_visibility, Menu},
    screens::Screen,
    settings::{sync_volume_to_settings, GameSettings},
    theme::prelude::*,
};

#[derive(Component)]
struct SettingsMenuRoot;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Menu::Settings),
        (spawn_settings_menu, ui_visibility::hide_gameplay_ui_panels),
    );
    app.add_systems(
        OnExit(Menu::Settings),
        ui_visibility::show_gameplay_ui_panels,
    );
    app.add_systems(
        Update,
        close_settings_menu_on_escape
            .run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(
        Update,
        (update_global_volume_label, sync_volume_to_settings).run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(
    mut commands: Commands,
    debug_logging: Res<DebugLogging>,
    global_volume: Res<GlobalVolume>,
) {
    crate::debug_println!(debug_logging, "📋 Spawning settings menu");

    let current_volume_percent =
        crate::constants::settings::PERCENTAGE_MULTIPLIER * global_volume.volume.to_linear();

    commands
        .spawn((
            widget::ui_root("Settings Menu"),
            GlobalZIndex(constants::z_index::MENU_OVERLAY_Z_INDEX),
            DespawnOnExit(Menu::Settings),
            SettingsMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn(widget::header("Settings"));

            parent
                .spawn((
                    Name::new("Volume Container"),
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: ui_palette::px(
                            crate::constants::ui::spacing::VOLUME_CONTROLS_GAP,
                        ),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(widget::label("Master Volume"));
                    parent.spawn(widget::button_small("-", widget::ButtonAction::LowerVolume));
                    parent.spawn((
                        Name::new("Volume Label"),
                        Text::new(format!("{current_volume_percent:3.0}%")),
                        TextFont {
                            font_size: constants::font_sizes::LABEL_TEXT_SIZE,
                            ..default()
                        },
                        TextColor(ui_palette::TEXT_PRIMARY),
                        GlobalVolumeLabel,
                    ));
                    parent.spawn(widget::button_small("+", widget::ButtonAction::RaiseVolume));
                });

            parent.spawn(widget::button(
                "Save Settings",
                widget::ButtonAction::SaveSettings,
            ));

            parent.spawn(widget::button("Back", widget::ButtonAction::GoBack));
        });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label_query: Query<&mut Text, With<GlobalVolumeLabel>>,
) {
    if global_volume.is_changed() {
        if let Some(mut text) = label_query.iter_mut().next() {
            let percent = crate::constants::settings::PERCENTAGE_MULTIPLIER
                * global_volume.volume.to_linear();
            **text = format!("{percent:3.0}%");
        }
    }
}

fn close_settings_menu_on_escape(
    current_screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    let target_menu = determine_target_menu_from_screen(**current_screen);
    next_menu.set(target_menu);
}

fn determine_target_menu_from_screen(screen: Screen) -> Menu {
    if screen == Screen::MainMenu {
        Menu::Main
    } else {
        Menu::Pause
    }
}

fn input_just_pressed(key: KeyCode) -> impl SystemCondition<()> {
    IntoSystem::into_system(move |input: Res<ButtonInput<KeyCode>>| input.just_pressed(key))
}

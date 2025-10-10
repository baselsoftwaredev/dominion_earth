//! The settings menu.

use bevy::audio::{GlobalVolume, Volume};
use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(
        Update,
        update_global_volume_label.run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands
        .spawn((
            widget::ui_root("Settings Menu"),
            GlobalZIndex(100),
            StateScoped(Menu::Settings),
        ))
        .with_children(|parent| {
            parent.spawn(widget::header("Settings"));

            // Volume settings
            parent
                .spawn((
                    Name::new("Volume Container"),
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: ui_palette::px(20.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(widget::label("Master Volume"));
                    parent.spawn(widget::button_small("-", widget::ButtonAction::LowerVolume));
                    parent.spawn((
                        Name::new("Volume Label"),
                        Text::new("100%"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(ui_palette::TEXT_PRIMARY),
                        GlobalVolumeLabel,
                    ));
                    parent.spawn(widget::button_small("+", widget::ButtonAction::RaiseVolume));
                });

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
    if let Ok(mut text) = label_query.single_mut() {
        let percent = 100.0 * global_volume.volume.to_linear();
        **text = format!("{percent:3.0}%");
    }
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if **screen == Screen::MainMenu {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn input_just_pressed(key: KeyCode) -> impl Condition<()> {
    IntoSystem::into_system(move |input: Res<ButtonInput<KeyCode>>| input.just_pressed(key))
}

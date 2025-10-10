//! The credits menu.

use bevy::prelude::*;

use crate::{menus::Menu, theme::prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_credits_menu(mut commands: Commands) {
    commands
        .spawn((
            widget::ui_root("Credits Menu"),
            GlobalZIndex(2),
            StateScoped(Menu::Credits),
        ))
        .with_children(|parent| {
            parent.spawn(widget::header("Credits"));

            // Created by section
            parent
                .spawn((
                    Name::new("Credits Content"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: ui_palette::px(10.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(widget::label("Dominion Earth Development Team"));
                    parent.spawn(widget::label("A turn-based strategy game"));
                    parent.spawn(widget::label("Built with Bevy Engine"));
                });

            parent.spawn(widget::button("Back", widget::ButtonAction::GoBack));
        });
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn input_just_pressed(key: KeyCode) -> impl Condition<()> {
    IntoSystem::into_system(move |input: Res<ButtonInput<KeyCode>>| input.just_pressed(key))
}

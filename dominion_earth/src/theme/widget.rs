//! Helper functions for creating common widgets.

use crate::theme::{interaction::InteractionPalette, palette::*};
use bevy::prelude::*;

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<String>) -> impl Bundle {
    (
        Name::new(name.into()),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20.0),
            ..default()
        },
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text::new(text.into()),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(TEXT_HEADER),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text::new(text.into()),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(TEXT_PRIMARY),
    )
}

/// A large rounded button with text - use with ButtonAction component for actions.
pub fn button(text: impl Into<String>, action: ButtonAction) -> impl Bundle {
    let text_string = text.into();
    (
        Button,
        Node {
            width: px(250.0),
            height: px(65.0),
            border: UiRect::all(Val::Px(3.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BUTTON_BACKGROUND),
        BorderColor(PANEL_BORDER),
        Name::new(format!("Button: {}", text_string)),
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        },
        action,
        ButtonText(text_string),
    )
}

/// A small square button with text - use with ButtonAction component for actions.
pub fn button_small(text: impl Into<String>, action: ButtonAction) -> impl Bundle {
    let text_string = text.into();
    (
        Button,
        Node {
            width: px(30.0),
            height: px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BUTTON_BACKGROUND),
        Name::new(format!("Small Button: {}", text_string)),
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        },
        action,
        ButtonText(text_string),
    )
}

/// Component to store button text
#[derive(Component)]
pub struct ButtonText(pub String);

/// Actions that buttons can perform
#[derive(Component, Clone, Copy, Debug)]
pub enum ButtonAction {
    EnterGameplay,
    OpenSettings,
    OpenCredits,
    ExitApp,
    CloseMenu,
    OpenPauseMenu,
    QuitToMenu,
    GoBack,
    LowerVolume,
    RaiseVolume,
}

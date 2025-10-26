//! Helper functions for creating common widgets.

use crate::theme::{constants, interaction::InteractionPalette, palette::*};
use bevy::prelude::*;
use bevy::ui::{percent, px};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<String>) -> impl Bundle {
    (
        Name::new(name.into()),
        Node {
            width: percent(constants::layout::FULL_SCREEN_PERCENTAGE),
            height: percent(constants::layout::FULL_SCREEN_PERCENTAGE),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(constants::layout::UI_ROOT_ROW_GAP),
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
            font_size: constants::font_sizes::HEADER_TEXT_SIZE,
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
            font_size: constants::font_sizes::LABEL_TEXT_SIZE,
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
            width: px(constants::button_dimensions::STANDARD_BUTTON_WIDTH),
            height: px(constants::button_dimensions::STANDARD_BUTTON_HEIGHT),
            border: UiRect::all(Val::Px(
                constants::button_dimensions::STANDARD_BUTTON_BORDER_WIDTH,
            )),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BUTTON_BACKGROUND),
        BorderColor::all(PANEL_BORDER),
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
            width: px(constants::button_dimensions::SMALL_BUTTON_SIZE),
            height: px(constants::button_dimensions::SMALL_BUTTON_SIZE),
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
    SaveGame,
    LoadGame,
}

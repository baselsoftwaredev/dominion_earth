pub mod constants;
pub mod production_section;
pub mod unit_info_section;

use bevy::prelude::*;
use core_sim::RequestTurnAdvance;

use crate::ui::constants::display_layout;
use constants::*;

// Re-export components from sub-modules
pub use production_section::*;
pub use unit_info_section::*;

// Main left panel components
#[derive(Component)]
pub struct LeftPanel;

#[derive(Component)]
pub struct GamePanel;

#[derive(Component)]
pub struct NextTurnButton;

/// Spawns the main left panel with all sections
pub fn spawn_left_panel(mut commands: Commands) {
    let production_panel = production_section::spawn_production_menu_panel(&mut commands);
    let unit_info_panel = unit_info_section::spawn_unit_info_panel(&mut commands);

    commands
        .spawn((
            LeftPanel,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(display_layout::HEADER_HEIGHT),
                width: Val::Px(display_layout::LEFT_SIDEBAR_WIDTH),
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.102, 0.102, 0.102, 1.0)),
            Name::new("Left Panel"),
        ))
        .with_children(|parent| {
            // Game Panel with Next Turn button
            parent
                .spawn((
                    GamePanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: GAME_PANEL_MIN_HEIGHT,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Game Panel"),
                ))
                .with_children(|game_parent| {
                    game_parent.spawn((
                        Text::new("Your Empire"),
                        TextFont {
                            font_size: TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Game Panel Title"),
                    ));

                    game_parent
                        .spawn((
                            NextTurnButton,
                            Button,
                            Node {
                                height: NEXT_TURN_BUTTON_HEIGHT,
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(BUTTON_PADDING),
                                margin: UiRect::bottom(SECTION_MARGIN_BOTTOM),
                                border: UiRect::all(BUTTON_BORDER_WIDTH),
                                ..default()
                            },
                            BackgroundColor(BUTTON_BACKGROUND),
                            BorderColor::from(BUTTON_BORDER),
                            BorderRadius::all(BUTTON_BORDER_RADIUS),
                            Name::new("Next Turn Button"),
                        ))
                        .with_children(|button_parent| {
                            button_parent.spawn((
                                Text::new("Next Turn"),
                                TextFont {
                                    font_size: SUBTITLE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_PRIMARY),
                            ));
                        });
                });
        })
        .add_children(&[production_panel, unit_info_panel]);
}

/// Handles Next Turn button interactions
pub fn handle_next_turn_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<NextTurnButton>),
    >,
    mut turn_advance_events: MessageWriter<RequestTurnAdvance>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut background, mut border) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(BUTTON_PRESSED_BACKGROUND);
                turn_advance_events.write(RequestTurnAdvance);
                info!("Player requested turn advancement");
                crate::audio::play_sound_effect(&mut commands, &asset_server, "sounds/click.ogg");
            }
            Interaction::Hovered => {
                *background = BackgroundColor(BUTTON_HOVER_BACKGROUND);
                *border = BorderColor::all(BUTTON_HOVER_BORDER);
            }
            Interaction::None => {
                *background = BackgroundColor(BUTTON_BACKGROUND);
                *border = BorderColor::all(BUTTON_BORDER);
            }
        }
    }
}

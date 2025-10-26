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

            // Production Menu Panel
            parent
                .spawn((
                    production_section::ProductionMenuPanel,
                    Node {
                        display: Display::None,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        max_height: PRODUCTION_MENU_MAX_HEIGHT,
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Production Menu Panel"),
                ))
                .with_children(|menu_parent| {
                    // Title
                    menu_parent.spawn((
                        Text::new("Production Menu"),
                        TextFont {
                            font_size: TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Production Menu Title"),
                    ));

                    // Capital info section
                    menu_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(SECTION_MARGIN_BOTTOM),
                                ..default()
                            },
                            Name::new("Capital Info Container"),
                        ))
                        .with_children(|info_parent| {
                            info_parent.spawn((
                                production_section::ProductionCapitalNameText,
                                Text::new("Capital: Unknown"),
                                TextFont {
                                    font_size: SUBTITLE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_PRIMARY),
                                Node {
                                    margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                                    ..default()
                                },
                            ));

                            info_parent.spawn((
                                production_section::ProductionCivNameText,
                                Text::new("Civilization: Unknown"),
                                TextFont {
                                    font_size: SUBTITLE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_PRIMARY),
                                Node {
                                    margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                                    ..default()
                                },
                            ));

                            info_parent.spawn((
                                production_section::ProductionGoldText,
                                Text::new("Gold: 0"),
                                TextFont {
                                    font_size: BODY_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_SECONDARY),
                                Node {
                                    margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                                    ..default()
                                },
                            ));

                            info_parent.spawn((
                                production_section::ProductionProductionText,
                                Text::new("Production: 0"),
                                TextFont {
                                    font_size: BODY_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_SECONDARY),
                            ));
                        });

                    // Separator
                    menu_parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: SEPARATOR_HEIGHT,
                            margin: UiRect::vertical(SEPARATOR_MARGIN),
                            ..default()
                        },
                        BackgroundColor(PANEL_BORDER),
                        Name::new("Separator"),
                    ));

                    // Available units section
                    menu_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                                ..default()
                            },
                            Name::new("Available Units"),
                        ))
                        .with_children(|units_parent| {
                            units_parent.spawn((
                                Text::new("Available Units:"),
                                TextFont {
                                    font_size: SUBTITLE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TITLE_COLOR),
                                Node {
                                    margin: UiRect::bottom(SECTION_MARGIN_BOTTOM),
                                    ..default()
                                },
                            ));

                            // Infantry button
                            units_parent
                                .spawn((
                                    production_section::InfantryButton,
                                    Button,
                                    Node {
                                        height: BUTTON_HEIGHT,
                                        width: Val::Percent(100.0),
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(BUTTON_PADDING),
                                        margin: UiRect::bottom(BUTTON_MARGIN),
                                        border: UiRect::all(BUTTON_BORDER_WIDTH),
                                        ..default()
                                    },
                                    BackgroundColor(BUTTON_BACKGROUND),
                                    BorderColor::from(BUTTON_BORDER),
                                    BorderRadius::all(BUTTON_BORDER_RADIUS),
                                    Name::new("Infantry Button"),
                                ))
                                .with_children(|button_parent| {
                                    button_parent.spawn((
                                        Text::new("Infantry"),
                                        TextFont {
                                            font_size: BODY_FONT_SIZE,
                                            ..default()
                                        },
                                        TextColor(TEXT_PRIMARY),
                                    ));

                                    button_parent.spawn((
                                        Text::new("20 gold, 15 production"),
                                        TextFont {
                                            font_size: SMALL_FONT_SIZE,
                                            ..default()
                                        },
                                        TextColor(TEXT_TERTIARY),
                                    ));
                                });

                            // Archer button
                            units_parent
                                .spawn((
                                    production_section::ArcherButton,
                                    Button,
                                    Node {
                                        height: BUTTON_HEIGHT,
                                        width: Val::Percent(100.0),
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(BUTTON_PADDING),
                                        margin: UiRect::bottom(BUTTON_MARGIN),
                                        border: UiRect::all(BUTTON_BORDER_WIDTH),
                                        ..default()
                                    },
                                    BackgroundColor(BUTTON_BACKGROUND),
                                    BorderColor::from(BUTTON_BORDER),
                                    BorderRadius::all(BUTTON_BORDER_RADIUS),
                                    Name::new("Archer Button"),
                                ))
                                .with_children(|button_parent| {
                                    button_parent.spawn((
                                        Text::new("Archer"),
                                        TextFont {
                                            font_size: BODY_FONT_SIZE,
                                            ..default()
                                        },
                                        TextColor(TEXT_PRIMARY),
                                    ));

                                    button_parent.spawn((
                                        Text::new("25 gold, 20 production"),
                                        TextFont {
                                            font_size: SMALL_FONT_SIZE,
                                            ..default()
                                        },
                                        TextColor(TEXT_TERTIARY),
                                    ));
                                });

                            // Cavalry button
                            units_parent
                                .spawn((
                                    production_section::CavalryButton,
                                    Button,
                                    Node {
                                        height: BUTTON_HEIGHT,
                                        width: Val::Percent(100.0),
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(BUTTON_PADDING),
                                        margin: UiRect::bottom(BUTTON_MARGIN),
                                        border: UiRect::all(BUTTON_BORDER_WIDTH),
                                        ..default()
                                    },
                                    BackgroundColor(BUTTON_BACKGROUND),
                                    BorderColor::from(BUTTON_BORDER),
                                    BorderRadius::all(BUTTON_BORDER_RADIUS),
                                    Name::new("Cavalry Button"),
                                ))
                                .with_children(|button_parent| {
                                    button_parent.spawn((
                                        Text::new("Cavalry"),
                                        TextFont {
                                            font_size: BODY_FONT_SIZE,
                                            ..default()
                                        },
                                        TextColor(TEXT_PRIMARY),
                                    ));

                                    button_parent.spawn((
                                        Text::new("40 gold, 30 production"),
                                        TextFont {
                                            font_size: SMALL_FONT_SIZE,
                                            ..default()
                                        },
                                        TextColor(TEXT_TERTIARY),
                                    ));
                                });
                        });

                    // Current production section
                    menu_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                                ..default()
                            },
                            Name::new("Current Production"),
                        ))
                        .with_children(|prod_parent| {
                            prod_parent.spawn((
                                Text::new("Currently Producing:"),
                                TextFont {
                                    font_size: SUBTITLE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TITLE_COLOR),
                                Node {
                                    margin: UiRect::bottom(SECTION_MARGIN_BOTTOM),
                                    ..default()
                                },
                            ));

                            prod_parent.spawn((
                                production_section::CurrentProductionNameText,
                                Text::new("None"),
                                TextFont {
                                    font_size: BODY_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_PRIMARY),
                                Node {
                                    margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                                    ..default()
                                },
                            ));

                            prod_parent.spawn((
                                production_section::CurrentProductionProgressText,
                                Text::new("Progress: 0%"),
                                TextFont {
                                    font_size: SMALL_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_SECONDARY),
                            ));
                        });

                    // Production queue section
                    menu_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                                ..default()
                            },
                            Name::new("Production Queue"),
                        ))
                        .with_children(|queue_parent| {
                            queue_parent.spawn((
                                Text::new("Production Queue:"),
                                TextFont {
                                    font_size: SUBTITLE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TITLE_COLOR),
                                Node {
                                    margin: UiRect::bottom(SECTION_MARGIN_BOTTOM),
                                    ..default()
                                },
                            ));

                            queue_parent.spawn((
                                production_section::ProductionQueueLengthText,
                                Text::new("Items queued: 0"),
                                TextFont {
                                    font_size: BODY_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(TEXT_SECONDARY),
                            ));
                        });

                    // Separator
                    menu_parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: SEPARATOR_HEIGHT,
                            margin: UiRect::vertical(SEPARATOR_MARGIN),
                            ..default()
                        },
                        BackgroundColor(PANEL_BORDER),
                        Name::new("Separator"),
                    ));

                    // Help text
                    menu_parent.spawn((
                        Text::new("Press [Esc] to close | Click buttons to queue units"),
                        TextFont {
                            font_size: SMALL_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_TERTIARY),
                    ));
                });

            // Unit Info Panel
            parent
                .spawn((
                    unit_info_section::UnitInfoPanel,
                    Node {
                        display: Display::None, // Hidden by default
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(PANEL_PADDING),
                        margin: UiRect::all(PANEL_MARGIN),
                        border: UiRect::all(PANEL_BORDER_WIDTH),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                    BorderColor::from(PANEL_BORDER),
                    BorderRadius::all(PANEL_BORDER_RADIUS),
                    Name::new("Unit Info Panel"),
                ))
                .with_children(|unit_parent| {
                    unit_parent.spawn((
                        Text::new("Unit Information"),
                        TextFont {
                            font_size: TITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TITLE_COLOR),
                        Node {
                            margin: UiRect::bottom(TITLE_MARGIN_BOTTOM),
                            ..default()
                        },
                        Name::new("Unit Info Title"),
                    ));

                    unit_parent.spawn((
                        unit_info_section::UnitTypeText,
                        Text::new("Type: Unknown"),
                        TextFont {
                            font_size: SUBTITLE_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_PRIMARY),
                        Node {
                            margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                            ..default()
                        },
                    ));

                    unit_parent.spawn((
                        unit_info_section::UnitHealthText,
                        Text::new("Health: 0/0"),
                        TextFont {
                            font_size: BODY_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_SECONDARY),
                        Node {
                            margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                            ..default()
                        },
                    ));

                    unit_parent.spawn((
                        unit_info_section::UnitStrengthText,
                        Text::new("Strength: 0"),
                        TextFont {
                            font_size: BODY_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_SECONDARY),
                        Node {
                            margin: UiRect::bottom(TEXT_MARGIN_BOTTOM),
                            ..default()
                        },
                    ));

                    unit_parent.spawn((
                        unit_info_section::UnitMovementText,
                        Text::new("Movement: 0/0"),
                        TextFont {
                            font_size: BODY_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_SECONDARY),
                    ));
                });
        });
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

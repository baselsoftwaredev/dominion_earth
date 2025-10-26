//! Native Bevy UI implementation for the left side panel.
//!
//! Displays game panel (Next Turn button), production menu, and unit info.

use bevy::prelude::*;
use core_sim::{
    Civilization, MilitaryUnit, PlayerControlled, PlayerProductionOrder, ProductionQueue,
    RequestTurnAdvance, UnitType,
};

use crate::production_input::SelectedCapital;
use crate::ui::constants::display_layout;

// ============================================================================
// Marker Components
// ============================================================================

/// Marker component for the left panel container
#[derive(Component)]
pub struct LeftPanel;

// Game Panel Components
/// Marker component for the game panel section
#[derive(Component)]
pub struct GamePanel;

/// Marker component for the Next Turn button
#[derive(Component)]
pub struct NextTurnButton;

// Production Menu Components
/// Marker component for the production menu section
#[derive(Component)]
pub struct ProductionMenuPanel;

#[derive(Component)]
pub struct ProductionCapitalNameText;

#[derive(Component)]
pub struct ProductionCivNameText;

#[derive(Component)]
pub struct ProductionGoldText;

#[derive(Component)]
pub struct ProductionProductionText;

#[derive(Component)]
pub struct CurrentProductionNameText;

#[derive(Component)]
pub struct CurrentProductionProgressText;

#[derive(Component)]
pub struct ProductionQueueLengthText;

/// Marker for Infantry production button
#[derive(Component)]
pub struct InfantryButton;

/// Marker for Archer production button
#[derive(Component)]
pub struct ArcherButton;

/// Marker for Cavalry production button
#[derive(Component)]
pub struct CavalryButton;

// Unit Info Components
/// Marker component for the unit info panel section
#[derive(Component)]
pub struct UnitInfoPanel;

#[derive(Component)]
pub struct UnitNameText;

#[derive(Component)]
pub struct UnitAttackText;

#[derive(Component)]
pub struct UnitDefenseText;

#[derive(Component)]
pub struct UnitHealthText;

#[derive(Component)]
pub struct UnitRangeText;

#[derive(Component)]
pub struct UnitMovementText;

#[derive(Component)]
pub struct UnitFatigueText;

#[derive(Component)]
pub struct UnitTypeText;

#[derive(Component)]
pub struct UnitStrengthText;

#[derive(Component)]
pub struct UnitSupplyText;

#[derive(Component)]
pub struct UnitDecayText;

#[derive(Component)]
pub struct UnitExperienceText;

#[derive(Component)]
pub struct UnitEffectiveAttackText;

#[derive(Component)]
pub struct UnitEffectiveDefenseText;

// ============================================================================
// Setup System
// ============================================================================

/// Spawn the left side panel UI hierarchy
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
            BackgroundColor(Color::srgba(0.102, 0.102, 0.102, 1.0)), // #1a1a1a
            Name::new("Left Panel"),
        ))
        .with_children(|parent| {
            // Game Panel (Your Empire + Next Turn Button)
            parent
                .spawn((
                    GamePanel,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                    BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    BorderRadius::all(Val::Px(8.0)),
                    Name::new("Game Panel"),
                ))
                .with_children(|game_parent| {
                    // Panel title
                    game_parent.spawn((
                        Text::new("Your Empire"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Node {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                        Name::new("Game Panel Title"),
                    ));

                    // Next Turn Button
                    game_parent
                        .spawn((
                            NextTurnButton,
                            Button,
                            Node {
                                height: Val::Px(50.0),
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(10.0)),
                                margin: UiRect::bottom(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                            BorderColor::from(Color::srgba(0.4, 0.4, 0.4, 1.0)),     // #666666
                            BorderRadius::all(Val::Px(5.0)),
                            Name::new("Next Turn Button"),
                        ))
                        .with_children(|button_parent| {
                            button_parent.spawn((
                                Text::new("Next Turn"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });

            // Production Menu Panel
            parent
                .spawn((
                    ProductionMenuPanel,
                    Node {
                        display: Display::None, // Hidden by default
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        max_height: Val::Px(500.0),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                    BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    BorderRadius::all(Val::Px(8.0)),
                    Name::new("Production Menu Panel"),
                ))
                .with_children(|menu_parent| {
                    // Header
                    menu_parent.spawn((
                        Text::new("Production Menu"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Node {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                        Name::new("Production Menu Title"),
                    ));

                    // Capital Information Container
                    menu_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            Name::new("Capital Info Container"),
                        ))
                        .with_children(|info_parent| {
                            info_parent.spawn((
                                ProductionCapitalNameText,
                                Text::new("Capital: Unknown"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    margin: UiRect::bottom(Val::Px(5.0)),
                                    ..default()
                                },
                            ));

                            info_parent.spawn((
                                ProductionCivNameText,
                                Text::new("Civilization: Unknown"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    margin: UiRect::bottom(Val::Px(5.0)),
                                    ..default()
                                },
                            ));

                            info_parent.spawn((
                                ProductionGoldText,
                                Text::new("Gold: 0"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)), // #cccccc
                                Node {
                                    margin: UiRect::bottom(Val::Px(5.0)),
                                    ..default()
                                },
                            ));

                            info_parent.spawn((
                                ProductionProductionText,
                                Text::new("Production: 0"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)), // #cccccc
                            ));
                        });

                    // Separator
                    menu_parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(2.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                        Name::new("Separator"),
                    ));

                    // Available Units Section
                    menu_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                            Name::new("Available Units"),
                        ))
                        .with_children(|units_parent| {
                            units_parent.spawn((
                                Text::new("Available Units:"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            // Infantry Button
                            units_parent
                                .spawn((
                                    InfantryButton,
                                    Button,
                                    Node {
                                        height: Val::Px(40.0),
                                        width: Val::Percent(100.0),
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        margin: UiRect::bottom(Val::Px(5.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                                    BorderColor::from(Color::srgba(0.4, 0.4, 0.4, 1.0)), // #666666
                                    BorderRadius::all(Val::Px(5.0)),
                                    Name::new("Infantry Button"),
                                ))
                                .with_children(|button_parent| {
                                    button_parent.spawn((
                                        Text::new("Infantry"),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));

                                    button_parent.spawn((
                                        Text::new("20 gold, 15 production"),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgba(0.6, 0.6, 0.6, 1.0)), // #999999
                                    ));
                                });

                            // Archer Button
                            units_parent
                                .spawn((
                                    ArcherButton,
                                    Button,
                                    Node {
                                        height: Val::Px(40.0),
                                        width: Val::Percent(100.0),
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        margin: UiRect::bottom(Val::Px(5.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                                    BorderColor::from(Color::srgba(0.4, 0.4, 0.4, 1.0)), // #666666
                                    BorderRadius::all(Val::Px(5.0)),
                                    Name::new("Archer Button"),
                                ))
                                .with_children(|button_parent| {
                                    button_parent.spawn((
                                        Text::new("Archer"),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));

                                    button_parent.spawn((
                                        Text::new("25 gold, 20 production"),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgba(0.6, 0.6, 0.6, 1.0)), // #999999
                                    ));
                                });

                            // Cavalry Button
                            units_parent
                                .spawn((
                                    CavalryButton,
                                    Button,
                                    Node {
                                        height: Val::Px(40.0),
                                        width: Val::Percent(100.0),
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        margin: UiRect::bottom(Val::Px(5.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                                    BorderColor::from(Color::srgba(0.4, 0.4, 0.4, 1.0)), // #666666
                                    BorderRadius::all(Val::Px(5.0)),
                                    Name::new("Cavalry Button"),
                                ))
                                .with_children(|button_parent| {
                                    button_parent.spawn((
                                        Text::new("Cavalry"),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));

                                    button_parent.spawn((
                                        Text::new("40 gold, 30 production"),
                                        TextFont {
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgba(0.6, 0.6, 0.6, 1.0)), // #999999
                                    ));
                                });
                        });

                    // Current Production Section
                    menu_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                            Name::new("Current Production"),
                        ))
                        .with_children(|prod_parent| {
                            prod_parent.spawn((
                                Text::new("Currently Producing:"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            prod_parent.spawn((
                                CurrentProductionNameText,
                                Text::new("None"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    margin: UiRect::bottom(Val::Px(5.0)),
                                    ..default()
                                },
                            ));

                            prod_parent.spawn((
                                CurrentProductionProgressText,
                                Text::new("Progress: 0%"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)), // #cccccc
                            ));
                        });

                    // Production Queue Section
                    menu_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                            Name::new("Production Queue"),
                        ))
                        .with_children(|queue_parent| {
                            queue_parent.spawn((
                                Text::new("Production Queue:"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            queue_parent.spawn((
                                ProductionQueueLengthText,
                                Text::new("Items queued: 0"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)), // #cccccc
                            ));
                        });

                    // Separator
                    menu_parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(2.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    ));

                    // Controls hint
                    menu_parent.spawn((
                        Text::new("Press [Esc] to close | Click buttons to queue units"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.6, 0.6, 0.6, 1.0)), // #999999
                    ));
                });

            // Unit Info Panel
            parent
                .spawn((
                    UnitInfoPanel,
                    Node {
                        display: Display::None, // Hidden by default
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)), // #2d2d2d
                    BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0)), // #444444
                    BorderRadius::all(Val::Px(8.0)),
                    Name::new("Unit Info Panel"),
                ))
                .with_children(|unit_parent| {
                    // Header
                    unit_parent.spawn((
                        Text::new("Unit Information"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0)), // #ffcc00
                        Node {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                        Name::new("Unit Info Title"),
                    ));

                    // Unit Type
                    unit_parent.spawn((
                        UnitTypeText,
                        Text::new("Type: Unknown"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                    ));

                    // Unit Health
                    unit_parent.spawn((
                        UnitHealthText,
                        Text::new("Health: 0/0"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)), // #cccccc
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                    ));

                    // Unit Strength
                    unit_parent.spawn((
                        UnitStrengthText,
                        Text::new("Strength: 0"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)), // #cccccc
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                    ));

                    // Unit Movement
                    unit_parent.spawn((
                        UnitMovementText,
                        Text::new("Movement: 0/0"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)), // #cccccc
                    ));
                });
        });
}

// ============================================================================
// Button Interaction Systems
// ============================================================================

/// Handle Next Turn button interactions
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
                *background = BackgroundColor(Color::srgba(0.0, 0.667, 0.667, 1.0)); // #0aa
                turn_advance_events.write(RequestTurnAdvance);
                info!("Player requested turn advancement");
                crate::audio::play_sound_effect(&mut commands, &asset_server, "sounds/click.ogg");
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::srgba(0.251, 0.251, 0.251, 1.0)); // #404040
                *border = BorderColor::all(Color::srgba(1.0, 0.8, 0.0, 1.0)); // #ffcc00
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)); // #2d2d2d
                *border = BorderColor::all(Color::srgba(0.4, 0.4, 0.4, 1.0)); // #666666
            }
        }
    }
}

/// Handle Infantry button interactions
pub fn handle_infantry_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<InfantryButton>)>,
    mut production_orders: MessageWriter<PlayerProductionOrder>,
    selected_capital: Res<SelectedCapital>,
    mut civilizations: Query<&mut Civilization>,
    mut production_queues: Query<&mut ProductionQueue>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            handle_unit_production(
                UnitType::Infantry,
                &mut production_orders,
                &selected_capital,
                &mut civilizations,
                &mut production_queues,
            );
        }
    }
}

/// Handle Archer button interactions
pub fn handle_archer_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ArcherButton>)>,
    mut production_orders: MessageWriter<PlayerProductionOrder>,
    selected_capital: Res<SelectedCapital>,
    mut civilizations: Query<&mut Civilization>,
    mut production_queues: Query<&mut ProductionQueue>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            handle_unit_production(
                UnitType::Archer,
                &mut production_orders,
                &selected_capital,
                &mut civilizations,
                &mut production_queues,
            );
        }
    }
}

/// Handle Cavalry button interactions
pub fn handle_cavalry_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CavalryButton>)>,
    mut production_orders: MessageWriter<PlayerProductionOrder>,
    selected_capital: Res<SelectedCapital>,
    mut civilizations: Query<&mut Civilization>,
    mut production_queues: Query<&mut ProductionQueue>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            handle_unit_production(
                UnitType::Cavalry,
                &mut production_orders,
                &selected_capital,
                &mut civilizations,
                &mut production_queues,
            );
        }
    }
}

/// Common logic for handling unit production
fn handle_unit_production(
    unit_type: UnitType,
    production_orders: &mut MessageWriter<PlayerProductionOrder>,
    selected_capital: &SelectedCapital,
    civilizations: &mut Query<&mut Civilization>,
    production_queues: &mut Query<&mut ProductionQueue>,
) {
    if !selected_capital.show_production_menu {
        return;
    }

    let (capital_entity, civ_entity) =
        match (selected_capital.capital_entity, selected_capital.civ_entity) {
            (Some(cap), Some(civ)) => (cap, civ),
            _ => return,
        };

    let unit_production_item = core_sim::ProductionItem::Unit(unit_type);
    let unit_cost = unit_production_item.gold_cost();

    let (mut civilization, mut production_queue) = match (
        civilizations.get_mut(civ_entity),
        production_queues.get_mut(capital_entity),
    ) {
        (Ok(civ), Ok(queue)) => (civ, queue),
        _ => return,
    };

    if civilization.economy.gold < unit_cost as f32 {
        warn!(
            "Insufficient gold to queue {:?}. Cost: {}, Available: {}",
            unit_type, unit_cost, civilization.economy.gold
        );
        return;
    }

    civilization.economy.gold -= unit_cost as f32;
    production_queue.add_to_queue(unit_production_item.clone());

    production_orders.write(PlayerProductionOrder {
        capital_entity,
        civ_entity,
        item: unit_production_item,
    });

    info!("Queued {:?} for production", unit_type);
}

/// Apply button hover effects for production buttons
pub fn update_production_button_visuals(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (
            Changed<Interaction>,
            Or<(
                With<InfantryButton>,
                With<ArcherButton>,
                With<CavalryButton>,
            )>,
        ),
    >,
) {
    for (interaction, mut background, mut border) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(Color::srgba(0.0, 0.667, 0.667, 1.0));
                // #0aa
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::srgba(0.251, 0.251, 0.251, 1.0)); // #404040
                *border = BorderColor::all(Color::srgba(1.0, 0.8, 0.0, 1.0)); // #ffcc00
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgba(0.176, 0.176, 0.176, 1.0)); // #2d2d2d
                *border = BorderColor::all(Color::srgba(0.4, 0.4, 0.4, 1.0)); // #666666
            }
        }
    }
}

// ============================================================================
// Update Systems
// ============================================================================

/// Update production menu visibility and data
pub fn update_production_menu(
    selected_capital: Res<SelectedCapital>,
    civilizations: Query<&Civilization>,
    production_queues: Query<&ProductionQueue>,
    mut menu_query: Query<&mut Node, With<ProductionMenuPanel>>,
    mut capital_name_text: Query<
        &mut Text,
        (
            With<ProductionCapitalNameText>,
            Without<ProductionCivNameText>,
            Without<ProductionGoldText>,
            Without<ProductionProductionText>,
            Without<CurrentProductionNameText>,
            Without<CurrentProductionProgressText>,
            Without<ProductionQueueLengthText>,
        ),
    >,
    mut civ_name_text: Query<
        &mut Text,
        (
            With<ProductionCivNameText>,
            Without<ProductionCapitalNameText>,
            Without<ProductionGoldText>,
            Without<ProductionProductionText>,
            Without<CurrentProductionNameText>,
            Without<CurrentProductionProgressText>,
            Without<ProductionQueueLengthText>,
        ),
    >,
    mut gold_text: Query<
        &mut Text,
        (
            With<ProductionGoldText>,
            Without<ProductionCapitalNameText>,
            Without<ProductionCivNameText>,
            Without<ProductionProductionText>,
            Without<CurrentProductionNameText>,
            Without<CurrentProductionProgressText>,
            Without<ProductionQueueLengthText>,
        ),
    >,
    mut production_text: Query<
        &mut Text,
        (
            With<ProductionProductionText>,
            Without<ProductionCapitalNameText>,
            Without<ProductionCivNameText>,
            Without<ProductionGoldText>,
            Without<CurrentProductionNameText>,
            Without<CurrentProductionProgressText>,
            Without<ProductionQueueLengthText>,
        ),
    >,
    mut current_prod_name_text: Query<
        &mut Text,
        (
            With<CurrentProductionNameText>,
            Without<ProductionCapitalNameText>,
            Without<ProductionCivNameText>,
            Without<ProductionGoldText>,
            Without<ProductionProductionText>,
            Without<CurrentProductionProgressText>,
            Without<ProductionQueueLengthText>,
        ),
    >,
    mut current_prod_progress_text: Query<
        &mut Text,
        (
            With<CurrentProductionProgressText>,
            Without<ProductionCapitalNameText>,
            Without<ProductionCivNameText>,
            Without<ProductionGoldText>,
            Without<ProductionProductionText>,
            Without<CurrentProductionNameText>,
            Without<ProductionQueueLengthText>,
        ),
    >,
    mut queue_length_text: Query<
        &mut Text,
        (
            With<ProductionQueueLengthText>,
            Without<ProductionCapitalNameText>,
            Without<ProductionCivNameText>,
            Without<ProductionGoldText>,
            Without<ProductionProductionText>,
            Without<CurrentProductionNameText>,
            Without<CurrentProductionProgressText>,
        ),
    >,
) {
    if selected_capital.is_changed() {
        // Update visibility
        if let Some(mut node) = menu_query.iter_mut().next() {
            node.display = if selected_capital.show_production_menu {
                Display::Flex
            } else {
                Display::None
            };
        }

        // Update content if visible
        if selected_capital.show_production_menu {
            if let (Some(capital_entity), Some(civ_entity)) =
                (selected_capital.capital_entity, selected_capital.civ_entity)
            {
                // Update capital name
                if let Some(mut text) = capital_name_text.iter_mut().next() {
                    **text = "Capital: Capital".to_string();
                }

                // Update civilization info
                if let Ok(civ) = civilizations.get(civ_entity) {
                    if let Some(mut text) = civ_name_text.iter_mut().next() {
                        **text = format!("Civilization: {}", civ.name);
                    }

                    if let Some(mut text) = gold_text.iter_mut().next() {
                        **text = format!("Gold: {}", civ.economy.gold as i32);
                    }

                    if let Some(mut text) = production_text.iter_mut().next() {
                        **text = format!("Production: {}", civ.economy.production as i32);
                    }
                }

                // Update production queue info
                if let Ok(queue) = production_queues.get(capital_entity) {
                    if let Some(mut text) = current_prod_name_text.iter_mut().next() {
                        **text = queue
                            .current_production
                            .as_ref()
                            .map(|item| item.name().to_string())
                            .unwrap_or_else(|| "None".to_string());
                    }

                    if let Some(mut text) = current_prod_progress_text.iter_mut().next() {
                        let progress = (queue.get_progress_percentage() * 100.0) as i32;
                        **text = format!("Progress: {}%", progress);
                    }

                    if let Some(mut text) = queue_length_text.iter_mut().next() {
                        **text = format!("Items queued: {}", queue.queue_length());
                    }
                }
            }
        }
    }
}

/// Update unit info panel visibility and data
pub fn update_unit_info(
    selected_unit: Res<core_sim::SelectedUnit>,
    units_query: Query<&MilitaryUnit>,
    mut panel_query: Query<&mut Node, With<UnitInfoPanel>>,
    mut unit_name_text: Query<
        &mut Text,
        (
            With<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut attack_text: Query<
        &mut Text,
        (
            With<UnitAttackText>,
            Without<UnitNameText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut defense_text: Query<
        &mut Text,
        (
            With<UnitDefenseText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut health_text: Query<
        &mut Text,
        (
            With<UnitHealthText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut range_text: Query<
        &mut Text,
        (
            With<UnitRangeText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut movement_text: Query<
        &mut Text,
        (
            With<UnitMovementText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut fatigue_text: Query<
        &mut Text,
        (
            With<UnitFatigueText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut supply_text: Query<
        &mut Text,
        (
            With<UnitSupplyText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut decay_text: Query<
        &mut Text,
        (
            With<UnitDecayText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut experience_text: Query<
        &mut Text,
        (
            With<UnitExperienceText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
        ),
    >,
) {
    if selected_unit.is_changed() {
        // Update visibility
        if let Some(mut node) = panel_query.iter_mut().next() {
            node.display = if selected_unit.unit_entity.is_some() {
                Display::Flex
            } else {
                Display::None
            };
        }

        // Update content if visible
        if let Some(unit_entity) = selected_unit.unit_entity {
            if let Ok(unit) = units_query.get(unit_entity) {
                if let Some(mut text) = unit_name_text.iter_mut().next() {
                    **text = format!("Unit #{} - {}", unit.id, unit.unit_type.name());
                }

                if let Some(mut text) = attack_text.iter_mut().next() {
                    **text = format!(
                        "Attack: {:.1} (Effective: {:.1})",
                        unit.attack,
                        unit.effective_attack()
                    );
                }

                if let Some(mut text) = defense_text.iter_mut().next() {
                    **text = format!(
                        "Defense: {:.1} (Effective: {:.1})",
                        unit.defense,
                        unit.effective_defense()
                    );
                }

                if let Some(mut text) = health_text.iter_mut().next() {
                    **text = format!("Health: {:.0} / {:.0}", unit.health, unit.max_health);
                }

                if let Some(mut text) = range_text.iter_mut().next() {
                    **text = format!("Range: {}", unit.range);
                }

                if let Some(mut text) = movement_text.iter_mut().next() {
                    **text = format!(
                        "Remaining: {} / {}",
                        unit.movement_remaining, unit.movement_range
                    );
                }

                if let Some(mut text) = fatigue_text.iter_mut().next() {
                    **text = format!("Fatigue: {:.0}%", unit.fatigue * 100.0);
                }

                if let Some(mut text) = supply_text.iter_mut().next() {
                    **text = format!("Supply: {:.0}%", unit.supply * 100.0);
                }

                if let Some(mut text) = decay_text.iter_mut().next() {
                    **text = format!("Decay: {:.0}%", unit.decay * 100.0);
                }

                if let Some(mut text) = experience_text.iter_mut().next() {
                    **text = format!("Experience: {:.0}%", unit.experience * 100.0);
                }
            }
        }
    }
}

use bevy::prelude::*;
use core_sim::{Civilization, PlayerProductionOrder, ProductionQueue, UnitType};

use super::constants::*;
use crate::production_input::SelectedCapital;

// Component markers for production menu UI elements
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

#[derive(Component)]
pub struct InfantryButton;

#[derive(Component)]
pub struct ArcherButton;

#[derive(Component)]
pub struct CavalryButton;

/// Spawns the complete production menu panel and returns its entity
pub fn spawn_production_menu_panel(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            ProductionMenuPanel,
            Node {
                display: Display::None,
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(PANEL_PADDING),
                margin: UiRect::all(PANEL_MARGIN),
                border: UiRect::all(PANEL_BORDER_WIDTH),
                flex_shrink: 0.0,
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
                        ProductionCapitalNameText,
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
                        ProductionCivNameText,
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
                        ProductionGoldText,
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
                        ProductionProductionText,
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
                            InfantryButton,
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
                            ArcherButton,
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
                            CavalryButton,
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
                        CurrentProductionNameText,
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
                        CurrentProductionProgressText,
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
                        ProductionQueueLengthText,
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
        })
        .id()
}

// Systems for handling production menu interactions

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
                *background = BackgroundColor(BUTTON_PRESSED_BACKGROUND);
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
        if let Some(mut node) = menu_query.iter_mut().next() {
            node.display = if selected_capital.show_production_menu {
                Display::Flex
            } else {
                Display::None
            };
        }

        if selected_capital.show_production_menu {
            if let (Some(capital_entity), Some(civ_entity)) =
                (selected_capital.capital_entity, selected_capital.civ_entity)
            {
                if let Some(mut text) = capital_name_text.iter_mut().next() {
                    **text = "Capital: Capital".to_string();
                }

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

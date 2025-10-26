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

// Component markers for production menu UI elements

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

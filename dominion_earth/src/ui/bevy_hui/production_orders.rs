use crate::debug_println;
use crate::debug_utils::DebugLogging;
use crate::production_input::SelectedCapital;
use bevy::prelude::*;
use bevy_hui::prelude::*;
use core_sim::{Civilization, PlayerProductionOrder};

use super::constants;

/// Register production order function handlers for unit creation
pub fn register_production_order_functions(html_functions: &mut HtmlFunctions) {
    register_infantry_production_function(html_functions);
    register_archer_production_function(html_functions);
    register_cavalry_production_function(html_functions);
}

/// Register infantry production function for UI button binding
fn register_infantry_production_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "queue_infantry",
        |In(entity): In<Entity>,
         mut production_orders: EventWriter<PlayerProductionOrder>,
         selected_capital: Res<SelectedCapital>,
         mut civilizations_query: Query<&mut Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>,
         mut production_queues: Query<&mut core_sim::ProductionQueue>| {
            process_unit_production_order(
                core_sim::UnitType::Infantry,
                &mut production_orders,
                &selected_capital,
                &mut civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
                &mut production_queues,
            );
        },
    );
}

/// Register archer production function for UI button binding
fn register_archer_production_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "queue_archer",
        |In(entity): In<Entity>,
         mut production_orders: EventWriter<PlayerProductionOrder>,
         selected_capital: Res<SelectedCapital>,
         mut civilizations_query: Query<&mut Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>,
         mut production_queues: Query<&mut core_sim::ProductionQueue>| {
            process_unit_production_order(
                core_sim::UnitType::Archer,
                &mut production_orders,
                &selected_capital,
                &mut civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
                &mut production_queues,
            );
        },
    );
}

/// Register cavalry production function for UI button binding  
fn register_cavalry_production_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "queue_cavalry",
        |In(entity): In<Entity>,
         mut production_orders: EventWriter<PlayerProductionOrder>,
         selected_capital: Res<SelectedCapital>,
         mut civilizations_query: Query<&mut Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>,
         mut production_queues: Query<&mut core_sim::ProductionQueue>| {
            process_unit_production_order(
                core_sim::UnitType::Cavalry,
                &mut production_orders,
                &selected_capital,
                &mut civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
                &mut production_queues,
            );
        },
    );
}

/// Process unit production order with unified logic for all unit types
fn process_unit_production_order(
    unit_type: core_sim::UnitType,
    production_orders: &mut EventWriter<PlayerProductionOrder>,
    selected_capital: &SelectedCapital,
    civilizations_query: &mut Query<&mut Civilization>,
    debug_logging: &DebugLogging,
    commands: &mut Commands,
    template_properties: &mut Query<&mut TemplateProperties>,
    ui_entities: &Query<Entity, (With<TemplateProperties>, With<Name>)>,
    entity_names: &Query<&Name>,
    production_queues: &mut Query<&mut core_sim::ProductionQueue>,
) {
    let unit_type_name = format!("{:?}", unit_type);
    debug_println!(debug_logging, "{} button clicked!", unit_type_name);

    let (capital_entity, civilization_entity) =
        match extract_selected_capital_entities(selected_capital, debug_logging, &unit_type_name) {
            Some(entities) => entities,
            None => return,
        };

    let unit_production_item = core_sim::ProductionItem::Unit(unit_type);
    let unit_cost = unit_production_item.gold_cost();

    // Get mutable access to civilization and production queue
    let (mut civilization, mut production_queue) = match (
        civilizations_query.get_mut(civilization_entity),
        production_queues.get_mut(capital_entity),
    ) {
        (Ok(civ), Ok(queue)) => (civ, queue),
        _ => {
            debug_println!(
                debug_logging,
                "Failed to get civilization or production queue for {}",
                unit_type_name
            );
            return;
        }
    };

    debug_println!(
        debug_logging,
        "{} cost: {}, Player gold: {}, Capital Entity: {:?}",
        unit_type_name,
        unit_cost,
        civilization.economy.gold,
        capital_entity
    );

    if civilization.economy.gold >= unit_cost {
        // Deduct gold immediately
        civilization.economy.gold -= unit_cost;

        // Add item to queue immediately
        production_queue.add_to_queue(unit_production_item.clone());
        debug_println!(
            debug_logging,
            "Added {} to queue. Queue vec length: {}, Current production: {:?}, Total queue_length(): {}",
            unit_type_name,
            production_queue.queue.len(),
            production_queue.current_production,
            production_queue.queue_length()
        );

        // Check if we need to start production
        if !production_queue.is_producing() {
            production_queue.start_next_production();
            debug_println!(
                debug_logging,
                "Started production. Queue vec length: {}, Current production: {:?}, Total queue_length(): {}",
                production_queue.queue.len(),
                production_queue.current_production,
                production_queue.queue_length()
            );
        }

        // Send production order event for consistency (though not strictly needed now)
        send_production_order(
            production_orders,
            capital_entity,
            civilization_entity,
            unit_production_item,
        );
        debug_println!(
            debug_logging,
            "{} production order processed!",
            unit_type_name
        );

        let updated_gold_amount = civilization.economy.gold as u32;
        let updated_queue_length = production_queue.queue_length();

        update_ui_panels_with_production_changes(
            commands,
            template_properties,
            ui_entities,
            entity_names,
            updated_gold_amount,
            updated_queue_length,
            &production_queue,
        );
        debug_println!(
            debug_logging,
            "UI updated with new gold: {} and queue length: {}",
            updated_gold_amount,
            updated_queue_length
        );
    } else {
        debug_println!(
            debug_logging,
            "Insufficient gold for {}!",
            unit_type_name.to_lowercase()
        );
    }
}

/// Extract selected capital entities with validation
fn extract_selected_capital_entities(
    selected_capital: &SelectedCapital,
    debug_logging: &DebugLogging,
    unit_type_name: &str,
) -> Option<(Entity, Entity)> {
    match (selected_capital.capital_entity, selected_capital.civ_entity) {
        (Some(capital_entity), Some(civ_entity)) => Some((capital_entity, civ_entity)),
        _ => {
            debug_println!(
                debug_logging,
                "No capital selected for {} production!",
                unit_type_name.to_lowercase()
            );
            None
        }
    }
}

/// Send production order to event writer
fn send_production_order(
    production_orders: &mut EventWriter<PlayerProductionOrder>,
    capital_entity: Entity,
    civilization_entity: Entity,
    production_item: core_sim::ProductionItem,
) {
    production_orders.write(PlayerProductionOrder {
        capital_entity,
        civ_entity: civilization_entity,
        item: production_item,
    });
}

/// Update UI panels with new gold amount after production order
fn update_ui_panels_with_new_gold(
    commands: &mut Commands,
    template_properties: &mut Query<&mut TemplateProperties>,
    ui_entities: &Query<Entity, (With<TemplateProperties>, With<Name>)>,
    entity_names: &Query<&Name>,
    new_gold_amount: u32,
) {
    for ui_entity in ui_entities.iter() {
        let (mut properties, entity_name) = match (
            template_properties.get_mut(ui_entity),
            entity_names.get(ui_entity),
        ) {
            (Ok(props), Ok(name)) => (props, name),
            _ => continue,
        };

        let entity_name_string = entity_name.as_str();
        update_panel_gold_property_by_name(&mut properties, entity_name_string, new_gold_amount);
        commands.trigger_targets(CompileContextEvent, ui_entity);
    }
}

/// Update UI panels with new gold amount and production queue length after production order
fn update_ui_panels_with_production_changes(
    commands: &mut Commands,
    template_properties: &mut Query<&mut TemplateProperties>,
    ui_entities: &Query<Entity, (With<TemplateProperties>, With<Name>)>,
    entity_names: &Query<&Name>,
    new_gold_amount: u32,
    new_queue_length: usize,
    production_queue: &core_sim::ProductionQueue,
) {
    for ui_entity in ui_entities.iter() {
        let (mut properties, entity_name) = match (
            template_properties.get_mut(ui_entity),
            entity_names.get(ui_entity),
        ) {
            (Ok(props), Ok(name)) => (props, name),
            _ => continue,
        };

        let entity_name_string = entity_name.as_str();
        update_panel_gold_property_by_name(&mut properties, entity_name_string, new_gold_amount);
        update_panel_queue_length_property_by_name(
            &mut properties,
            entity_name_string,
            new_queue_length,
        );
        // Note: Current production properties are now handled by property_updates.rs system
        commands.trigger_targets(CompileContextEvent, ui_entity);
    }
}

/// Update gold property based on panel name
fn update_panel_gold_property_by_name(
    template_properties: &mut TemplateProperties,
    panel_name: &str,
    gold_amount: u32,
) {
    match panel_name {
        constants::ui_component_names::TOP_PANEL_NAME => {
            template_properties.insert(
                constants::ui_properties::PLAYER_GOLD_PROPERTY.to_string(),
                gold_amount.to_string(),
            );
        }
        constants::ui_component_names::LEFT_SIDE_PANEL_NAME => {
            template_properties.insert(
                constants::ui_properties::CIVILIZATION_GOLD_PROPERTY.to_string(),
                gold_amount.to_string(),
            );
        }
        _ => {}
    }
}

/// Update production queue length property based on panel name
fn update_panel_queue_length_property_by_name(
    template_properties: &mut TemplateProperties,
    panel_name: &str,
    queue_length: usize,
) {
    match panel_name {
        constants::ui_component_names::LEFT_SIDE_PANEL_NAME
        | constants::ui_component_names::PRODUCTION_MENU_NAME => {
            template_properties.insert(
                constants::ui_properties::PRODUCTION_QUEUE_LENGTH_PROPERTY.to_string(),
                queue_length.to_string(),
            );
        }
        _ => {}
    }
}

/// Update current production properties based on panel name
fn update_panel_current_production_properties_by_name(
    template_properties: &mut TemplateProperties,
    panel_name: &str,
    production_queue: &core_sim::ProductionQueue,
) {
    match panel_name {
        constants::ui_component_names::LEFT_SIDE_PANEL_NAME
        | constants::ui_component_names::PRODUCTION_MENU_NAME => {
            // Update current production name
            let current_production_name =
                if let Some(ref current) = production_queue.current_production {
                    match current {
                        core_sim::ProductionItem::Unit(unit_type) => format!("{:?}", unit_type),
                        core_sim::ProductionItem::Building(building_type) => {
                            format!("{:?}", building_type)
                        }
                    }
                } else {
                    "None".to_string()
                };

            template_properties.insert(
                constants::ui_properties::CURRENT_PRODUCTION_NAME_PROPERTY.to_string(),
                current_production_name,
            );

            // Update current production progress
            let progress_percentage = (production_queue.get_progress_percentage() * 100.0) as u32;
            template_properties.insert(
                constants::ui_properties::CURRENT_PRODUCTION_PROGRESS_PROPERTY.to_string(),
                progress_percentage.to_string(),
            );
        }
        _ => {}
    }
}

pub fn handle_production_updated_events(
    mut production_events: EventReader<core_sim::ProductionUpdated>,
    mut commands: Commands,
    mut template_properties: Query<&mut TemplateProperties>,
    ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
    entity_names: Query<&Name>,
    production_queues: Query<&core_sim::ProductionQueue>,
) {
    for event in production_events.read() {
        if let Ok(production_queue) = production_queues.get(event.capital_entity) {
            update_ui_panels_with_production_queue_changes(
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
                production_queue,
            );
        }
    }
}

fn update_ui_panels_with_production_queue_changes(
    commands: &mut Commands,
    template_properties: &mut Query<&mut TemplateProperties>,
    ui_entities: &Query<Entity, (With<TemplateProperties>, With<Name>)>,
    entity_names: &Query<&Name>,
    production_queue: &core_sim::ProductionQueue,
) {
    for ui_entity in ui_entities.iter() {
        let (mut properties, entity_name) = match (
            template_properties.get_mut(ui_entity),
            entity_names.get(ui_entity),
        ) {
            (Ok(props), Ok(name)) => (props, name),
            _ => continue,
        };

        let entity_name_string = entity_name.as_str();
        update_panel_queue_length_property_by_name(
            &mut properties,
            entity_name_string,
            production_queue.queue_length(),
        );
        // Note: Current production properties are now handled by property_updates.rs system
        commands.trigger_targets(CompileContextEvent, ui_entity);
    }
}

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
         civilizations_query: Query<&Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>| {
            process_unit_production_order(
                core_sim::UnitType::Infantry,
                &mut production_orders,
                &selected_capital,
                &civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
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
         civilizations_query: Query<&Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>| {
            process_unit_production_order(
                core_sim::UnitType::Archer,
                &mut production_orders,
                &selected_capital,
                &civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
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
         civilizations_query: Query<&Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>| {
            process_unit_production_order(
                core_sim::UnitType::Cavalry,
                &mut production_orders,
                &selected_capital,
                &civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
            );
        },
    );
}

/// Process unit production order with unified logic for all unit types
fn process_unit_production_order(
    unit_type: core_sim::UnitType,
    production_orders: &mut EventWriter<PlayerProductionOrder>,
    selected_capital: &SelectedCapital,
    civilizations_query: &Query<&Civilization>,
    debug_logging: &DebugLogging,
    commands: &mut Commands,
    template_properties: &mut Query<&mut TemplateProperties>,
    ui_entities: &Query<Entity, (With<TemplateProperties>, With<Name>)>,
    entity_names: &Query<&Name>,
) {
    let unit_type_name = format!("{:?}", unit_type);
    debug_println!(debug_logging, "{} button clicked!", unit_type_name);

    let (capital_entity, civilization_entity) =
        match extract_selected_capital_entities(selected_capital, debug_logging, &unit_type_name) {
            Some(entities) => entities,
            None => return,
        };

    let civilization = match civilizations_query.get(civilization_entity) {
        Ok(civ) => civ,
        Err(_) => return,
    };

    let unit_production_item = core_sim::ProductionItem::Unit(unit_type);
    let unit_cost = unit_production_item.gold_cost();

    debug_println!(
        debug_logging,
        "{} cost: {}, Player gold: {}",
        unit_type_name,
        unit_cost,
        civilization.economy.gold
    );

    if civilization.economy.gold >= unit_cost {
        send_production_order(
            production_orders,
            capital_entity,
            civilization_entity,
            unit_production_item,
        );
        debug_println!(debug_logging, "{} production order sent!", unit_type_name);

        let updated_gold_amount = (civilization.economy.gold - unit_cost) as u32;
        update_ui_panels_with_new_gold(
            commands,
            template_properties,
            ui_entities,
            entity_names,
            updated_gold_amount,
        );
        debug_println!(
            debug_logging,
            "UI updated with new gold: {}",
            updated_gold_amount
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

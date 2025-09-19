use crate::{
    components::{
        city::City,
        production::{ProductionItem, ProductionQueue},
        Capital, MilitaryUnit, PlayerActionsComplete, PlayerControlled,
    },
    constants::civilization_management::{PLAYER_CIVILIZATION_ID, STARTING_UNIT_ID_COUNTER},
    resources::CurrentTurn,
    Position, WorldMap,
};
use bevy_ecs::prelude::*;

#[derive(Event)]
pub struct ProductionUpdated {
    pub capital_entity: Entity,
}

pub fn handle_turn_advance_requests(
    mut turn_requests: EventReader<RequestTurnAdvance>,
    mut current_turn: ResMut<CurrentTurn>,
    mut player_actions: ResMut<PlayerActionsComplete>,
    mut units: Query<&mut MilitaryUnit>,
    mut production_query: Query<(Entity, &mut ProductionQueue, &mut City, &Capital, &Position)>,
    mut commands: Commands,
    mut unit_id_counter: Local<u32>,
    world_map: Res<WorldMap>,
    mut production_events: EventWriter<ProductionUpdated>,
) {
    for _request in turn_requests.read() {
        let next_turn_number = calculate_next_turn_number(current_turn.0);

        process_all_city_production_for_turn(
            &mut production_query,
            &mut commands,
            &mut unit_id_counter,
            &world_map,
            next_turn_number,
            &mut production_events,
        );

        advance_current_turn(&mut current_turn);
        reset_all_unit_movement_points(&mut units);
        reset_player_action_tracking(&mut player_actions);

        tracing::info!("Advanced to turn {}", current_turn.0);
    }
}

fn calculate_next_turn_number(current_turn: u32) -> u32 {
    current_turn + 1
}

fn advance_current_turn(current_turn: &mut ResMut<CurrentTurn>) {
    current_turn.0 += 1;
}

fn reset_all_unit_movement_points(units: &mut Query<&mut MilitaryUnit>) {
    for mut unit in units.iter_mut() {
        unit.reset_movement();
    }
}

fn reset_player_action_tracking(player_actions: &mut ResMut<PlayerActionsComplete>) {
    player_actions.reset();
}

fn process_all_city_production_for_turn(
    production_query: &mut Query<(Entity, &mut ProductionQueue, &mut City, &Capital, &Position)>,
    commands: &mut Commands,
    unit_id_counter: &mut Local<u32>,
    world_map: &WorldMap,
    turn_number: u32,
    production_events: &mut EventWriter<ProductionUpdated>,
) {
    for (entity, mut production_queue, mut city, capital, position) in production_query.iter_mut() {
        let had_production_before =
            production_queue.current_production.is_some() || !production_queue.queue.is_empty();

        if let Some(completed_item) = production_queue.add_production(city.production) {
            spawn_completed_production_item(
                commands,
                &completed_item,
                &capital.owner,
                position,
                unit_id_counter,
                &mut city,
                world_map,
                turn_number,
            );
        }

        let has_production_after =
            production_queue.current_production.is_some() || !production_queue.queue.is_empty();

        if had_production_before || has_production_after {
            production_events.write(ProductionUpdated {
                capital_entity: entity,
            });
        }
    }
}

fn process_production_for_turn(
    production_query: &mut Query<(&mut ProductionQueue, &mut City, &Capital, &Position)>,
    commands: &mut Commands,
    unit_id_counter: &mut Local<u32>,
    world_map: &WorldMap,
    turn_number: u32,
) {
    for (mut production_queue, mut city, capital, position) in production_query.iter_mut() {
        if let Some(completed_item) = production_queue.add_production(city.production) {
            spawn_completed_production_item(
                commands,
                &completed_item,
                &capital.owner,
                position,
                unit_id_counter,
                &mut city,
                world_map,
                turn_number,
            );
        }
    }
}

fn spawn_completed_production_item(
    commands: &mut Commands,
    item: &ProductionItem,
    owner: &crate::components::CivId,
    position: &Position,
    unit_id_counter: &mut Local<u32>,
    city: &mut City,
    _world_map: &WorldMap,
    _current_turn: u32,
) {
    match item {
        ProductionItem::Unit(unit_type) => {
            spawn_military_unit_at_position(commands, unit_type, owner, position, unit_id_counter);
        }
        ProductionItem::Building(building_type) => {
            add_building_to_city(city, building_type);
        }
    }
}

fn spawn_military_unit_at_position(
    commands: &mut Commands,
    unit_type: &crate::components::military::UnitType,
    owner: &crate::components::CivId,
    position: &Position,
    unit_id_counter: &mut Local<u32>,
) {
    let unit = crate::MilitaryUnit::new(**unit_id_counter, *owner, *unit_type, *position);
    **unit_id_counter += 1;

    let mut entity_commands = commands.spawn((unit, *position));

    if is_player_controlled_civilization(owner) {
        entity_commands.insert(crate::PlayerControlled);
    }
}

fn add_building_to_city(city: &mut City, building_type: &crate::components::city::BuildingType) {
    city.add_building(building_type.clone());
}

fn is_player_controlled_civilization(owner: &crate::components::CivId) -> bool {
    owner.0 == PLAYER_CIVILIZATION_ID
}

pub fn auto_advance_turn_system(
    player_civs: Query<Entity, With<PlayerControlled>>,
    mut turn_advance: EventWriter<RequestTurnAdvance>,
) {
    if should_auto_advance_turn_for_ai_only_game(&player_civs) {
        turn_advance.write(RequestTurnAdvance);
    }
}

fn should_auto_advance_turn_for_ai_only_game(
    player_civs: &Query<Entity, With<PlayerControlled>>,
) -> bool {
    player_civs.is_empty()
}

#[derive(Event)]
pub struct RequestTurnAdvance;

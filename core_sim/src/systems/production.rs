use crate::{
    City, MilitaryUnit, Position, WorldMap, 
    Capital, Civilization, PlayerControlled, CivId
};
use crate::components::production::{ProductionItem, ProductionQueue, PlayerActionsComplete};
use crate::resources::CurrentTurn;
use bevy::prelude::*;

/// System to process production queues each turn
pub fn process_production_queues(
    mut query: Query<(&mut ProductionQueue, &mut City, &Capital, &Position)>,
    mut commands: Commands,
    mut unit_id_counter: Local<u32>,
    world_map: Res<WorldMap>,
    current_turn: Res<CurrentTurn>,
) {
    for (mut production_queue, mut city, capital, position) in query.iter_mut() {
        if let Some(completed_item) = production_queue.add_production(city.production) {
            spawn_completed_production(
                &mut commands,
                &completed_item,
                &capital.owner,
                position,
                &mut unit_id_counter,
                &mut city,
                &world_map,
                current_turn.0,
            );
        }
    }
}

/// Handle a completed production item
fn spawn_completed_production(
    commands: &mut Commands,
    item: &ProductionItem,
    owner: &crate::components::CivId,
    position: &Position,
    unit_id_counter: &mut u32,
    city: &mut City,
    _world_map: &WorldMap,
    _current_turn: u32,
) {
    match item {
        ProductionItem::Unit(unit_type) => {
            // Spawn new military unit at capital position
            let unit = MilitaryUnit::new(*unit_id_counter, *owner, *unit_type, *position);
            *unit_id_counter += 1;

            // Add PlayerControlled component for player civilizations (CivId(0))
            let mut entity_commands = commands.spawn((unit, *position));
            if owner.0 == 0 {
                entity_commands.insert(PlayerControlled);
            }
        }
        ProductionItem::Building(building_type) => {
            // Add building to city
                        city.add_building(building_type.clone());
        }
    }
}

/// System to check if all player actions are complete
pub fn check_player_actions_complete(
    player_civs: Query<&Civilization, With<PlayerControlled>>,
    player_units: Query<&MilitaryUnit>,
    player_capitals: Query<&ProductionQueue, (With<Capital>, With<PlayerControlled>)>,
    mut player_actions: ResMut<PlayerActionsComplete>,
) {
    // If no player civilizations, all actions are complete
    if player_civs.is_empty() {
        player_actions.all_units_moved = true;
        player_actions.all_productions_queued = true;
        player_actions.update_can_end_turn();
        return;
    }

    // Get the player civilization ID
    let player_civ_id = if let Some(player_civ) = player_civs.iter().next() {
        player_civ.id
    } else {
        player_actions.all_units_moved = true;
        player_actions.all_productions_queued = true;
        player_actions.update_can_end_turn();
        return;
    };

    // Check if all player units have moved or have no movement left
    let mut all_units_moved = true;
    for unit in player_units.iter() {
        if unit.owner == player_civ_id && unit.can_move() {
            all_units_moved = false;
            break;
        }
    }
    player_actions.all_units_moved = all_units_moved;

    // Check if player has made production decisions for this turn
    // Player can either:
    // 1. Have no capitals (no production decisions required)
    // 2. Have made production decisions or explicitly skipped
    player_actions.all_productions_queued = player_capitals.is_empty() || 
        player_actions.production_decisions_made_this_turn;

    player_actions.update_can_end_turn();
}

/// System to initialize production queues for capitals
pub fn initialize_production_queues(
    mut commands: Commands,
    capitals_without_queues: Query<
        (Entity, &Capital),
        (With<Capital>, Without<ProductionQueue>),
    >,
) {
    for (entity, capital) in capitals_without_queues.iter() {
        commands
            .entity(entity)
            .insert(ProductionQueue::new(capital.owner));
    }
}

/// System to handle player production orders
pub fn handle_player_production_orders(
    mut production_orders: EventReader<PlayerProductionOrder>,
    mut production_queues: Query<&mut ProductionQueue>,
    mut civilizations: Query<&mut Civilization>,
    mut player_actions: ResMut<PlayerActionsComplete>,
) {
    for order in production_orders.read() {
        if let Ok(mut queue) = production_queues.get_mut(order.capital_entity) {
            if let Ok(mut civ) = civilizations.get_mut(order.civ_entity) {
                let item_cost = order.item.gold_cost();
                
                // Check if civilization can afford the item
                if civ.economy.gold >= item_cost {
                    civ.economy.gold -= item_cost;
                    queue.add_to_queue(order.item.clone());
                    
                    // Mark that player has made production decisions this turn
                    player_actions.production_decisions_made_this_turn = true;
                }
            }
        }
    }
}

/// System to handle player skipping production this turn
pub fn handle_skip_production(
    mut skip_events: EventReader<SkipProductionThisTurn>,
    mut player_actions: ResMut<PlayerActionsComplete>,
) {
    for _skip in skip_events.read() {
        player_actions.production_decisions_made_this_turn = true;
    }
}

/// Event for player production orders
#[derive(Event)]
pub struct PlayerProductionOrder {
    pub civ_entity: Entity,
    pub capital_entity: Entity,
    pub item: ProductionItem,
}

/// Event to indicate player is skipping production this turn
#[derive(Event)]
pub struct SkipProductionThisTurn;

/// System to reset unit movement at start of turn
pub fn reset_unit_movement(
    mut units: Query<&mut MilitaryUnit>,
    mut player_actions: ResMut<PlayerActionsComplete>,
) {
    for mut unit in units.iter_mut() {
        unit.reset_movement();
    }
    
    // Reset player actions tracking
    player_actions.reset();
}

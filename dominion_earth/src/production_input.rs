use crate::game::GameState;
use bevy::prelude::*;
use core_sim::{
    Capital, Civilization, PlayerActionsComplete, PlayerControlled, PlayerProductionOrder,
    ProductionItem, ProductionQueue, RequestTurnAdvance, SkipProductionThisTurn, UnitType,
};

/// Resource to track the currently selected capital for production
#[derive(Resource, Default)]
pub struct SelectedCapital {
    pub capital_entity: Option<Entity>,
    pub civ_entity: Option<Entity>,
    pub show_production_menu: bool,
}

/// System to handle production input
pub fn handle_production_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_capital: ResMut<SelectedCapital>,
    mut production_orders: EventWriter<PlayerProductionOrder>,
    mut skip_events: EventWriter<SkipProductionThisTurn>,
    capitals_query: Query<(Entity, &Capital, &ProductionQueue)>,
    player_civs: Query<Entity, With<PlayerControlled>>,
    civs_query: Query<&Civilization>,
    game_state: Res<GameState>,
) {
    if game_state.ai_only {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyB) {
        if let Ok(player_civ) = player_civs.single() {
            for (capital_entity, capital, _queue) in capitals_query.iter() {
                if capital.owner.0 == player_civ.index() {
                    selected_capital.capital_entity = Some(capital_entity);
                    selected_capital.civ_entity = Some(player_civ);
                    selected_capital.show_production_menu = !selected_capital.show_production_menu;
                    break;
                }
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        skip_events.write(SkipProductionThisTurn);
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        selected_capital.show_production_menu = false;
    }

    if selected_capital.show_production_menu {
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            if let Some(capital_entity) = selected_capital.capital_entity {
                if let Some(civ_entity) = selected_capital.civ_entity {
                    production_orders.write(PlayerProductionOrder {
                        capital_entity,
                        civ_entity,
                        item: ProductionItem::Unit(UnitType::Infantry),
                    });
                }
            }
        }

        if keyboard_input.just_pressed(KeyCode::Digit2) {
            if let Some(capital_entity) = selected_capital.capital_entity {
                if let Some(civ_entity) = selected_capital.civ_entity {
                    production_orders.write(PlayerProductionOrder {
                        capital_entity,
                        civ_entity,
                        item: ProductionItem::Unit(UnitType::Archer),
                    });
                }
            }
        }

        if keyboard_input.just_pressed(KeyCode::Digit3) {
            if let Some(capital_entity) = selected_capital.capital_entity {
                if let Some(civ_entity) = selected_capital.civ_entity {
                    production_orders.write(PlayerProductionOrder {
                        capital_entity,
                        civ_entity,
                        item: ProductionItem::Unit(UnitType::Cavalry),
                    });
                }
            }
        }
    }
}

/// System to handle end turn input
pub fn handle_end_turn_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_capital: ResMut<SelectedCapital>,
    mut turn_advance_events: EventWriter<RequestTurnAdvance>,
    mut skip_events: EventWriter<SkipProductionThisTurn>,
    mut player_actions_complete: ResMut<PlayerActionsComplete>,
    player_civs: Query<Entity, With<PlayerControlled>>,
    game_state: Res<GameState>,
) {
    if game_state.ai_only {
        if keyboard_input.just_pressed(KeyCode::Enter)
            || keyboard_input.just_pressed(KeyCode::NumpadEnter)
            || keyboard_input.just_pressed(KeyCode::Space)
        {
            turn_advance_events.write(RequestTurnAdvance);
        }
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Enter)
        || keyboard_input.just_pressed(KeyCode::NumpadEnter)
        || keyboard_input.just_pressed(KeyCode::Space)
    {
        if player_actions_complete.can_end_turn {
            turn_advance_events.write(RequestTurnAdvance);
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) && !selected_capital.show_production_menu {
        if let Ok(player_civ) = player_civs.single() {
            player_actions_complete.can_end_turn = true;
            skip_events.write(SkipProductionThisTurn);
        }
    }
}

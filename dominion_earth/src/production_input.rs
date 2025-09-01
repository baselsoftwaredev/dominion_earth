use bevy::prelude::*;
use bevy_egui::egui;
use core_sim::{
    Capital, Civilization, PlayerActionsComplete, PlayerControlled, PlayerProductionOrder,
    ProductionItem, ProductionQueue, UnitType, RequestTurnAdvance, SkipProductionThisTurn,
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
) {
    // Toggle production menu with B (Build)
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        // Find first player capital
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

    // Skip production with S key
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        skip_events.write(SkipProductionThisTurn);
    }

    // Queue infantry with 1 key (if production menu is open)
    if keyboard_input.just_pressed(KeyCode::Digit1) && selected_capital.show_production_menu {
        if let (Some(capital_entity), Some(civ_entity)) = 
            (selected_capital.capital_entity, selected_capital.civ_entity) {
            
            if let Ok(civ) = civs_query.get(civ_entity) {
                let infantry_cost = ProductionItem::Unit(UnitType::Infantry).gold_cost();
                if civ.economy.gold >= infantry_cost {
                                production_orders.write(PlayerProductionOrder {
                        capital_entity,
                        civ_entity,
                        item: ProductionItem::Unit(UnitType::Infantry),
                    });
                }
            }
        }
    }

    // Close production menu with Escape
    if keyboard_input.just_pressed(KeyCode::Escape) {
        selected_capital.show_production_menu = false;
    }
}/// System to handle end turn input - only allow if all actions are complete
pub fn handle_end_turn_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_actions: Res<PlayerActionsComplete>,
    mut turn_end_events: EventWriter<RequestTurnAdvance>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) || keyboard_input.just_pressed(KeyCode::Space) {
        if player_actions.can_end_turn {
            turn_end_events.write(RequestTurnAdvance);
        }
    }
}



/// System to display production UI
pub fn display_production_ui(
    mut contexts: bevy_egui::EguiContexts,
    selected_capital: Res<SelectedCapital>,
    capitals_query: Query<&ProductionQueue>,
    civs_query: Query<&Civilization>,
    player_actions: Res<PlayerActionsComplete>,
) {
    if let Ok(ctx) = contexts.ctx_mut() {
        if selected_capital.show_production_menu {
            if let Some(capital_entity) = selected_capital.capital_entity {
                if let Ok(production_queue) = capitals_query.get(capital_entity) {
                    if let Some(civ_entity) = selected_capital.civ_entity {
                        if let Ok(civilization) = civs_query.get(civ_entity) {
                            egui::Window::new("Production Menu")
                                .collapsible(false)
                                .resizable(false)
                                .show(ctx, |ui| {
                                    ui.label(format!("Capital: {}", civilization.name));
                                    ui.label(format!("Gold: {:.0}", civilization.economy.gold));
                                    ui.label(format!("Production: {:.1}", civilization.economy.production));
                                    
                                    ui.separator();
                                    
                                    ui.label("Available Units:");
                                    ui.label("Press [1] to queue Infantry (Cost: 20 gold, 15 production)");
                                    
                                    ui.separator();
                                    
                                    if production_queue.is_producing() {
                                        ui.label("Currently Producing:");
                                        if let Some(ref current) = production_queue.current_production {
                                            ui.label(format!("• {}", current.name()));
                                            ui.label(format!("Progress: {:.1}%", production_queue.get_progress_percentage() * 100.0));
                                        }
                                    }
                                    
                                    if production_queue.queue_length() > 0 {
                                        ui.label(format!("Queue length: {}", production_queue.queue_length()));
                                    }
                                    
                                    ui.separator();
                                    ui.label("Press [Esc] to close");
                                });
                        }
                    }
                }
            }
        }

        // Show turn status
        egui::Window::new("Turn Status")
            .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-10.0, 10.0))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Player Actions:");
                
                let units_status = if player_actions.all_units_moved { "✓" } else { "✗" };
                ui.label(format!("{} All units moved", units_status));
                
                let production_status = if player_actions.all_productions_queued { "✓" } else { "✗" };
                ui.label(format!("{} Production queued", production_status));
                
                ui.separator();
                
                if player_actions.can_end_turn {
                    ui.colored_label(egui::Color32::GREEN, "Press [Enter] to end turn");
                } else {
                    ui.colored_label(egui::Color32::RED, "Complete all actions to end turn");
                }
                
                ui.separator();
                ui.label("Press [B] to open production menu");
            });
    }
}

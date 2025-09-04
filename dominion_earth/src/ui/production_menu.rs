use crate::production_input::SelectedCapital;
use bevy::prelude::*;
use bevy_egui::egui;
use core_sim::{Civilization, ProductionQueue};

pub fn render_capital_production_interface(
    ui: &mut egui::Ui,
    selected_capital: &SelectedCapital,
    production_query: &Query<&ProductionQueue>,
    civs: &Query<&Civilization>,
) {
    ui.separator();
    ui.heading("Production Menu");

    if let Some(capital_entity) = selected_capital.capital_entity {
        if let Ok(production_queue) = production_query.get(capital_entity) {
            if let Some(civilization_entity) = selected_capital.civ_entity {
                if let Ok(civilization) = civs.get(civilization_entity) {
                    render_civilization_economic_information(ui, civilization);
                    ui.separator();

                    render_available_production_units(ui);
                    ui.separator();

                    render_current_production_status(ui, production_queue);

                    render_production_queue_information(ui, production_queue);

                    ui.separator();
                    ui.label("Press [Esc] to close");
                }
            }
        }
    }
}

fn render_civilization_economic_information(ui: &mut egui::Ui, civilization: &Civilization) {
    ui.label(format!("Capital: {}", civilization.name));
    ui.label(format!("Gold: {:.0}", civilization.economy.gold));
    ui.label(format!(
        "Production: {:.1}",
        civilization.economy.production
    ));
}

fn render_available_production_units(ui: &mut egui::Ui) {
    ui.label("Available Units:");
    ui.label("Press [1] to queue Infantry (Cost: 20 gold, 15 production)");
}

fn render_current_production_status(ui: &mut egui::Ui, production_queue: &ProductionQueue) {
    if production_queue.is_producing() {
        ui.label("Currently Producing:");
        if let Some(ref current_production_item) = production_queue.current_production {
            ui.label(format!("• {}", current_production_item.name()));
            ui.label(format!(
                "Progress: {:.1}%",
                production_queue.get_progress_percentage() * 100.0
            ));
        }
    }
}

fn render_production_queue_information(ui: &mut egui::Ui, production_queue: &ProductionQueue) {
    if production_queue.queue_length() > 0 {
        ui.label(format!(
            "Queue length: {}",
            production_queue.queue_length()
        ));
    }
}

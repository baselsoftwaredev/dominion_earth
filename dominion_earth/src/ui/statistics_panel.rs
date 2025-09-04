use crate::ui::resources::TerrainCounts;
use bevy::prelude::*;
use bevy_egui::egui;
use core_sim::{resources::WorldMap, Civilization};

pub fn render_world_statistics_panel(
    ctx: &egui::Context,
    world_map: &WorldMap,
    terrain_counts: &TerrainCounts,
    civs: &Query<&Civilization>,
) {
    egui::TopBottomPanel::bottom("statistics_panel")
        .resizable(false)
        .min_height(60.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                render_world_map_dimensions(ui, world_map);
                ui.separator();
                render_terrain_type_statistics(ui, terrain_counts);
                ui.separator();
                render_civilization_count_statistics(ui, civs);
            });
        });
}

fn render_world_map_dimensions(ui: &mut egui::Ui, world_map: &WorldMap) {
    ui.vertical(|ui| {
        ui.label("World Map");
        ui.label(format!("Size: {} x {}", world_map.width, world_map.height));
    });
}

fn render_terrain_type_statistics(ui: &mut egui::Ui, terrain_counts: &TerrainCounts) {
    ui.vertical(|ui| {
        ui.label("Terrain Stats");
        ui.horizontal(|ui| {
            ui.label(format!("Plains: {}", terrain_counts.plains));
            ui.label(format!("Hills: {}", terrain_counts.hills));
            ui.label(format!("Forest: {}", terrain_counts.forest));
            ui.label(format!("Ocean: {}", terrain_counts.ocean));
        });
        ui.horizontal(|ui| {
            ui.label(format!("Coast: {}", terrain_counts.coast));
            ui.label(format!("Mountains: {}", terrain_counts.mountains));
            ui.label(format!("Desert: {}", terrain_counts.desert));
            ui.label(format!("River: {}", terrain_counts.river));
        });
    });
}

fn render_civilization_count_statistics(ui: &mut egui::Ui, civs: &Query<&Civilization>) {
    ui.vertical(|ui| {
        ui.label("Civilizations");
        ui.label(format!("Total: {}", civs.iter().count()));
    });
}

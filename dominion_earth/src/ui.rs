// --- TerrainCounts resource and update system ---

#[derive(Resource, Default, Clone)]
pub struct TerrainCounts {
    pub plains: usize,
    pub hills: usize,
    pub forest: usize,
    pub ocean: usize,
    pub coast: usize,
    pub mountains: usize,
    pub desert: usize,
    pub river: usize,
}

/// System to update terrain counts when the world map changes
pub fn update_terrain_counts(
    world_map: Res<WorldMap>,
    mut terrain_counts: ResMut<TerrainCounts>,
) {
    if !world_map.is_changed() {
        return;
    }
    let mut plains = 0;
    let mut hills = 0;
    let mut forest = 0;
    let mut ocean = 0;
    let mut coast = 0;
    let mut mountains = 0;
    let mut desert = 0;
    let mut river = 0;
    for x in 0..world_map.width {
        for y in 0..world_map.height {
            if let Some(tile) = world_map.get_tile(Position::new(x as i32, y as i32)) {
                match tile.terrain {
                    TerrainType::Plains => plains += 1,
                    TerrainType::Hills => hills += 1,
                    TerrainType::Forest => forest += 1,
                    TerrainType::Ocean => ocean += 1,
                    TerrainType::Coast => coast += 1,
                    TerrainType::Mountains => mountains += 1,
                    TerrainType::Desert => desert += 1,
                    TerrainType::River => river += 1,
                }
            }
        }
    }
    *terrain_counts = TerrainCounts {
        plains,
        hills,
        forest,
        ocean,
        coast,
        mountains,
        desert,
        river,
    };
}
use crate::game::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use core_sim::{
    resources::{CurrentTurn, WorldMap},
    Civilization, Position, TerrainType,
};

/// Main UI system using egui
pub fn ui_system(
    mut contexts: EguiContexts,
    current_turn: Res<CurrentTurn>,
    mut game_state: ResMut<GameState>,
    world_map: Res<WorldMap>,
    terrain_counts: Res<TerrainCounts>,
    civs: Query<&Civilization>,
) {
    let ctx = contexts.ctx_mut();

    // Main game UI panel
    egui::SidePanel::left("game_panel")
        .resizable(false)
        .default_width(400.0)
        .show(ctx, |ui| {
            ui.heading("Dominion Earth");
            ui.separator();

            // Turn information
            ui.label(format!("Turn: {}", current_turn.0));
            ui.label(format!(
                "Status: {}",
                if game_state.paused {
                    "Paused"
                } else {
                    "Running"
                }
            ));
    ui.label(format!(
        "Auto-advance: {} (debug: {})",
        if game_state.auto_advance { "On" } else { "Off" },
        game_state.auto_advance
    ));
            ui.separator();

            // Civilization list
            ui.heading("Civilizations");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for civ in civs.iter() {
                    ui.group(|ui| {
                        ui.label(&civ.name);
                        ui.label(format!("Gold: {:.0}", civ.economy.gold));
                        ui.label(format!("Military: {:.0}", civ.military.total_strength));
                        ui.label(format!("Cities: {}", civ.id.0)); // Simplified

                        // Personality traits (unique label per civ, use stable id)
                        ui.collapsing(format!("Personality [{}]", civ.id.0), |ui| {
                            ui.label(format!("Land Hunger: {:.2}", civ.personality.land_hunger));
                            ui.label(format!("Militarism: {:.2}", civ.personality.militarism));
                            ui.label(format!("Tech Focus: {:.2}", civ.personality.tech_focus));
                            ui.label(format!("Industry: {:.2}", civ.personality.industry_focus));
                        });
                    });
                }
            });

            ui.separator();

    // Controls
    ui.heading("Controls");
    if !game_state.auto_advance {
        ui.label("Manual mode: Press Next Turn to advance.");
        if ui.button("Next Turn").clicked() {
            game_state.next_turn_requested = true;
        }
    }
    ui.label("Space: Advance turn");
    ui.label("P: Pause/Resume");
    ui.label("A: Toggle auto-advance");
    ui.label("Mouse: Pan map");
    ui.label("Scroll: Zoom");
        });

    // Statistics panel
    egui::TopBottomPanel::bottom("stats_panel")
        .resizable(true)
        .default_height(150.0)
        .show(ctx, |ui| {
            ui.heading("World Statistics");
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.label("World Map");
                    ui.label(format!("Size: {} x {}", world_map.width, world_map.height));
                    ui.label(format!(
                        "Plains: {}  Hills: {}  Forest: {}  Ocean: {}  Coast: {}  Mountains: {}  Desert: {}  River: {}",
                        terrain_counts.plains, terrain_counts.hills, terrain_counts.forest, terrain_counts.ocean, terrain_counts.coast, terrain_counts.mountains, terrain_counts.desert, terrain_counts.river
                    ));
                });
                ui.group(|ui| {
                    ui.label("Civilizations");
                    let civ_count = civs.iter().count();
                    ui.label(format!("Active: {}", civ_count));
                    let total_gold: f32 = civs.iter().map(|c| c.economy.gold).sum();
                    ui.label(format!("Total Gold: {:.0}", total_gold));
                    let total_military: f32 = civs.iter().map(|c| c.military.total_strength).sum();
                    ui.label(format!("Total Military: {:.0}", total_military));
                });
            });
        });

    // Minimap (placeholder)
    egui::Window::new("Minimap")
        .resizable(false)
        .default_size([200.0, 100.0])
        .show(ctx, |ui| {
            ui.label("Minimap");
            ui.colored_label(egui::Color32::GRAY, "Map visualization would go here");

            // Simple world representation
            let available_size = ui.available_size();
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

            if response.hovered() {
                // Draw a simple representation of the world
                let rect = response.rect;
                painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(50, 100, 200));

                // Draw some sample points for civilizations
                let civs_vec: Vec<_> = civs.iter().collect();
                for (_i, civ) in civs_vec.iter().enumerate().take(10) {
                    if let Some(capital) = civ.capital {
                        let x_ratio = capital.x as f32 / 100.0; // Assuming 100 width
                        let y_ratio = capital.y as f32 / 50.0; // Assuming 50 height

                        let point = rect.min
                            + egui::Vec2::new(x_ratio * rect.width(), y_ratio * rect.height());

                        painter.circle_filled(
                            point,
                            3.0,
                            egui::Color32::from_rgb(
                                (civ.color[0] * 255.0) as u8,
                                (civ.color[1] * 255.0) as u8,
                                (civ.color[2] * 255.0) as u8,
                            ),
                        );
                    }
                }
            }
        });
}

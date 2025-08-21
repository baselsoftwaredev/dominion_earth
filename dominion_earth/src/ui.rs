#[derive(Resource, Default, Clone)]
pub struct SelectedTile {
    pub position: Option<Position>,
}

#[derive(Resource, Clone)]
pub struct DebugLogging(pub bool);

impl Default for DebugLogging {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Resource, Default, Clone)]
pub struct LastLoggedTile {
    pub position: Option<Position>,
}
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
// pub fn update_terrain_counts(world_map: Res<WorldMap>, mut terrain_counts: ResMut<TerrainCounts>) {
//     if !world_map.is_changed() {
//         return;
//     }
//     let mut plains = 0;
//     let mut hills = 0;
//     let mut forest = 0;
//     let mut ocean = 0;
//     let mut coast = 0;
//     let mut mountains = 0;
//     let mut desert = 0;
//     let mut river = 0;
//     for x in 0..world_map.width {
//         for y in 0..world_map.height {
//             if let Some(tile) = world_map.get_tile(Position::new(x as i32, y as i32)) {
//                 match tile.terrain {
//                     TerrainType::Plains => plains += 1,
//                     TerrainType::Hills => hills += 1,
//                     TerrainType::Forest => forest += 1,
//                     TerrainType::Ocean => ocean += 1,
//                     TerrainType::Coast => coast += 1,
//                     TerrainType::Mountains => mountains += 1,
//                     TerrainType::Desert => desert += 1,
//                     TerrainType::River => river += 1,
//                 }
//             }
//         }
//     }
//     *terrain_counts = TerrainCounts {
//         plains,
//         hills,
//         forest,
//         ocean,
//         coast,
//         mountains,
//         desert,
//         river,
//     };
// }
use crate::game::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use core_sim::{
    resources::{CurrentTurn, WorldMap},
    Civilization, Position,
};

/// Main UI system that orchestrates all UI components
pub fn ui_system(
    mut contexts: EguiContexts,
    current_turn: Res<CurrentTurn>,
    mut game_state: ResMut<GameState>,
    world_map: Res<WorldMap>,
    terrain_counts: Res<TerrainCounts>,
    civs: Query<&Civilization>,
    selected_tile: Res<SelectedTile>,
    mut last_logged_tile: ResMut<LastLoggedTile>,
    world_tile_query: Query<(&core_sim::tile::tile_components::WorldTile, &core_sim::tile::tile_components::TileNeighbors)>,
) {
    if let Ok(ctx) = contexts.ctx_mut() {
        // Render main game panel (left sidebar)
        render_game_panel(
            ctx,
            &current_turn,
            &mut game_state,
            &selected_tile,
            &mut last_logged_tile,
            &world_tile_query,
            &civs,
            &world_map,
        );

        // Render statistics panel (bottom)
        render_statistics_panel(ctx, &world_map, &terrain_counts, &civs);

        // Render minimap window
        render_minimap(ctx, &civs);
    }
}

/// Renders the main game information panel on the left side
fn render_game_panel(
    ctx: &egui::Context,
    current_turn: &CurrentTurn,
    game_state: &mut GameState,
    selected_tile: &SelectedTile,
    last_logged_tile: &mut LastLoggedTile,
    world_tile_query: &Query<(&core_sim::tile::tile_components::WorldTile, &core_sim::tile::tile_components::TileNeighbors)>,
    civs: &Query<&Civilization>,
    world_map: &WorldMap,
) {
    egui::SidePanel::left("game_panel")
        .resizable(true)
        .default_width(400.0)
        .min_width(300.0)
        .max_width(600.0)
        .show(ctx, |ui| {
            ui.heading("Dominion Earth");
            ui.separator();

            render_turn_info(ui, current_turn, game_state);
            ui.separator();

            render_selected_tile_info(ui, selected_tile, last_logged_tile, world_tile_query, world_map);

            render_civilizations_list(ui, civs);

            ui.separator();
            render_controls(ui, game_state);
        });
}

/// Displays current turn information and game status
fn render_turn_info(ui: &mut egui::Ui, current_turn: &CurrentTurn, game_state: &GameState) {
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
}

/// Shows information about the currently selected tile
fn render_selected_tile_info(
    ui: &mut egui::Ui,
    selected_tile: &SelectedTile,
    last_logged_tile: &mut LastLoggedTile,
    world_tile_query: &Query<(&core_sim::tile::tile_components::WorldTile, &core_sim::tile::tile_components::TileNeighbors)>,
    world_map: &WorldMap,
) {
    ui.heading("Selected Tile Info");
    
    if let Some(pos) = selected_tile.position {
        ui.label(format!("Position: ({}, {})", pos.x, pos.y));
        
        // Check if this is a new tile selection (to prevent log spam)
        let should_log = last_logged_tile.position != Some(pos);
        if should_log {
            last_logged_tile.position = Some(pos);
        }
        
        // Show data from ECS WorldTile entities
        let mut found_ecs = false;
        let mut ecs_terrain = None;
        let mut ecs_viewpoint = None;
        let mut neighbors_info = Vec::new();
        
        for (world_tile, tile_neighbors) in world_tile_query.iter() {
            if world_tile.grid_pos == pos {
                ecs_terrain = Some(world_tile.terrain_type.clone());
                ecs_viewpoint = Some(world_tile.default_view_point.clone());
                ui.label(format!("ECS Terrain: {:?}", world_tile.terrain_type));
                ui.label(format!("ECS Facing: {:?}", world_tile.default_view_point));
                
                // Collect neighbor information
                neighbors_info = collect_neighbor_info(tile_neighbors, world_tile_query);
                
                // Display neighbors in UI
                ui.separator();
                ui.label("Neighbors:");
                for (direction, terrain) in &neighbors_info {
                    ui.label(format!("  {}: {:?}", direction, terrain));
                }
                
                found_ecs = true;
                break;
            }
        }
        
        if !found_ecs {
            ui.label("No ECS tile data found.");
        }
        
        // Show data from WorldMap resource for comparison
        let mut worldmap_terrain = None;
        if let Some(world_map_tile) = world_map.get_tile(pos) {
            worldmap_terrain = Some(world_map_tile.terrain.clone());
            ui.label(format!("WorldMap Terrain: {:?}", world_map_tile.terrain));
        } else {
            ui.label("No WorldMap tile data found.");
        }
        
        // Log comparison for debugging (only once per tile selection)
        if found_ecs && should_log {
            println!("=== UI DISPLAY: Tile ({}, {}) Data ===", pos.x, pos.y);
            println!("UI DISPLAY - ECS Terrain: {:?}", ecs_terrain.as_ref().unwrap());
            println!("UI DISPLAY - ECS Facing: {:?}", ecs_viewpoint.as_ref().unwrap());
            
            // Print neighbor terrain types
            println!("UI DISPLAY - Neighbors:");
            for (direction, terrain) in &neighbors_info {
                println!("  {}: {:?}", direction, terrain);
            }
            
            if let Some(ref wm_terrain) = worldmap_terrain {
                println!("UI DISPLAY - WorldMap Terrain: {:?}", wm_terrain);
                if ecs_terrain.as_ref().unwrap() != wm_terrain {
                    println!("⚠️  TERRAIN MISMATCH: ECS={:?} vs WorldMap={:?}", ecs_terrain.as_ref().unwrap(), wm_terrain);
                }
            }
            println!("=====================================");
        }
        
        ui.separator();
    } else {
        ui.label("No tile selected.");
        // Clear the last logged tile when no tile is selected
        last_logged_tile.position = None;
    }
}

/// Helper function to collect neighbor terrain information
fn collect_neighbor_info(
    tile_neighbors: &core_sim::tile::tile_components::TileNeighbors,
    world_tile_query: &Query<(&core_sim::tile::tile_components::WorldTile, &core_sim::tile::tile_components::TileNeighbors)>,
) -> Vec<(String, String)> {
    let mut neighbors = Vec::new();
    
    // Check North neighbor
    if let Some(north_entity) = tile_neighbors.north {
        if let Ok((north_tile, _)) = world_tile_query.get(north_entity) {
            neighbors.push(("North".to_string(), format!("{:?}", north_tile.terrain_type)));
        }
    } else {
        neighbors.push(("North".to_string(), "OutOfBounds".to_string()));
    }
    
    // Check South neighbor
    if let Some(south_entity) = tile_neighbors.south {
        if let Ok((south_tile, _)) = world_tile_query.get(south_entity) {
            neighbors.push(("South".to_string(), format!("{:?}", south_tile.terrain_type)));
        }
    } else {
        neighbors.push(("South".to_string(), "OutOfBounds".to_string()));
    }
    
    // Check East neighbor
    if let Some(east_entity) = tile_neighbors.east {
        if let Ok((east_tile, _)) = world_tile_query.get(east_entity) {
            neighbors.push(("East".to_string(), format!("{:?}", east_tile.terrain_type)));
        }
    } else {
        neighbors.push(("East".to_string(), "OutOfBounds".to_string()));
    }
    
    // Check West neighbor
    if let Some(west_entity) = tile_neighbors.west {
        if let Ok((west_tile, _)) = world_tile_query.get(west_entity) {
            neighbors.push(("West".to_string(), format!("{:?}", west_tile.terrain_type)));
        }
    } else {
        neighbors.push(("West".to_string(), "OutOfBounds".to_string()));
    }
    
    neighbors
}

/// Renders a scrollable list of all civilizations with their details
fn render_civilizations_list(ui: &mut egui::Ui, civs: &Query<&Civilization>) {
    ui.heading("Civilizations");
    
    egui::ScrollArea::vertical()
        .max_height(300.0) // Limit height so controls section remains visible
        .show(ui, |ui| {
            for civ in civs.iter() {
                render_civilization_card(ui, civ);
            }
        });
}

/// Renders a single civilization's information card
fn render_civilization_card(ui: &mut egui::Ui, civ: &Civilization) {
    ui.group(|ui| {
        ui.label(&civ.name);
        ui.label(format!("Gold: {:.0}", civ.economy.gold));
        ui.label(format!("Military: {:.0}", civ.military.total_strength));
        ui.label(format!("Cities: {}", civ.id.0)); // Simplified

        // Personality traits collapsible section
        ui.collapsing(format!("Personality [{}]", civ.id.0), |ui| {
            render_civilization_personality(ui, civ);
        });
    });
}

/// Shows detailed personality traits for a civilization
fn render_civilization_personality(ui: &mut egui::Ui, civ: &Civilization) {
    ui.label(format!("Land Hunger: {:.2}", civ.personality.land_hunger));
    ui.label(format!("Militarism: {:.2}", civ.personality.militarism));
    ui.label(format!("Tech Focus: {:.2}", civ.personality.tech_focus));
    ui.label(format!("Industry: {:.2}", civ.personality.industry_focus));
}

/// Displays game controls and interaction help
fn render_controls(ui: &mut egui::Ui, game_state: &mut GameState) {
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
}

/// Renders the bottom statistics panel with world and civilization data
fn render_statistics_panel(
    ctx: &egui::Context,
    world_map: &WorldMap,
    terrain_counts: &TerrainCounts,
    civs: &Query<&Civilization>,
) {
    egui::TopBottomPanel::bottom("stats_panel")
        .resizable(true)
        .default_height(150.0)
        .show(ctx, |ui| {
            ui.heading("World Statistics");
            ui.horizontal(|ui| {
                render_world_statistics(ui, world_map, terrain_counts);
                render_civilization_statistics(ui, civs);
            });
        });
}

/// Shows world map size and terrain distribution statistics
fn render_world_statistics(ui: &mut egui::Ui, world_map: &WorldMap, terrain_counts: &TerrainCounts) {
    ui.group(|ui| {
        ui.label("World Map");
        ui.label(format!("Size: {} x {}", world_map.width, world_map.height));
        ui.label(format!(
            "Plains: {}  Hills: {}  Forest: {}  Ocean: {}  Coast: {}  Mountains: {}  Desert: {}  River: {}",
            terrain_counts.plains, 
            terrain_counts.hills, 
            terrain_counts.forest, 
            terrain_counts.ocean, 
            terrain_counts.coast, 
            terrain_counts.mountains, 
            terrain_counts.desert, 
            terrain_counts.river
        ));
    });
}

/// Shows aggregate civilization statistics
fn render_civilization_statistics(ui: &mut egui::Ui, civs: &Query<&Civilization>) {
    ui.group(|ui| {
        ui.label("Civilizations");
        let civ_count = civs.iter().count();
        ui.label(format!("Active: {}", civ_count));
        
        let total_gold: f32 = civs.iter().map(|c| c.economy.gold).sum();
        ui.label(format!("Total Gold: {:.0}", total_gold));
        
        let total_military: f32 = civs.iter().map(|c| c.military.total_strength).sum();
        ui.label(format!("Total Military: {:.0}", total_military));
    });
}

/// Renders the minimap window with world visualization
fn render_minimap(ctx: &egui::Context, civs: &Query<&Civilization>) {
    egui::Window::new("Minimap")
        .resizable(false)
        .default_size([200.0, 100.0])
        .show(ctx, |ui| {
            ui.label("Minimap");
            ui.colored_label(egui::Color32::GRAY, "Map visualization would go here");

            render_minimap_visualization(ui, civs);
        });
}

/// Draws the actual minimap visualization with civilization positions
fn render_minimap_visualization(ui: &mut egui::Ui, civs: &Query<&Civilization>) {
    let available_size = ui.available_size();
    let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

    if response.hovered() {
        // Draw ocean background
        let rect = response.rect;
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(50, 100, 200));

        // Draw civilization capital positions
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
}

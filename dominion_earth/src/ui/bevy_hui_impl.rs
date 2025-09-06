use bevy::prelude::*;
use bevy_hui::prelude::*;
use crate::ui::traits::*;
use crate::ui::resources::*;
use crate::debug_utils::DebugLogging;
use crate::game::GameState;
use crate::production_input::SelectedCapital;
use core_sim::{
    components::{Capital, MilitaryUnit},
    resources::{CurrentTurn, WorldMap},
    Civilization, Position, ProductionQueue,
};

/// Bevy HUI implementation of the UI system
pub struct BevyHuiSystem;

impl BevyHuiSystem {
    pub fn setup_plugins(app: &mut App) {
        app.add_plugins((
            HuiPlugin,
            HuiAutoLoadPlugin::new(&["ui"]),
        ))
        .add_systems(Startup, setup_main_ui)
        .add_systems(Update, update_ui_properties);
    }
}

impl UiSystem for BevyHuiSystem {
    fn initialize(&self, app: &mut App) {
        Self::setup_plugins(app);
    }
    
    fn render_main_game_panel(&self, _data: &GamePanelData) {
        // For bevy_hui, rendering is handled by the component system
    }
    
    fn render_production_menu(&self, _data: &ProductionMenuData) {
        // For bevy_hui, rendering is handled by the component system
    }
    
    fn render_statistics_panel(&self, _data: &StatisticsPanelData) {
        // For bevy_hui, rendering is handled by the component system
    }
    
    fn render_tile_info(&self, _data: &TileInfoData) {
        // For bevy_hui, rendering is handled by the component system
    }
    
    fn render_minimap(&self, _data: &MinimapData) {
        // For bevy_hui, rendering is handled by the component system
    }
    
    fn render_resources(&self, _data: &ResourcesData) {
        // For bevy_hui, rendering is handled by the component system
    }
}

/// Component that holds a template handle for bevy_hui elements
#[derive(Component)]
pub struct HuiComponent {
    pub template: Handle<HtmlTemplate>,
}

impl UiComponent for HuiComponent {
    type Id = Entity;
    
    fn get_id(&self) -> Self::Id {
        Entity::from_raw(0) // This would be set when spawned
    }
    
    fn update_data(&mut self, _data: Box<dyn std::any::Any + Send + Sync>) {
        // Update template properties based on data
    }
}

/// Setup main UI 
fn setup_main_ui(
    mut cmd: Commands,
    server: Res<AssetServer>,
) {
    // Spawn main UI layout
    cmd.spawn((
        HtmlNode(server.load("ui/main_layout.html")),
        Name::new("MainUI"),
        TemplateProperties::default()
            .with("currentTurn", "1")
            .with("gameTitle", "Dominion Earth"),
    ));
}

/// Update UI properties with current game data
fn update_ui_properties(
    current_turn: Res<CurrentTurn>,
    terrain_counts: Res<TerrainCounts>,
    selected_tile: Res<SelectedTile>,
    mut ui_nodes: Query<&mut TemplateProperties, With<HtmlNode>>,
) {
    // Update template properties when resources change
    if current_turn.is_changed() || terrain_counts.is_changed() || selected_tile.is_changed() {
        for mut properties in ui_nodes.iter_mut() {
            // Update current turn
            properties.insert("currentTurn".to_string(), current_turn.0.to_string());
            
            // Update terrain counts
            properties.insert("plains".to_string(), terrain_counts.plains.to_string());
            properties.insert("hills".to_string(), terrain_counts.hills.to_string());
            properties.insert("forest".to_string(), terrain_counts.forest.to_string());
            properties.insert("ocean".to_string(), terrain_counts.ocean.to_string());
            properties.insert("coast".to_string(), terrain_counts.coast.to_string());
            properties.insert("mountains".to_string(), terrain_counts.mountains.to_string());
            properties.insert("desert".to_string(), terrain_counts.desert.to_string());
            properties.insert("river".to_string(), terrain_counts.river.to_string());
            
            // Update selected tile info
            if let Some(pos) = selected_tile.position {
                properties.insert("selectedTileX".to_string(), pos.x.to_string());
                properties.insert("selectedTileY".to_string(), pos.y.to_string());
                properties.insert("hasSelectedTile".to_string(), "true".to_string());
            } else {
                properties.insert("hasSelectedTile".to_string(), "false".to_string());
            }
        }
    }
}

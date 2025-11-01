use crate::{
    game::GameState,
    production_input::SelectedCapital,
    ui::resources::{LastLoggedTile, SelectedTile, TerrainCounts},
};
use bevy::prelude::*;
use core_sim::{
    components::{Capital, MilitaryUnit},
    resources::{CurrentTurn, WorldMap},
    Civilization, Position, ProductionQueue,
};

/// Trait for UI system implementations
/// This allows us to swap between different UI frameworks (egui, bevy_hui, etc.)
pub trait UiSystem: Send + Sync + 'static {
    /// Initialize the UI system
    fn initialize(&self, app: &mut App);

    /// Render the main game panel
    fn render_main_game_panel(&self, data: &GamePanelData);

    /// Render the production menu
    fn render_production_menu(&self, data: &ProductionMenuData);

    /// Render the statistics panel
    fn render_statistics_panel(&self, data: &StatisticsPanelData);

    /// Render the tile info panel
    fn render_tile_info(&self, data: &TileInfoData);

    /// Render the minimap
    fn render_minimap(&self, data: &MinimapData);

    /// Render the resources panel
    fn render_resources(&self, data: &ResourcesData);
}

/// Data structure for the main game panel
pub struct GamePanelData {
    pub current_turn: u32,
    // Remove game_state for now since GameState doesn't implement Clone
    // pub game_state: GameState,
    pub world_map: WorldMap,
    pub terrain_counts: TerrainCounts,
    pub civilizations: Vec<Civilization>,
    pub player_civilizations: Vec<Civilization>,
    pub selected_tile: Option<(i32, i32)>,
    pub selected_unit: Option<Entity>,
    pub last_logged_tile: Option<(i32, i32)>,
    pub capitals: Vec<(Capital, Position)>,
    pub units: Vec<(MilitaryUnit, Position)>,
    pub selected_capital: Option<Entity>,
    pub production_queues: Vec<ProductionQueue>,
}

/// Data structure for the production menu
pub struct ProductionMenuData {
    pub selected_capital: Option<Entity>,
    pub production_queue: Option<ProductionQueue>,
    pub available_items: Vec<String>,
}

/// Data structure for the statistics panel
pub struct StatisticsPanelData {
    pub turn: u32,
    pub civilizations: Vec<Civilization>,
    pub player_civilizations: Vec<Civilization>,
}

/// Data structure for the tile info panel
pub struct TileInfoData {
    pub selected_tile: Option<(i32, i32)>,
    pub world_map: WorldMap,
    pub debug_logging: bool,
}

/// Data structure for the minimap
pub struct MinimapData {
    pub world_map: WorldMap,
    pub capitals: Vec<(Capital, Position)>,
    pub units: Vec<(MilitaryUnit, Position)>,
}

/// Data structure for the resources panel
pub struct ResourcesData {
    pub player_civilizations: Vec<Civilization>,
}

/// UI Component trait for individual UI elements
pub trait UiComponent: Component + Send + Sync {
    /// The UI framework-specific identifier
    type Id: Send + Sync + Clone + 'static;

    /// Get the UI element identifier
    fn get_id(&self) -> Self::Id;

    /// Update the component's data
    fn update_data(&mut self, data: Box<dyn std::any::Any + Send + Sync>);
}

/// Resource to hold the current UI system implementation
#[derive(Resource)]
pub struct UiSystemResource {
    pub system: Box<dyn UiSystem>,
}

impl UiSystemResource {
    pub fn new(system: Box<dyn UiSystem>) -> Self {
        Self { system }
    }
}

/// Marker components for different UI panels
#[derive(Component)]
pub struct GamePanelComponent;

#[derive(Component)]
pub struct ProductionMenuComponent;

#[derive(Component)]
pub struct StatisticsPanelComponent;

#[derive(Component)]
pub struct TileInfoComponent;

#[derive(Component)]
pub struct MinimapComponent;

#[derive(Component)]
pub struct ResourcesComponent;

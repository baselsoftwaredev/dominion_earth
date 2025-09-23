use crate::debug_utils::DebugLogging;
use crate::ui;
use crate::{game, production_input};
use bevy::prelude::*;
use core_sim::{
    influence_map::InfluenceMap,
    resources::{ActiveCivTurn, CurrentTurn, GameConfig, GameRng, WorldMap},
    PlayerActionsComplete,
};

/// Plugin for initializing all game resources
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            // UI Resources
            .init_resource::<ui::TerrainCounts>()
            .init_resource::<ui::SelectedTile>()
            .init_resource::<ui::HoveredTile>()
            .init_resource::<ui::LastLoggedTile>()
            .insert_resource(ui::UiSystemResource::new(Box::new(ui::BevyHuiSystem)))
            // Core Simulation Resources
            .init_resource::<CurrentTurn>()
            .init_resource::<ActiveCivTurn>()
            .init_resource::<core_sim::components::player::SelectedUnit>()
            .init_resource::<GameRng>()
            .init_resource::<WorldMap>()
            .init_resource::<core_sim::resources::TurnAdvanceRequest>()
            .init_resource::<InfluenceMap>()
            .init_resource::<PlayerActionsComplete>()
            // Production Resources
            .init_resource::<production_input::SelectedCapital>()
            // Events
            .add_event::<core_sim::PlayerProductionOrder>()
            .add_event::<core_sim::SkipProductionThisTurn>()
            .add_event::<core_sim::RequestTurnAdvance>();
    }
}

/// Configuration for resource plugin
#[derive(Resource)]
pub struct ResourceConfig {
    pub auto_advance: bool,
    pub ai_only: bool,
    pub total_civs: u32,
    pub seed: Option<u64>,
    pub debug_logging: bool,
}

impl ResourcesPlugin {
    /// Configure the plugin with specific settings
    pub fn with_config(config: ResourceConfig) -> ResourcesPluginWithConfig {
        ResourcesPluginWithConfig { config }
    }
}

pub struct ResourcesPluginWithConfig {
    config: ResourceConfig,
}

impl Plugin for ResourcesPluginWithConfig {
    fn build(&self, app: &mut App) {
        // First add the base resources
        app.add_plugins(ResourcesPlugin);

        // Then add configured resources
        app.insert_resource(bevy::winit::WinitSettings::game())
            .insert_resource({
                let mut game_config = GameConfig::default();
                if let Some(seed) = self.config.seed {
                    game_config.random_seed = seed;
                    println!("Using custom random seed: {}", seed);
                }
                game_config.debug_logging = self.config.debug_logging;
                game_config
            })
            .insert_resource(game::GameState::new(
                self.config.auto_advance,
                self.config.ai_only,
                self.config.total_civs,
            ))
            .insert_resource(DebugLogging(self.config.debug_logging));
    }
}

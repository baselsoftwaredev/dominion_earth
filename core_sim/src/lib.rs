use std::collections::HashMap;

pub mod components;
pub mod constants;
pub mod data_loader;
pub mod debug_utils;
pub mod influence_map;
pub mod pathfinding;
pub mod resource_loading;
pub mod resources;
pub mod systems;
pub mod world_gen;

pub mod tile;

// Import specific components to avoid ambiguous glob reexports
pub use components::{
    // Action queue components
    action_queue::{ActionQueue, QueuedAction},
    // AI components
    ai::{AIAction, AIDecision, CivilizationData, DecisionType},
    // City components
    city::{
        Building, BuildingType, Capital, CapitalAge, CapitalEvolutionRequirements, City, Territory,
    },
    // Civilization components
    civilization::{
        CivPersonality, CivStats, Civilization, Economy, Military, Technologies, TradeRoute,
    },
    // Diplomacy components
    diplomacy::{DiplomaticAction, DiplomaticRelation, Treaty},
    // Fog of war components
    fog_of_war::{FogOfWarMaps, ProvidesVision, VisibilityMap, VisibilityState},
    // Military components
    military::{MilitaryUnit, UnitType},
    // Orders components
    orders::ActiveThisTurn,
    // Player components
    player::{PlayerControlled, PlayerMovementOrder, SelectedUnit, UnitSelected},
    // Position components
    position::{Direction, MovementOrder, Position},
    // Production components
    production::{
        PlayerAction, PlayerActionType, PlayerActionsComplete, ProductionItem, ProductionQueue,
    },
    // Rendering components
    rendering::SpriteEntityReference,
    // Terrain components
    terrain::TerrainType,
    // Turn phases components
    turn_phases::{AITurnComplete, AllAITurnsComplete, ProcessAITurn, StartPlayerTurn, TurnPhase},
    // Core component types
    CivId,
};

pub use data_loader::{CivilizationDataCollection, CivilizationDataLoader, CivilizationDefinition};
pub use debug_utils::CoreDebugUtils;

// Import specific systems to avoid ambiguous glob reexports
pub use systems::{
    action_queue::{
        log_all_action_queue_status, populate_action_queues_from_ai_decisions,
        process_civilization_action_queues, spawn_action_queues_for_new_civilizations,
    },
    ai_decision::*,
    combat_resolution::*,
    economic_update::*,
    fog_of_war::{
        filter_visible_cities, filter_visible_units, get_explored_positions, get_visible_positions,
        initialize_fog_of_war_for_civ, is_position_explored, is_position_visible,
        update_fog_of_war,
    },
    movement::{
        clear_completed_movement_orders, execute_ai_movement_orders, execute_movement_orders,
    },
    production::{
        check_player_actions_complete, handle_player_production_orders, handle_skip_production,
        initialize_production_queues, process_production_queues, reset_unit_movement,
        PlayerProductionOrder, SkipProductionThisTurn,
    },
    turn_management::{
        auto_advance_turn_system, handle_ai_turn_completion, handle_ai_turn_processing,
        handle_all_ai_turns_complete, handle_turn_advance_requests, ProductionUpdated,
        RequestTurnAdvance,
    },
};

// Import specific items from resources to avoid Resource trait conflict
pub use resources::{
    DiplomaticEvent,
    DiplomaticState,
    GlobalEconomy,
    Resource as GameResource, // Rename to avoid conflict with bevy_ecs::Resource
    WorldMap,
};

/// Game state representing the current state of the simulation
#[derive(Debug, Clone)]
pub struct GameState {
    pub turn: u32,
    pub civilizations: HashMap<CivId, components::ai::CivilizationData>,
    pub current_player: Option<CivId>,
}

// Manual Resource implementation
impl bevy_ecs::resource::Resource for GameState {}

impl Default for GameState {
    fn default() -> Self {
        Self {
            turn: 1,
            civilizations: HashMap::new(),
            current_player: None,
        }
    }
}

/// Errors related to civilization operations
#[derive(Debug, Clone, PartialEq)]
pub enum CivError {
    /// Civilization with given ID not found
    CivNotFound(CivId),
}

/// Core simulation error types
#[derive(Debug)]
pub enum SimError {
    CivNotFound(CivId),
    InvalidPosition { x: i32, y: i32 },
    Serialization(ron::Error),
    JsonSerialization(serde_json::Error),
    Io(std::io::Error),
}

impl From<ron::Error> for SimError {
    fn from(error: ron::Error) -> Self {
        SimError::Serialization(error)
    }
}

impl From<serde_json::Error> for SimError {
    fn from(error: serde_json::Error) -> Self {
        SimError::JsonSerialization(error)
    }
}

impl From<std::io::Error> for SimError {
    fn from(error: std::io::Error) -> Self {
        SimError::Io(error)
    }
}

pub type SimResult<T> = Result<T, SimError>;

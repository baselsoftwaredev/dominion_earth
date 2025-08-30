use std::collections::HashMap;

pub mod components;
pub mod constants;
pub mod debug_utils;
pub mod influence_map;
pub mod pathfinding;
pub mod resources;
pub mod systems;
pub mod world_gen;

// Temporarily disabled due to proc macro version issues
// pub mod economy;
// pub mod combat;
// pub mod diplomacy;
// pub mod serialization;

pub mod tile;

pub use components::*;
pub use debug_utils::CoreDebugUtils;
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

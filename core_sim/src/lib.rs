pub mod components;
pub mod resources;
pub mod systems;
pub mod world_gen;
pub mod influence_map;
pub mod pathfinding;
pub mod economy;
pub mod combat;
pub mod diplomacy;
pub mod serialization;

pub use components::*;
pub use resources::*;

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core game state that can be serialized/deserialized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub turn: u32,
    pub civilizations: HashMap<CivId, CivilizationData>,
    pub world_map: WorldMap,
    pub global_economy: GlobalEconomy,
    pub diplomatic_state: DiplomaticState,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            turn: 1,
            civilizations: HashMap::new(),
            world_map: WorldMap::default(),
            global_economy: GlobalEconomy::default(),
            diplomatic_state: DiplomaticState::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CivId(pub u32);

impl From<u32> for CivId {
    fn from(id: u32) -> Self {
        CivId(id)
    }
}

/// Core simulation error types
#[derive(thiserror::Error, Debug)]
pub enum SimError {
    #[error("Civilization not found: {0:?}")]
    CivNotFound(CivId),
    #[error("Invalid position: {x}, {y}")]
    InvalidPosition { x: i32, y: i32 },
    #[error("Serialization error: {0}")]
    Serialization(#[from] ron::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type SimResult<T> = Result<T, SimError>;

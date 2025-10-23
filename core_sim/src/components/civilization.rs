use super::position::Position;
use bevy::prelude::Reflect;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for civilizations
/// Note: Hash trait is required for use as HashMap key in FogOfWarMaps
///
/// This is a "Model" component - core game state that should be saved.
/// Note: Add the `Save` component manually when spawning civilizations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(Component, Hash, PartialEq)]
pub struct CivId(pub u32);

// Manual Component implementation
impl Component for CivId {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl From<u32> for CivId {
    fn from(id: u32) -> Self {
        CivId(id)
    }
}

/// Civilization component representing a playable faction
///
/// This is a "Model" component - core game state.
/// Note: Add the `Save` component manually when spawning civilizations.
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Civilization {
    pub id: CivId,
    pub name: String,
    pub color: [f32; 3],
    pub capital: Option<Position>,
    pub personality: CivPersonality,
    pub technologies: Technologies,
    pub economy: Economy,
    pub military: Military,
}

// Manual Component implementation
impl Component for Civilization {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// AI personality traits that drive decision making
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CivPersonality {
    pub land_hunger: f32,     // 0.0 - 1.0, desire to expand territory
    pub industry_focus: f32,  // 0.0 - 1.0, focus on economic development
    pub tech_focus: f32,      // 0.0 - 1.0, investment in research
    pub interventionism: f32, // 0.0 - 1.0, willingness to interfere abroad
    pub risk_tolerance: f32,  // 0.0 - 1.0, willingness to take risks
    pub honor_treaties: f32,  // 0.0 - 1.0, diplomatic reliability
    pub militarism: f32,      // 0.0 - 1.0, focus on military strength
    pub isolationism: f32,    // 0.0 - 1.0, preference for isolation
}

// Manual Component implementation
impl Component for CivPersonality {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl Default for CivPersonality {
    fn default() -> Self {
        Self {
            land_hunger: 0.5,
            industry_focus: 0.5,
            tech_focus: 0.5,
            interventionism: 0.5,
            risk_tolerance: 0.5,
            honor_treaties: 0.5,
            militarism: 0.5,
            isolationism: 0.5,
        }
    }
}

/// Technology research state
#[derive(Debug, Clone, Reflect)]
pub struct Technologies {
    #[reflect(skip_serializing)]
    pub known: HashMap<String, bool>,
    pub research_points: f32,
    pub current_research: Option<String>,
}

impl Default for Technologies {
    fn default() -> Self {
        Self {
            known: HashMap::new(),
            research_points: 0.0,
            current_research: None,
        }
    }
}

/// Economic state of a civilization
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Economy {
    pub gold: f32,
    pub income: f32,
    pub expenses: f32,
    pub production: f32,
    #[reflect(skip_serializing)]
    pub trade_routes: Vec<TradeRoute>,
}

// Manual Component implementation
impl Component for Economy {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            gold: 100.0,
            income: 10.0,
            expenses: 5.0,
            production: 8.0,
            trade_routes: Vec::new(),
        }
    }
}

/// Trade route between cities/regions
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct TradeRoute {
    pub from: Position,
    pub to: Position,
    pub value: f32,
    pub security: f32,
}

// Manual Component implementation
impl Component for TradeRoute {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Military forces and capabilities
#[derive(Debug, Clone, Reflect)]
pub struct Military {
    pub units: Vec<super::military::MilitaryUnit>,
    pub total_strength: f32,
    pub maintenance_cost: f32,
}

impl Default for Military {
    fn default() -> Self {
        Self {
            units: Vec::new(),
            total_strength: 0.0,
            maintenance_cost: 0.0,
        }
    }
}

/// Basic civilization statistics for simple tracking
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CivStats {
    pub name: String,
    pub population: u32,
    pub cities: u32,
    pub military_strength: f32,
}

// Manual Component implementation
impl Component for CivStats {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

// Re-export ActiveThisTurn from orders module
pub use super::orders::ActiveThisTurn;

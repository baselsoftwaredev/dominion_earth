use bevy_ecs::component::{Component, Mutable};
use serde::{Deserialize, Serialize};

use super::{
    city::{BuildingType, City, Territory},
    civilization::CivId,
    diplomacy::{DiplomaticAction, DiplomaticRelation},
    military::UnitType,
    position::Position,
};

/// AI decision component
#[derive(Debug, Clone)]
pub struct AIDecision {
    pub decision_type: DecisionType,
    pub priority: f32,
    pub target: Option<Position>,
}

// Manual Component implementation
impl Component for AIDecision {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

#[derive(Debug, Clone)]
pub enum DecisionType {
    Expand,
    Research(String),
    BuildUnit(UnitType),
    BuildBuilding(BuildingType),
    Attack(CivId),
    MakePeace(CivId),
    Trade(CivId),
}

/// AI actions that can be taken by civilizations (GOAP-based)
#[derive(Debug, Clone)]
pub enum AIAction {
    Expand {
        target_position: Position,
        priority: f32,
    },
    Research {
        technology: String,
        priority: f32,
    },
    BuildUnit {
        unit_type: UnitType,
        position: Position,
        priority: f32,
    },
    BuildBuilding {
        building_type: BuildingType,
        position: Position,
        priority: f32,
    },
    Trade {
        partner: CivId,
        resource: crate::GameResource,
        priority: f32,
    },
    Attack {
        target: CivId,
        target_position: Position,
        priority: f32,
    },
    Diplomacy {
        target: CivId,
        action: DiplomaticAction,
        priority: f32,
    },
    Defend {
        position: Position,
        priority: f32,
    },
}

/// Data structure for AI serialization that contains full civilization state
#[derive(Debug, Clone)]
pub struct CivilizationData {
    pub civilization: crate::components::civilization::Civilization,
    pub cities: Vec<crate::components::city::City>,
    pub territories: Vec<(
        crate::components::position::Position,
        crate::components::city::Territory,
    )>,
    pub diplomatic_relations: Vec<crate::components::diplomacy::DiplomaticRelation>,
}

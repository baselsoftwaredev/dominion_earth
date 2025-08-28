use bevy_ecs::component::{Component, Mutable};
use serde::{Deserialize, Serialize};

use super::civilization::CivId;

/// Diplomatic relationship between two civilizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticRelation {
    pub civ_a: CivId,
    pub civ_b: CivId,
    pub relation_value: f32, // -100 to 100
    pub treaties: Vec<Treaty>,
    pub trade_agreement: bool,
}

// Manual Component implementation
impl Component for DiplomaticRelation {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Treaty {
    NonAggression { turns_remaining: u32 },
    Alliance { turns_remaining: u32 },
    TradePact { turns_remaining: u32 },
    War { started_turn: u32 },
}

/// Diplomatic actions that civilizations can take
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiplomaticAction {
    ProposeAlliance,
    ProposeNonAggression,
    ProposeTradePact,
    DeclareWar,
    MakePeace,
    BreakTreaty,
}

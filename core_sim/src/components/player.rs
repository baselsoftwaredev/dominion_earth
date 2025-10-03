use super::civilization::CivId;
use bevy::reflect::Reflect;
use bevy_ecs::component::{Component, Mutable};
use bevy_ecs::prelude::Resource;

/// Marker component for player-controlled civilizations
#[derive(Debug, Clone, Reflect)]
pub struct PlayerControlled;

impl Component for PlayerControlled {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Resource for tracking currently selected unit
#[derive(Default, Resource)]
pub struct SelectedUnit {
    pub unit_entity: Option<bevy_ecs::entity::Entity>,
    pub unit_id: Option<u32>,
    pub owner: Option<CivId>,
}

/// Component to mark a unit as selected for player interaction
#[derive(Debug, Clone, Reflect)]
pub struct UnitSelected;

impl Component for UnitSelected {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Component for pending player movement order
#[derive(Debug, Clone, Reflect)]
pub struct PlayerMovementOrder {
    pub target_position: super::position::Position,
}

impl Component for PlayerMovementOrder {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

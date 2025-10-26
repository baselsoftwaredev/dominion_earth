use super::civilization::CivId;
use bevy::reflect::Reflect;
use bevy_ecs::component::{Component, Mutable};
use bevy_ecs::prelude::Resource;
use moonshine_save::prelude::*;

/// Marker component for player-controlled civilizations
#[derive(Component, Debug, Clone, Reflect)]
#[require(Save)]
pub struct PlayerControlled;

/// Resource for tracking currently selected unit
#[derive(Default, Resource)]
pub struct SelectedUnit {
    pub unit_entity: Option<bevy_ecs::entity::Entity>,
    pub unit_id: Option<u32>,
    pub owner: Option<CivId>,
}

/// Component to mark a unit as selected for player interaction
#[derive(Component, Debug, Clone, Reflect)]
#[require(Save)]
pub struct UnitSelected;

/// Component for pending player movement order
#[derive(Component, Debug, Clone, Reflect)]
#[require(Save)]
pub struct PlayerMovementOrder {
    pub target_position: super::position::Position,
}

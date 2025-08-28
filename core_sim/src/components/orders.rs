use bevy_ecs::component::{Component, Mutable};

// Re-export MovementOrder from position module
pub use super::position::MovementOrder;

/// Turn marker for entities that should act this turn
#[derive(Debug, Clone)]
pub struct ActiveThisTurn;

// Manual Component implementation
impl Component for ActiveThisTurn {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

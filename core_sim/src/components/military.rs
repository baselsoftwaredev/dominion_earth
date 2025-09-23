use super::civilization::CivId;
use super::position::Position;
use bevy::prelude::Reflect;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Individual military unit
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MilitaryUnit {
    pub id: u32,
    pub owner: CivId,
    pub unit_type: UnitType,
    pub position: Position,
    pub strength: f32,
    pub movement_remaining: u32,
    pub experience: f32,
}

// Manual Component implementation
impl Component for MilitaryUnit {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl MilitaryUnit {
    pub fn new(id: u32, owner: CivId, unit_type: UnitType, position: Position) -> Self {
        Self {
            id,
            owner,
            unit_type,
            position,
            strength: unit_type.base_strength(),
            movement_remaining: unit_type.movement_points(),
            experience: 0.0,
        }
    }

    pub fn can_move(&self) -> bool {
        self.movement_remaining > 0
    }

    pub fn move_to(&mut self, new_position: Position) -> bool {
        if self.can_move() {
            self.position = new_position;
            self.movement_remaining -= 1;
            true
        } else {
            false
        }
    }

    pub fn reset_movement(&mut self) {
        self.movement_remaining = self.unit_type.movement_points();
    }

    pub fn gain_experience(&mut self, amount: f32) {
        self.experience += amount;
        // Strength increases with experience
        self.strength = self.unit_type.base_strength() * (1.0 + self.experience * 0.1);
    }
}

/// Different types of military units
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum UnitType {
    Infantry,
    Cavalry,
    Archer,
    Siege,
    Naval,
}

impl UnitType {
    pub fn base_strength(&self) -> f32 {
        match self {
            UnitType::Infantry => 10.0,
            UnitType::Cavalry => 12.0,
            UnitType::Archer => 8.0,
            UnitType::Siege => 15.0,
            UnitType::Naval => 20.0,
        }
    }

    pub fn movement_points(&self) -> u32 {
        match self {
            UnitType::Infantry => 2,
            UnitType::Cavalry => 3,
            UnitType::Archer => 2,
            UnitType::Siege => 1,
            UnitType::Naval => 4,
        }
    }

    pub fn cost(&self) -> f32 {
        match self {
            UnitType::Infantry => 20.0,
            UnitType::Cavalry => 35.0,
            UnitType::Archer => 25.0,
            UnitType::Siege => 50.0,
            UnitType::Naval => 60.0,
        }
    }

    pub fn maintenance_cost(&self) -> f32 {
        self.cost() * 0.1 // 10% of build cost per turn
    }

    pub fn production_cost(&self) -> f32 {
        match self {
            UnitType::Infantry => 15.0,
            UnitType::Cavalry => 25.0,
            UnitType::Archer => 20.0,
            UnitType::Siege => 40.0,
            UnitType::Naval => 50.0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            UnitType::Infantry => "Infantry",
            UnitType::Cavalry => "Cavalry",
            UnitType::Archer => "Archer",
            UnitType::Siege => "Siege Engine",
            UnitType::Naval => "Naval Unit",
        }
    }
}

use super::civilization::CivId;
use super::position::Position;
use crate::constants::unit_stats;
use bevy::prelude::Reflect;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use moonshine_save::prelude::*;
use serde::{Deserialize, Serialize};

/// Military unit component - represents a game unit
///
/// This is a "Model" component containing game state that should be saved.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
#[require(Save)]
pub struct MilitaryUnit {
    pub id: u32,
    pub owner: CivId,
    pub unit_type: UnitType,
    pub position: Position,

    pub attack: f32,
    pub defense: f32,
    pub health: f32,
    pub max_health: f32,
    pub movement_range: u32,
    pub movement_remaining: u32,
    pub range: u32,

    pub fatigue: f32,
    pub supply: f32,
    pub decay: f32,

    pub morale: f32,
    pub loyalty: f32,
    pub corruption: f32,

    pub experience: f32,
}

impl MilitaryUnit {
    pub fn new(id: u32, owner: CivId, unit_type: UnitType, position: Position) -> Self {
        let max_health = unit_type.base_health();
        Self {
            id,
            owner,
            unit_type,
            position,
            attack: unit_type.base_attack(),
            defense: unit_type.base_defense(),
            health: max_health,
            max_health,
            movement_range: unit_type.movement_points(),
            movement_remaining: unit_type.movement_points(),
            range: unit_type.attack_range(),
            fatigue: 0.0,
            supply: 1.0,
            decay: 0.0,
            morale: 1.0,
            loyalty: 1.0,
            corruption: 0.0,
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
        let supply_modifier = unit_stats::SUPPLY_MODIFIER_MINIMUM
            + (self.supply * unit_stats::SUPPLY_MODIFIER_MINIMUM);
        self.movement_remaining = (self.movement_range as f32 * supply_modifier) as u32;
    }

    pub fn gain_experience(&mut self, amount: f32) {
        self.experience += amount;
        let exp_bonus = 1.0 + (self.experience * unit_stats::EXPERIENCE_BONUS_MULTIPLIER);
        self.attack = self.unit_type.base_attack() * exp_bonus;
        self.defense = self.unit_type.base_defense() * exp_bonus;
    }

    pub fn effective_attack(&self) -> f32 {
        let fatigue_penalty = 1.0 - (self.fatigue * unit_stats::FATIGUE_PENALTY_MULTIPLIER);
        let decay_penalty = 1.0 - (self.decay * unit_stats::DECAY_ATTACK_PENALTY_MULTIPLIER);
        let morale_modifier =
            unit_stats::MORALE_ATTACK_BASE + (self.morale * unit_stats::MORALE_ATTACK_RANGE);
        self.attack * fatigue_penalty * decay_penalty * morale_modifier
    }

    pub fn effective_defense(&self) -> f32 {
        let decay_penalty = 1.0 - (self.decay * unit_stats::DECAY_DEFENSE_PENALTY_MULTIPLIER);
        let morale_modifier =
            unit_stats::MORALE_DEFENSE_BASE + (self.morale * unit_stats::MORALE_DEFENSE_RANGE);
        self.defense * decay_penalty * morale_modifier
    }

    pub fn is_reliable(&self) -> bool {
        self.loyalty >= unit_stats::LOYALTY_THRESHOLD
            && self.corruption < unit_stats::CORRUPTION_THRESHOLD
    }

    /// Apply fatigue from combat or forced march
    pub fn add_fatigue(&mut self, amount: f32) {
        self.fatigue = (self.fatigue + amount).min(1.0);
    }

    /// Reduce supply (happens each turn without proper supply lines)
    pub fn consume_supply(&mut self, amount: f32) {
        self.supply = (self.supply - amount).max(0.0);
    }

    /// Increase decay over time or from harsh conditions
    pub fn add_decay(&mut self, amount: f32) {
        self.decay = (self.decay + amount).min(1.0);
    }

    /// Reduce morale from casualties or harsh leadership
    pub fn reduce_morale(&mut self, amount: f32) {
        self.morale = (self.morale - amount).max(0.0);
    }

    /// Reduce loyalty (can lead to defection)
    pub fn reduce_loyalty(&mut self, amount: f32) {
        self.loyalty = (self.loyalty - amount).max(0.0);
    }

    /// Increase corruption from atrocities or exploitation
    pub fn add_corruption(&mut self, amount: f32) {
        self.corruption = (self.corruption + amount).min(1.0);
    }

    pub fn rest(&mut self) {
        self.fatigue = (self.fatigue - 0.3).max(0.0);
        self.morale = (self.morale + 0.1).min(1.0);
    }

    pub fn resupply(&mut self) {
        self.supply = 1.0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum UnitType {
    Infantry,
    Cavalry,
    Archer,
    Siege,
    Naval,
}

impl UnitType {
    pub fn base_attack(&self) -> f32 {
        match self {
            UnitType::Infantry => 8.0,
            UnitType::Cavalry => 12.0,
            UnitType::Archer => 6.0,
            UnitType::Siege => 15.0,
            UnitType::Naval => 18.0,
        }
    }

    pub fn base_defense(&self) -> f32 {
        match self {
            UnitType::Infantry => 10.0,
            UnitType::Cavalry => 6.0,
            UnitType::Archer => 5.0,
            UnitType::Siege => 3.0,
            UnitType::Naval => 12.0,
        }
    }

    pub fn base_health(&self) -> f32 {
        match self {
            UnitType::Infantry => 100.0,
            UnitType::Cavalry => 80.0,
            UnitType::Archer => 70.0,
            UnitType::Siege => 60.0,
            UnitType::Naval => 150.0,
        }
    }

    pub fn attack_range(&self) -> u32 {
        match self {
            UnitType::Infantry => 1, // Melee
            UnitType::Cavalry => 1,  // Melee
            UnitType::Archer => 2,   // Ranged
            UnitType::Siege => 3,    // Long range
            UnitType::Naval => 2,    // Naval bombardment
        }
    }

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

use bevy::prelude::Reflect;
use bevy_ecs::prelude::*;
use moonshine_save::prelude::*;
use serde::{Deserialize, Serialize};

use super::civilization::CivId;

/// Represents a game event that can be triggered
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[require(Save)]
pub struct GameEvent {
    pub event_id: String,
    pub affected_civ: CivId,
    pub title: String,
    pub description: String,
    pub effects: Vec<EventEffect>,
    pub choices: Vec<EventChoice>,
    pub has_been_shown: bool,
    pub turn_triggered: u32,
}

/// The type of trigger condition for an event
#[derive(Debug, Clone, Serialize, Deserialize, Reflect, PartialEq)]
pub enum EventTriggerType {
    /// Random event with probability per turn
    Random { probability: f32 },
    /// Triggered when a resource reaches a threshold
    ResourceThreshold {
        resource: ResourceType,
        threshold: f32,
        comparison: ThresholdComparison,
    },
    /// Triggered when a unit type is created for the first time
    FirstUnitCreated { unit_type: String },
    /// Triggered when meeting another civilization for the first time
    FirstCivContact { other_civ: Option<CivId> },
    /// Triggered when a technology is researched
    TechnologyResearched { tech_name: String },
    /// Triggered when a city is founded
    CityFounded { city_count: u32 },
    /// Triggered at a specific turn
    SpecificTurn { turn: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, PartialEq)]
pub enum ResourceType {
    Gold,
    Food,
    Production,
    Science,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, PartialEq)]
pub enum ThresholdComparison {
    GreaterThan,
    LessThan,
    Equal,
}

/// Effects that an event can have on a civilization
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum EventEffect {
    /// Modify gold
    GoldChange(f32),
    /// Modify happiness/stability
    HappinessChange(f32),
    /// Grant a free unit
    GrantUnit { unit_type: String },
    /// Grant a technology
    GrantTechnology { tech_name: String },
    /// Modify production in all cities
    ProductionModifier {
        multiplier: f32,
        duration_turns: u32,
    },
    /// Apply a temporary trait modifier
    TemporaryTraitModifier {
        trait_name: String,
        modifier: f32,
        duration_turns: u32,
    },
}

/// A choice the player (or AI) can make in response to an event
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EventChoice {
    pub text: String,
    pub effects: Vec<EventEffect>,
}

/// Component marking civilizations that have experienced specific event types
/// Used to track "first time" events
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[require(Save)]
pub struct EventHistory {
    pub units_created: Vec<String>,
    pub civs_met: Vec<CivId>,
    pub technologies_researched: Vec<String>,
    pub triggered_events: Vec<String>,
}

impl EventHistory {
    pub fn has_created_unit(&self, unit_type: &str) -> bool {
        self.units_created.contains(&unit_type.to_string())
    }

    pub fn record_unit_created(&mut self, unit_type: String) {
        if !self.units_created.contains(&unit_type) {
            self.units_created.push(unit_type);
        }
    }

    pub fn has_met_civ(&self, civ_id: CivId) -> bool {
        self.civs_met.contains(&civ_id)
    }

    pub fn record_civ_met(&mut self, civ_id: CivId) {
        if !self.civs_met.contains(&civ_id) {
            self.civs_met.push(civ_id);
        }
    }

    pub fn has_researched_tech(&self, tech_name: &str) -> bool {
        self.technologies_researched
            .contains(&tech_name.to_string())
    }

    pub fn record_tech_researched(&mut self, tech_name: String) {
        if !self.technologies_researched.contains(&tech_name) {
            self.technologies_researched.push(tech_name);
        }
    }

    pub fn has_triggered_event(&self, event_id: &str) -> bool {
        self.triggered_events.contains(&event_id.to_string())
    }

    pub fn record_event_triggered(&mut self, event_id: String) {
        if !self.triggered_events.contains(&event_id) {
            self.triggered_events.push(event_id);
        }
    }
}

/// Marker component for pending events that need to be displayed
#[derive(Component, Debug, Clone)]
pub struct PendingEvent;

/// Component to track active event modifiers
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[require(Save)]
pub struct ActiveEventModifiers {
    pub production_multipliers: Vec<(f32, u32)>,
    pub trait_modifiers: Vec<(String, f32, u32)>,
}

impl ActiveEventModifiers {
    pub fn add_production_multiplier(&mut self, multiplier: f32, duration: u32) {
        self.production_multipliers.push((multiplier, duration));
    }

    pub fn add_trait_modifier(&mut self, trait_name: String, modifier: f32, duration: u32) {
        self.trait_modifiers.push((trait_name, modifier, duration));
    }

    pub fn update_turn(&mut self) {
        self.production_multipliers.retain_mut(|(_, turns)| {
            *turns = turns.saturating_sub(1);
            *turns > 0
        });

        self.trait_modifiers.retain_mut(|(_, _, turns)| {
            *turns = turns.saturating_sub(1);
            *turns > 0
        });
    }

    pub fn get_total_production_multiplier(&self) -> f32 {
        self.production_multipliers
            .iter()
            .map(|(mult, _)| mult)
            .product()
    }
}

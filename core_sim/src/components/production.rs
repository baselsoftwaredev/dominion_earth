use super::civilization::CivId;
use super::military::UnitType;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Production queue for a capital/city
#[derive(Debug, Clone)]
pub struct ProductionQueue {
    pub owner: CivId,
    pub queue: Vec<ProductionItem>,
    pub current_production: Option<ProductionItem>,
    pub accumulated_production: f32,
}

// Manual Component implementation
impl Component for ProductionQueue {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl ProductionQueue {
    pub fn new(owner: CivId) -> Self {
        Self {
            owner,
            queue: Vec::new(),
            current_production: None,
            accumulated_production: 0.0,
        }
    }

    pub fn add_to_queue(&mut self, item: ProductionItem) {
        self.queue.push(item);
    }

    pub fn start_next_production(&mut self) {
        if self.current_production.is_none() && !self.queue.is_empty() {
            self.current_production = Some(self.queue.remove(0));
            self.accumulated_production = 0.0;
        }
    }

    pub fn add_production(&mut self, amount: f32) -> Option<ProductionItem> {
        if let Some(ref current) = self.current_production {
            self.accumulated_production += amount;
            
            if self.accumulated_production >= current.production_cost() {
                let completed_item = self.current_production.take();
                self.accumulated_production = 0.0;
                self.start_next_production();
                completed_item
            } else {
                None
            }
        } else {
            self.start_next_production();
            None
        }
    }

    pub fn get_progress_percentage(&self) -> f32 {
        if let Some(ref current) = self.current_production {
            (self.accumulated_production / current.production_cost()).min(1.0)
        } else {
            0.0
        }
    }

    pub fn cancel_current_production(&mut self) -> Option<ProductionItem> {
        let cancelled = self.current_production.take();
        self.accumulated_production = 0.0;
        self.start_next_production();
        cancelled
    }

    pub fn is_producing(&self) -> bool {
        self.current_production.is_some()
    }

    pub fn queue_length(&self) -> usize {
        self.queue.len()
    }
}

/// Items that can be produced in a city
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductionItem {
    Unit(UnitType),
    Building(super::city::BuildingType),
}

impl ProductionItem {
    pub fn production_cost(&self) -> f32 {
        match self {
            ProductionItem::Unit(unit_type) => unit_type.production_cost(),
            ProductionItem::Building(building_type) => building_type.production_cost(),
        }
    }

    pub fn gold_cost(&self) -> f32 {
        match self {
            ProductionItem::Unit(unit_type) => unit_type.cost(),
            ProductionItem::Building(building_type) => building_type.gold_cost(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ProductionItem::Unit(unit_type) => unit_type.name(),
            ProductionItem::Building(building_type) => building_type.name(),
        }
    }
}

/// Player action marker for tracking what the player still needs to do
#[derive(Debug, Clone)]
pub struct PlayerAction {
    pub action_type: PlayerActionType,
    pub completed: bool,
}

// Manual Component implementation
impl Component for PlayerAction {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerActionType {
    MoveUnit(Entity),
    QueueProduction(Entity), // Entity is the capital
    EndTurn,
}

/// Resource to track if all player actions are completed
#[derive(Resource, Debug, Default)]
pub struct PlayerActionsComplete {
    pub all_units_moved: bool,
    pub all_productions_queued: bool,
    pub production_decisions_made_this_turn: bool,
    pub can_end_turn: bool,
}

impl PlayerActionsComplete {
    pub fn reset(&mut self) {
        self.all_units_moved = false;
        self.all_productions_queued = false;
        self.production_decisions_made_this_turn = false;
        self.can_end_turn = false;
    }

    pub fn update_can_end_turn(&mut self) {
        self.can_end_turn = self.all_units_moved && self.all_productions_queued;
    }
}

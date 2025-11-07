use super::civilization::CivId;
use super::military::UnitType;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Relationship component: marks a queue item entity as belonging to a production queue
#[derive(Component, Debug, Clone, Copy)]
pub struct QueueItemOf(pub Entity);

/// Relationship target: tracks all queue items that belong to a production queue
#[derive(Component, Debug, Clone, Default)]
pub struct QueuedItems(pub Vec<Entity>);

/// Data component for a queued production item
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueItem(pub ProductionItem);

/// Production queue for a capital/city
/// Now manages queue items as entities instead of inline Vec<ProductionItem>
#[derive(Debug, Clone)]
pub struct ProductionQueue {
    pub owner: CivId,
    /// Queue of item entities (in order)
    pub queue: Vec<Entity>,
    /// Current item being produced (entity reference)
    pub current_production: Option<Entity>,
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

    /// Add a queue item entity to the queue
    pub fn add_to_queue(&mut self, item_entity: Entity) {
        self.queue.push(item_entity);
    }

    /// Start producing the next item in the queue
    /// Returns the entity of the new current production item, or None if queue is empty
    pub fn start_next_production(&mut self) -> Option<Entity> {
        if self.current_production.is_none() && !self.queue.is_empty() {
            self.current_production = Some(self.queue.remove(0));
            self.accumulated_production = 0.0;
            self.current_production
        } else {
            None
        }
    }

    /// Add production progress. Requires access to queue item data to get production costs.
    /// Returns the completed item entity if production finished, None otherwise.
    pub fn add_production(
        &mut self,
        amount: f32,
        queue_items: &Query<&QueueItem>,
    ) -> Option<Entity> {
        if let Some(current_entity) = self.current_production {
            self.accumulated_production += amount;

            if let Ok(queue_item) = queue_items.get(current_entity) {
                if self.accumulated_production >= queue_item.0.production_cost() {
                    let completed_entity = self.current_production.take();
                    self.accumulated_production = 0.0;
                    self.start_next_production();
                    completed_entity
                } else {
                    None
                }
            } else {
                // Item entity not found, skip it
                self.current_production = None;
                self.accumulated_production = 0.0;
                self.start_next_production();
                None
            }
        } else {
            self.start_next_production();
            None
        }
    }

    /// Get the production cost of the current item
    pub fn get_current_production_cost(&self, queue_items: &Query<&QueueItem>) -> Option<f32> {
        self.current_production.and_then(|entity| {
            queue_items
                .get(entity)
                .ok()
                .map(|item| item.0.production_cost())
        })
    }

    pub fn get_progress_percentage(&self, queue_items: &Query<&QueueItem>) -> f32 {
        if let Some(cost) = self.get_current_production_cost(queue_items) {
            (self.accumulated_production / cost).min(1.0)
        } else {
            0.0
        }
    }

    pub fn cancel_current_production(&mut self) -> Option<Entity> {
        let cancelled = self.current_production.take();
        self.accumulated_production = 0.0;
        self.start_next_production();
        cancelled
    }

    pub fn is_producing(&self) -> bool {
        self.current_production.is_some()
    }

    pub fn queue_length(&self) -> usize {
        let current_count = if self.current_production.is_some() {
            1
        } else {
            0
        };
        current_count + self.queue.len()
    }

    /// Get the current production item data (requires Query access)
    pub fn get_current_production_item(
        &self,
        queue_items: &Query<&QueueItem>,
    ) -> Option<ProductionItem> {
        self.current_production
            .and_then(|entity| queue_items.get(entity).ok().map(|item| item.0.clone()))
    }

    /// Remove an item from the queue (returns the entity if found)
    pub fn remove_from_queue(&mut self, position: usize) -> Option<Entity> {
        if position < self.queue.len() {
            Some(self.queue.remove(position))
        } else {
            None
        }
    }

    /// Clear the entire queue and current production
    pub fn clear(&mut self) -> Vec<Entity> {
        let mut all_items = self.queue.drain(..).collect::<Vec<_>>();
        if let Some(current) = self.current_production.take() {
            all_items.insert(0, current);
        }
        self.accumulated_production = 0.0;
        all_items
    }
}

/// Serializable representation of production queue for save/load
/// This stores the actual ProductionItem data rather than entity IDs,
/// allowing queue state to be preserved across save/load cycles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableProductionQueue {
    pub owner: CivId,
    pub queue: Vec<ProductionItem>,
    pub current_production: Option<ProductionItem>,
    pub accumulated_production: f32,
}

impl SerializableProductionQueue {
    /// Convert a ProductionQueue (with entity references) to a serializable form
    /// Requires access to queue items to extract their data
    pub fn from_production_queue(queue: &ProductionQueue, queue_items: &Query<&QueueItem>) -> Self {
        let queue_items_data = queue
            .queue
            .iter()
            .filter_map(|entity| queue_items.get(*entity).ok().map(|item| item.0.clone()))
            .collect();

        let current_production_data = queue
            .current_production
            .and_then(|entity| queue_items.get(entity).ok().map(|item| item.0.clone()));

        SerializableProductionQueue {
            owner: queue.owner,
            queue: queue_items_data,
            current_production: current_production_data,
            accumulated_production: queue.accumulated_production,
        }
    }

    /// Convert from serializable form back to ProductionQueue (without entities yet)
    /// Entities should be spawned separately
    pub fn to_production_queue(&self) -> ProductionQueue {
        ProductionQueue {
            owner: self.owner,
            queue: Vec::new(),        // Will be populated by entity creation
            current_production: None, // Will be populated by entity creation
            accumulated_production: self.accumulated_production,
        }
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

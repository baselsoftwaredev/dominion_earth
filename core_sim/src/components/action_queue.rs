use bevy_ecs::component::{Component, Mutable};
use std::collections::VecDeque;
use crate::components::{
    ai::AIAction,
    civilization::CivId,
};

/// Action queue component for each civilization's AI
/// Each civilization maintains its own queue of actions to execute
#[derive(Debug, Clone)]
pub struct ActionQueue {
    pub civilization_id: CivId,
    pub queued_actions: VecDeque<QueuedAction>,
    pub max_queue_size: usize,
    pub current_turn_processed: usize,
    pub actions_per_turn_limit: usize,
}

// Manual Component implementation
impl Component for ActionQueue {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// A queued action with metadata for execution control
#[derive(Debug, Clone)]
pub struct QueuedAction {
    pub action: AIAction,
    pub turn_queued: u32,
    pub execution_turn: Option<u32>, // When this should be executed (for delayed actions)
    pub retry_count: u8,
    pub max_retries: u8,
    pub queue_priority: f32, // Higher values execute first
}

impl ActionQueue {
    pub fn new(civilization_id: CivId) -> Self {
        Self {
            civilization_id,
            queued_actions: VecDeque::new(),
            max_queue_size: constants::DEFAULT_MAX_QUEUE_SIZE,
            current_turn_processed: 0,
            actions_per_turn_limit: constants::DEFAULT_ACTIONS_PER_TURN,
        }
    }

    /// Add an action to the queue with default settings
    pub fn queue_action(&mut self, action: AIAction, current_turn: u32) {
        self.queue_action_with_settings(
            action,
            current_turn,
            None, // Execute immediately when possible
            constants::DEFAULT_MAX_RETRIES,
        );
    }

    /// Add an action to the queue with custom execution settings
    pub fn queue_action_with_settings(
        &mut self,
        action: AIAction,
        current_turn: u32,
        execution_turn: Option<u32>,
        max_retries: u8,
    ) {
        // Don't add if queue is full
        if self.queued_actions.len() >= self.max_queue_size {
            return;
        }

        let priority = Self::calculate_action_priority(&action);
        
        let queued_action = QueuedAction {
            action,
            turn_queued: current_turn,
            execution_turn,
            retry_count: 0,
            max_retries,
            queue_priority: priority,
        };

        // Insert in priority order (higher priority first)
        let insert_position = self.queued_actions
            .iter()
            .position(|existing| existing.queue_priority < priority)
            .unwrap_or(self.queued_actions.len());

        self.queued_actions.insert(insert_position, queued_action);
    }

    /// Remove and return the next action ready for execution
    pub fn dequeue_next_action(&mut self, current_turn: u32) -> Option<QueuedAction> {
        // Find the first action that's ready to execute
        if let Some(index) = self.queued_actions.iter().position(|action| {
            action.execution_turn.map_or(true, |turn| turn <= current_turn)
        }) {
            self.queued_actions.remove(index)
        } else {
            None
        }
    }

    /// Peek at the next action without removing it
    pub fn peek_next_action(&self, current_turn: u32) -> Option<&QueuedAction> {
        self.queued_actions.iter().find(|action| {
            action.execution_turn.map_or(true, |turn| turn <= current_turn)
        })
    }

    /// Requeue a failed action (up to max retries)
    pub fn requeue_failed_action(&mut self, mut action: QueuedAction, current_turn: u32) {
        action.retry_count += 1;
        if action.retry_count <= action.max_retries {
            // Delay retry by one turn
            action.execution_turn = Some(current_turn + 1);
            self.queued_actions.push_back(action);
        }
        // If max retries exceeded, action is dropped
    }

    /// Get number of actions remaining in queue
    pub fn get_queue_length(&self) -> usize {
        self.queued_actions.len()
    }

    /// Check if queue has capacity for more actions
    pub fn has_capacity(&self) -> bool {
        self.queued_actions.len() < self.max_queue_size
    }

    /// Clear all actions from the queue
    pub fn clear_queue(&mut self) {
        self.queued_actions.clear();
    }

    /// Get actions ready for current turn
    pub fn get_ready_actions_count(&self, current_turn: u32) -> usize {
        self.queued_actions.iter().filter(|action| {
            action.execution_turn.map_or(true, |turn| turn <= current_turn)
        }).count()
    }

    /// Reset turn processing counter
    pub fn reset_turn_processing(&mut self) {
        self.current_turn_processed = 0;
    }

    /// Check if we can process more actions this turn
    pub fn can_process_more_actions(&self) -> bool {
        self.current_turn_processed < self.actions_per_turn_limit
    }

    /// Increment the turn processing counter
    pub fn increment_turn_processing(&mut self) {
        self.current_turn_processed += 1;
    }

    /// Calculate priority for different action types
    fn calculate_action_priority(action: &AIAction) -> f32 {
        match action {
            AIAction::Defend { priority, .. } => priority + constants::DEFENSE_PRIORITY_BONUS,
            AIAction::Attack { priority, .. } => priority + constants::ATTACK_PRIORITY_BONUS,
            AIAction::BuildUnit { priority, .. } => priority + constants::UNIT_PRIORITY_BONUS,
            AIAction::Research { priority, .. } => priority + constants::RESEARCH_PRIORITY_BONUS,
            AIAction::Expand { priority, .. } => priority + constants::EXPANSION_PRIORITY_BONUS,
            AIAction::BuildBuilding { priority, .. } => priority + constants::BUILDING_PRIORITY_BONUS,
            AIAction::Trade { priority, .. } => priority + constants::TRADE_PRIORITY_BONUS,
            AIAction::Diplomacy { priority, .. } => priority + constants::DIPLOMACY_PRIORITY_BONUS,
        }
    }
}

/// Constants for action queue behavior
pub mod constants {
    pub const DEFAULT_MAX_QUEUE_SIZE: usize = 20;
    pub const DEFAULT_ACTIONS_PER_TURN: usize = 3;
    pub const DEFAULT_MAX_RETRIES: u8 = 2;
    
    // Priority bonuses for different action types (added to action's base priority)
    pub const DEFENSE_PRIORITY_BONUS: f32 = 10.0;
    pub const ATTACK_PRIORITY_BONUS: f32 = 8.0;
    pub const UNIT_PRIORITY_BONUS: f32 = 5.0;
    pub const RESEARCH_PRIORITY_BONUS: f32 = 3.0;
    pub const EXPANSION_PRIORITY_BONUS: f32 = 4.0;
    pub const BUILDING_PRIORITY_BONUS: f32 = 2.0;
    pub const TRADE_PRIORITY_BONUS: f32 = 1.0;
    pub const DIPLOMACY_PRIORITY_BONUS: f32 = 6.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{ai::AIAction, military::UnitType, position::Position};

    #[test]
    fn test_action_queue_basic_operations() {
        let civ_id = CivId(1);
        let mut queue = ActionQueue::new(civ_id);
        
        let action = AIAction::BuildUnit {
            unit_type: UnitType::Infantry,
            position: Position { x: 0, y: 0 },
            priority: 5.0,
        };
        
        queue.queue_action(action, 1);
        assert_eq!(queue.get_queue_length(), 1);
        
        let dequeued = queue.dequeue_next_action(1);
        assert!(dequeued.is_some());
        assert_eq!(queue.get_queue_length(), 0);
    }

    #[test]
    fn test_priority_ordering() {
        let civ_id = CivId(1);
        let mut queue = ActionQueue::new(civ_id);
        
        // Add low priority action first
        let low_priority = AIAction::Trade {
            partner: CivId(2),
            resource: crate::resources::Resource::Gold,
            priority: 1.0,
        };
        queue.queue_action(low_priority, 1);
        
        // Add high priority action second
        let high_priority = AIAction::Defend {
            position: Position { x: 0, y: 0 },
            priority: 8.0,
        };
        queue.queue_action(high_priority, 1);
        
        // High priority should come out first
        let first_action = queue.dequeue_next_action(1).unwrap();
        if let AIAction::Defend { .. } = first_action.action {
            // This is correct
        } else {
            panic!("Expected defense action to have higher priority");
        }
    }
}

use crate::{
    components::{AIAction, ActionQueue, CivId, Civilization, QueuedAction},
    resources::CurrentTurn,
};
use bevy_ecs::prelude::*;

/// System to spawn action queues for new civilizations
pub fn spawn_action_queues_for_new_civilizations(
    mut commands: Commands,
    new_civs_query: Query<(Entity, &CivId), (Added<Civilization>, Without<ActionQueue>)>,
) {
    for (entity, civ_id) in new_civs_query.iter() {
        let action_queue = ActionQueue::new(*civ_id);
        commands.entity(entity).insert(action_queue);
    }
}

/// System to process action queues for all civilizations each turn
pub fn process_civilization_action_queues(
    mut queue_query: Query<(Entity, &mut ActionQueue, &CivId)>,
    current_turn: Res<CurrentTurn>,
    mut commands: Commands,
) {
    let current_turn_number = current_turn.0;

    for (entity, mut action_queue, _civ_id) in queue_query.iter_mut() {
        action_queue.reset_turn_processing();

        let mut failed_actions = Vec::new();

        while action_queue.can_process_more_actions() {
            if let Some(queued_action) = action_queue.dequeue_next_action(current_turn_number) {
                let execution_result = execute_queued_action(&queued_action, entity, &mut commands);

                if execution_result.is_ok() {
                    action_queue.increment_turn_processing();
                } else {
                    failed_actions.push(queued_action);
                }
            } else {
                break;
            }
        }

        for failed_action in failed_actions {
            action_queue.requeue_failed_action(failed_action, current_turn_number);
        }
    }
}

/// System to populate action queues with AI-generated decisions
pub fn populate_action_queues_from_ai_decisions(
    ai_decisions: Vec<(CivId, Vec<AIAction>)>,
    mut queue_query: Query<(&mut ActionQueue, &CivId)>,
    current_turn: Res<CurrentTurn>,
) {
    let current_turn_number = current_turn.0;

    for (civ_id, decisions) in ai_decisions {
        if let Some((mut action_queue, _)) = queue_query
            .iter_mut()
            .find(|(_, queue_civ_id)| **queue_civ_id == civ_id)
        {
            for decision in decisions {
                if action_queue.has_capacity() {
                    action_queue.queue_action(decision.clone(), current_turn_number);
                }
            }
        }
    }
}

/// Execute a queued action - returns Ok if successful, Err if failed
fn execute_queued_action(
    queued_action: &QueuedAction,
    _civ_entity: Entity,
    _commands: &mut Commands,
) -> Result<(), ActionExecutionError> {
    match &queued_action.action {
        AIAction::BuildUnit { .. } => Ok(()),

        AIAction::Research { .. } => Ok(()),

        AIAction::Expand { .. } => Ok(()),

        AIAction::BuildBuilding { .. } => Ok(()),

        AIAction::Trade { .. } => Ok(()),

        AIAction::Attack { .. } => Ok(()),

        AIAction::Diplomacy { .. } => Ok(()),

        AIAction::Defend { .. } => Ok(()),

        AIAction::Explore { .. } => Ok(()),
    }
}

/// Error types for action execution
#[derive(Debug)]
pub enum ActionExecutionError {
    InsufficientResources,
    InvalidTarget,
    TileOccupied,
    DiplomaticRestriction,
    TechnicalFailure,
}

/// Helper function to add urgent actions to front of queue
pub fn add_urgent_action_to_queue(
    civ_id: CivId,
    action: AIAction,
    mut queue_query: Query<(&mut ActionQueue, &CivId)>,
    current_turn: &CurrentTurn,
) {
    if let Some((mut action_queue, _)) = queue_query
        .iter_mut()
        .find(|(_, queue_civ_id)| **queue_civ_id == civ_id)
    {
        if action_queue.has_capacity() {
            // Set high priority and immediate execution
            action_queue.queue_action_with_settings(
                action.clone(),
                current_turn.0,
                None, // Execute immediately
                1,    // Only one retry for urgent actions
            );
        }
    }
}

/// Debug function to log queue status for all civilizations
pub fn log_all_action_queue_status(
    queue_query: Query<(&ActionQueue, &CivId)>,
    current_turn: Res<CurrentTurn>,
) {
    for (action_queue, _civ_id) in queue_query.iter() {
        let _ready_actions = action_queue.get_ready_actions_count(current_turn.0);
        // TODO: Add logging when needed
    }
}

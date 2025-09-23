use crate::game;
use crate::ui::bevy_hui::production_orders::handle_production_updated_events;
use bevy::prelude::*;

pub struct CoreSimulationPlugin;

impl Plugin for CoreSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<core_sim::ProductionUpdated>()
            .add_event::<core_sim::ProcessAITurn>()
            .add_event::<core_sim::AITurnComplete>()
            .add_event::<core_sim::AllAITurnsComplete>()
            .add_event::<core_sim::StartPlayerTurn>()
            .init_resource::<core_sim::TurnPhase>()
            .add_systems(Startup, game::setup_game)
            .add_systems(
                Update,
                (
                    core_sim::spawn_action_queues_for_new_civilizations,
                    crate::ai_decision_systems::generate_ai_decisions_on_ai_turn,
                    core_sim::process_civilization_action_queues,
                    core_sim::initialize_production_queues,
                    core_sim::handle_player_production_orders,
                    core_sim::handle_skip_production,
                    core_sim::execute_movement_orders,
                    core_sim::execute_ai_movement_orders,
                    core_sim::clear_completed_movement_orders,
                    core_sim::check_player_actions_complete,
                    core_sim::handle_turn_advance_requests,
                    core_sim::handle_ai_turn_processing,
                    core_sim::handle_ai_turn_completion,
                    core_sim::handle_all_ai_turns_complete,
                    core_sim::auto_advance_turn_system,
                    handle_production_updated_events,
                )
                    .chain(),
            );
    }
}

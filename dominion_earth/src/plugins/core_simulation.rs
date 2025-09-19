use crate::game;
use crate::ui::bevy_hui::production_orders::handle_production_updated_events;
use bevy::prelude::*;

pub struct CoreSimulationPlugin;

impl Plugin for CoreSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<core_sim::ProductionUpdated>()
            .add_systems(Startup, game::setup_game)
            .add_systems(
                Update,
                (
                    core_sim::spawn_action_queues_for_new_civilizations,
                    game::generate_and_populate_ai_decisions,
                    core_sim::process_civilization_action_queues,
                    core_sim::initialize_production_queues,
                    core_sim::handle_player_production_orders,
                    core_sim::handle_skip_production,
                    core_sim::execute_movement_orders,
                    core_sim::clear_completed_movement_orders,
                    core_sim::check_player_actions_complete,
                    core_sim::handle_turn_advance_requests,
                    core_sim::auto_advance_turn_system,
                    handle_production_updated_events,
                )
                    .chain(),
            );
    }
}

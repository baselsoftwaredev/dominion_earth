use crate::game;
use bevy::prelude::*;

/// Plugin for core simulation systems
pub struct CoreSimulationPlugin;

impl Plugin for CoreSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Game Setup
            .add_systems(Startup, game::setup_game)
            // Core Game Loop Systems
            .add_systems(
                Update,
                (
                    // Action Queue Systems (run first)
                    core_sim::spawn_action_queues_for_new_civilizations,
                    game::generate_and_populate_ai_decisions,
                    core_sim::process_civilization_action_queues,
                    // Production and Economy Systems
                    core_sim::initialize_production_queues,
                    core_sim::handle_player_production_orders,
                    core_sim::handle_skip_production,
                    core_sim::process_production_queues,
                    // Movement and Actions
                    core_sim::execute_movement_orders,
                    core_sim::clear_completed_movement_orders,
                    core_sim::check_player_actions_complete,
                    // Turn Management
                    core_sim::handle_turn_advance_requests,
                    core_sim::auto_advance_turn_system,
                )
                    .chain(),
            );
    }
}

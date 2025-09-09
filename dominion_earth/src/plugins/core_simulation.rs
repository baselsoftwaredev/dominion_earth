use bevy::prelude::*;
use crate::game;

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
                    // Production and Economy Systems
                    core_sim::initialize_production_queues,
                    core_sim::handle_player_production_orders,
                    core_sim::handle_skip_production,
                    core_sim::process_production_queues,
                    
                    // Movement and Actions
                    core_sim::process_player_movement_orders,
                    core_sim::check_player_actions_complete,
                    
                    // Turn Management
                    core_sim::handle_turn_advance_requests,
                    core_sim::auto_advance_turn_system,
                ).chain(), // Chain ensures proper execution order within the frame
            );
    }
}

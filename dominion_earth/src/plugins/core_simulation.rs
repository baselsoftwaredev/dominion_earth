use crate::game;
use crate::screens::Screen;
use bevy::prelude::*;

pub struct CoreSimulationPlugin;

impl Plugin for CoreSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<core_sim::ProductionUpdated>()
            .add_message::<core_sim::ProcessAITurn>()
            .add_message::<core_sim::AITurnComplete>()
            .add_message::<core_sim::AllAITurnsComplete>()
            .add_message::<core_sim::StartPlayerTurn>()
            .init_resource::<core_sim::TurnPhase>()
            .init_resource::<core_sim::TurnOrder>()
            .init_resource::<core_sim::FogOfWarMaps>()
            .add_systems(
                OnEnter(Screen::Gameplay),
                (
                    game::sync_settings_to_game_config,
                    load_event_definitions,
                    game::setup_game.after(game::sync_settings_to_game_config),
                    game::cleanup_extra_tiled_layers.after(game::setup_game),
                ),
            )
            .add_systems(
                Update,
                (
                    // Map loading and spawning - run until map is loaded
                    // Note: TMX loading is handled by bevy_ecs_tiled plugin automatically
                    game::convert_tiled_map_to_world_map.run_if(in_state(Screen::Gameplay)),
                    game::spawn_civilizations_when_ready
                        .after(game::convert_tiled_map_to_world_map)
                        .run_if(in_state(Screen::Gameplay)),
                    game::initialize_fog_of_war
                        .after(game::spawn_civilizations_when_ready)
                        .run_if(in_state(Screen::Gameplay)),
                ),
            )
            .add_systems(
                Update,
                (
                    // Core gameplay systems
                    game::initialize_active_civ_turn
                        .run_if(resource_exists::<core_sim::resources::ActiveCivTurn>),
                    game::initialize_turn_order.run_if(resource_exists::<core_sim::TurnOrder>),
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
                    core_sim::handle_turn_transition_complete,
                    core_sim::auto_advance_turn_system,
                    // Event systems
                    core_sim::systems::events::check_event_triggers,
                    core_sim::systems::events::update_event_modifiers
                        .run_if(resource_changed::<core_sim::resources::CurrentTurn>),
                    core_sim::systems::events::apply_event_effects,
                )
                    .chain()
                    .run_if(in_state(Screen::Gameplay)),
            )
            .add_systems(
                Update,
                core_sim::update_fog_of_war.run_if(in_state(Screen::Gameplay)),
            );
    }
}

/// Loads event definitions from RON file during game setup.
///
/// This system runs on entering the Gameplay screen. It loads event definitions
/// from `dominion_earth/assets/data/events.ron` and inserts them as a resource.
/// If loading fails, an empty resource is inserted as a fallback to prevent crashes.
fn load_event_definitions(mut commands: Commands) {
    use core_sim::EventDataLoader;

    match EventDataLoader::load_from_ron("dominion_earth/assets/data/events.ron") {
        Ok(event_definitions) => {
            println!(
                "✅ Loaded {} event definitions",
                event_definitions.events.len()
            );
            commands.insert_resource(event_definitions);
        }
        Err(e) => {
            println!("❌ Failed to load event definitions: {}", e);
            commands.insert_resource(core_sim::resources::EventDefinitions::default());
        }
    }
}

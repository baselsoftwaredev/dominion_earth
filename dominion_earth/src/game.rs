//! Main game state and world setup for Dominion Earth

use crate::civilization_spawning::spawn_initial_civilizations;
use crate::constants::game::{map, timing};
use crate::debug_utils::{DebugLogging, DebugUtils};
use ai_planner::ai_coordinator::AICoordinatorSystem;
use bevy::prelude::*;
use core_sim::{
    resources::{GameConfig, GameRng, TurnAdvanceRequest, WorldMap},
    world_gen,
};
use rand::SeedableRng;

/// Main game state resource
#[derive(Resource)]

pub struct GameState {
    pub _ai_coordinator: AICoordinatorSystem,
    pub paused: bool,
    pub auto_advance: bool,
    pub ai_only: bool,
    pub total_civilizations: u32,
    pub turn_timer: Timer,
    pub next_turn_requested: bool,
}

impl GameState {
    pub fn with_auto_advance(auto: bool) -> Self {
        Self {
            _ai_coordinator: AICoordinatorSystem::new(),
            paused: false,
            auto_advance: auto,
            ai_only: false,
            total_civilizations: 2,
            turn_timer: Timer::from_seconds(timing::BASE_TURN_TIMER_SECONDS, TimerMode::Repeating),
            next_turn_requested: false,
        }
    }

    pub fn with_auto_advance_and_ai_only(auto: bool, ai_only: bool) -> Self {
        Self {
            _ai_coordinator: AICoordinatorSystem::new(),
            paused: false,
            auto_advance: auto,
            ai_only,
            total_civilizations: 2,
            turn_timer: Timer::from_seconds(timing::BASE_TURN_TIMER_SECONDS, TimerMode::Repeating),
            next_turn_requested: false,
        }
    }

    pub fn new(auto_advance: bool, ai_only: bool, total_civilizations: u32) -> Self {
        Self {
            _ai_coordinator: AICoordinatorSystem::new(),
            paused: false,
            auto_advance,
            ai_only,
            total_civilizations,
            turn_timer: Timer::from_seconds(timing::BASE_TURN_TIMER_SECONDS, TimerMode::Repeating),
            next_turn_requested: false,
        }
    }
}

/// Setup the initial game world
pub fn setup_game(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    mut rng: ResMut<GameRng>,
    game_config: Res<GameConfig>,
    game_state: Res<GameState>,
    debug_logging: Res<DebugLogging>,
) {
    // Initialize the random number generator with configured seed
    rng.0 = rand_pcg::Pcg64::seed_from_u64(game_config.random_seed);
    DebugUtils::log_world_generation(&debug_logging, game_config.random_seed);

    // Generate the world map
    *world_map =
        world_gen::generate_island_map(map::DEFAULT_WIDTH, map::DEFAULT_HEIGHT, &mut rng.0);

    println!(
        "World map generated with size {}x{}",
        world_map.width, world_map.height
    );

    // Spawn initial civilizations
    println!(
        "About to spawn {} civilizations",
        game_state.total_civilizations
    );

    spawn_initial_civilizations(
        &mut commands,
        &mut world_map,
        &mut rng.0,
        &debug_logging,
        game_state.ai_only,
        game_state.total_civilizations,
    );

    println!("Finished spawning civilizations");
    DebugUtils::log_world_initialization(&debug_logging, world_map.width, world_map.height);
}

/// Initialize fog of war for all civilizations after they're spawned
pub fn initialize_fog_of_war(
    mut fog_of_war: ResMut<core_sim::FogOfWarMaps>,
    world_map: Res<WorldMap>,
    civilizations: Query<&core_sim::Civilization>,
) {
    println!(
        "FOG_OF_WAR: Initializing fog of war for {} civilizations",
        civilizations.iter().count()
    );
    for civ in civilizations.iter() {
        core_sim::initialize_fog_of_war_for_civ(civ.id, &mut fog_of_war, &world_map);
        println!(
            "FOG_OF_WAR: Initialized for civ {:?} ({})",
            civ.id, civ.name
        );
    }
}

/// Main game update system - optimized to only update when necessary
pub fn game_update_system(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut turn_advance: ResMut<TurnAdvanceRequest>,
) {
    if game_state.paused {
        return;
    }

    game_state.turn_timer.tick(time.delta());

    let should_advance = if game_state.auto_advance {
        game_state.turn_timer.just_finished()
    } else {
        game_state.next_turn_requested
    };

    // Only update when the flag actually changes to prevent unnecessary triggers
    if turn_advance.0 != should_advance {
        turn_advance.0 = should_advance;
    }

    if should_advance {
        game_state.next_turn_requested = false;
    }
}

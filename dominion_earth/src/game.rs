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
    save_load_state: Res<crate::plugins::save_load::SaveLoadState>,
) {
    // Skip setup if we're loading from a save file
    if save_load_state.is_loading_from_save {
        println!("Skipping game setup - loading from save");
        return;
    }

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

pub fn initialize_active_civ_turn(
    mut active_civ_turn: ResMut<core_sim::resources::ActiveCivTurn>,
    civilizations: Query<&core_sim::Civilization>,
    mut initialized: Local<bool>,
) {
    if *initialized || civilizations.is_empty() {
        return;
    }

    let mut civ_ids: Vec<core_sim::CivId> = civilizations.iter().map(|civ| civ.id).collect();

    civ_ids.sort_by_key(|id| id.0);

    active_civ_turn.civs_per_turn = civ_ids.clone();
    active_civ_turn.current_civ_index = 0;

    *initialized = true;
}

pub fn initialize_turn_order(
    mut turn_order: ResMut<core_sim::TurnOrder>,
    mut turn_phase: ResMut<core_sim::TurnPhase>,
    civilizations: Query<&core_sim::Civilization>,
    player_civs: Query<&core_sim::Civilization, With<core_sim::PlayerControlled>>,
    mut initialized: Local<bool>,
    debug_logging: Res<DebugLogging>,
) {
    if *initialized || civilizations.is_empty() {
        return;
    }

    let mut civ_ids: Vec<core_sim::CivId> = civilizations.iter().map(|civ| civ.id).collect();

    civ_ids.sort_by_key(|id| id.0);

    DebugUtils::log_info(
        &debug_logging,
        &format!(
            "Initializing turn order with {} civilizations",
            civ_ids.len()
        ),
    );

    for civ_id in &civ_ids {
        if let Some(civ) = civilizations.iter().find(|c| c.id == *civ_id) {
            DebugUtils::log_info(
                &debug_logging,
                &format!("  - Civ {}: {}", civ_id.0, civ.name),
            );
        }
    }

    turn_order.civilizations = civ_ids.clone();
    turn_order.current_index = 0;

    if let Some(first_civ) = turn_order.current_civ() {
        initialize_starting_turn_phase(
            turn_phase.as_mut(),
            first_civ,
            &player_civs,
            &debug_logging,
        );
    }

    *initialized = true;

    info!(
        "🎵 Initialized TurnOrder with {} civilizations: {:?}",
        civ_ids.len(),
        civ_ids
    );
}

fn initialize_starting_turn_phase(
    turn_phase: &mut core_sim::TurnPhase,
    first_civ: core_sim::CivId,
    player_civs: &Query<&core_sim::Civilization, With<core_sim::PlayerControlled>>,
    debug_logging: &Res<DebugLogging>,
) {
    let is_player = player_civs.iter().any(|civ| civ.id == first_civ);

    if is_player {
        *turn_phase = core_sim::TurnPhase::CivilizationTurn {
            current_civ: first_civ,
        };
        DebugUtils::log_info(
            debug_logging,
            &format!("Starting with player civilization {}", first_civ.0),
        );
    } else {
        *turn_phase = core_sim::TurnPhase::WaitingForNextTurn {
            next_civ: first_civ,
        };
        DebugUtils::log_info(
            debug_logging,
            &format!(
                "Starting with AI civilization {} (waiting for user to press Next Turn)",
                first_civ.0
            ),
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

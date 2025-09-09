use crate::constants::game::{civilizations, map, personality, timing};
use crate::debug_utils::{DebugLogging, DebugUtils};
use ai_planner::ai_coordinator::AICoordinatorSystem;
use bevy::prelude::*;
use core_sim::{
    self,
    resources::{GameConfig, GameRng, WorldMap},
    world_gen, ActiveThisTurn, Building, BuildingType, Capital, CapitalAge, City, CivId,
    CivPersonality, Civilization, Economy, Military, MilitaryUnit, PlayerControlled,
    ProductionQueue, Technologies, UnitType,
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
    pub simulation_speed: f32,
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
            simulation_speed: timing::DEFAULT_SIMULATION_SPEED,
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
            simulation_speed: timing::DEFAULT_SIMULATION_SPEED,
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
            simulation_speed: timing::DEFAULT_SIMULATION_SPEED,
            turn_timer: Timer::from_seconds(timing::BASE_TURN_TIMER_SECONDS, TimerMode::Repeating),
            next_turn_requested: false,
        }
    }
}

/// Setup the initial game world
pub fn setup_game(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    // mut influence_map: ResMut<InfluenceMap>,
    mut rng: ResMut<GameRng>,
    game_config: Res<GameConfig>,
    game_state: Res<GameState>,
    debug_logging: Res<DebugLogging>,
) {
    // Initialize the random number generator with configured seed
    rng.0 = rand_pcg::Pcg64::seed_from_u64(game_config.random_seed);
    DebugUtils::log_world_generation(&debug_logging, game_config.random_seed);

    // Generate the world map (reduced size for better performance)
    *world_map =
        world_gen::generate_island_map(map::DEFAULT_WIDTH, map::DEFAULT_HEIGHT, &mut rng.0);

    // Initialize influence map
    // *influence_map = InfluenceMap::new(world_map.width, world_map.height);
    // influence_map.add_layer(InfluenceType::Strategic);
    // influence_map.add_layer(InfluenceType::Threat);

    // Spawn initial civilizations
    spawn_initial_civilizations(
        &mut commands,
        &mut world_map,
        &mut rng.0,
        &debug_logging,
        game_state.ai_only,
        game_state.total_civilizations,
    );

    DebugUtils::log_world_initialization(&debug_logging, world_map.width, world_map.height);
}

fn spawn_initial_civilizations(
    commands: &mut Commands,
    world_map: &mut WorldMap,
    rng: &mut rand_pcg::Pcg64,
    debug_logging: &DebugLogging,
    ai_only: bool,
    total_civilizations: u32,
) {
    use core_sim::{CivilizationDataLoader, CivilizationDefinition};
    use rand::seq::SliceRandom;

    // Load civilization data from RON file
    let civilization_data = match CivilizationDataLoader::load_from_ron("dominion_earth/assets/data/civilizations.ron") {
        Ok(data) => data,
        Err(e) => {
            DebugUtils::log_info(debug_logging, &format!("Failed to load civilization data: {}", e));
            return;
        }
    };

    // Select a random subset of civilizations
    let mut available_civs = civilization_data.civilizations.clone();
    available_civs.shuffle(rng);
    let selected_civs: Vec<CivilizationDefinition> = available_civs
        .into_iter()
        .take(total_civilizations as usize)
        .collect();

    // Generate random starting positions instead of using the fixed ones from RON
    let min_distance_between_civs = 5; // Minimum distance between civilizations
    let random_positions = CivilizationDataLoader::generate_random_starting_positions(
        &selected_civs,
        world_map,
        rng,
        min_distance_between_civs,
    );

    let mut spawned_count = 0;

    for (civ_index, civ_def) in selected_civs.into_iter().enumerate() {
        let civ_id = CivId(civ_index as u32);
        
        // Get the random position for this civilization
        let position = match random_positions.get(&civ_def.name) {
            Some(&pos) => pos,
            None => {
                DebugUtils::log_capital_spawn_skip(&debug_logging, &civ_def.name, 0, 0);
                continue;
            }
        };

        // Check if the position is on a buildable tile (double-check from random generation)
        let is_buildable = if let Some(tile) = world_map.get_tile(position) {
            match tile.terrain {
                core_sim::TerrainType::Plains | core_sim::TerrainType::Coast | core_sim::TerrainType::Forest => true,
                _ => false,
            }
        } else {
            false
        };

        if !is_buildable {
            DebugUtils::log_capital_spawn_skip(&debug_logging, &civ_def.name, position.x, position.y);
            continue;
        }

        // Convert civilization definition to game civilization
        let color = [civ_def.color.0, civ_def.color.1, civ_def.color.2];
        let personality = CivPersonality::from(civ_def.personality);

        let civilization = Civilization {
            id: civ_id,
            name: civ_def.name.clone(),
            color,
            capital: Some(position),
            personality,
            technologies: Technologies::default(),
            economy: Economy::default(),
            military: Military::default(),
        };

        // Spawn civilization entity
        let mut civ_entity_commands = commands.spawn((civilization, position, ActiveThisTurn));

        // Mark the first civilization as player-controlled if not ai_only mode
        let civ_index = civ_index as u32;
        if !ai_only && civ_index == 0 {
            civ_entity_commands.insert(core_sim::PlayerControlled);
            DebugUtils::log_info(
                &debug_logging,
                &format!("Marking {} as player-controlled civilization", civ_def.name),
            );
        } else {
            DebugUtils::log_info(
                &debug_logging,
                &format!("Marking {} as AI-controlled civilization", civ_def.name),
            );
        }

        // Create capital city using the capital name from RON data
        let city = City {
            name: civ_def.capital_name.clone(), // Use the actual capital name from RON
            owner: civ_id,
            population: 1000,
            production: 5.0,
            defense: 10.0,
            buildings: vec![Building {
                building_type: BuildingType::Granary,
                level: 1,
            }],
        };

        let capital = Capital {
            owner: civ_id,
            age: CapitalAge::Neolithic,
            sprite_index: CapitalAge::Neolithic.sprite_index(),
            established_turn: 0, // Starting at turn 0
        };

        DebugUtils::log_capital_spawn_success(
            &debug_logging,
            &civ_def.capital_name,
            &position,
            capital.sprite_index as usize,
        );

        // Spawn capital entity with both City and Capital components and ProductionQueue
        let mut capital_commands = commands.spawn((city, capital, position));

        // Add ProductionQueue to the capital
        capital_commands.insert(ProductionQueue::new(civ_id));

        // If this is a player-controlled civilization, mark the capital as player-controlled too
        if !ai_only && civ_index == 0 {
            capital_commands.insert(core_sim::PlayerControlled);
        }

        // Claim starting territory
        if let Some(tile) = world_map.get_tile_mut(position) {
            tile.owner = Some(civ_id);
            tile.city = Some(civ_def.capital_name.clone()); // Use actual capital name
        }

        // Spawn initial military unit
        let initial_unit = MilitaryUnit {
            id: civ_index as u32,
            owner: civ_id,
            unit_type: UnitType::Infantry,
            position,
            strength: 10.0,
            movement_remaining: 2,
            experience: 0.0,
        };

        let mut unit_commands = commands.spawn((initial_unit, position));

        // If this is a player-controlled civilization, mark the unit as player-controlled too
        if !ai_only && civ_index == 0 {
            unit_commands.insert(core_sim::PlayerControlled);
        }
        spawned_count += 1;
    }

    DebugUtils::log_civilization_spawn(&debug_logging, spawned_count);
}

/// Main game update system - optimized to only update when necessary
pub fn game_update_system(
    mut game_state: ResMut<GameState>,
    // mut current_turn: ResMut<CurrentTurn>,
    time: Res<Time>,
    mut turn_advance: ResMut<core_sim::resources::TurnAdvanceRequest>,
) {
    if game_state.paused {
        return;
    }

    // Update turn timer
    game_state.turn_timer.tick(time.delta());

    let should_advance = if game_state.auto_advance {
        game_state.turn_timer.just_finished()
    } else {
        game_state.next_turn_requested
    };

    // Only update the TurnAdvanceRequest resource when the flag actually changes
    // This prevents the turn system from being triggered unnecessarily every frame
    if turn_advance.0 != should_advance {
        turn_advance.0 = should_advance;
    }

    if should_advance {
        game_state.next_turn_requested = false;
    }
}

// fn advance_turn(
//     current_turn: &mut CurrentTurn,
//     game_state: &mut GameState,
//     _commands: &mut Commands,
// ) {
//     let turn_start = std::time::Instant::now();

//     // Extract current game state (simplified - using a basic core_sim::GameState)
//     let game_state_snapshot = core_sim::GameState {
//         turn: current_turn.0,
//         civilizations: std::collections::HashMap::new(), // TODO: Extract from ECS
//         current_player: None,
//     };

//     // Generate AI decisions
//     let ai_decisions = game_state
//         ._ai_coordinator
//         .generate_turn_decisions(&game_state_snapshot);

//     // Execute AI decisions (this would normally modify the world through systems)
//     let mut modified_state = game_state_snapshot;
//     let execution_results = game_state
//         ._ai_coordinator
//         .execute_decisions(&ai_decisions, &mut modified_state);

//     // Log execution results
//     for result in execution_results {
//         match result {
//             ExecutionResult::Success {
//                 civ_id,
//                 action_description,
//             } => {
//                 println!(
//                     "Turn {}: Civ {:?} - {}",
//                     current_turn.0, civ_id, action_description
//                 );
//             }
//             ExecutionResult::Failed { civ_id, reason } => {
//                 println!(
//                     "Turn {}: Civ {:?} failed - {}",
//                     current_turn.0, civ_id, reason
//                 );
//             }
//         }
//     }

//     // Advance turn counter
//     current_turn.0 += 1;

//     let turn_duration = turn_start.elapsed();
//     println!("Turn {} completed in {:?}", current_turn.0, turn_duration);

//     // Check for game end conditions
//     if current_turn.0 >= 500 {
//         println!("Game completed after 500 turns!");
//         // Could trigger end game logic here
//     }
// }

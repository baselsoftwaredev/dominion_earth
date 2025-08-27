use ai_planner::ai_coordinator::AICoordinatorSystem;
use bevy::prelude::*;
use core_sim::{
    self,
    resources::{GameConfig, GameRng, WorldMap},
    world_gen, ActiveThisTurn, Building, BuildingType, Capital, CapitalAge, City, CivId,
    CivPersonality, Civilization, Economy, Military, MilitaryUnit, Technologies, UnitType,
};
use rand::SeedableRng;

/// Main game state resource
#[derive(Resource)]

pub struct GameState {
    pub _ai_coordinator: AICoordinatorSystem,
    pub paused: bool,
    pub auto_advance: bool,
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
            simulation_speed: 1.0,
            turn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
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
) {
    // Initialize the random number generator with configured seed
    rng.0 = rand_pcg::Pcg64::seed_from_u64(game_config.random_seed);
    println!(
        "Generating world with random seed: {}",
        game_config.random_seed
    );

    // Generate the world map (reduced size for better performance)
    *world_map = world_gen::generate_island_map(50, 25, &mut rng.0);

    // Initialize influence map
    // *influence_map = InfluenceMap::new(world_map.width, world_map.height);
    // influence_map.add_layer(InfluenceType::Strategic);
    // influence_map.add_layer(InfluenceType::Threat);

    // Spawn initial civilizations
    spawn_initial_civilizations(&mut commands, &mut world_map, &mut rng.0);

    println!(
        "Game world initialized with {} x {} map (reduced size for performance)",
        world_map.width, world_map.height
    );
}

fn spawn_initial_civilizations(
    commands: &mut Commands,
    world_map: &mut WorldMap,
    rng: &mut rand_pcg::Pcg64,
) {
    let starting_positions = world_gen::get_starting_positions();

    for (i, (name, position, color)) in starting_positions.into_iter().take(20).enumerate() {
        let civ_id = CivId(i as u32);

        // Create civilization with random personality
        use rand::Rng;

        let personality = CivPersonality {
            land_hunger: rng.gen_range(0.2..0.8),
            industry_focus: rng.gen_range(0.2..0.8),
            tech_focus: rng.gen_range(0.2..0.8),
            interventionism: rng.gen_range(0.1..0.7),
            risk_tolerance: rng.gen_range(0.2..0.8),
            honor_treaties: rng.gen_range(0.3..0.9),
            militarism: rng.gen_range(0.2..0.8),
            isolationism: rng.gen_range(0.1..0.6),
        };

        let civilization = Civilization {
            id: civ_id,
            name: name.clone(),
            color,
            capital: Some(position),
            personality,
            technologies: Technologies::default(),
            economy: Economy::default(),
            military: Military::default(),
        };

        // Spawn civilization entity
        commands.spawn((civilization, position, ActiveThisTurn));

        // Spawn capital city with Capital component
        let city = City {
            name: format!("{} Capital", name),
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

        println!(
            "DEBUG: Spawning capital for {} at {:?} with sprite index {}",
            name, position, capital.sprite_index
        );

        // Spawn capital entity with both City and Capital components
        commands.spawn((city, capital, position));

        // Claim starting territory
        if let Some(tile) = world_map.get_tile_mut(position) {
            tile.owner = Some(civ_id);
            tile.city = Some(name);
        }

        // Spawn initial military unit
        let initial_unit = MilitaryUnit {
            id: i as u32,
            owner: civ_id,
            unit_type: UnitType::Infantry,
            position,
            strength: 10.0,
            movement_remaining: 2,
            experience: 0.0,
        };

        commands.spawn((initial_unit, position));
    }

    println!("Spawned 20 civilizations");
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

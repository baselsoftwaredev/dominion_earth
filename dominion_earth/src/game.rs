//! Main game state and world setup for Dominion Earth

use crate::civilization_spawning::spawn_initial_civilizations;
use crate::constants::game::{map, timing};
use crate::debug_utils::DebugUtils;
use ai_planner::ai_coordinator::AICoordinatorSystem;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core_sim::resources::{GameConfig, GameRng, TurnAdvanceRequest, WorldMap};
use rand::SeedableRng;

/// Main game state resource
#[derive(Resource)]

pub struct GameState {
    pub _ai_coordinator: AICoordinatorSystem,
    pub paused: bool,
    pub ai_only: bool,
    pub total_civilizations: u32,
    pub turn_timer: Timer,
    pub next_turn_requested: bool,
}

/// Resource to track if the TMX map has been loaded and converted
#[derive(Resource, Default)]
pub struct MapLoaded(pub bool);

impl GameState {
    pub fn with_auto_advance(auto: bool) -> Self {
        Self {
            _ai_coordinator: AICoordinatorSystem::new(),
            paused: false,
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
            ai_only,
            total_civilizations,
            turn_timer: Timer::from_seconds(timing::BASE_TURN_TIMER_SECONDS, TimerMode::Repeating),
            next_turn_requested: false,
        }
    }
}

/// Sync GameSettings to GameConfig before entering gameplay
/// This ensures that any settings changes (especially seed) are applied to the new game
pub fn sync_settings_to_game_config(
    game_settings: Res<crate::settings::GameSettings>,
    mut game_config: ResMut<GameConfig>,
    mut game_state: ResMut<GameState>,
) {
    // Update seed from settings
    if let Some(seed) = game_settings.seed {
        if game_config.random_seed != seed {
            game_config.random_seed = seed;
            crate::debug_println!("🎲 Updated game seed from settings: {}", seed);
        }
    } else {
        // Generate random seed if none is set
        use std::time::{SystemTime, UNIX_EPOCH};
        let random_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(42))
            .as_secs();
        game_config.random_seed = random_seed;
        crate::debug_println!("🎲 Generated random seed: {}", random_seed);
    }

    // Update AI-only mode from settings
    if game_config.ai_only != game_settings.ai_only || game_state.ai_only != game_settings.ai_only {
        game_config.ai_only = game_settings.ai_only;
        game_state.ai_only = game_settings.ai_only;
        crate::debug_println!(
            "🤖 Updated AI-only mode from settings: {}",
            game_settings.ai_only
        );
    }

    // Map is always Debug (static TMX loading)
    game_config.world_size = core_sim::resources::WorldSize::Debug;

    // Update civilization count from settings
    if game_state.total_civilizations != game_settings.num_civilizations {
        game_state.total_civilizations = game_settings.num_civilizations;
        crate::debug_println!(
            "👥 Updated civilization count from settings: {}",
            game_settings.num_civilizations
        );
    }
}

/// Setup the initial game world by spawning the TiledMap entity
/// The actual map loading and conversion happens asynchronously via the convert_tiled_to_world_map system
pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<GameRng>,
    game_config: Res<GameConfig>,
) {
    // Initialize the random number generator with configured seed
    rng.0 = rand_pcg::Pcg64::seed_from_u64(game_config.random_seed);
    DebugUtils::log_world_generation(game_config.random_seed);

    // For now, only support the Debug map loaded from TMX
    let map_path = "tiles/grass-land.tmx";

    commands.spawn((
        TiledMap(asset_server.load(map_path)),
        TilemapAnchor::BottomLeft,
    ));

    // Insert the MapLoaded resource to track when the map is ready
    commands.insert_resource(MapLoaded(false));

    println!("Spawned TiledMap entity for {}", map_path);
}

/// Clean up extra TiledMap background layers that might be stretching beyond the map bounds
/// bevy_ecs_tiled may spawn background/object layer entities that we don't need
pub fn cleanup_extra_tiled_layers(mut commands: Commands) {
    // This cleanup is handled automatically by bevy_ecs_tiled
    // If there are extra layers visible, they may be from the TMX file itself
    // Check the TMX file to ensure it only has one layer
}

/// Convert the Tiled map data to our WorldMap resource
/// This system reads the TilemapSize from the rendered tilemap
/// and populates the WorldMap resource with default terrain data
pub fn convert_tiled_map_to_world_map(
    mut world_map: ResMut<WorldMap>,
    mut map_loaded: ResMut<MapLoaded>,
    tilemap_q: Query<(&TilemapSize, &TilemapType), Added<TilemapSize>>,
    mut initialized: Local<bool>,
) {
    if *initialized {
        return;
    }

    let Ok((map_size, _map_type)) = tilemap_q.single() else {
        return;
    };

    println!(
        "Converting Tiled map to WorldMap: {}x{}",
        map_size.x, map_size.y
    );

    // Create new 2D tile vector with proper dimensions
    // For now, initialize all tiles as Plains (buildable terrain)
    let mut new_tiles: Vec<Vec<core_sim::resources::MapTile>> =
        vec![
            vec![
                core_sim::resources::MapTile {
                    terrain: core_sim::TerrainType::Plains,
                    owner: None,
                    city: None,
                    resource: None,
                };
                map_size.y as usize
            ];
            map_size.x as usize
        ];

    // Update the WorldMap resource
    world_map.width = map_size.x;
    world_map.height = map_size.y;
    world_map.tiles = new_tiles;

    println!(
        "Populated WorldMap with {}x{} tiles (all Plains)",
        world_map.width, world_map.height
    );
    map_loaded.0 = true;
    *initialized = true;
}

/// Tile ID to TerrainType mapping
/// Maps Tiled tile IDs to our terrain type system
fn map_tile_id_to_terrain(tile_id: u32) -> core_sim::components::TerrainType {
    use core_sim::components::TerrainType;

    match tile_id {
        1 => TerrainType::Plains,
        2 => TerrainType::Plains, // Grassland -> Plains
        3 => TerrainType::Desert,
        4 => TerrainType::Desert, // Tundra -> Desert (placeholder)
        5 => TerrainType::Ocean,
        6 => TerrainType::Mountains, // Note: Mountains (plural)
        7 => TerrainType::Hills,
        8 => TerrainType::Forest,
        _ => TerrainType::Plains, // default
    }
}

// Spawn civilizations only after the map has been fully loaded and converted
pub fn spawn_civilizations_when_ready(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    map_loaded: Res<MapLoaded>,
    mut rng: ResMut<GameRng>,
    game_state: Res<GameState>,
    mut initialized: Local<bool>,
) {
    if *initialized {
        return;
    }

    if !map_loaded.0 {
        // Still waiting for map to load
        return;
    }

    if world_map.width == 0 {
        warn!("Map not ready yet (width is 0)");
        return;
    }

    println!("🎮 MAP READY! Converting starting civilization spawning");
    println!(
        "About to spawn {} civilizations on {}x{} map",
        game_state.total_civilizations, world_map.width, world_map.height
    );

    spawn_initial_civilizations(
        &mut commands,
        &mut world_map,
        &mut rng.0,
        game_state.ai_only,
        game_state.total_civilizations,
    );

    println!("Finished spawning civilizations");
    DebugUtils::log_world_initialization(world_map.width, world_map.height);

    *initialized = true;
}

/// Initialize fog of war for all civilizations after they're spawned
pub fn initialize_fog_of_war(
    mut fog_of_war: ResMut<core_sim::FogOfWarMaps>,
    world_map: Res<WorldMap>,
    civilizations: Query<&core_sim::Civilization>,
    mut initialized: Local<bool>,
) {
    if *initialized {
        return;
    }

    if civilizations.is_empty() {
        return;
    }

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

    *initialized = true;
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
) {
    if *initialized || civilizations.is_empty() {
        return;
    }

    let mut civ_ids: Vec<core_sim::CivId> = civilizations.iter().map(|civ| civ.id).collect();

    civ_ids.sort_by_key(|id| id.0);

    DebugUtils::log_info(&format!(
        "Initializing turn order with {} civilizations",
        civ_ids.len()
    ));

    for civ_id in &civ_ids {
        if let Some(civ) = civilizations.iter().find(|c| c.id == *civ_id) {
            DebugUtils::log_info(&format!("  - Civ {}: {}", civ_id.0, civ.name));
        }
    }

    turn_order.civilizations = civ_ids.clone();
    turn_order.current_index = 0;

    if let Some(first_civ) = turn_order.current_civ() {
        initialize_starting_turn_phase(turn_phase.as_mut(), first_civ, &player_civs);
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
) {
    let is_player = player_civs.iter().any(|civ| civ.id == first_civ);

    if is_player {
        *turn_phase = core_sim::TurnPhase::CivilizationTurn {
            current_civ: first_civ,
        };
        DebugUtils::log_info(&format!(
            "Starting with player civilization {}",
            first_civ.0
        ));
    } else {
        *turn_phase = core_sim::TurnPhase::WaitingForNextTurn {
            next_civ: first_civ,
        };
        DebugUtils::log_info(&format!(
            "Starting with AI civilization {} (waiting for user to press Next Turn)",
            first_civ.0
        ));
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

    let should_advance = game_state.next_turn_requested;

    // Only update when the flag actually changes to prevent unnecessary triggers
    if turn_advance.0 != should_advance {
        turn_advance.0 = should_advance;
    }

    if should_advance {
        game_state.next_turn_requested = false;
    }
}

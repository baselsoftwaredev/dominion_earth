use bevy::prelude::*;
use bevy::scene::{DynamicScene, DynamicSceneRoot};
use bevy::tasks::IoTaskPool;
use core_sim::components::rendering::SpriteEntityReference;
use core_sim::components::turn_phases::TurnPhase;
use core_sim::resources::{ActiveCivTurn, CurrentTurn, GameConfig, MapTile, Resource, WorldMap};
use core_sim::{
    Building, BuildingType, Capital, CapitalAge, City, CivId, CivPersonality, CivStats,
    Civilization, Direction, Economy, FogOfWarMaps, Military, MilitaryUnit, PlayerControlled,
    PlayerMovementOrder, Position, Technologies, TerrainType, TradeRoute, UnitSelected, UnitType,
    VisibilityMap, VisibilityState,
};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Helper function to find save file path in the configured directory
fn find_save_file_path(filename: &str, save_dir: &str) -> Option<String> {
    let path = format!("{}/saves/{}", save_dir, filename);

    if Path::new(&path).exists() {
        info!("Found save file at: {}", path);
        Some(path)
    } else {
        warn!("Save file not found: {}", path);
        None
    }
}
/// Get the save directory from state or use default
fn get_save_directory(save_state: &SaveLoadState) -> String {
    if save_state.save_directory.is_empty() {
        // Default to project root
        std::env::current_dir()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string())
    } else {
        save_state.save_directory.clone()
    }
}

/// Plugin for handling game state saving and loading
pub struct SaveLoadPlugin {
    save_directory: Option<String>,
}

impl SaveLoadPlugin {
    pub fn new() -> Self {
        Self {
            save_directory: None,
        }
    }

    pub fn with_save_directory(save_dir: String) -> Self {
        Self {
            save_directory: Some(save_dir),
        }
    }
}

impl Default for SaveLoadPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        // Use configured save directory or default to project root
        let save_directory = if let Some(dir) = &self.save_directory {
            dir.clone()
        } else {
            std::env::current_dir()
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string())
        };

        app.insert_resource(SaveLoadState::with_save_directory(save_directory))
            .add_systems(Startup, setup_save_load_registry)
            .add_systems(
                Update,
                (
                    handle_save_requests,
                    handle_load_requests,
                    track_loaded_scenes,
                    restore_player_control_after_load,
                    refresh_fog_of_war_after_load,
                ),
            );
    }
}

/// Resource to track save/load operations
#[derive(Resource, Default)]
pub struct SaveLoadState {
    pub save_requested: bool,
    pub load_requested: bool,
    pub save_path: String,
    pub load_path: String,
    pub save_directory: String,
    pub needs_player_restore: bool,
    pub fog_of_war_needs_refresh: bool,
}

impl SaveLoadState {
    pub fn with_save_directory(save_dir: String) -> Self {
        Self {
            save_directory: save_dir,
            needs_player_restore: false,
            fog_of_war_needs_refresh: false,
            ..Default::default()
        }
    }
}

/// Event to request a save operation
#[derive(Event)]
pub struct SaveGameEvent {
    pub save_name: String,
}

/// Event to request a load operation
#[derive(Event)]
pub struct LoadGameEvent {
    pub save_name: String,
}

/// Component to mark entities that should be saved
#[derive(Component)]
pub struct Saveable;

/// Component to mark that we've already logged the load completion for a DynamicSceneRoot
#[derive(Component)]
struct LoadLoggedMarker;

/// Setup the type registry for reflection-based serialization
fn setup_save_load_registry(world: &mut World) {
    // Register all our custom types for reflection
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Position>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Direction>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<CivId>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Civilization>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<CivPersonality>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Technologies>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Economy>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Military>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<TradeRoute>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<MilitaryUnit>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<UnitType>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<TerrainType>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<City>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Capital>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<CapitalAge>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Building>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<BuildingType>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<CivStats>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<SpriteEntityReference>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<PlayerControlled>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<UnitSelected>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<PlayerMovementOrder>();

    // Register resources
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<WorldMap>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<CurrentTurn>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<ActiveCivTurn>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<TurnPhase>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<GameConfig>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<Resource>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<MapTile>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<FogOfWarMaps>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<VisibilityMap>();
    world
        .resource_mut::<AppTypeRegistry>()
        .write()
        .register::<VisibilityState>();

    info!("Save/Load type registry initialized");
}

/// Handle save game requests
fn handle_save_requests(
    mut save_state: ResMut<SaveLoadState>,
    current_turn: Option<Res<CurrentTurn>>,
    active_civ_turn: Option<Res<ActiveCivTurn>>,
    turn_phase: Option<Res<TurnPhase>>,
    game_config: Option<Res<GameConfig>>,
    world_map: Option<Res<WorldMap>>,
    fog_of_war: Option<Res<FogOfWarMaps>>,
    query: Query<(
        Entity,
        &Position,
        Option<&Civilization>,
        Option<&City>,
        Option<&Capital>,
        Option<&MilitaryUnit>,
        Option<&TerrainType>,
        Option<&CivId>,
        Option<&PlayerControlled>,
        Option<&UnitSelected>,
        Option<&PlayerMovementOrder>,
    )>,
    type_registry: Res<AppTypeRegistry>,
) {
    if !save_state.save_requested {
        return;
    }

    save_state.save_requested = false;

    // Create a new world with only the saveable entities and resources
    let mut save_world = World::new();

    // Copy the type registry to the save world
    let type_registry_clone = type_registry.clone();
    save_world.insert_resource(type_registry_clone);

    // Copy saveable resources
    if let Some(current_turn) = current_turn {
        save_world.insert_resource((*current_turn).clone());
        info!("Saved CurrentTurn: {:?}", *current_turn);
    }
    if let Some(active_civ_turn) = active_civ_turn {
        save_world.insert_resource((*active_civ_turn).clone());
        info!("Saved ActiveCivTurn: {:?}", *active_civ_turn);
    }
    if let Some(turn_phase) = turn_phase {
        save_world.insert_resource((*turn_phase).clone());
        info!("Saved TurnPhase: {:?}", *turn_phase);
    } else {
        warn!("TurnPhase resource not found during save!");
    }
    if let Some(game_config) = game_config {
        save_world.insert_resource((*game_config).clone());
        info!("Saved GameConfig: {:?}", *game_config);
    } else {
        warn!("GameConfig resource not found during save!");
    }
    if let Some(world_map) = world_map {
        save_world.insert_resource((*world_map).clone());
        info!("Saved WorldMap: {}x{}", world_map.width, world_map.height);
    }
    if let Some(fog_of_war) = fog_of_war {
        save_world.insert_resource((*fog_of_war).clone());
        info!(
            "Saved FogOfWarMaps with {} civilization maps",
            fog_of_war.maps.len()
        );
    } else {
        warn!("FogOfWarMaps resource not found during save!");
    }

    // Copy entities with their components
    for (
        entity,
        position,
        civilization,
        city,
        capital,
        military_unit,
        terrain,
        civ_id,
        player_controlled,
        unit_selected,
        player_movement_order,
    ) in query.iter()
    {
        let mut entity_commands = save_world.spawn((*position,));

        if let Some(civilization) = civilization {
            entity_commands.insert(civilization.clone());
        }

        if let Some(city) = city {
            entity_commands.insert(city.clone());
        }

        if let Some(capital) = capital {
            entity_commands.insert(capital.clone());
        }

        if let Some(military_unit) = military_unit {
            entity_commands.insert(military_unit.clone());
        }

        if let Some(terrain) = terrain {
            entity_commands.insert(terrain.clone());
        }

        if let Some(civ_id) = civ_id {
            entity_commands.insert(*civ_id);
        }

        if let Some(player_controlled) = player_controlled {
            entity_commands.insert(player_controlled.clone());
        }

        if let Some(unit_selected) = unit_selected {
            entity_commands.insert(unit_selected.clone());
        }

        if let Some(player_movement_order) = player_movement_order {
            entity_commands.insert(player_movement_order.clone());
        }
    }

    // Create the dynamic scene
    let scene = DynamicScene::from_world(&save_world);

    // Serialize the scene
    let type_registry = type_registry.read();
    let serialized_scene = scene.serialize(&type_registry).unwrap();

    info!("Serialized save game ({} bytes)", serialized_scene.len());

    // Save to file asynchronously
    let save_path = save_state.save_path.clone();
    let save_directory = get_save_directory(&save_state);
    IoTaskPool::get()
        .spawn(async move {
            let path = format!("{}/saves/{}", save_directory, save_path);
            if let Some(parent) = Path::new(&path).parent() {
                std::fs::create_dir_all(parent).expect("Failed to create save directory");
            }

            File::create(&path)
                .and_then(|mut file| file.write_all(serialized_scene.as_bytes()))
                .expect("Failed to write save file");

            info!("Game saved to: {}", path);
        })
        .detach();
}

/// Handle load game requests
fn handle_load_requests(
    mut commands: Commands,
    mut save_state: ResMut<SaveLoadState>,
    asset_server: Res<AssetServer>,
    // Query for any game entity - units, cities, civilizations, etc.
    game_entities: Query<
        Entity,
        Or<(
            With<Position>,
            With<Civilization>,
            With<City>,
            With<Capital>,
            With<MilitaryUnit>,
        )>,
    >,
    // Query for entities with sprite references to despawn their sprites too
    sprite_ref_entities: Query<&SpriteEntityReference>,
    cameras: Query<Entity, With<Camera>>,
) {
    if !save_state.load_requested {
        return;
    }

    save_state.load_requested = false;

    // Mark that we need to restore player control after this load
    save_state.needs_player_restore = true;
    // Mark that fog of war needs to be refreshed after load
    save_state.fog_of_war_needs_refresh = true;

    info!("Starting game load - clearing all existing game state");

    // First, despawn all referenced sprite entities
    let mut sprite_entities_to_despawn: std::collections::HashSet<Entity> =
        std::collections::HashSet::new();
    for sprite_ref in sprite_ref_entities.iter() {
        sprite_entities_to_despawn.insert(sprite_ref.sprite_entity);
    }

    info!(
        "Despawning {} referenced sprite entities before load",
        sprite_entities_to_despawn.len()
    );
    for entity in sprite_entities_to_despawn.iter() {
        commands.entity(*entity).despawn();
    }

    // Collect all game entities, excluding cameras
    let camera_entities: std::collections::HashSet<Entity> = cameras.iter().collect();
    let entities_to_despawn: Vec<Entity> = game_entities
        .iter()
        .filter(|e| !camera_entities.contains(e))
        .collect();

    info!(
        "Despawning {} game entities before load",
        entities_to_despawn.len()
    );
    for entity in entities_to_despawn {
        commands.entity(entity).despawn();
    }

    // Note: Resources will be overwritten by the loaded scene
    // The DynamicScene will insert the saved resources (WorldMap, FogOfWarMaps, CurrentTurn, etc.)
    // This ensures a clean slate matching the save file

    // Try to find the save file in the configured directory
    let primary_save_dir = get_save_directory(&save_state);
    if let Some(file_path) = find_save_file_path(&save_state.load_path, &primary_save_dir) {
        // Copy save file to assets directory for asset server to load
        let temp_path = format!("dominion_earth/assets/saves/{}", save_state.load_path);
        if let Some(parent) = Path::new(&temp_path).parent() {
            std::fs::create_dir_all(parent).ok();
        }

        if std::fs::copy(&file_path, &temp_path).is_ok() {
            let load_path = format!("saves/{}", save_state.load_path);
            commands.spawn(DynamicSceneRoot(asset_server.load(load_path)));
            info!(
                "Loading game from: {} (copied to assets for loading)",
                file_path
            );
        } else {
            error!("Failed to copy save file for loading: {}", file_path);
        }
    } else {
        error!("Failed to find save file: {}", save_state.load_path);
    }
}

/// Track loaded scenes and log when they're ready
fn track_loaded_scenes(
    mut commands: Commands,
    query: Query<(Entity, &DynamicSceneRoot), Without<LoadLoggedMarker>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, scene_root) in query.iter() {
        if let Some(load_state) = asset_server.get_load_state(&scene_root.0) {
            match load_state {
                bevy::asset::LoadState::Loaded => {
                    info!("Save game loaded successfully");
                    // Mark this entity so we don't log again, but keep the DynamicSceneRoot
                    // alive so the scene entities can be instantiated
                    commands.entity(entity).insert(LoadLoggedMarker);
                }
                bevy::asset::LoadState::Failed(err) => {
                    error!("Failed to load save game: {}", err);
                    // Mark as logged to prevent repeated error messages
                    commands.entity(entity).insert(LoadLoggedMarker);
                }
                _ => {}
            }
        }
    }
}

/// Trigger a save operation
pub fn save_game(save_state: &mut ResMut<SaveLoadState>, save_name: &str) {
    save_state.save_requested = true;
    save_state.save_path = format!("{}.scn.ron", save_name);
    info!("Save requested: {}", save_name);
}

/// Trigger a load operation
pub fn load_game(save_state: &mut ResMut<SaveLoadState>, save_name: &str) {
    save_state.load_requested = true;
    save_state.load_path = format!("{}.scn.ron", save_name);
    info!("Load requested: {}", save_name);
}

/// Restore player control for old saves that don't have PlayerControlled components
/// This system runs after loading and ensures at least one civilization is player-controlled
/// Only runs when triggered by a load request
fn restore_player_control_after_load(
    mut commands: Commands,
    mut save_state: ResMut<SaveLoadState>,
    civilizations_query: Query<(Entity, &Civilization), Without<PlayerControlled>>,
    player_civilizations_query: Query<&Civilization, With<PlayerControlled>>,
    cities_query: Query<(Entity, &City), Without<PlayerControlled>>,
    units_query: Query<(Entity, &MilitaryUnit), Without<PlayerControlled>>,
) {
    // Only run when we need to restore and have no player civilizations yet
    if !save_state.needs_player_restore
        || !player_civilizations_query.is_empty()
        || civilizations_query.is_empty()
    {
        return;
    }

    // Find the first civilization (by CivId(0) if possible, otherwise the first one)
    let mut civilizations: Vec<_> = civilizations_query.iter().collect();
    civilizations.sort_by_key(|(_, civ)| civ.id.0);

    if let Some((first_civ_entity, first_civ)) = civilizations.first() {
        // Mark the first civilization as player-controlled
        commands.entity(*first_civ_entity).insert(PlayerControlled);

        let player_civ_id = first_civ.id;
        info!(
            "Restored player control to civilization: {} (CivId: {:?})",
            first_civ.name, player_civ_id
        );

        // Mark all cities belonging to this civilization as player-controlled
        for (city_entity, city) in cities_query.iter() {
            if city.owner == player_civ_id {
                commands.entity(city_entity).insert(PlayerControlled);
            }
        }

        // Mark all units belonging to this civilization as player-controlled
        for (unit_entity, unit) in units_query.iter() {
            if unit.owner == player_civ_id {
                commands.entity(unit_entity).insert(PlayerControlled);
            }
        }

        // Mark that we've completed the restoration
        save_state.needs_player_restore = false;
        // Mark that fog of war needs to be refreshed
        save_state.fog_of_war_needs_refresh = true;
        info!("Player control restoration complete");
    }
}

/// Force a refresh of fog of war tile visuals after loading
/// This ensures tiles properly reflect the loaded fog of war state
fn refresh_fog_of_war_after_load(
    mut save_state: ResMut<SaveLoadState>,
    fog_of_war: Option<Res<FogOfWarMaps>>,
    player_query: Query<&Civilization, With<PlayerControlled>>,
    mut tile_query: Query<&mut bevy_ecs_tilemap::tiles::TileColor>,
) {
    use bevy_ecs_tilemap::tiles::TileColor;

    if !save_state.fog_of_war_needs_refresh {
        return;
    }

    // Only proceed if we have the necessary resources
    let Some(fog_of_war) = fog_of_war else {
        return;
    };

    let Ok(player_civ) = player_query.single() else {
        return;
    };

    let Some(visibility_map) = fog_of_war.get(player_civ.id) else {
        return;
    };

    // Force all tiles to update their color based on current visibility
    let mut updated_count = 0;
    for mut tile_color in tile_query.iter_mut() {
        // Set to a different color first to force detection of change
        tile_color.0 = Color::WHITE;
        updated_count += 1;
    }

    if updated_count > 0 {
        info!("Forced refresh of {} tile colors after load", updated_count);
        save_state.fog_of_war_needs_refresh = false;
    }
}

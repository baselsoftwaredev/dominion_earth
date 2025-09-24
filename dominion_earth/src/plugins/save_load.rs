use bevy::prelude::*;
use bevy::scene::{DynamicScene, DynamicSceneRoot};
use bevy::tasks::IoTaskPool;
use core_sim::components::rendering::SpriteEntityReference;
use core_sim::components::turn_phases::TurnPhase;
use core_sim::resources::{ActiveCivTurn, CurrentTurn, GameConfig, MapTile, Resource, WorldMap};
use core_sim::{
    Building, BuildingType, City, CivId, CivPersonality, CivStats, Civilization, Direction,
    Economy, Military, MilitaryUnit, Position, Technologies, TerrainType, TradeRoute, UnitType,
};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Plugin for handling game state saving and loading
pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveLoadState>()
            .add_systems(Startup, setup_save_load_registry)
            .add_systems(
                Update,
                (
                    handle_save_requests,
                    handle_load_requests,
                    track_loaded_scenes,
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
    query: Query<(
        Entity,
        &Position,
        Option<&Civilization>,
        Option<&City>,
        Option<&MilitaryUnit>,
        Option<&TerrainType>,
        Option<&CivId>,
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
        save_world.insert_resource(current_turn.clone());
    }
    if let Some(active_civ_turn) = active_civ_turn {
        save_world.insert_resource(active_civ_turn.clone());
    }
    if let Some(turn_phase) = turn_phase {
        save_world.insert_resource(turn_phase.clone());
    }
    if let Some(game_config) = game_config {
        save_world.insert_resource(game_config.clone());
    }
    if let Some(world_map) = world_map {
        save_world.insert_resource(world_map.clone());
    }

    // Copy entities with their components
    for (entity, position, civilization, city, military_unit, terrain, civ_id) in query.iter() {
        let mut entity_commands = save_world.spawn((*position,));

        if let Some(civilization) = civilization {
            entity_commands.insert(civilization.clone());
        }

        if let Some(city) = city {
            entity_commands.insert(city.clone());
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
    }

    // Create the dynamic scene
    let scene = DynamicScene::from_world(&save_world);

    // Serialize the scene
    let type_registry = type_registry.read();
    let serialized_scene = scene.serialize(&type_registry).unwrap();

    info!("Serialized save game ({} bytes)", serialized_scene.len());

    // Save to file asynchronously
    let save_path = save_state.save_path.clone();
    IoTaskPool::get()
        .spawn(async move {
            let path = format!("assets/saves/{}", save_path);
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
    query: Query<Entity, (With<Position>, Without<Camera>)>,
) {
    if !save_state.load_requested {
        return;
    }

    save_state.load_requested = false;

    // Clear existing game entities (but not cameras or UI)
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Load the scene
    let load_path = format!("saves/{}", save_state.load_path);
    commands.spawn(DynamicSceneRoot(asset_server.load(load_path)));

    info!("Loading game from: assets/saves/{}", save_state.load_path);
}

/// Track loaded scenes and log when they're ready
fn track_loaded_scenes(query: Query<&DynamicSceneRoot>, asset_server: Res<AssetServer>) {
    for scene_root in query.iter() {
        if let Some(load_state) = asset_server.get_load_state(&scene_root.0) {
            match load_state {
                bevy::asset::LoadState::Loaded => {
                    info!("Save game loaded successfully");
                }
                bevy::asset::LoadState::Failed(err) => {
                    error!("Failed to load save game: {}", err);
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

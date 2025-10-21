use bevy::prelude::*;
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions};
use bevy_save::prelude::*;
use core_sim::components::rendering::SpriteEntityReference;
use core_sim::components::turn_phases::TurnPhase;
use core_sim::resources::{ActiveCivTurn, CurrentTurn, GameConfig, MapTile, Resource, WorldMap};
use core_sim::{
    Building, BuildingType, Capital, CapitalAge, City, CivId, CivPersonality, CivStats,
    Civilization, Direction, Economy, FogOfWarMaps, Military, MilitaryUnit, PlayerControlled,
    PlayerMovementOrder, Position, ProvidesVision, Technologies, TerrainType, TradeRoute,
    UnitSelected, UnitType, VisibilityMap, VisibilityState,
};

pub struct DominionEarthPipeline {
    save_name: String,
}

impl DominionEarthPipeline {
    pub fn new(save_name: String) -> Self {
        Self { save_name }
    }
}

impl Pipeline for DominionEarthPipeline {
    type Backend = DefaultBackend;
    type Format = DefaultDebugFormat;
    type Key<'a> = String;

    fn key(&self) -> Self::Key<'_> {
        format!("dominion_earth/{}", self.save_name)
    }

    fn capture(&self, builder: BuilderRef) -> Snapshot {
        builder
            .extract_entities_matching(|e| {
                e.contains::<Position>()
                    || e.contains::<Civilization>()
                    || e.contains::<City>()
                    || e.contains::<MilitaryUnit>()
                    || e.contains::<TerrainType>()
            })
            .deny::<SpriteEntityReference>()
            .extract_resource::<WorldMap>()
            .extract_resource::<CurrentTurn>()
            .extract_resource::<ActiveCivTurn>()
            .extract_resource::<TurnPhase>()
            .extract_resource::<GameConfig>()
            .extract_resource::<FogOfWarMaps>()
            .build()
    }

    fn apply(&self, world: &mut World, snapshot: &Snapshot) -> Result<(), bevy_save::Error> {
        snapshot
            .applier(world)
            .despawn::<Or<(
                With<Position>,
                With<Civilization>,
                With<City>,
                With<MilitaryUnit>,
                With<TerrainType>,
            )>>()
            .apply()?;

        info!("Game state loaded successfully from: {}", self.save_name);
        Ok(())
    }
}

pub struct SaveLoadPlugin;

impl Default for SaveLoadPlugin {
    fn default() -> Self {
        Self
    }
}

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SavePlugins)
            .register_type::<Position>()
            .register_type::<Direction>()
            .register_type::<CivId>()
            .register_type::<Civilization>()
            .register_type::<CivPersonality>()
            .register_type::<Technologies>()
            .register_type::<Economy>()
            .register_type::<Military>()
            .register_type::<TradeRoute>()
            .register_type::<MilitaryUnit>()
            .register_type::<UnitType>()
            .register_type::<TerrainType>()
            .register_type::<City>()
            .register_type::<Capital>()
            .register_type::<CapitalAge>()
            .register_type::<Building>()
            .register_type::<BuildingType>()
            .register_type::<CivStats>()
            .register_type::<PlayerControlled>()
            .register_type::<UnitSelected>()
            .register_type::<PlayerMovementOrder>()
            .register_type::<ProvidesVision>()
            .register_type::<WorldMap>()
            .register_type::<CurrentTurn>()
            .register_type::<ActiveCivTurn>()
            .register_type::<TurnPhase>()
            .register_type::<GameConfig>()
            .register_type::<Resource>()
            .register_type::<MapTile>()
            .register_type::<FogOfWarMaps>()
            .register_type::<VisibilityMap>()
            .register_type::<VisibilityState>()
            .insert_resource(SaveLoadState::default())
            .add_systems(
                Update,
                (
                    handle_save_requests,
                    handle_load_requests,
                    restore_player_control_after_load,
                    refresh_fog_of_war_after_load,
                    respawn_ui_after_load,
                ),
            );

        info!("SaveLoadPlugin initialized with bevy_save");
    }
}

#[derive(Resource, Default)]
pub struct SaveLoadState {
    pub save_requested: Option<String>,
    pub load_requested: Option<String>,
    pub needs_player_restore: bool,
    pub fog_of_war_needs_refresh: bool,
    pub ui_needs_respawn: bool,
}

fn handle_save_requests(world: &mut World) {
    let save_name = {
        let mut save_state = world.resource_mut::<SaveLoadState>();
        save_state.save_requested.take()
    };

    if let Some(save_name) = save_name {
        info!("Saving game: {}", save_name);
        let pipeline = DominionEarthPipeline::new(save_name.clone());

        match world.save(&pipeline) {
            Ok(_) => info!("Game saved successfully: {}", save_name),
            Err(e) => error!("Failed to save game: {:?}", e),
        }
    }
}

fn handle_load_requests(world: &mut World) {
    let load_name = {
        let mut save_state = world.resource_mut::<SaveLoadState>();
        save_state.load_requested.take()
    };

    if let Some(load_name) = load_name {
        info!("Loading game: {}", load_name);

        despawn_referenced_sprites(world);
        despawn_ui_panels(world);

        {
            let mut save_state = world.resource_mut::<SaveLoadState>();
            save_state.needs_player_restore = true;
            save_state.fog_of_war_needs_refresh = true;
            save_state.ui_needs_respawn = true;
        }

        let pipeline = DominionEarthPipeline::new(load_name.clone());

        match world.load(&pipeline) {
            Ok(_) => info!("Game loaded successfully: {}", load_name),
            Err(e) => error!("Failed to load game: {:?}", e),
        }
    }
}

fn despawn_referenced_sprites(world: &mut World) {
    let mut sprite_entities_to_despawn = Vec::new();

    let mut query = world.query::<&SpriteEntityReference>();
    for sprite_ref in query.iter(world) {
        sprite_entities_to_despawn.push(sprite_ref.sprite_entity);
    }

    let despawn_count = sprite_entities_to_despawn.len();
    for sprite_entity in sprite_entities_to_despawn {
        if let Ok(entity_mut) = world.get_entity_mut(sprite_entity) {
            entity_mut.despawn();
        }
    }

    if despawn_count > 0 {
        info!("Despawned {} sprite entities before loading", despawn_count);
    }
}

fn despawn_ui_panels(world: &mut World) {
    use bevy_hui::prelude::HtmlNode;

    let mut ui_entities_to_despawn = Vec::new();

    let mut query = world.query_filtered::<Entity, With<HtmlNode>>();
    for entity in query.iter(world) {
        ui_entities_to_despawn.push(entity);
    }

    let despawn_count = ui_entities_to_despawn.len();
    for entity in ui_entities_to_despawn {
        if let Ok(entity_mut) = world.get_entity_mut(entity) {
            entity_mut.despawn();
        }
    }

    if despawn_count > 0 {
        info!(
            "Despawned {} UI panel entities before loading",
            despawn_count
        );
    }
}

pub fn save_game(save_state: &mut ResMut<SaveLoadState>, save_name: &str) {
    save_state.save_requested = Some(save_name.to_string());
    info!("Save requested: {}", save_name);
}

pub fn load_game(save_state: &mut ResMut<SaveLoadState>, save_name: &str) {
    save_state.load_requested = Some(save_name.to_string());
    info!("Load requested: {}", save_name);
}

fn restore_player_control_after_load(
    mut commands: Commands,
    mut save_state: ResMut<SaveLoadState>,
    civilizations_query: Query<(Entity, &Civilization), Without<PlayerControlled>>,
    player_civilizations_query: Query<&Civilization, With<PlayerControlled>>,
    cities_query: Query<(Entity, &City), Without<PlayerControlled>>,
    units_query: Query<(Entity, &MilitaryUnit), Without<PlayerControlled>>,
) {
    if !save_state.needs_player_restore
        || !player_civilizations_query.is_empty()
        || civilizations_query.is_empty()
    {
        return;
    }

    let mut civilizations: Vec<_> = civilizations_query.iter().collect();
    civilizations.sort_by_key(|(_, civ)| civ.id.0);

    if let Some((first_civ_entity, first_civ)) = civilizations.first() {
        commands.entity(*first_civ_entity).insert(PlayerControlled);

        let player_civ_id = first_civ.id;
        info!(
            "Restored player control to civilization: {} (CivId: {:?})",
            first_civ.name, player_civ_id
        );

        for (city_entity, city) in cities_query.iter() {
            if city.owner == player_civ_id {
                commands.entity(city_entity).insert(PlayerControlled);
            }
        }

        for (unit_entity, unit) in units_query.iter() {
            if unit.owner == player_civ_id {
                commands.entity(unit_entity).insert(PlayerControlled);
            }
        }

        save_state.needs_player_restore = false;
        save_state.fog_of_war_needs_refresh = true;
        info!("Player control restoration complete");
    }
}

fn refresh_fog_of_war_after_load(
    mut save_state: ResMut<SaveLoadState>,
    mut fog_of_war: Option<ResMut<FogOfWarMaps>>,
    world_map: Option<Res<WorldMap>>,
    player_query: Query<&Civilization, With<PlayerControlled>>,
    civilizations: Query<&Civilization>,
    units: Query<(&Position, &CivId, &core_sim::ProvidesVision), With<MilitaryUnit>>,
    cities: Query<(&Position, &CivId, &core_sim::ProvidesVision), With<City>>,
) {
    if !save_state.fog_of_war_needs_refresh {
        return;
    }

    let Some(mut fog_of_war) = fog_of_war else {
        return;
    };

    let Some(world_map) = world_map else {
        return;
    };

    let Ok(player_civ) = player_query.single() else {
        return;
    };

    info!(
        "Refreshing fog of war after load for civilization: {}",
        player_civ.name
    );

    for civ in civilizations.iter() {
        if fog_of_war.get(civ.id).is_none() {
            fog_of_war.init_for_civ(civ.id, world_map.width, world_map.height);
            info!("Reinitialized fog of war map for civ: {}", civ.name);
        }
    }

    for civ in civilizations.iter() {
        if let Some(vis_map) = fog_of_war.get_mut(civ.id) {
            vis_map.reset_visibility();

            for (pos, civ_id, provides_vision) in units.iter() {
                if *civ_id == civ.id {
                    vis_map.mark_visible(*pos, provides_vision.range);
                }
            }

            for (pos, civ_id, provides_vision) in cities.iter() {
                if *civ_id == civ.id {
                    vis_map.mark_visible(*pos, provides_vision.range);
                }
            }
        }
    }

    info!("Fog of war refresh complete after load");
    save_state.fog_of_war_needs_refresh = false;
}

fn respawn_ui_after_load(
    mut commands: Commands,
    mut save_state: ResMut<SaveLoadState>,
    asset_server: Res<AssetServer>,
    mut html_components: HtmlComponents,
    mut html_functions: HtmlFunctions,
) {
    if !save_state.ui_needs_respawn {
        return;
    }

    info!("Respawning UI panels after load");

    // Call the setup_main_ui function to recreate all UI panels
    crate::ui::bevy_hui::setup_main_ui(commands, asset_server, html_components, html_functions);

    save_state.ui_needs_respawn = false;
    info!("UI respawn complete after load");
}

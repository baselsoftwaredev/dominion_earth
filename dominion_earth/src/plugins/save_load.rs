use bevy::audio::{GlobalVolume, Volume};
use bevy::prelude::*;
use core_sim::components::turn_phases::TurnPhase;
use core_sim::resources::{ActiveCivTurn, CurrentTurn, GameConfig, MapTile, Resource, WorldMap};
use core_sim::{
    Building, BuildingType, Capital, CapitalAge, City, CivId, CivPersonality, CivStats,
    Civilization, Direction, Economy, FogOfWarMaps, Military, MilitaryUnit, PlayerControlled,
    PlayerMovementOrder, Position, ProvidesVision, Technologies, TerrainType, TradeRoute, UnitType,
    VisibilityMap, VisibilityState,
};
use moonshine_save::prelude::*;

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct SavedMusicVolume {
    pub volume: f32,
}

impl Default for SavedMusicVolume {
    fn default() -> Self {
        Self {
            volume: crate::constants::audio::DEFAULT_MUSIC_VOLUME,
        }
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
        app.add_observer(save_on_default_event)
            .add_observer(load_on_default_event)
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
            .register_type::<SavedMusicVolume>()
            .insert_resource(SavedMusicVolume::default())
            .insert_resource(SaveLoadState::default())
            .add_systems(
                Update,
                (
                    handle_save_requests,
                    // Load sequence must run in strict order to prevent sprite recreation
                    (
                        handle_load_requests,
                        cleanup_world_before_load,
                        trigger_pending_load,
                    )
                        .chain(),
                    restore_player_control_after_load,
                    refresh_fog_of_war_after_load,
                    respawn_ui_after_load,
                    restore_music_volume_after_load,
                    clear_loading_flag,
                ),
            );

        info!("SaveLoadPlugin initialized with moonshine-save MVC architecture");
    }
}

#[derive(Resource, Default)]
pub struct SaveLoadState {
    pub save_requested: Option<String>,
    pub load_requested: Option<String>,
    pub needs_player_restore: bool,
    pub fog_of_war_needs_refresh: bool,
    pub ui_needs_respawn: bool,
    pub is_loading_from_save: bool,
    pub pending_load_name: Option<String>,
    pub frames_since_load_triggered: u32,
}

fn handle_save_requests(
    mut commands: Commands,
    mut save_state: ResMut<SaveLoadState>,
    global_volume: Res<GlobalVolume>,
    mut saved_volume: ResMut<SavedMusicVolume>,
) {
    if let Some(save_name) = save_state.save_requested.take() {
        info!("Saving game: {}", save_name);

        sync_current_volume_to_save(&global_volume, &mut saved_volume);

        let file_path = format!("saves/{}.ron", save_name);

        commands.trigger_save(
            SaveWorld::default_into_file(file_path)
                .include_resource::<WorldMap>()
                .include_resource::<CurrentTurn>()
                .include_resource::<ActiveCivTurn>()
                .include_resource::<TurnPhase>()
                .include_resource::<GameConfig>()
                .include_resource::<FogOfWarMaps>()
                .include_resource::<SavedMusicVolume>(),
        );

        info!("Game save triggered: {}", save_name);
    }
}

fn sync_current_volume_to_save(
    global_volume: &Res<GlobalVolume>,
    saved_volume: &mut ResMut<SavedMusicVolume>,
) {
    saved_volume.volume = global_volume.volume.to_linear();
    info!("Saved music volume: {}", saved_volume.volume);
}

/// Handle load requests and set the loading flag to prevent sprite systems from running
/// This must be public so rendering systems can order themselves after it
pub fn handle_load_requests(mut save_state: ResMut<SaveLoadState>) {
    if let Some(load_name) = save_state.load_requested.take() {
        info!("Load requested: {}", load_name);
        // Store the pending load - we'll clean up first, then trigger the load
        save_state.pending_load_name = Some(load_name);
        // Set loading flag IMMEDIATELY to prevent any sprite systems from running
        save_state.is_loading_from_save = true;
        info!("🚫 Loading flag set - sprite systems should be blocked");
    }
}

fn cleanup_world_before_load(
    mut commands: Commands,
    mut save_state: ResMut<SaveLoadState>,
    sprite_entities: Query<Entity, (With<Sprite>, Without<Position>)>,
    label_entities: Query<Entity, Or<(With<crate::ui::CapitalLabel>, With<crate::ui::UnitLabel>)>>,
    entities_with_sprite_refs: Query<(
        Entity,
        &core_sim::components::rendering::SpriteEntityReference,
    )>,
) {
    if save_state.pending_load_name.is_none() {
        return;
    }

    info!("🧹 Cleaning up visual entities before load");

    // Despawn sprite entities that are referenced by game entities
    // AND remove the SpriteEntityReference components
    let sprite_ref_count = entities_with_sprite_refs.iter().count();
    for (entity, sprite_ref) in entities_with_sprite_refs.iter() {
        // Despawn the actual sprite entity (silently to avoid errors if already despawned)
        commands.entity(sprite_ref.sprite_entity).despawn();
        // Remove the reference component from the game entity
        commands
            .entity(entity)
            .remove::<core_sim::components::rendering::SpriteEntityReference>();
    }
    if sprite_ref_count > 0 {
        info!(
            "🧹 Despawned {} referenced sprite entities and removed their reference components",
            sprite_ref_count
        );
    }

    // Despawn any other sprites that aren't referenced (orphaned sprites)
    let sprite_count = sprite_entities.iter().count();
    for sprite_entity in sprite_entities.iter() {
        commands.entity(sprite_entity).despawn();
    }
    if sprite_count > 0 {
        info!(
            "🧹 Despawned {} total sprite entities from query",
            sprite_count
        );
    }

    // Despawn all labels
    let label_count = label_entities.iter().count();
    for label_entity in label_entities.iter() {
        commands.entity(label_entity).despawn();
    }
    if label_count > 0 {
        info!("🧹 Despawned {} label entities", label_count);
    }
}

fn trigger_pending_load(mut commands: Commands, mut save_state: ResMut<SaveLoadState>) {
    if let Some(load_name) = save_state.pending_load_name.take() {
        info!("Triggering load for: {}", load_name);

        mark_all_post_load_restoration_flags(&mut save_state);

        let file_path = format!("saves/{}.ron", load_name);
        commands.trigger_load(LoadWorld::default_from_file(file_path));

        save_state.frames_since_load_triggered = 0;

        info!("Game load triggered: {}", load_name);
    }

    // Increment frame counter while loading
    if save_state.is_loading_from_save {
        save_state.frames_since_load_triggered += 1;
    }
}

fn mark_all_post_load_restoration_flags(save_state: &mut ResMut<SaveLoadState>) {
    save_state.needs_player_restore = true;
    save_state.fog_of_war_needs_refresh = true;
    save_state.ui_needs_respawn = true;
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

fn respawn_ui_after_load(mut commands: Commands, mut save_state: ResMut<SaveLoadState>) {
    if !save_state.ui_needs_respawn {
        return;
    }

    info!("Respawning UI panels after load");

    crate::ui::top_panel::spawn_top_panel(commands.reborrow());
    crate::ui::right_panel::spawn_right_panel(commands.reborrow());
    crate::ui::left_panel::spawn_left_panel(commands.reborrow());

    save_state.ui_needs_respawn = false;
    info!("UI respawn complete after load");
}

fn restore_music_volume_after_load(
    saved_volume: Res<SavedMusicVolume>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    if saved_volume.is_changed() && !saved_volume.is_added() {
        global_volume.volume = Volume::Linear(saved_volume.volume);
        info!("Restored music volume to: {}", saved_volume.volume);
    }
}

fn clear_loading_flag(mut save_state: ResMut<SaveLoadState>) {
    // Clear the loading flag after all restoration is complete AND a few frames have passed
    // This ensures sprite systems don't run during the load process
    const MIN_FRAMES_AFTER_LOAD: u32 = 3;

    if save_state.is_loading_from_save
        && !save_state.needs_player_restore
        && !save_state.fog_of_war_needs_refresh
        && !save_state.ui_needs_respawn
        && save_state.frames_since_load_triggered >= MIN_FRAMES_AFTER_LOAD
    {
        info!("All load restoration complete, clearing loading flag");
        save_state.is_loading_from_save = false;
        save_state.frames_since_load_triggered = 0;
    }
}

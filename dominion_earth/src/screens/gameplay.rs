use bevy::prelude::*;

use crate::{
    debug_utils::DebugLogging, entity_utils, menus::Menu, screens::Screen,
    theme::widget::ButtonAction,
};

pub fn plugin(app: &mut App) {
    // Force cleanup any menu UI when entering gameplay
    app.add_systems(OnEnter(Screen::Gameplay), cleanup_all_menu_entities);

    // Toggle pause menu on Escape
    app.add_systems(
        Update,
        open_pause_menu.run_if(
            in_state(Screen::Gameplay)
                .and(in_state(Menu::None))
                .and(input_just_pressed(KeyCode::Escape)),
        ),
    );

    app.add_systems(
        Update,
        close_menu.run_if(
            in_state(Screen::Gameplay)
                .and(not(in_state(Menu::None)))
                .and(input_just_pressed(KeyCode::Escape)),
        ),
    );

    app.add_systems(OnExit(Screen::Gameplay), close_menu);
    app.add_systems(
        OnExit(Screen::Gameplay),
        (despawn_all_game_entities, reset_all_game_resources).chain(),
    );
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn input_just_pressed(key: KeyCode) -> impl SystemCondition<()> {
    IntoSystem::into_system(move |input: Res<ButtonInput<KeyCode>>| input.just_pressed(key))
}

fn cleanup_all_menu_entities(
    mut commands: Commands,
    menu_query: Query<Entity, (With<Node>, With<GlobalZIndex>)>,
    z_index_query: Query<&GlobalZIndex>,
    children_query: Query<&Children>,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(debug_logging, "🧹 Cleaning up menu UI on entering Gameplay");
    let mut despawned = std::collections::HashSet::new();
    for entity in &menu_query {
        if let Ok(z_index) = z_index_query.get(entity) {
            if z_index.0 >= crate::theme::constants::z_index::MENU_OVERLAY_Z_INDEX {
                crate::debug_println!(
                    debug_logging,
                    "  Despawning menu entity with z-index {}",
                    z_index.0
                );
                entity_utils::recursively_despawn_entity_with_children(
                    &mut commands,
                    entity,
                    &children_query,
                    &mut despawned,
                );
            }
        }
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    game_entities: Query<
        (Entity, Option<&core_sim::SpriteEntityReference>),
        With<core_sim::Position>,
    >,
    sprite_entities: Query<Entity, (With<Sprite>, Without<core_sim::Position>)>,
    tile_entities: Query<Entity, With<core_sim::tile::tile_components::WorldTile>>,
    tilemap_entities: Query<Entity, With<bevy_ecs_tilemap::tiles::TileStorage>>,
    capital_label_entities: Query<Entity, With<crate::ui::CapitalLabel>>,
    unit_label_entities: Query<Entity, With<crate::ui::UnitLabel>>,
    tilemap_id: Option<ResMut<crate::rendering::common::TilemapIdResource>>,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(
        debug_logging,
        "🧹 Cleaning up game world - exiting Gameplay screen"
    );

    let mut referenced_sprites = std::collections::HashSet::new();
    for (_entity, sprite_ref) in &game_entities {
        if let Some(sprite_ref) = sprite_ref {
            referenced_sprites.insert(sprite_ref.sprite_entity);
        }
    }

    let mut sprite_count = 0;
    for sprite_entity in &sprite_entities {
        commands.entity(sprite_entity).despawn();
        sprite_count += 1;
    }
    crate::debug_println!(
        debug_logging,
        "  Despawned {} sprite entities",
        sprite_count
    );

    let tile_count = tile_entities.iter().count();
    if tile_count > 0 {
        crate::debug_println!(debug_logging, "  Despawning {} tile entities", tile_count);
        for tile_entity in &tile_entities {
            commands.entity(tile_entity).despawn();
        }
    }

    let tilemap_count = tilemap_entities.iter().count();
    if tilemap_count > 0 {
        crate::debug_println!(
            debug_logging,
            "  Despawning {} tilemap entities",
            tilemap_count
        );
        for tilemap_entity in &tilemap_entities {
            commands.entity(tilemap_entity).despawn();
        }
    }

    let capital_label_count = capital_label_entities.iter().count();
    if capital_label_count > 0 {
        crate::debug_println!(
            debug_logging,
            "  Despawning {} capital label entities",
            capital_label_count
        );
        for label_entity in &capital_label_entities {
            commands.entity(label_entity).despawn();
        }
    }

    let unit_label_count = unit_label_entities.iter().count();
    if unit_label_count > 0 {
        crate::debug_println!(
            debug_logging,
            "  Despawning {} unit label entities",
            unit_label_count
        );
        for label_entity in &unit_label_entities {
            commands.entity(label_entity).despawn();
        }
    }

    if tilemap_id.is_some() {
        commands.remove_resource::<crate::rendering::common::TilemapIdResource>();
        crate::debug_println!(debug_logging, "  Removed TilemapIdResource");
    }

    let entity_count = game_entities.iter().count();
    crate::debug_println!(debug_logging, "  Despawning {} game entities", entity_count);
    for (entity, _) in &game_entities {
        commands.entity(entity).despawn();
    }
}

fn reset_all_game_resources(
    mut world_map: ResMut<core_sim::WorldMap>,
    mut current_turn: ResMut<core_sim::resources::CurrentTurn>,
    mut active_civ_turn: ResMut<core_sim::resources::ActiveCivTurn>,
    mut turn_phase: ResMut<core_sim::TurnPhase>,
    mut fog_of_war: ResMut<core_sim::FogOfWarMaps>,
    mut turn_advance: ResMut<core_sim::resources::TurnAdvanceRequest>,
    mut game_state: ResMut<crate::game::GameState>,
    mut player_actions: ResMut<core_sim::PlayerActionsComplete>,
    mut selected_capital: ResMut<crate::production_input::SelectedCapital>,
    game_config: Res<core_sim::resources::GameConfig>,
    debug_logging: Res<DebugLogging>,
) {
    crate::debug_println!(debug_logging, "  Resetting game resources");
    *world_map = core_sim::WorldMap::default();
    *current_turn = core_sim::resources::CurrentTurn::default();
    *active_civ_turn = core_sim::resources::ActiveCivTurn::default();
    *turn_phase = core_sim::TurnPhase::default();
    *fog_of_war = core_sim::FogOfWarMaps::new();
    *turn_advance = core_sim::resources::TurnAdvanceRequest::default();
    *player_actions = core_sim::PlayerActionsComplete::default();
    *selected_capital = crate::production_input::SelectedCapital::default();

    *game_state = crate::game::GameState::new(
        game_state.auto_advance,
        game_config.ai_only,
        game_state.total_civilizations,
    );

    crate::debug_println!(debug_logging, "✅ Game world cleanup complete");
}

//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::widget::ButtonAction};

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

    app.add_systems(OnExit(Screen::Gameplay), (close_menu, cleanup_game_world));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn input_just_pressed(key: KeyCode) -> impl Condition<()> {
    IntoSystem::into_system(move |input: Res<ButtonInput<KeyCode>>| input.just_pressed(key))
}

/// Force cleanup any leftover menu UI entities when entering gameplay
fn cleanup_all_menu_entities(
    mut commands: Commands,
    menu_query: Query<Entity, (With<Node>, With<GlobalZIndex>)>,
    z_index_query: Query<&GlobalZIndex>,
) {
    println!("🧹 Cleaning up menu UI on entering Gameplay");
    // Despawn any UI with GlobalZIndex (menus use this, game UI doesn't)
    for entity in &menu_query {
        if let Ok(z_index) = z_index_query.get(entity) {
            if z_index.0 >= 100 {
                println!("  Despawning menu entity with z-index {}", z_index.0);
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Clean up the entire game world when exiting gameplay screen
fn cleanup_game_world(
    mut commands: Commands,
    game_entities: Query<
        (Entity, Option<&core_sim::SpriteEntityReference>),
        With<core_sim::Position>,
    >,
    sprite_entities: Query<Entity, (With<Sprite>, Without<core_sim::Position>)>,
    tile_entities: Query<Entity, With<core_sim::tile::tile_components::WorldTile>>,
    tilemap_entities: Query<Entity, With<bevy_ecs_tilemap::tiles::TileStorage>>,
    tilemap_id: Option<ResMut<crate::rendering::common::TilemapIdResource>>,
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
) {
    println!("🧹 Cleaning up game world - exiting Gameplay screen");

    // First, despawn all sprite entities referenced by game entities
    let mut sprite_count = 0;
    for (_entity, sprite_ref) in &game_entities {
        if let Some(sprite_ref) = sprite_ref {
            commands.entity(sprite_ref.sprite_entity).despawn();
            sprite_count += 1;
        }
    }
    println!("  Despawned {} referenced sprite entities", sprite_count);

    // Despawn all remaining sprite entities (catch any orphaned sprites)
    let remaining_sprite_count = sprite_entities.iter().count();
    if remaining_sprite_count > 0 {
        println!(
            "  Despawning {} remaining sprite entities",
            remaining_sprite_count
        );
        for sprite_entity in &sprite_entities {
            commands.entity(sprite_entity).despawn();
        }
    }

    // Despawn all tile entities (the terrain tiles)
    let tile_count = tile_entities.iter().count();
    if tile_count > 0 {
        println!("  Despawning {} tile entities", tile_count);
        for tile_entity in &tile_entities {
            commands.entity(tile_entity).despawn();
        }
    }

    // Despawn all tilemap entities (the tilemap container)
    let tilemap_count = tilemap_entities.iter().count();
    if tilemap_count > 0 {
        println!("  Despawning {} tilemap entities", tilemap_count);
        for tilemap_entity in &tilemap_entities {
            commands.entity(tilemap_entity).despawn();
        }
    }

    // Remove the TilemapIdResource so a new tilemap can be created
    if tilemap_id.is_some() {
        commands.remove_resource::<crate::rendering::common::TilemapIdResource>();
        println!("  Removed TilemapIdResource");
    }

    // Despawn all game entities (anything with a Position component)
    let entity_count = game_entities.iter().count();
    println!("  Despawning {} game entities", entity_count);
    for (entity, _) in &game_entities {
        commands.entity(entity).despawn();
    }

    // Reset game resources to default state
    println!("  Resetting game resources");
    *world_map = core_sim::WorldMap::default();
    *current_turn = core_sim::resources::CurrentTurn::default();
    *active_civ_turn = core_sim::resources::ActiveCivTurn::default();
    *turn_phase = core_sim::TurnPhase::default();
    *fog_of_war = core_sim::FogOfWarMaps::new();
    *turn_advance = core_sim::resources::TurnAdvanceRequest::default();
    *player_actions = core_sim::PlayerActionsComplete::default();
    *selected_capital = crate::production_input::SelectedCapital::default();

    // Reset GameState but preserve configuration settings
    *game_state = crate::game::GameState::new(
        game_state.auto_advance,
        game_config.ai_only,
        game_state.total_civilizations,
    );

    println!("✅ Game world cleanup complete");
}

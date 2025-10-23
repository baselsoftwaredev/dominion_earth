use crate::ui::resources::*;
use bevy::prelude::*;
use bevy_hui::prelude::*;
use core_sim::{resources::CurrentTurn, Civilization};

use super::constants;
use super::production_orders::register_production_order_functions;

/// Setup main UI with proper component registration
pub fn setup_main_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut html_components: HtmlComponents,
    mut html_functions: HtmlFunctions,
) {
    register_ui_update_functions(&mut html_functions);
    register_production_order_functions(&mut html_functions);
    spawn_main_ui_layout_panels(&mut commands, &asset_server);
    register_html_component_templates(&mut html_components, &asset_server);
}

/// Register UI update function handlers for dynamic property updates
fn register_ui_update_functions(html_functions: &mut HtmlFunctions) {
    register_player_gold_update_function(html_functions);
    register_turn_update_function(html_functions);
    register_turn_advancement_function(html_functions);
}

/// Register player gold update function for UI property binding
fn register_player_gold_update_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "update_player_gold",
        |In(entity): In<Entity>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         template_scopes: Query<&TemplateScope>,
         player_civilizations: Query<&Civilization, With<core_sim::PlayerControlled>>| {
            let scope_entity = match template_scopes.get(entity) {
                Ok(scope) => **scope,
                Err(_) => return,
            };

            let mut properties = match template_properties.get_mut(scope_entity) {
                Ok(props) => props,
                Err(_) => return,
            };

            let player_gold_amount = extract_player_gold_amount(&player_civilizations);
            update_player_gold_property(&mut properties, player_gold_amount);
            commands.trigger_targets(CompileContextEvent, scope_entity);
        },
    );
}

/// Register turn update function for UI property binding
fn register_turn_update_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "update_turn",
        |In(entity): In<Entity>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         template_scopes: Query<&TemplateScope>,
         current_turn: Res<CurrentTurn>| {
            let scope_entity = match template_scopes.get(entity) {
                Ok(scope) => **scope,
                Err(_) => return,
            };

            let mut properties = match template_properties.get_mut(scope_entity) {
                Ok(props) => props,
                Err(_) => return,
            };

            update_current_turn_property(&mut properties, current_turn.0);
            commands.trigger_targets(CompileContextEvent, scope_entity);
        },
    );
}

/// Register turn advancement function for next turn button
fn register_turn_advancement_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "advance_turn",
        |In(_entity): In<Entity>,
         mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut turn_advance_events: MessageWriter<core_sim::RequestTurnAdvance>| {
            turn_advance_events.write(core_sim::RequestTurnAdvance);
            info!("Player requested turn advancement");

            // Play click sound using audio helper
            crate::audio::play_sound_effect(&mut commands, &asset_server, "sounds/click.ogg");
        },
    );
}

/// Spawn main UI layout panels with initial properties
fn spawn_main_ui_layout_panels(commands: &mut Commands, asset_server: &AssetServer) {
    spawn_top_panel(commands, asset_server);
    spawn_right_side_panel(commands, asset_server);
    spawn_left_side_panel(commands, asset_server);
}

/// Spawn top panel with game information display
fn spawn_top_panel(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        HtmlNode(asset_server.load(constants::ui_templates::TOP_PANEL_PATH)),
        Name::new(constants::ui_component_names::TOP_PANEL_NAME),
        TemplateProperties::default()
            .with(
                constants::ui_properties::GAME_TITLE_PROPERTY,
                "Dominion Earth",
            )
            .with(
                constants::ui_properties::CURRENT_TURN_PROPERTY,
                constants::ui_initial_values::INITIAL_TURN_VALUE,
            )
            .with(
                constants::ui_properties::PLAYER_GOLD_PROPERTY,
                constants::ui_initial_values::INITIAL_GOLD_VALUE,
            )
            .with(
                constants::ui_properties::PLAYER_PRODUCTION_PROPERTY,
                constants::ui_initial_values::INITIAL_PRODUCTION_VALUE,
            ),
    ));
}

/// Spawn right side panel with terrain and civilization information
fn spawn_right_side_panel(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        HtmlNode(asset_server.load(constants::ui_templates::RIGHT_SIDE_PANEL_PATH)),
        Name::new(constants::ui_component_names::RIGHT_SIDE_PANEL_NAME),
        TemplateProperties::default()
            .with(
                constants::ui_properties::CURRENT_TURN_PROPERTY,
                constants::ui_initial_values::INITIAL_TURN_VALUE,
            )
            .with(
                "terrain_land_count",
                constants::ui_initial_values::INITIAL_TERRAIN_COUNT_VALUE,
            )
            .with(
                "terrain_water_count",
                constants::ui_initial_values::INITIAL_TERRAIN_COUNT_VALUE,
            )
            .with(
                "terrain_mountain_count",
                constants::ui_initial_values::INITIAL_TERRAIN_COUNT_VALUE,
            )
            .with(
                "selected_position",
                constants::ui_update::POSITION_NONE_TEXT,
            )
            .with("selected_terrain", constants::ui_update::TERRAIN_NONE_TEXT)
            .with(
                "civilizations_list",
                constants::ui_initial_values::LOADING_CIVILIZATIONS_MESSAGE,
            ),
    ));
}

/// Spawn left side panel with production and capital information
fn spawn_left_side_panel(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        HtmlNode(asset_server.load(constants::ui_templates::LEFT_SIDE_PANEL_PATH)),
        Name::new(constants::ui_component_names::LEFT_SIDE_PANEL_NAME),
        TemplateProperties::default()
            .with(
                constants::ui_properties::SHOW_PRODUCTION_MENU_PROPERTY,
                constants::ui_initial_values::MENU_DISPLAY_NONE_VALUE,
            )
            .with(
                constants::ui_properties::CAPITAL_NAME_PROPERTY,
                constants::ui_update::UNKNOWN_CAPITAL_NAME,
            )
            .with(
                constants::ui_properties::CIVILIZATION_NAME_PROPERTY,
                constants::ui_update::UNKNOWN_CIVILIZATION_NAME,
            )
            .with(
                constants::ui_properties::CIVILIZATION_GOLD_PROPERTY,
                constants::ui_initial_values::INITIAL_GOLD_VALUE,
            )
            .with(
                constants::ui_properties::CIVILIZATION_PRODUCTION_PROPERTY,
                constants::ui_initial_values::INITIAL_PRODUCTION_VALUE,
            )
            .with(
                constants::ui_properties::CURRENT_PRODUCTION_NAME_PROPERTY,
                constants::ui_update::NO_PRODUCTION_NAME,
            )
            .with(
                constants::ui_properties::CURRENT_PRODUCTION_PROGRESS_PROPERTY,
                constants::ui_initial_values::INITIAL_PRODUCTION_VALUE,
            )
            .with(
                constants::ui_properties::PRODUCTION_QUEUE_LENGTH_PROPERTY,
                constants::ui_initial_values::INITIAL_PRODUCTION_VALUE,
            ),
    ));
}

/// Register HTML component templates for dynamic UI elements
fn register_html_component_templates(
    html_components: &mut HtmlComponents,
    asset_server: &AssetServer,
) {
    register_panel_component_templates(html_components, asset_server);
    register_utility_component_templates(html_components, asset_server);
}

/// Register main panel component templates
fn register_panel_component_templates(
    html_components: &mut HtmlComponents,
    asset_server: &AssetServer,
) {
    html_components.register(
        constants::ui_component_names::LEFT_SIDE_TOP_NAME,
        asset_server.load(constants::ui_templates::LEFT_SIDE_TOP_PATH),
    );
    html_components.register(
        constants::ui_component_names::GAME_PANEL_NAME,
        asset_server.load(constants::ui_templates::GAME_PANEL_PATH),
    );
    html_components.register(
        constants::ui_component_names::PRODUCTION_MENU_NAME,
        asset_server.load(constants::ui_templates::PRODUCTION_MENU_PATH),
    );
    html_components.register(
        constants::ui_component_names::STATISTICS_PANEL_NAME,
        asset_server.load(constants::ui_templates::STATISTICS_PANEL_PATH),
    );
}

/// Register utility component templates for information display
fn register_utility_component_templates(
    html_components: &mut HtmlComponents,
    asset_server: &AssetServer,
) {
    html_components.register(
        constants::ui_component_names::HOVERED_TILE_INFO_NAME,
        asset_server.load(constants::ui_templates::HOVERED_TILE_INFO_PATH),
    );
    html_components.register(
        constants::ui_component_names::CIVILIZATIONS_LIST_NAME,
        asset_server.load(constants::ui_templates::CIVILIZATIONS_LIST_PATH),
    );
    html_components.register(
        constants::ui_component_names::MINIMAP_NAME,
        asset_server.load(constants::ui_templates::MINIMAP_PATH),
    );
}

/// Extract player gold amount from civilization query
fn extract_player_gold_amount(
    player_civilizations: &Query<&Civilization, With<core_sim::PlayerControlled>>,
) -> i32 {
    player_civilizations
        .iter()
        .next()
        .map(|civilization| civilization.economy.gold as i32)
        .unwrap_or(constants::ui_initial_values::HTML_FUNCTION_ENTITY_RAW_VALUE as i32)
}

/// Update player gold property in template properties
fn update_player_gold_property(template_properties: &mut TemplateProperties, gold_amount: i32) {
    template_properties.insert(
        constants::ui_properties::PLAYER_GOLD_PROPERTY.to_string(),
        gold_amount.to_string(),
    );
}

/// Update current turn property in template properties
fn update_current_turn_property(template_properties: &mut TemplateProperties, turn_number: u32) {
    template_properties.insert(
        constants::ui_properties::CURRENT_TURN_PROPERTY.to_string(),
        turn_number.to_string(),
    );
}

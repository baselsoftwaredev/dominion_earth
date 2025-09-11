use crate::debug_println;
use crate::debug_utils::DebugLogging;
use crate::game::GameState;
use crate::production_input::SelectedCapital;
use crate::ui::resources::*;
use crate::ui::traits::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_hui::prelude::*;
use core_sim::{
    components::{Capital, City, MilitaryUnit},
    resources::{CurrentTurn, WorldMap},
    Civilization, PlayerProductionOrder, Position, ProductionQueue,
};

/// Bevy HUI implementation of the UI system
pub struct BevyHuiSystem;

impl BevyHuiSystem {
    pub fn setup_plugins(app: &mut App) {
        app.add_plugins((HuiPlugin, HuiAutoLoadPlugin::new(&["ui"])))
            .add_systems(Startup, setup_main_ui)
            .add_systems(
                Update,
                (
                    update_ui_properties,
                    spawn_capital_labels,
                    update_capital_labels,
                ),
            );
    }
}

impl UiSystem for BevyHuiSystem {
    fn initialize(&self, app: &mut App) {
        Self::setup_plugins(app);
    }

    fn render_main_game_panel(&self, _data: &GamePanelData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_production_menu(&self, _data: &ProductionMenuData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_statistics_panel(&self, _data: &StatisticsPanelData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_tile_info(&self, _data: &TileInfoData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_minimap(&self, _data: &MinimapData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_resources(&self, _data: &ResourcesData) {
        // For bevy_hui, rendering is handled by the component system
    }
}

/// Component that holds a template handle for bevy_hui elements
#[derive(Component)]
pub struct HuiComponent {
    pub template: Handle<HtmlTemplate>,
}

impl UiComponent for HuiComponent {
    type Id = Entity;

    fn get_id(&self) -> Self::Id {
        Entity::from_raw(0) // This would be set when spawned
    }

    fn update_data(&mut self, _data: Box<dyn std::any::Any + Send + Sync>) {
        // Update template properties based on data
    }
}

/// Setup main UI with proper component registration
fn setup_main_ui(
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

/// Register production order function handlers for unit creation
fn register_production_order_functions(html_functions: &mut HtmlFunctions) {
    register_infantry_production_function(html_functions);
    register_archer_production_function(html_functions);
    register_cavalry_production_function(html_functions);
}

/// Register infantry production function for UI button binding
fn register_infantry_production_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "queue_infantry",
        |In(entity): In<Entity>,
         mut production_orders: EventWriter<PlayerProductionOrder>,
         selected_capital: Res<SelectedCapital>,
         civilizations_query: Query<&Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>| {
            process_unit_production_order(
                core_sim::UnitType::Infantry,
                &mut production_orders,
                &selected_capital,
                &civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
            );
        },
    );
}

/// Register archer production function for UI button binding
fn register_archer_production_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "queue_archer",
        |In(entity): In<Entity>,
         mut production_orders: EventWriter<PlayerProductionOrder>,
         selected_capital: Res<SelectedCapital>,
         civilizations_query: Query<&Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>| {
            process_unit_production_order(
                core_sim::UnitType::Archer,
                &mut production_orders,
                &selected_capital,
                &civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
            );
        },
    );
}

/// Register cavalry production function for UI button binding  
fn register_cavalry_production_function(html_functions: &mut HtmlFunctions) {
    html_functions.register(
        "queue_cavalry",
        |In(entity): In<Entity>,
         mut production_orders: EventWriter<PlayerProductionOrder>,
         selected_capital: Res<SelectedCapital>,
         civilizations_query: Query<&Civilization>,
         debug_logging: Res<DebugLogging>,
         mut commands: Commands,
         mut template_properties: Query<&mut TemplateProperties>,
         ui_entities: Query<Entity, (With<TemplateProperties>, With<Name>)>,
         entity_names: Query<&Name>| {
            process_unit_production_order(
                core_sim::UnitType::Cavalry,
                &mut production_orders,
                &selected_capital,
                &civilizations_query,
                &debug_logging,
                &mut commands,
                &mut template_properties,
                &ui_entities,
                &entity_names,
            );
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
        constants::ui_component_names::TILE_INFO_NAME,
        asset_server.load(constants::ui_templates::TILE_INFO_PATH),
    );
    html_components.register(
        constants::ui_component_names::CIVILIZATIONS_LIST_NAME,
        asset_server.load(constants::ui_templates::CIVILIZATIONS_LIST_PATH),
    );
    html_components.register(
        constants::ui_component_names::MINIMAP_NAME,
        asset_server.load(constants::ui_templates::MINIMAP_PATH),
    );
    html_components.register(
        constants::ui_component_names::CAPITAL_LABEL_NAME,
        asset_server.load(constants::ui_templates::CAPITAL_LABEL_PATH),
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

/// Process unit production order with unified logic for all unit types
fn process_unit_production_order(
    unit_type: core_sim::UnitType,
    production_orders: &mut EventWriter<PlayerProductionOrder>,
    selected_capital: &SelectedCapital,
    civilizations_query: &Query<&Civilization>,
    debug_logging: &DebugLogging,
    commands: &mut Commands,
    template_properties: &mut Query<&mut TemplateProperties>,
    ui_entities: &Query<Entity, (With<TemplateProperties>, With<Name>)>,
    entity_names: &Query<&Name>,
) {
    let unit_type_name = format!("{:?}", unit_type);
    debug_println!(debug_logging, "{} button clicked!", unit_type_name);

    let (capital_entity, civilization_entity) =
        match extract_selected_capital_entities(selected_capital, debug_logging, &unit_type_name) {
            Some(entities) => entities,
            None => return,
        };

    let civilization = match civilizations_query.get(civilization_entity) {
        Ok(civ) => civ,
        Err(_) => return,
    };

    let unit_production_item = core_sim::ProductionItem::Unit(unit_type);
    let unit_cost = unit_production_item.gold_cost();

    debug_println!(
        debug_logging,
        "{} cost: {}, Player gold: {}",
        unit_type_name,
        unit_cost,
        civilization.economy.gold
    );

    if civilization.economy.gold >= unit_cost {
        send_production_order(
            production_orders,
            capital_entity,
            civilization_entity,
            unit_production_item,
        );
        debug_println!(debug_logging, "{} production order sent!", unit_type_name);

        let updated_gold_amount = (civilization.economy.gold - unit_cost) as u32;
        update_ui_panels_with_new_gold(
            commands,
            template_properties,
            ui_entities,
            entity_names,
            updated_gold_amount,
        );
        debug_println!(
            debug_logging,
            "UI updated with new gold: {}",
            updated_gold_amount
        );
    } else {
        debug_println!(
            debug_logging,
            "Insufficient gold for {}!",
            unit_type_name.to_lowercase()
        );
    }
}

/// Extract selected capital entities with validation
fn extract_selected_capital_entities(
    selected_capital: &SelectedCapital,
    debug_logging: &DebugLogging,
    unit_type_name: &str,
) -> Option<(Entity, Entity)> {
    match (selected_capital.capital_entity, selected_capital.civ_entity) {
        (Some(capital_entity), Some(civ_entity)) => Some((capital_entity, civ_entity)),
        _ => {
            debug_println!(
                debug_logging,
                "No capital selected for {} production!",
                unit_type_name.to_lowercase()
            );
            None
        }
    }
}

/// Send production order to event writer
fn send_production_order(
    production_orders: &mut EventWriter<PlayerProductionOrder>,
    capital_entity: Entity,
    civilization_entity: Entity,
    production_item: core_sim::ProductionItem,
) {
    production_orders.write(PlayerProductionOrder {
        capital_entity,
        civ_entity: civilization_entity,
        item: production_item,
    });
}

/// Update UI panels with new gold amount after production order
fn update_ui_panels_with_new_gold(
    commands: &mut Commands,
    template_properties: &mut Query<&mut TemplateProperties>,
    ui_entities: &Query<Entity, (With<TemplateProperties>, With<Name>)>,
    entity_names: &Query<&Name>,
    new_gold_amount: u32,
) {
    for ui_entity in ui_entities.iter() {
        let (mut properties, entity_name) = match (
            template_properties.get_mut(ui_entity),
            entity_names.get(ui_entity),
        ) {
            (Ok(props), Ok(name)) => (props, name),
            _ => continue,
        };

        let entity_name_string = entity_name.as_str();
        update_panel_gold_property_by_name(&mut properties, entity_name_string, new_gold_amount);
        commands.trigger_targets(CompileContextEvent, ui_entity);
    }
}

/// Update gold property based on panel name
fn update_panel_gold_property_by_name(
    template_properties: &mut TemplateProperties,
    panel_name: &str,
    gold_amount: u32,
) {
    match panel_name {
        constants::ui_component_names::TOP_PANEL_NAME => {
            template_properties.insert(
                constants::ui_properties::PLAYER_GOLD_PROPERTY.to_string(),
                gold_amount.to_string(),
            );
        }
        constants::ui_component_names::LEFT_SIDE_PANEL_NAME => {
            template_properties.insert(
                constants::ui_properties::CIVILIZATION_GOLD_PROPERTY.to_string(),
                gold_amount.to_string(),
            );
        }
        _ => {}
    }
}

mod constants {
    pub mod ui_update {
        pub const UNKNOWN_CAPITAL_NAME: &str = "Unknown Capital";
        pub const UNKNOWN_CIVILIZATION_NAME: &str = "Unknown Civilization";
        pub const NO_PRODUCTION_NAME: &str = "None";
        pub const NO_CAPITALS_FOUNDED_MESSAGE: &str = "No capitals founded";
        pub const NO_CITIES_FOUNDED_MESSAGE: &str = "No cities founded";
        pub const NO_CIVILIZATIONS_MESSAGE: &str = "No civilizations";
        pub const UNKNOWN_TERRAIN_TYPE: &str = "Unknown";
        pub const POSITION_NONE_TEXT: &str = "None";
        pub const TERRAIN_NONE_TEXT: &str = "None";
        pub const PRODUCTION_MENU_DISPLAY_FLEX: &str = "flex";
        pub const PRODUCTION_MENU_DISPLAY_NONE: &str = "none";
        pub const PLAYER_CIVILIZATION_TYPE: &str = "Player";
        pub const AI_CIVILIZATION_TYPE: &str = "AI";
        pub const PERCENTAGE_MULTIPLIER: f32 = 100.0;
    }

    pub mod ui_templates {
        pub const TOP_PANEL_PATH: &str = "ui/top_panel.html";
        pub const RIGHT_SIDE_PANEL_PATH: &str = "ui/right_side_panel.html";
        pub const LEFT_SIDE_PANEL_PATH: &str = "ui/left_side_panel.html";
        pub const CAPITAL_LABEL_PATH: &str = "ui/components/capital_label.html";
        pub const LEFT_SIDE_TOP_PATH: &str = "ui/components/top_panel/left_side_top.html";
        pub const GAME_PANEL_PATH: &str = "ui/components/left_side_panel/game_panel.html";
        pub const PRODUCTION_MENU_PATH: &str =
            "ui/components/right_side_panel/production_menu.html";
        pub const STATISTICS_PANEL_PATH: &str =
            "ui/components/left_side_panel/statistics_panel.html";
        pub const TILE_INFO_PATH: &str = "ui/components/right_side_panel/tile_info.html";
        pub const CIVILIZATIONS_LIST_PATH: &str =
            "ui/components/right_side_panel/civilizations_list.html";
        pub const MINIMAP_PATH: &str = "ui/components/right_side_panel/minimap.html";
    }

    pub mod ui_component_names {
        pub const TOP_PANEL_NAME: &str = "top_panel";
        pub const RIGHT_SIDE_PANEL_NAME: &str = "right_side_panel";
        pub const LEFT_SIDE_PANEL_NAME: &str = "left_side_panel";
        pub const CAPITAL_LABEL_NAME: &str = "capital_label";
        pub const LEFT_SIDE_TOP_NAME: &str = "left_side_top";
        pub const GAME_PANEL_NAME: &str = "game_panel";
        pub const PRODUCTION_MENU_NAME: &str = "production_menu";
        pub const STATISTICS_PANEL_NAME: &str = "statistics_panel";
        pub const TILE_INFO_NAME: &str = "tile_info";
        pub const CIVILIZATIONS_LIST_NAME: &str = "civilizations_list";
        pub const MINIMAP_NAME: &str = "minimap";
    }

    pub mod ui_properties {
        pub const GAME_TITLE_PROPERTY: &str = "game_title";
        pub const CURRENT_TURN_PROPERTY: &str = "current_turn";
        pub const PLAYER_GOLD_PROPERTY: &str = "player_gold";
        pub const PLAYER_PRODUCTION_PROPERTY: &str = "player_production";
        pub const SHOW_PRODUCTION_MENU_PROPERTY: &str = "show_production_menu";
        pub const CAPITAL_NAME_PROPERTY: &str = "capital_name";
        pub const CIVILIZATION_NAME_PROPERTY: &str = "civilization_name";
        pub const CIVILIZATION_GOLD_PROPERTY: &str = "civilization_gold";
        pub const CIVILIZATION_PRODUCTION_PROPERTY: &str = "civilization_production";
        pub const CURRENT_PRODUCTION_NAME_PROPERTY: &str = "current_production_name";
        pub const CURRENT_PRODUCTION_PROGRESS_PROPERTY: &str = "current_production_progress";
        pub const PRODUCTION_QUEUE_LENGTH_PROPERTY: &str = "production_queue_length";
        pub const POSITION_X_PROPERTY: &str = "position_x";
        pub const POSITION_Y_PROPERTY: &str = "position_y";
    }

    pub mod ui_initial_values {
        pub const INITIAL_TURN_VALUE: &str = "1";
        pub const INITIAL_GOLD_VALUE: &str = "0";
        pub const INITIAL_PRODUCTION_VALUE: &str = "0";
        pub const INITIAL_TERRAIN_COUNT_VALUE: &str = "0";
        pub const LOADING_CIVILIZATIONS_MESSAGE: &str = "Loading…";
        pub const MENU_DISPLAY_NONE_VALUE: &str = "none";
        pub const DEFAULT_CAPITAL_NAME_FALLBACK: &str = "Capital";
        pub const HTML_FUNCTION_ENTITY_RAW_VALUE: u32 = 0;
    }

    pub mod capital_labels {
        pub const LABEL_VERTICAL_OFFSET: f32 = 40.0;
        pub const DEFAULT_CAPITAL_NAME: &str = "Capital";
        pub const CAPITAL_LABEL_COMPONENT_NAME: &str = "capital_label";
        pub const CAPITAL_LABEL_TEMPLATE_PATH: &str = "ui/components/capital_label.html";
        pub const CAPITAL_NAME_PROPERTY_KEY: &str = "capital_name";
        pub const POSITION_X_PROPERTY_KEY: &str = "position_x";
        pub const POSITION_Y_PROPERTY_KEY: &str = "position_y";
        pub const DEBUG_CAPITAL_PROCESSING_MESSAGE: &str = "Processing NEW capital at position";
        pub const DEBUG_CAPITAL_NAME_FOUND_MESSAGE: &str = "Found capital name";
        pub const DEBUG_NO_CITY_COMPONENT_MESSAGE: &str =
            "No city component found, using default name";
        pub const DEBUG_SPAWNING_LABEL_MESSAGE: &str = "Spawning capital label";
        pub const DEBUG_NO_TILEMAP_MESSAGE: &str = "No tilemap found for capital labels";
        pub const DEBUG_SPAWN_SYSTEM_RUNNING_MESSAGE: &str = "spawn_capital_labels system running";
        pub const DEBUG_FOUND_CAPITALS_MESSAGE: &str = "Found {} capitals for label spawning";
    }
}

struct GameDataCollection<'a> {
    all_civilizations: Vec<&'a Civilization>,
    player_civilizations: Vec<&'a Civilization>,
    capital_list: Vec<(&'a Capital, &'a Position)>,
    city_list: Vec<(&'a core_sim::City, &'a Position)>,
}

struct PlayerStatistics {
    player_gold: i32,
    total_production: i32,
    total_cities_count: usize,
}

struct UiProductionMenuData {
    display_style: String,
    capital_name: String,
    civilization_name: String,
    civilization_gold: i32,
    civilization_production: i32,
    current_production_name: String,
    current_production_progress: i32,
    production_queue_length: usize,
}

struct TerrainStatistics {
    land_count: usize,
    water_count: usize,
    mountain_count: usize,
}

struct SelectedTileInformation {
    position_text: String,
    terrain_type_text: String,
}

/// Update UI properties with current game data
fn update_ui_properties(
    current_turn: Res<CurrentTurn>,
    terrain_counts: Res<TerrainCounts>,
    selected_tile: Res<SelectedTile>,
    selected_capital: Res<SelectedCapital>,
    debug_logging: Res<DebugLogging>,
    civs: Query<&Civilization>,
    player_civs: Query<&Civilization, With<core_sim::PlayerControlled>>,
    capitals: Query<(&Capital, &Position)>,
    cities: Query<(&core_sim::City, &Position)>,
    production_queues: Query<&ProductionQueue>,
    mut ui_nodes: Query<(Entity, &mut TemplateProperties), With<HtmlNode>>,
    mut cmd: Commands,
) {
    if should_update_ui_properties(
        &current_turn,
        &terrain_counts,
        &selected_tile,
        &selected_capital,
    ) {
        let game_data = collect_game_data_from_queries(&civs, &player_civs, &capitals, &cities);

        log_collected_game_data(&debug_logging, &game_data);

        let player_stats = calculate_player_statistics(&game_data, &production_queues);
        let production_menu_data =
            build_production_menu_data(&selected_capital, &civs, &production_queues);
        let capital_names_text = format_capital_and_city_names(&game_data);
        let civilization_details_text = format_civilization_details(&game_data, &player_civs);
        let terrain_stats = calculate_terrain_statistics(&terrain_counts);
        let selected_tile_info = format_selected_tile_information(&selected_tile);

        update_all_ui_node_properties(
            &mut ui_nodes,
            &mut cmd,
            &current_turn,
            &player_stats,
            &production_menu_data,
            &capital_names_text,
            &civilization_details_text,
            &terrain_stats,
            &selected_tile_info,
        );
    }
}

fn should_update_ui_properties(
    current_turn: &Res<CurrentTurn>,
    terrain_counts: &Res<TerrainCounts>,
    selected_tile: &Res<SelectedTile>,
    selected_capital: &Res<SelectedCapital>,
) -> bool {
    current_turn.is_changed()
        || terrain_counts.is_changed()
        || selected_tile.is_changed()
        || selected_capital.is_changed()
}

fn collect_game_data_from_queries<'a>(
    civs: &'a Query<&Civilization>,
    player_civs: &'a Query<&Civilization, With<core_sim::PlayerControlled>>,
    capitals: &'a Query<(&Capital, &Position)>,
    cities: &'a Query<(&core_sim::City, &Position)>,
) -> GameDataCollection<'a> {
    GameDataCollection {
        all_civilizations: civs.iter().collect(),
        player_civilizations: player_civs.iter().collect(),
        capital_list: capitals.iter().collect(),
        city_list: cities.iter().collect(),
    }
}

fn log_collected_game_data(debug_logging: &DebugLogging, game_data: &GameDataCollection) {
    debug_println!(
        debug_logging,
        "UI UPDATE: Found {} civilizations, {} player civs, {} capitals, {} cities",
        game_data.all_civilizations.len(),
        game_data.player_civilizations.len(),
        game_data.capital_list.len(),
        game_data.city_list.len()
    );
}

fn calculate_player_statistics(
    game_data: &GameDataCollection,
    production_queues: &Query<&ProductionQueue>,
) -> PlayerStatistics {
    let player_gold = game_data
        .player_civilizations
        .first()
        .map(|civ| civ.economy.gold as i32)
        .unwrap_or(0);

    let total_production_float: f32 = production_queues
        .iter()
        .map(|queue| queue.accumulated_production)
        .sum();

    let total_cities_count = game_data.capital_list.len() + game_data.city_list.len();

    PlayerStatistics {
        player_gold,
        total_production: total_production_float as i32,
        total_cities_count,
    }
}

fn build_production_menu_data(
    selected_capital: &SelectedCapital,
    civs: &Query<&Civilization>,
    production_queues: &Query<&ProductionQueue>,
) -> UiProductionMenuData {
    if !selected_capital.show_production_menu {
        return create_hidden_production_menu_data();
    }

    match (selected_capital.capital_entity, selected_capital.civ_entity) {
        (Some(capital_entity), Some(civ_entity)) => {
            create_visible_production_menu_data(capital_entity, civ_entity, civs, production_queues)
        }
        _ => create_hidden_production_menu_data(),
    }
}

fn create_hidden_production_menu_data() -> UiProductionMenuData {
    UiProductionMenuData {
        display_style: constants::ui_update::PRODUCTION_MENU_DISPLAY_NONE.to_string(),
        capital_name: constants::ui_update::UNKNOWN_CAPITAL_NAME.to_string(),
        civilization_name: constants::ui_update::UNKNOWN_CIVILIZATION_NAME.to_string(),
        civilization_gold: 0,
        civilization_production: 0,
        current_production_name: constants::ui_update::NO_PRODUCTION_NAME.to_string(),
        current_production_progress: 0,
        production_queue_length: 0,
    }
}

fn create_visible_production_menu_data(
    capital_entity: Entity,
    civ_entity: Entity,
    civs: &Query<&Civilization>,
    production_queues: &Query<&ProductionQueue>,
) -> UiProductionMenuData {
    let capital_name = "Capital".to_string(); // TODO: Get actual capital name

    let (civilization_name, civilization_gold, civilization_production) =
        extract_civilization_information(civ_entity, civs);

    let (current_production_name, current_production_progress, production_queue_length) =
        extract_production_queue_information(capital_entity, production_queues);

    UiProductionMenuData {
        display_style: constants::ui_update::PRODUCTION_MENU_DISPLAY_FLEX.to_string(),
        capital_name,
        civilization_name,
        civilization_gold,
        civilization_production,
        current_production_name,
        current_production_progress,
        production_queue_length,
    }
}

fn extract_civilization_information(
    civ_entity: Entity,
    civs: &Query<&Civilization>,
) -> (String, i32, i32) {
    match civs.get(civ_entity) {
        Ok(civilization) => (
            civilization.name.clone(),
            civilization.economy.gold as i32,
            civilization.economy.production as i32,
        ),
        Err(_) => (
            constants::ui_update::UNKNOWN_CIVILIZATION_NAME.to_string(),
            0,
            0,
        ),
    }
}

fn extract_production_queue_information(
    capital_entity: Entity,
    production_queues: &Query<&ProductionQueue>,
) -> (String, i32, usize) {
    match production_queues.get(capital_entity) {
        Ok(production_queue) => {
            let current_production_name = production_queue
                .current_production
                .as_ref()
                .map(|item| item.name().to_string())
                .unwrap_or_else(|| constants::ui_update::NO_PRODUCTION_NAME.to_string());

            let progress_percentage = (production_queue.get_progress_percentage()
                * constants::ui_update::PERCENTAGE_MULTIPLIER)
                as i32;

            let queue_length = production_queue.queue_length();

            (current_production_name, progress_percentage, queue_length)
        }
        Err(_) => (constants::ui_update::NO_PRODUCTION_NAME.to_string(), 0, 0),
    }
}

fn format_capital_and_city_names(game_data: &GameDataCollection) -> String {
    if game_data.capital_list.is_empty() && game_data.city_list.is_empty() {
        return constants::ui_update::NO_CAPITALS_FOUNDED_MESSAGE.to_string();
    }

    let mut settlement_names = Vec::new();

    add_capital_names_to_list(&mut settlement_names, game_data);
    add_city_names_to_list(&mut settlement_names, game_data);

    if settlement_names.is_empty() {
        constants::ui_update::NO_CITIES_FOUNDED_MESSAGE.to_string()
    } else {
        settlement_names.join(", ")
    }
}

fn add_capital_names_to_list(settlement_names: &mut Vec<String>, game_data: &GameDataCollection) {
    for (capital, position) in &game_data.capital_list {
        let civilization_name =
            find_civilization_name_by_id(capital.owner, &game_data.all_civilizations);
        settlement_names.push(format!(
            "{} Capital at ({}, {})",
            civilization_name, position.x, position.y
        ));
    }
}

fn add_city_names_to_list(settlement_names: &mut Vec<String>, game_data: &GameDataCollection) {
    for (city, position) in &game_data.city_list {
        settlement_names.push(format!("{} at ({}, {})", city.name, position.x, position.y));
    }
}

fn find_civilization_name_by_id<'a>(
    civ_id: core_sim::CivId,
    civilizations: &'a [&Civilization],
) -> &'a str {
    civilizations
        .iter()
        .find(|civ| civ.id == civ_id)
        .map(|civ| civ.name.as_str())
        .unwrap_or("Unknown")
}

fn format_civilization_details(
    game_data: &GameDataCollection,
    player_civs: &Query<&Civilization, With<core_sim::PlayerControlled>>,
) -> String {
    if game_data.all_civilizations.is_empty() {
        return constants::ui_update::NO_CIVILIZATIONS_MESSAGE.to_string();
    }

    game_data
        .all_civilizations
        .iter()
        .map(|civilization| format_single_civilization_detail(civilization, player_civs))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_single_civilization_detail(
    civilization: &Civilization,
    player_civs: &Query<&Civilization, With<core_sim::PlayerControlled>>,
) -> String {
    let civilization_type = determine_civilization_type(civilization, player_civs);
    format!(
        "{} - {} (Gold: {})",
        civilization.name, civilization_type, civilization.economy.gold as i32
    )
}

fn determine_civilization_type(
    civilization: &Civilization,
    player_civs: &Query<&Civilization, With<core_sim::PlayerControlled>>,
) -> &'static str {
    if player_civs
        .iter()
        .any(|player_civ| player_civ.id == civilization.id)
    {
        constants::ui_update::PLAYER_CIVILIZATION_TYPE
    } else {
        constants::ui_update::AI_CIVILIZATION_TYPE
    }
}

fn calculate_terrain_statistics(terrain_counts: &TerrainCounts) -> TerrainStatistics {
    let land_count = terrain_counts.plains
        + terrain_counts.hills
        + terrain_counts.forest
        + terrain_counts.desert;

    let water_count = terrain_counts.ocean + terrain_counts.coast + terrain_counts.river;

    TerrainStatistics {
        land_count,
        water_count,
        mountain_count: terrain_counts.mountains,
    }
}

fn format_selected_tile_information(selected_tile: &SelectedTile) -> SelectedTileInformation {
    match selected_tile.position {
        Some(position) => SelectedTileInformation {
            position_text: format!("({}, {})", position.x, position.y),
            terrain_type_text: constants::ui_update::UNKNOWN_TERRAIN_TYPE.to_string(),
        },
        None => SelectedTileInformation {
            position_text: constants::ui_update::POSITION_NONE_TEXT.to_string(),
            terrain_type_text: constants::ui_update::TERRAIN_NONE_TEXT.to_string(),
        },
    }
}

fn update_all_ui_node_properties(
    ui_nodes: &mut Query<(Entity, &mut TemplateProperties), With<HtmlNode>>,
    cmd: &mut Commands,
    current_turn: &CurrentTurn,
    player_stats: &PlayerStatistics,
    production_menu_data: &UiProductionMenuData,
    capital_names_text: &str,
    civilization_details_text: &str,
    terrain_stats: &TerrainStatistics,
    selected_tile_info: &SelectedTileInformation,
) {
    for (entity, mut template_properties) in ui_nodes.iter_mut() {
        update_game_state_properties(&mut template_properties, current_turn, player_stats);
        update_production_menu_properties(&mut template_properties, production_menu_data);
        update_settlement_and_civilization_properties(
            &mut template_properties,
            capital_names_text,
            civilization_details_text,
        );
        update_terrain_statistics_properties(&mut template_properties, terrain_stats);
        update_selected_tile_properties(&mut template_properties, selected_tile_info);

        cmd.trigger_targets(CompileContextEvent, entity);
    }
}

fn update_game_state_properties(
    template_properties: &mut TemplateProperties,
    current_turn: &CurrentTurn,
    player_stats: &PlayerStatistics,
) {
    template_properties.insert("current_turn".to_string(), current_turn.0.to_string());
    template_properties.insert(
        "player_gold".to_string(),
        player_stats.player_gold.to_string(),
    );
    template_properties.insert(
        "player_production".to_string(),
        player_stats.total_production.to_string(),
    );
    template_properties.insert(
        "total_production".to_string(),
        player_stats.total_production.to_string(),
    );
    template_properties.insert(
        "player_cities".to_string(),
        player_stats.total_cities_count.to_string(),
    );
    template_properties.insert(
        "capital_count".to_string(),
        player_stats.total_cities_count.to_string(),
    );
}

fn update_production_menu_properties(
    template_properties: &mut TemplateProperties,
    production_menu_data: &UiProductionMenuData,
) {
    template_properties.insert(
        "show_production_menu".to_string(),
        production_menu_data.display_style.clone(),
    );
    template_properties.insert(
        "capital_name".to_string(),
        production_menu_data.capital_name.clone(),
    );
    template_properties.insert(
        "civilization_name".to_string(),
        production_menu_data.civilization_name.clone(),
    );
    template_properties.insert(
        "civilization_gold".to_string(),
        production_menu_data.civilization_gold.to_string(),
    );
    template_properties.insert(
        "civilization_production".to_string(),
        production_menu_data.civilization_production.to_string(),
    );
    template_properties.insert(
        "current_production_name".to_string(),
        production_menu_data.current_production_name.clone(),
    );
    template_properties.insert(
        "current_production_progress".to_string(),
        production_menu_data.current_production_progress.to_string(),
    );
    template_properties.insert(
        "production_queue_length".to_string(),
        production_menu_data.production_queue_length.to_string(),
    );
}

fn update_settlement_and_civilization_properties(
    template_properties: &mut TemplateProperties,
    capital_names_text: &str,
    civilization_details_text: &str,
) {
    template_properties.insert("capital_names".to_string(), capital_names_text.to_string());
    template_properties.insert(
        "civilizations_list".to_string(),
        civilization_details_text.to_string(),
    );
}

fn update_terrain_statistics_properties(
    template_properties: &mut TemplateProperties,
    terrain_stats: &TerrainStatistics,
) {
    template_properties.insert(
        "terrain_land_count".to_string(),
        terrain_stats.land_count.to_string(),
    );
    template_properties.insert(
        "terrain_water_count".to_string(),
        terrain_stats.water_count.to_string(),
    );
    template_properties.insert(
        "terrain_mountain_count".to_string(),
        terrain_stats.mountain_count.to_string(),
    );
}

fn update_selected_tile_properties(
    template_properties: &mut TemplateProperties,
    selected_tile_info: &SelectedTileInformation,
) {
    template_properties.insert(
        "selected_position".to_string(),
        selected_tile_info.position_text.clone(),
    );
    template_properties.insert(
        "selected_terrain".to_string(),
        selected_tile_info.terrain_type_text.clone(),
    );
}

/// Component to mark capital label entities
#[derive(Component)]
pub struct CapitalLabel {
    pub capital_entity: Entity,
}

/// System to spawn capital labels over capital tiles
pub fn spawn_capital_labels(
    mut commands_for_spawning: Commands,
    asset_server: Res<AssetServer>,
    capitals_requiring_labels: Query<
        (Entity, &Capital, &Position),
        (With<City>, Without<CapitalLabel>),
    >,
    cities_with_names: Query<&City>,
    tilemap_for_coordinate_conversion: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    existing_capital_labels: Query<&CapitalLabel>,
    debug_logging: Res<DebugLogging>,
) {
    debug_println!(debug_logging, "DEBUG: spawn_capital_labels system running");

    let tilemap_data = match extract_tilemap_information_for_coordinate_conversion(
        &tilemap_for_coordinate_conversion,
        &debug_logging,
    ) {
        Some(data) => data,
        None => return,
    };

    debug_println!(
        debug_logging,
        "DEBUG: Found {} capitals for label spawning",
        capitals_requiring_labels.iter().count()
    );

    for (capital_entity, _capital_component, capital_position) in capitals_requiring_labels.iter() {
        if capital_label_already_exists_for_entity(capital_entity, &existing_capital_labels) {
            continue;
        }

        debug_println!(
            debug_logging,
            "DEBUG: Processing NEW capital at position ({}, {})",
            capital_position.x,
            capital_position.y
        );

        let capital_display_name = extract_capital_display_name_from_city_component(
            capital_entity,
            &cities_with_names,
            &debug_logging,
        );

        let label_world_coordinates =
            calculate_capital_label_world_position(capital_position, &tilemap_data);

        spawn_single_capital_label_entity(
            &mut commands_for_spawning,
            &asset_server,
            capital_entity,
            &capital_display_name,
            &label_world_coordinates,
            &debug_logging,
        );
    }
}

struct TilemapCoordinateData {
    tilemap_size: TilemapSize,
    tile_size: TilemapTileSize,
    grid_size: TilemapGridSize,
    map_type: TilemapType,
    anchor: TilemapAnchor,
}

struct LabelWorldCoordinates {
    x: f32,
    y: f32,
}

fn extract_tilemap_information_for_coordinate_conversion(
    tilemap_query: &Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
    debug_logging: &DebugLogging,
) -> Option<TilemapCoordinateData> {
    match tilemap_query.single() {
        Ok((tilemap_size, tile_size, grid_size, map_type, anchor)) => Some(TilemapCoordinateData {
            tilemap_size: *tilemap_size,
            tile_size: *tile_size,
            grid_size: *grid_size,
            map_type: *map_type,
            anchor: *anchor,
        }),
        Err(_) => {
            debug_println!(debug_logging, "DEBUG: No tilemap found for capital labels");
            None
        }
    }
}

fn capital_label_already_exists_for_entity(
    capital_entity: Entity,
    existing_labels: &Query<&CapitalLabel>,
) -> bool {
    existing_labels
        .iter()
        .any(|label| label.capital_entity == capital_entity)
}

fn extract_capital_display_name_from_city_component(
    capital_entity: Entity,
    cities_query: &Query<&City>,
    debug_logging: &DebugLogging,
) -> String {
    match cities_query.get(capital_entity) {
        Ok(city) => {
            debug_println!(debug_logging, "DEBUG: Found capital name: {}", city.name);
            city.name.clone()
        }
        Err(_) => {
            debug_println!(
                debug_logging,
                "DEBUG: No city component found, using default name"
            );
            constants::capital_labels::DEFAULT_CAPITAL_NAME.to_string()
        }
    }
}

fn calculate_capital_label_world_position(
    capital_position: &Position,
    tilemap_data: &TilemapCoordinateData,
) -> LabelWorldCoordinates {
    let tile_position = TilePos {
        x: capital_position.x as u32,
        y: capital_position.y as u32,
    };

    let world_position = tile_position.center_in_world(
        &tilemap_data.tilemap_size,
        &tilemap_data.grid_size,
        &tilemap_data.tile_size,
        &tilemap_data.map_type,
        &tilemap_data.anchor,
    );

    LabelWorldCoordinates {
        x: world_position.x,
        y: world_position.y + constants::capital_labels::LABEL_VERTICAL_OFFSET,
    }
}

fn spawn_single_capital_label_entity(
    commands: &mut Commands,
    asset_server: &AssetServer,
    capital_entity: Entity,
    capital_name: &str,
    world_coordinates: &LabelWorldCoordinates,
    debug_logging: &DebugLogging,
) {
    debug_println!(
        debug_logging,
        "DEBUG: Spawning capital label '{}' at world position ({}, {})",
        capital_name,
        world_coordinates.x,
        world_coordinates.y
    );

    commands.spawn((
        HtmlNode(asset_server.load(constants::capital_labels::CAPITAL_LABEL_TEMPLATE_PATH)),
        TemplateProperties::default()
            .with(
                constants::capital_labels::CAPITAL_NAME_PROPERTY_KEY,
                capital_name,
            )
            .with(
                constants::capital_labels::POSITION_X_PROPERTY_KEY,
                &world_coordinates.x.to_string(),
            )
            .with(
                constants::capital_labels::POSITION_Y_PROPERTY_KEY,
                &world_coordinates.y.to_string(),
            ),
        CapitalLabel { capital_entity },
        Name::new(constants::capital_labels::CAPITAL_LABEL_COMPONENT_NAME),
    ));
}

/// System to update capital label positions when needed
pub fn update_capital_labels(
    mut capital_labels_requiring_updates: Query<
        (&CapitalLabel, &mut TemplateProperties),
        Changed<Position>,
    >,
    capital_positions: Query<&Position, With<Capital>>,
    capital_city_names: Query<&City, Changed<City>>,
    tilemap_for_coordinate_conversion: Query<(
        &TilemapSize,
        &TilemapTileSize,
        &TilemapGridSize,
        &TilemapType,
        &TilemapAnchor,
    )>,
) {
    let tilemap_data = match extract_tilemap_information_for_coordinate_conversion(
        &tilemap_for_coordinate_conversion,
        &DebugLogging(false),
    ) {
        Some(data) => data,
        None => return,
    };

    for (capital_label, mut template_properties) in capital_labels_requiring_updates.iter_mut() {
        update_capital_label_position_if_changed(
            capital_label,
            &mut template_properties,
            &capital_positions,
            &tilemap_data,
        );

        update_capital_label_name_if_changed(
            capital_label,
            &mut template_properties,
            &capital_city_names,
        );
    }
}

fn update_capital_label_position_if_changed(
    capital_label: &CapitalLabel,
    template_properties: &mut TemplateProperties,
    capital_positions: &Query<&Position, With<Capital>>,
    tilemap_data: &TilemapCoordinateData,
) {
    if let Ok(capital_position) = capital_positions.get(capital_label.capital_entity) {
        let label_world_coordinates =
            calculate_capital_label_world_position(capital_position, tilemap_data);

        template_properties.insert(
            constants::capital_labels::POSITION_X_PROPERTY_KEY.to_string(),
            label_world_coordinates.x.to_string(),
        );
        template_properties.insert(
            constants::capital_labels::POSITION_Y_PROPERTY_KEY.to_string(),
            label_world_coordinates.y.to_string(),
        );
    }
}

fn update_capital_label_name_if_changed(
    capital_label: &CapitalLabel,
    template_properties: &mut TemplateProperties,
    capital_city_names: &Query<&City, Changed<City>>,
) {
    if let Ok(city) = capital_city_names.get(capital_label.capital_entity) {
        template_properties.insert(
            constants::capital_labels::CAPITAL_NAME_PROPERTY_KEY.to_string(),
            city.name.clone(),
        );
    }
}

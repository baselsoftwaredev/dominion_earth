use crate::debug_println;
use crate::debug_utils::DebugLogging;
use crate::production_input::SelectedCapital;
use crate::ui::resources::*;
use bevy::prelude::*;
use bevy_hui::prelude::*;
use core_sim::{
    components::{Capital, City, TerrainType},
    resources::CurrentTurn,
    ActionQueue, Civilization, Position, ProductionQueue,
};

use super::constants;

fn format_terrain_type_display(terrain: &TerrainType) -> String {
    match terrain {
        TerrainType::Plains => "Plains".to_string(),
        TerrainType::Hills => "Hills".to_string(),
        TerrainType::Mountains => "Mountains".to_string(),
        TerrainType::Forest => "Forest".to_string(),
        TerrainType::Desert => "Desert".to_string(),
        TerrainType::Coast => "Coast".to_string(),
        TerrainType::ShallowCoast => "Shallow Coast".to_string(),
        TerrainType::Ocean => "Ocean".to_string(),
        TerrainType::River => "River".to_string(),
    }
}

pub struct GameDataCollection<'a> {
    pub all_civilizations: Vec<&'a Civilization>,
    pub player_civilizations: Vec<&'a Civilization>,
    pub capital_list: Vec<(&'a Capital, &'a Position)>,
    pub city_list: Vec<(&'a core_sim::City, &'a Position)>,
}

pub struct PlayerStatistics {
    pub player_gold: i32,
    pub total_production: i32,
    pub total_cities_count: usize,
}

pub struct UiProductionMenuData {
    pub display_style: String,
    pub capital_name: String,
    pub civilization_name: String,
    pub civilization_gold: i32,
    pub civilization_production: i32,
    pub current_production_name: String,
    pub current_production_progress: i32,
    pub production_queue_length: usize,
    pub action_queue_length: usize,
    pub current_action_name: String,
}

pub struct TerrainStatistics {
    pub land_count: usize,
    pub water_count: usize,
    pub mountain_count: usize,
}

pub struct HoveredTileInformation {
    pub position_text: String,
    pub terrain_type_text: String,
}

/// Update UI properties with current game data
pub fn update_ui_properties_system(
    mut ui_nodes: Query<(Entity, &mut TemplateProperties), With<HtmlNode>>,
    mut cmd: Commands,
    civs: Query<&Civilization>,
    player_civs: Query<&Civilization, With<core_sim::PlayerControlled>>,
    capitals: Query<(&Capital, &Position)>,
    cities: Query<(&City, &Position)>,
    production_queues: Query<&ProductionQueue>,
    changed_production_queues: Query<Entity, Changed<ProductionQueue>>,
    action_queues: Query<&ActionQueue>,
    current_turn: Res<CurrentTurn>,
    terrain_counts: Res<TerrainCounts>,
    hovered_tile: Res<HoveredTile>,
    selected_capital: Res<SelectedCapital>,
    debug_logging: Res<DebugLogging>,
) {
    if !should_update_ui_properties(
        &current_turn,
        &terrain_counts,
        &hovered_tile,
        &selected_capital,
        &changed_production_queues,
    ) {
        return;
    }

    let game_data = collect_game_data_from_queries(&civs, &player_civs, &capitals, &cities);
    log_collected_game_data(&debug_logging, &game_data);

    let player_stats = calculate_player_statistics(&game_data, &production_queues);
    let production_menu_data =
        build_production_menu_data(&selected_capital, &civs, &production_queues, &action_queues);

    debug_println!(
        debug_logging,
        "UI SYSTEM: Updating production menu - current_production_name: '{}', progress: {}%",
        production_menu_data.current_production_name,
        production_menu_data.current_production_progress
    );

    let capital_names_text = format_capital_and_city_names(&game_data);
    let civilization_details_text = format_civilization_details(&game_data, &player_civs);
    let terrain_stats = calculate_terrain_statistics(&terrain_counts);
    let hovered_tile_info = format_hovered_tile_information(&hovered_tile);

    update_all_ui_node_properties(
        &mut ui_nodes,
        &mut cmd,
        &current_turn,
        &player_stats,
        &production_menu_data,
        &capital_names_text,
        &civilization_details_text,
        &terrain_stats,
        &hovered_tile_info,
    );
}
fn should_update_ui_properties(
    current_turn: &Res<CurrentTurn>,
    terrain_counts: &Res<TerrainCounts>,
    hovered_tile: &Res<HoveredTile>,
    selected_capital: &Res<SelectedCapital>,
    changed_production_queues: &Query<Entity, Changed<ProductionQueue>>,
) -> bool {
    current_turn.is_changed()
        || terrain_counts.is_changed()
        || hovered_tile.is_changed()
        || selected_capital.is_changed()
        || !changed_production_queues.is_empty()
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
    action_queues: &Query<&ActionQueue>,
) -> UiProductionMenuData {
    if !selected_capital.show_production_menu {
        return create_hidden_production_menu_data();
    }

    match (selected_capital.capital_entity, selected_capital.civ_entity) {
        (Some(capital_entity), Some(civ_entity)) => create_visible_production_menu_data(
            capital_entity,
            civ_entity,
            civs,
            production_queues,
            action_queues,
        ),
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
        action_queue_length: 0,
        current_action_name: "No Action".to_string(),
    }
}

fn create_visible_production_menu_data(
    capital_entity: Entity,
    civ_entity: Entity,
    civs: &Query<&Civilization>,
    production_queues: &Query<&ProductionQueue>,
    action_queues: &Query<&core_sim::ActionQueue>,
) -> UiProductionMenuData {
    let capital_name = "Capital".to_string(); // TODO: Get actual capital name

    let (civilization_name, civilization_gold, civilization_production) =
        extract_civilization_information(civ_entity, civs);

    let (current_production_name, current_production_progress, production_queue_length) =
        extract_production_queue_information(capital_entity, production_queues);

    let (action_queue_length, current_action_name) =
        extract_action_queue_information(civ_entity, action_queues);

    UiProductionMenuData {
        display_style: constants::ui_update::PRODUCTION_MENU_DISPLAY_FLEX.to_string(),
        capital_name,
        civilization_name,
        civilization_gold,
        civilization_production,
        current_production_name,
        current_production_progress,
        production_queue_length,
        action_queue_length,
        current_action_name,
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

fn extract_action_queue_information(
    civ_entity: Entity,
    action_queues: &Query<&ActionQueue>,
) -> (usize, String) {
    match action_queues.get(civ_entity) {
        Ok(action_queue) => {
            let queue_length = action_queue.get_queue_length();

            let current_action_name = if let Some(next_action) = action_queue.peek_next_action(0) {
                match &next_action.action {
                    core_sim::AIAction::BuildUnit { unit_type, .. } => {
                        format!("Build {:?}", unit_type)
                    }
                    core_sim::AIAction::Research { technology, .. } => {
                        format!("Research {}", technology)
                    }
                    core_sim::AIAction::Expand { .. } => "Expand Territory".to_string(),
                    core_sim::AIAction::BuildBuilding { building_type, .. } => {
                        format!("Build {:?}", building_type)
                    }
                    core_sim::AIAction::Trade { .. } => "Trade".to_string(),
                    core_sim::AIAction::Attack { .. } => "Attack".to_string(),
                    core_sim::AIAction::Diplomacy { .. } => "Diplomacy".to_string(),
                    core_sim::AIAction::Defend { .. } => "Defend".to_string(),
                }
            } else {
                "No Actions Queued".to_string()
            };

            (queue_length, current_action_name)
        }
        Err(_) => (0, "No Queue".to_string()),
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

fn format_hovered_tile_information(hovered_tile: &HoveredTile) -> HoveredTileInformation {
    match hovered_tile.position {
        Some(position) => {
            let terrain_text = match &hovered_tile.terrain_type {
                Some(terrain) => format_terrain_type_display(terrain),
                None => constants::ui_update::UNKNOWN_TERRAIN_TYPE.to_string(),
            };
            HoveredTileInformation {
                position_text: format!("({}, {})", position.x, position.y),
                terrain_type_text: terrain_text,
            }
        }
        None => HoveredTileInformation {
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
    hovered_tile_info: &HoveredTileInformation,
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
        update_hovered_tile_properties(&mut template_properties, hovered_tile_info);

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
    template_properties.insert(
        "action_queue_length".to_string(),
        production_menu_data.action_queue_length.to_string(),
    );
    template_properties.insert(
        "current_action_name".to_string(),
        production_menu_data.current_action_name.clone(),
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

fn update_hovered_tile_properties(
    template_properties: &mut TemplateProperties,
    hovered_tile_info: &HoveredTileInformation,
) {
    template_properties.insert(
        "hovered_position".to_string(),
        hovered_tile_info.position_text.clone(),
    );
    template_properties.insert(
        "hovered_terrain".to_string(),
        hovered_tile_info.terrain_type_text.clone(),
    );
}

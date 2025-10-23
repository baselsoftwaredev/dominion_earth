use crate::debug_println;
use crate::debug_utils::DebugLogging;
use crate::production_input::SelectedCapital;
use crate::ui::resources::*;
use bevy::prelude::*;
use bevy_hui::prelude::*;
use core_sim::{
    components::{Capital, City, TerrainType},
    resources::CurrentTurn,
    ActionQueue, AllAITurnsComplete, Civilization, PlayerProductionOrder, Position,
    ProductionQueue, ProductionUpdated, SkipProductionThisTurn, StartPlayerTurn,
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

pub struct UnitInformation {
    pub is_visible: String,
    pub unit_name: String,
    pub unit_type: String,
    pub attack: String,
    pub defense: String,
    pub health: String,
    pub max_health: String,
    pub movement_remaining: String,
    pub movement_range: String,
    pub range: String,
    pub experience: String,
    pub fatigue: String,
    pub supply: String,
    pub decay: String,
    pub effective_attack: String,
    pub effective_defense: String,
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
    action_queues: Query<&ActionQueue>,
    current_turn: Res<CurrentTurn>,
    terrain_counts: Res<TerrainCounts>,
    hovered_tile: Res<HoveredTile>,
    selected_capital: Res<SelectedCapital>,
    selected_unit: Res<core_sim::SelectedUnit>,
    units_query: Query<&core_sim::MilitaryUnit>,
    debug_logging: Res<DebugLogging>,
) {
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
    let unit_info = build_unit_info_data(&selected_unit, &units_query, &selected_capital);

    debug_println!(
        debug_logging,
        "UI SYSTEM: Unit info - is_visible: {}, unit_entity: {:?}, show_production_menu: {}",
        unit_info.is_visible,
        selected_unit.unit_entity,
        selected_capital.show_production_menu
    );

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
        &unit_info,
    );
}

/// Condition function to determine when UI should update
/// This runs much more efficiently than the full UI update system
pub fn should_update_ui_this_frame(
    // Game state change events that affect UI
    start_player_events: EventReader<StartPlayerTurn>,
    ai_complete_events: EventReader<AllAITurnsComplete>,
    production_events: EventReader<ProductionUpdated>,

    // Player action events that affect UI state
    player_production_events: EventReader<PlayerProductionOrder>,
    skip_production_events: EventReader<SkipProductionThisTurn>,

    // Resource changes that affect UI
    current_turn: Res<CurrentTurn>,
    terrain_counts: Res<TerrainCounts>,
    hovered_tile: Res<HoveredTile>,
    selected_capital: Res<SelectedCapital>,
    selected_unit: Res<core_sim::SelectedUnit>,

    // Component changes
    changed_production_queues: Query<Entity, Changed<ProductionQueue>>,
) -> bool {
    let mut reasons = Vec::new();

    // Check for game events that should trigger UI updates
    let has_start_player_events = !start_player_events.is_empty();
    let has_ai_complete_events = !ai_complete_events.is_empty();
    let has_production_events = !production_events.is_empty();

    let has_game_events =
        has_start_player_events || has_ai_complete_events || has_production_events;

    if has_start_player_events {
        reasons.push("StartPlayerTurn events".to_string());
    }
    if has_ai_complete_events {
        reasons.push("AllAITurnsComplete events".to_string());
    }
    if has_production_events {
        reasons.push("ProductionUpdated events".to_string());
    }

    // Check for player action events
    let has_player_production_events = !player_production_events.is_empty();
    let has_skip_production_events = !skip_production_events.is_empty();

    let has_player_action_events = has_player_production_events || has_skip_production_events;

    if has_player_production_events {
        reasons.push("PlayerProductionOrder events".to_string());
    }
    if has_skip_production_events {
        reasons.push("SkipProductionThisTurn events".to_string());
    }

    // Check for resource changes (excluding HoveredTile which changes too frequently)
    let current_turn_changed = current_turn.is_changed();
    let terrain_counts_changed = terrain_counts.is_changed();
    let selected_capital_changed = selected_capital.is_changed();
    let selected_unit_changed = selected_unit.is_changed();

    let has_resource_changes = current_turn_changed
        || terrain_counts_changed
        || selected_capital_changed
        || selected_unit_changed;

    if current_turn_changed {
        reasons.push("CurrentTurn changed".to_string());
    }
    if terrain_counts_changed {
        reasons.push("TerrainCounts changed".to_string());
    }
    if selected_capital_changed {
        reasons.push("SelectedCapital changed".to_string());
    }
    if selected_unit_changed {
        reasons.push("SelectedUnit changed".to_string());
    }

    let hovered_tile_changed = hovered_tile.is_changed();
    if hovered_tile_changed {
        reasons.push("HoveredTile changed (throttled)".to_string());
    }

    let production_queue_count = changed_production_queues.iter().count();
    let has_production_changes = production_queue_count > 0;

    if has_production_changes {
        reasons.push(format!(
            "{} ProductionQueue components changed",
            production_queue_count
        ));
    }

    let should_update = has_game_events
        || has_player_action_events
        || has_resource_changes
        || has_production_changes;

    if should_update {
        println!("🔄 UI UPDATE TRIGGERED: {}", reasons.join(", "));
    }

    should_update
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
    let capital_name = "Capital".to_string();

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

fn build_unit_info_data(
    selected_unit: &core_sim::SelectedUnit,
    units_query: &Query<&core_sim::MilitaryUnit>,
    selected_capital: &SelectedCapital,
) -> UnitInformation {
    if selected_capital.show_production_menu {
        return UnitInformation {
            is_visible: "none".to_string(),
            unit_name: "None".to_string(),
            unit_type: "None".to_string(),
            attack: "0".to_string(),
            defense: "0".to_string(),
            health: "0".to_string(),
            max_health: "0".to_string(),
            movement_remaining: "0".to_string(),
            movement_range: "0".to_string(),
            range: "0".to_string(),
            experience: "0".to_string(),
            fatigue: "0".to_string(),
            supply: "0".to_string(),
            decay: "0".to_string(),
            effective_attack: "0".to_string(),
            effective_defense: "0".to_string(),
        };
    }

    if let Some(unit_entity) = selected_unit.unit_entity {
        if let Ok(unit) = units_query.get(unit_entity) {
            return UnitInformation {
                is_visible: "flex".to_string(),
                unit_name: format!("Unit #{}", unit.id),
                unit_type: unit.unit_type.name().to_string(),
                attack: format!("{:.1}", unit.attack),
                defense: format!("{:.1}", unit.defense),
                health: format!("{:.0}", unit.health),
                max_health: format!("{:.0}", unit.max_health),
                movement_remaining: unit.movement_remaining.to_string(),
                movement_range: unit.movement_range.to_string(),
                range: unit.range.to_string(),
                experience: format!("{:.0}", unit.experience * 100.0),
                fatigue: format!("{:.0}", unit.fatigue * 100.0),
                supply: format!("{:.0}", unit.supply * 100.0),
                decay: format!("{:.0}", unit.decay * 100.0),
                effective_attack: format!("{:.1}", unit.effective_attack()),
                effective_defense: format!("{:.1}", unit.effective_defense()),
            };
        }
    }

    UnitInformation {
        is_visible: "none".to_string(),
        unit_name: "None".to_string(),
        unit_type: "None".to_string(),
        attack: "0".to_string(),
        defense: "0".to_string(),
        health: "0".to_string(),
        max_health: "0".to_string(),
        movement_remaining: "0".to_string(),
        movement_range: "0".to_string(),
        range: "0".to_string(),
        experience: "0".to_string(),
        fatigue: "0".to_string(),
        supply: "0".to_string(),
        decay: "0".to_string(),
        effective_attack: "0".to_string(),
        effective_defense: "0".to_string(),
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
    unit_info: &UnitInformation,
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
        update_unit_info_properties(&mut template_properties, unit_info);

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

fn update_unit_info_properties(
    template_properties: &mut TemplateProperties,
    unit_info: &UnitInformation,
) {
    template_properties.insert("is_visible".to_string(), unit_info.is_visible.clone());
    template_properties.insert("unit_name".to_string(), unit_info.unit_name.clone());
    template_properties.insert("unit_type".to_string(), unit_info.unit_type.clone());
    template_properties.insert("attack".to_string(), unit_info.attack.clone());
    template_properties.insert("defense".to_string(), unit_info.defense.clone());
    template_properties.insert("health".to_string(), unit_info.health.clone());
    template_properties.insert("max_health".to_string(), unit_info.max_health.clone());
    template_properties.insert(
        "movement_remaining".to_string(),
        unit_info.movement_remaining.clone(),
    );
    template_properties.insert(
        "movement_range".to_string(),
        unit_info.movement_range.clone(),
    );
    template_properties.insert("range".to_string(), unit_info.range.clone());
    template_properties.insert("experience".to_string(), unit_info.experience.clone());
    template_properties.insert("fatigue".to_string(), unit_info.fatigue.clone());
    template_properties.insert("supply".to_string(), unit_info.supply.clone());
    template_properties.insert("decay".to_string(), unit_info.decay.clone());
    template_properties.insert(
        "effective_attack".to_string(),
        unit_info.effective_attack.clone(),
    );
    template_properties.insert(
        "effective_defense".to_string(),
        unit_info.effective_defense.clone(),
    );
}

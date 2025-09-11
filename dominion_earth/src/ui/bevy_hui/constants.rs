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
    pub const LEFT_SIDE_TOP_PATH: &str = "ui/components/top_panel/left_side_top.html";
    pub const GAME_PANEL_PATH: &str = "ui/components/left_side_panel/game_panel.html";
    pub const PRODUCTION_MENU_PATH: &str = "ui/components/right_side_panel/production_menu.html";
    pub const STATISTICS_PANEL_PATH: &str = "ui/components/left_side_panel/statistics_panel.html";
    pub const TILE_INFO_PATH: &str = "ui/components/right_side_panel/tile_info.html";
    pub const CIVILIZATIONS_LIST_PATH: &str =
        "ui/components/right_side_panel/civilizations_list.html";
    pub const MINIMAP_PATH: &str = "ui/components/right_side_panel/minimap.html";
}

pub mod ui_component_names {
    pub const TOP_PANEL_NAME: &str = "top_panel";
    pub const RIGHT_SIDE_PANEL_NAME: &str = "right_side_panel";
    pub const LEFT_SIDE_PANEL_NAME: &str = "left_side_panel";
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

pub mod production_orders {
    pub const UNIT_INFANTRY_TYPE_NAME: &str = "Infantry";
    pub const UNIT_ARCHER_TYPE_NAME: &str = "Archer";
    pub const UNIT_CAVALRY_TYPE_NAME: &str = "Cavalry";
    pub const INSUFFICIENT_GOLD_DEBUG_PREFIX: &str = "Insufficient gold for";
    pub const PRODUCTION_ORDER_SUCCESS_SUFFIX: &str = "production order sent!";
    pub const UI_GOLD_UPDATE_DEBUG_PREFIX: &str = "UI updated with new gold:";
}

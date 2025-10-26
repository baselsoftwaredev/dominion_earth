pub mod colors {
    use bevy::prelude::Color;

    pub const BACKGROUND_DARK: Color = Color::srgba(0.102, 0.102, 0.102, 1.0);
    pub const BACKGROUND_MEDIUM: Color = Color::srgba(0.176, 0.176, 0.176, 1.0);
    pub const BACKGROUND_LIGHT: Color = Color::srgba(0.251, 0.251, 0.251, 1.0);
    pub const BORDER_DARK: Color = Color::srgba(0.267, 0.267, 0.267, 1.0);
    pub const BORDER_MEDIUM: Color = Color::srgba(0.4, 0.4, 0.4, 1.0);
    pub const TEXT_WHITE: Color = Color::WHITE;
    pub const TEXT_GOLD: Color = Color::srgba(1.0, 0.8, 0.0, 1.0);
    pub const TEXT_GRAY_LIGHT: Color = Color::srgba(0.8, 0.8, 0.8, 1.0);
    pub const TEXT_GRAY_MEDIUM: Color = Color::srgba(0.6, 0.6, 0.6, 1.0);
    pub const BUTTON_HOVER: Color = Color::srgba(0.251, 0.251, 0.251, 1.0);
    pub const BUTTON_PRESSED: Color = Color::srgba(0.0, 0.667, 0.667, 1.0);
    pub const SEPARATOR_COLOR: Color = Color::srgba(0.267, 0.267, 0.267, 1.0);
}

pub mod dimensions {
    use bevy::prelude::Val;

    pub const PANEL_PADDING: Val = Val::Px(10.0);
    pub const SECTION_PADDING: Val = Val::Px(15.0);
    pub const SECTION_MARGIN: Val = Val::Px(5.0);
    pub const BORDER_WIDTH: Val = Val::Px(2.0);
    pub const BORDER_RADIUS: Val = Val::Px(8.0);
    pub const BUTTON_BORDER_RADIUS: Val = Val::Px(5.0);

    pub const GAME_PANEL_MIN_HEIGHT: Val = Val::Px(200.0);
    pub const TITLE_MARGIN_BOTTOM: Val = Val::Px(15.0);
    pub const NEXT_TURN_BUTTON_HEIGHT: Val = Val::Px(50.0);
    pub const NEXT_TURN_BUTTON_PADDING: Val = Val::Px(10.0);
    pub const NEXT_TURN_BUTTON_MARGIN_BOTTOM: Val = Val::Px(10.0);

    pub const PRODUCTION_MENU_MAX_HEIGHT: Val = Val::Px(500.0);
    pub const INFO_ITEM_MARGIN_BOTTOM: Val = Val::Px(5.0);
    pub const SECTION_MARGIN_BOTTOM: Val = Val::Px(15.0);
    pub const SEPARATOR_MARGIN_VERTICAL: Val = Val::Px(10.0);
    pub const SEPARATOR_HEIGHT: Val = Val::Px(2.0);

    pub const UNIT_BUTTON_HEIGHT: Val = Val::Px(40.0);
    pub const UNIT_BUTTON_PADDING: Val = Val::Px(10.0);
    pub const UNIT_BUTTON_MARGIN_BOTTOM: Val = Val::Px(5.0);

    pub const INFO_CONTAINER_MARGIN_BOTTOM: Val = Val::Px(10.0);
}

pub mod font_sizes {
    pub const TITLE_SIZE: f32 = 20.0;
    pub const SECTION_HEADER_SIZE: f32 = 16.0;
    pub const BUTTON_TEXT_SIZE: f32 = 16.0;
    pub const INFO_TEXT_SIZE: f32 = 14.0;
    pub const SMALL_TEXT_SIZE: f32 = 12.0;
}

pub mod text_content {
    pub const GAME_PANEL_TITLE: &str = "Your Empire";
    pub const NEXT_TURN_BUTTON_TEXT: &str = "Next Turn";
    pub const PRODUCTION_MENU_TITLE: &str = "Production Menu";
    pub const AVAILABLE_UNITS_HEADER: &str = "Available Units:";
    pub const CURRENTLY_PRODUCING_HEADER: &str = "Currently Producing:";
    pub const PRODUCTION_QUEUE_HEADER: &str = "Production Queue:";
    pub const UNIT_INFO_TITLE: &str = "Unit Information";
    pub const CONTROLS_HINT: &str = "Press [Esc] to close | Click buttons to queue units";

    pub const CAPITAL_UNKNOWN: &str = "Capital: Unknown";
    pub const CIVILIZATION_UNKNOWN: &str = "Civilization: Unknown";
    pub const GOLD_ZERO: &str = "Gold: 0";
    pub const PRODUCTION_ZERO: &str = "Production: 0";
    pub const PRODUCTION_NONE: &str = "None";
    pub const PROGRESS_ZERO: &str = "Progress: 0%";
    pub const QUEUE_EMPTY: &str = "Items queued: 0";

    pub const TYPE_UNKNOWN: &str = "Type: Unknown";
    pub const HEALTH_ZERO: &str = "Health: 0/0";
    pub const STRENGTH_ZERO: &str = "Strength: 0";
    pub const MOVEMENT_ZERO: &str = "Movement: 0/0";

    pub const UNIT_INFANTRY: &str = "Infantry";
    pub const UNIT_ARCHER: &str = "Archer";
    pub const UNIT_CAVALRY: &str = "Cavalry";

    pub const INFANTRY_COST_TEXT: &str = "20 gold, 15 production";
    pub const ARCHER_COST_TEXT: &str = "25 gold, 20 production";
    pub const CAVALRY_COST_TEXT: &str = "40 gold, 30 production";
}

pub mod ui_names {
    pub const LEFT_PANEL: &str = "Left Panel";
    pub const GAME_PANEL: &str = "Game Panel";
    pub const GAME_PANEL_TITLE: &str = "Game Panel Title";
    pub const NEXT_TURN_BUTTON: &str = "Next Turn Button";
    pub const PRODUCTION_MENU_PANEL: &str = "Production Menu Panel";
    pub const PRODUCTION_MENU_TITLE: &str = "Production Menu Title";
    pub const CAPITAL_INFO_CONTAINER: &str = "Capital Info Container";
    pub const SEPARATOR: &str = "Separator";
    pub const AVAILABLE_UNITS: &str = "Available Units";
    pub const INFANTRY_BUTTON: &str = "Infantry Button";
    pub const ARCHER_BUTTON: &str = "Archer Button";
    pub const CAVALRY_BUTTON: &str = "Cavalry Button";
    pub const CURRENT_PRODUCTION: &str = "Current Production";
    pub const PRODUCTION_QUEUE: &str = "Production Queue";
    pub const UNIT_INFO_PANEL: &str = "Unit Info Panel";
    pub const UNIT_INFO_TITLE: &str = "Unit Info Title";
}

pub mod percentage_multiplier {
    pub const TO_PERCENTAGE: f32 = 100.0;
}

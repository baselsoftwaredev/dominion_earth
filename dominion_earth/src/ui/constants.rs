pub mod display_layout {
    pub const GAME_PANEL_MINIMUM_WIDTH: f32 = 300.0;
    pub const GAME_PANEL_MAXIMUM_WIDTH: f32 = 600.0;
    pub const GAME_PANEL_DEFAULT_WIDTH: f32 = 400.0;

    // UI Panel Layout Constants
    pub const HEADER_HEIGHT: f32 = 80.0;
    pub const LEFT_SIDEBAR_WIDTH: f32 = 300.0;
    pub const RIGHT_SIDEBAR_WIDTH: f32 = 300.0;
}

pub mod minimap_display {
    pub const MINIMAP_DEFAULT_WIDTH: f32 = 200.0;
    pub const MINIMAP_DEFAULT_HEIGHT: f32 = 100.0;
    pub const CAPITAL_MARKER_RADIUS: f32 = 3.0;
    pub const MAXIMUM_CIVILIZATIONS_DISPLAYED: usize = 10;
}

pub mod world_dimensions {
    pub const ASSUMED_WORLD_WIDTH: f32 = 100.0;
    pub const ASSUMED_WORLD_HEIGHT: f32 = 50.0;
}

pub mod color_values {
    pub const RGB_COLOR_CONVERSION_FACTOR: f32 = 255.0;
    pub const OCEAN_BACKGROUND_RED: u8 = 50;
    pub const OCEAN_BACKGROUND_GREEN: u8 = 100;
    pub const OCEAN_BACKGROUND_BLUE: u8 = 200;
}

pub mod scroll_behavior {
    pub const MOUSE_WHEEL_PIXELS_PER_LINE: f32 = 20.0;
}

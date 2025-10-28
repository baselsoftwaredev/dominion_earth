//! Constants for UI theme elements.

pub mod font_sizes {
    pub const HEADER_TEXT_SIZE: f32 = 48.0;
    pub const LABEL_TEXT_SIZE: f32 = 28.0;
    pub const BUTTON_TEXT_SIZE: f32 = 32.0;
}

pub mod button_dimensions {
    pub const STANDARD_BUTTON_WIDTH: f32 = 220.0;
    pub const STANDARD_BUTTON_HEIGHT: f32 = 55.0;
    pub const STANDARD_BUTTON_BORDER_WIDTH: f32 = 3.0;

    pub const SMALL_BUTTON_SIZE: f32 = 30.0;
}

pub mod layout {
    pub const UI_ROOT_ROW_GAP: f32 = 20.0;
    pub const FULL_SCREEN_PERCENTAGE: f32 = 100.0;
}

pub mod z_index {
    pub const MENU_OVERLAY_Z_INDEX: i32 = 100;
}

pub mod audio {
    pub const VOLUME_ADJUSTMENT_STEP: f32 = 0.1;
    pub const MINIMUM_VOLUME_LEVEL: f32 = 0.0;
    pub const MAXIMUM_VOLUME_LEVEL: f32 = 3.0;
}

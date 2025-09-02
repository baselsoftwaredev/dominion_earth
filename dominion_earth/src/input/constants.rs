pub mod camera {
    pub const ZOOM_STEP_SIZE: f32 = 0.1;
    pub const MINIMUM_ZOOM_SCALE: f32 = 0.5;
    pub const MAXIMUM_ZOOM_SCALE: f32 = 3.0;
}

pub mod movement {
    pub const MINIMUM_ADJACENT_DISTANCE: i32 = 1;
    pub const DEFAULT_MOVEMENT_COST: u32 = 1;
    pub const NO_MOVEMENT_REMAINING: u32 = 0;
}

pub mod unit_actions {
    pub const SKIP_TURN_MOVEMENT_REMAINING: u32 = 0;
}

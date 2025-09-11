use crate::debug_println;
use crate::debug_utils::DebugLogging;
use bevy::prelude::*;
use core_sim::{
    components::{Capital, City},
    Position,
};

/// Component to mark capital label entities (placeholder)
#[derive(Component)]
pub struct CapitalLabel {
    pub capital_entity: Entity,
}

/// System to spawn capital labels - currently disabled
pub fn spawn_capital_labels(debug_logging: Res<DebugLogging>) {
    debug_println!(
        debug_logging,
        "Capital labels system disabled - traditional Bevy UI approach pending"
    );
}

/// System to update capital labels - currently disabled
pub fn update_capital_labels() {
    // Capital labels disabled - traditional Bevy UI approach pending
}

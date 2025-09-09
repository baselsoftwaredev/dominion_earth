use bevy::prelude::*;
use crate::ui;

/// Plugin for UI system integration
pub struct UiIntegrationPlugin;

impl Plugin for UiIntegrationPlugin {
    fn build(&self, app: &mut App) {
        // Initialize UI system with bevy_hui
        ui::BevyHuiSystem::setup_plugins(app);
    }
}

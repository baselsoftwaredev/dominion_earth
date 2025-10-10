use crate::screens::Screen;
use crate::ui;
use bevy::prelude::*;

/// Plugin for UI system integration
pub struct UiIntegrationPlugin;

impl Plugin for UiIntegrationPlugin {
    fn build(&self, app: &mut App) {
        // Initialize UI system with bevy_hui
        // The UI setup is scoped to only display during gameplay
        ui::BevyHuiSystem::setup_plugins_for_screen(app, Screen::Gameplay);
    }
}

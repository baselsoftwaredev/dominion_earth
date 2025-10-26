use crate::screens::Screen;
use crate::ui;
use bevy::prelude::*;

/// Plugin for UI system integration
pub struct UiIntegrationPlugin;

impl Plugin for UiIntegrationPlugin {
    fn build(&self, app: &mut App) {
        // The UI setup is scoped to only display during gameplay
        ui::BevyUiSystem::setup_plugins_for_screen(app, Screen::Gameplay);
    }
}

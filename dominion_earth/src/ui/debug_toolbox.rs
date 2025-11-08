/// Debug toolbox UI - only enabled in debug builds
/// Provides development utilities as an egui window
///
/// Based on bevy-inspector-egui patterns for proper egui context handling
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPrimaryContextPass, PrimaryEguiContext};
use egui::Color32;

#[derive(Resource, Default)]
pub struct DebugToolboxState {
    pub is_visible: bool,
    pub test_checkbox: bool,
}

pub struct DebugToolboxPlugin;

impl Plugin for DebugToolboxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugToolboxState>()
            .add_systems(Startup, spawn_debug_toolbox)
            .add_systems(Update, toggle_debug_toolbox)
            .add_systems(EguiPrimaryContextPass, update_debug_toolbox);
    }
}

pub fn spawn_debug_toolbox(mut debug_state: ResMut<DebugToolboxState>) {
    debug_state.is_visible = true;
}

pub fn update_debug_toolbox(world: &mut World) {
    let debug_state = world.resource::<DebugToolboxState>();
    if !debug_state.is_visible {
        return;
    }

    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };

    let mut egui_context = egui_context.clone();

    egui::Window::new("Debug Toolbox")
        .default_open(true)
        .resizable(true)
        .collapsible(true)
        .show(egui_context.get_mut(), |ui| {
            // Title
            ui.heading("Development Tools");
            ui.separator();

            // Test Label
            ui.label("Test Label Placeholder:");
            ui.label("This is a debug toolbox window for development utilities");

            ui.separator();

            // Test Checkbox
            let mut debug_state = world.resource_mut::<DebugToolboxState>();
            ui.horizontal(|ui| {
                ui.label("Test Feature:");
                ui.checkbox(&mut debug_state.test_checkbox, "Enable test feature");
            });

            if debug_state.test_checkbox {
                ui.colored_label(Color32::GREEN, "✓ Test feature is enabled");
            }

            ui.separator();

            // Close button
            if ui.button("Close Toolbox").clicked() {
                let mut debug_state = world.resource_mut::<DebugToolboxState>();
                debug_state.is_visible = false;
            }
        });
}

pub fn toggle_debug_toolbox(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<DebugToolboxState>,
) {
    // Toggle toolbox with Cmd+D on macOS or Ctrl+D on other platforms
    #[cfg(target_os = "macos")]
    let is_modifier_pressed =
        keyboard.pressed(KeyCode::SuperLeft) || keyboard.pressed(KeyCode::SuperRight);

    #[cfg(not(target_os = "macos"))]
    let is_modifier_pressed =
        keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);

    if is_modifier_pressed && keyboard.just_pressed(KeyCode::KeyD) {
        debug_state.is_visible = !debug_state.is_visible;
    }
}

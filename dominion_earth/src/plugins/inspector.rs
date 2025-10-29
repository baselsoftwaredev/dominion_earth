use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy_egui::EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ));
    }
}

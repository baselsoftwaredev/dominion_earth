use bevy::prelude::*;

/// Plugin for menu and screen state management
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            crate::theme::plugin,
            crate::screens::plugin,
            crate::menus::plugin,
        ));
    }
}

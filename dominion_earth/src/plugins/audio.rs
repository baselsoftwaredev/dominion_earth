use bevy::prelude::*;

/// Plugin for audio management and playback
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::audio::plugin);
    }
}

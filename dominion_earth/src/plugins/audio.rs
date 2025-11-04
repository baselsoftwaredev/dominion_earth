use bevy::prelude::*;

/// Plugin for audio management and playback
/// This now delegates to our audio module which sets up bevy_kira_audio
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::audio::plugin);
    }
}

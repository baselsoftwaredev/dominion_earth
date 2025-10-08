use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

/// Helper function to play a one-shot sound effect.
///
/// # Example
/// ```rust
/// use crate::audio;
///
/// fn my_system(mut commands: Commands, asset_server: Res<AssetServer>) {
///     audio::play_sound_effect(&mut commands, &asset_server, "sounds/click.ogg");
/// }
/// ```
pub fn play_sound_effect(commands: &mut Commands, asset_server: &AssetServer, sound_path: &str) {
    commands.spawn(sound_effect(asset_server.load(sound_path)));
}

/// Helper function to play looping background music.
///
/// # Example
/// ```rust
/// use crate::audio;
///
/// fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>) {
///     audio::play_music(&mut commands, &asset_server, "music/background.ogg");
/// }
/// ```
pub fn play_music(commands: &mut Commands, asset_server: &AssetServer, music_path: &str) {
    commands.spawn(music(asset_server.load(music_path)));
}

/// Stop all music currently playing.
///
/// # Example
/// ```rust
/// fn stop_all_music(mut commands: Commands, music_query: Query<Entity, With<Music>>) {
///     crate::audio::stop_all_music(&mut commands, &music_query);
/// }
/// ```
pub fn stop_all_music(commands: &mut Commands, music_query: &Query<Entity, With<Music>>) {
    for entity in music_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Stop all sound effects currently playing.
pub fn stop_all_sound_effects(
    commands: &mut Commands,
    sound_query: &Query<Entity, With<SoundEffect>>,
) {
    for entity in sound_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

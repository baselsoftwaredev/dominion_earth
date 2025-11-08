use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin)
        .add_systems(Update, cleanup_expired_sound_effects);
}

/// An organizational marker component that should be added to a spawned audio instance if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// An organizational marker component that should be added to a spawned audio instance if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// Helper function to play a one-shot sound effect.
///
/// # Example
/// ```rust
/// use crate::audio;
///
/// fn my_system(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
///     audio::play_sound_effect(&mut commands, &asset_server, &audio, "sounds/click.ogg");
/// }
/// ```
pub fn play_sound_effect(
    commands: &mut Commands,
    asset_server: &AssetServer,
    audio: &Audio,
    sound_path: impl Into<String>,
) {
    let path = sound_path.into();
    let handle = asset_server.load(&path);
    audio.play(handle.clone()).with_volume(0.5);

    // Spawn a marker entity for tracking with automatic despawn after 5 seconds
    // (typical sound effect duration, prevents entity accumulation)
    commands.spawn((
        SoundEffect,
        Name::new(format!("SFX: {}", path)),
        DespawnMarker(Timer::from_seconds(5.0, TimerMode::Once)),
    ));
}

/// Marker component to automatically despawn an entity after a timer expires
#[derive(Component)]
pub struct DespawnMarker(pub Timer);

/// System to despawn sound effect marker entities when their timer expires
pub fn cleanup_expired_sound_effects(
    mut commands: Commands,
    mut sound_effects: Query<(Entity, &mut DespawnMarker)>,
    time: Res<Time>,
) {
    for (entity, mut despawn_marker) in &mut sound_effects {
        despawn_marker.0.tick(time.delta());
        if despawn_marker.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Helper function to play looping background music.
///
/// # Example
/// ```rust
/// use crate::audio;
///
/// fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
///     let (entity, handle) = audio::play_music(&mut commands, &asset_server, &audio, "music/background.ogg");
/// }
/// ```
pub fn play_music(
    commands: &mut Commands,
    asset_server: &AssetServer,
    audio: &Audio,
    music_path: impl Into<String>,
) -> (Entity, Handle<AudioInstance>) {
    let path = music_path.into();
    let handle = asset_server.load(&path);
    let instance_handle = audio
        .play(handle.clone())
        .looped()
        .with_volume(0.5)
        .handle();

    // Spawn a marker entity for tracking
    let entity = commands
        .spawn((Music, Name::new(format!("Music: {}", path))))
        .id();

    (entity, instance_handle)
}

/// Stop all music currently playing.
///
/// # Example
/// ```rust
/// fn stop_all_music(audio: Res<Audio>, mut commands: Commands, music_query: Query<Entity, With<Music>>) {
///     crate::audio::stop_all_music(&audio, &mut commands, &music_query);
/// }
/// ```
pub fn stop_all_music(
    audio: &Audio,
    commands: &mut Commands,
    music_query: &Query<Entity, With<Music>>,
) {
    audio.stop();
    for entity in music_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Stop all sound effects currently playing.
pub fn stop_all_sound_effects(
    audio: &Audio,
    commands: &mut Commands,
    sound_query: &Query<Entity, With<SoundEffect>>,
) {
    audio.stop();
    for entity in sound_query.iter() {
        commands.entity(entity).despawn();
    }
}

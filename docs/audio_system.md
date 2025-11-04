# Audio System Documentation

## Overview

The audio system provides a centralized way to manage sounds and music in Dominion Earth. It uses `bevy_kira_audio` (version 0.24) for high-quality audio playback with helper functions and marker components for better organization.

## Components

### `Music`

Marker component for background music and soundtrack. Music loops by default.

### `SoundEffect`

Marker component for sound effects (UI clicks, game events, etc.). Sound effects are one-shot by default.

## Helper Functions

### Playing Sounds

```rust
// Play a one-shot sound effect
audio::play_sound_effect(&mut commands, &asset_server, &audio, "sounds/click.ogg");

// Play looping background music (returns Entity for tracking)
let music_entity = audio::play_music(&mut commands, &asset_server, &audio, "music/background.ogg");
```

### Stopping Sounds

```rust
// Stop all music
audio::stop_all_music(&audio, &mut commands, &music_query);

// Stop all sound effects
audio::stop_all_sound_effects(&audio, &mut commands, &sound_effects_query);
```

### Advanced Usage

```rust
// Query for all music
fn my_system(music_query: Query<Entity, With<audio::Music>>) {
    for entity in music_query.iter() {
        // Do something with music entities
    }
}
```

## Volume Control

The system uses `bevy_kira_audio`'s Audio resource for volume control. Volume is stored in `GameSettings` and applied when playing audio.

Volume is controlled through the settings system and ranges from 0.0 to 1.0.

## Audio File Locations

- Sound effects: `assets/sounds/`
- Music: `assets/music/`

## Supported Formats

With `bevy_kira_audio`, the following formats are supported:

- OGG Vorbis (`.ogg`) - recommended for music and effects

## Examples

### UI Click Sound

```rust
fn on_button_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    audio::play_sound_effect(&mut commands, &asset_server, &audio, "sounds/click.ogg");
}
```

### Background Music

```rust
fn setup_menu_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    audio::play_music(&mut commands, &asset_server, &audio, "music/menu_theme.ogg");
}
```

### Combat Sound

```rust
fn on_unit_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    audio::play_sound_effect(&mut commands, &asset_server, &audio, "sounds/sword_clash.ogg");
}
```

## Integration

The audio plugin is automatically included in `DominionEarthPlugins`. The `bevy_kira_audio::AudioPlugin` is initialized through our audio module. No additional setup required.

## Migration from Bevy Audio

The system has been migrated from Bevy's built-in audio to `bevy_kira_audio` for better audio quality and reliability. Key changes:

- All audio playback functions now require the `Audio` resource parameter
- Volume control is managed through `GameSettings` rather than `GlobalVolume`
- Audio files use OGG Vorbis format for optimal compatibility

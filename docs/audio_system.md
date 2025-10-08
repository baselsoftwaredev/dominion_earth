# Audio System Documentation

## Overview

The audio system provides a centralized way to manage sounds and music in Dominion Earth. It uses Bevy's audio system with helper functions and marker components for better organization.

## Components

### `Music`

Marker component for background music and soundtrack. Music loops by default.

### `SoundEffect`

Marker component for sound effects (UI clicks, game events, etc.). Sound effects despawn after playing.

## Helper Functions

### Playing Sounds

```rust
// Play a one-shot sound effect (auto-despawns when finished)
audio::play_sound_effect(&mut commands, &asset_server, "sounds/click.ogg");

// Play looping background music
audio::play_music(&mut commands, &asset_server, "music/background.ogg");
```

### Stopping Sounds

```rust
// Stop all music
audio::stop_all_music(&mut commands, &music_query);

// Stop all sound effects
audio::stop_all_sound_effects(&mut commands, &sound_effects_query);
```

### Advanced Usage

```rust
// Spawn music with custom settings
commands.spawn(audio::music(asset_server.load("music/theme.ogg")));

// Spawn sound effect with custom settings
commands.spawn(audio::sound_effect(asset_server.load("sounds/explosion.ogg")));

// Query for all music
fn my_system(music_query: Query<Entity, With<audio::Music>>) {
    for entity in music_query.iter() {
        // Do something with music entities
    }
}
```

## Volume Control

The system automatically applies `GlobalVolume` changes to all playing audio. To change the global volume:

```rust
fn change_volume(mut global_volume: ResMut<GlobalVolume>) {
    global_volume.volume = 0.5; // 50% volume
}
```

## Audio File Locations

- Sound effects: `assets/sounds/`
- Music: `assets/music/`

## Examples

### UI Click Sound

```rust
fn on_button_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    audio::play_sound_effect(&mut commands, &asset_server, "sounds/click.ogg");
}
```

### Background Music

```rust
fn setup_menu_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    audio::play_music(&mut commands, &asset_server, "music/menu_theme.ogg");
}
```

### Combat Sound

```rust
fn on_unit_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    audio::play_sound_effect(&mut commands, &asset_server, "sounds/sword_clash.ogg");
}
```

## Integration

The audio plugin is automatically included in `DominionEarthPlugins`. No additional setup required.

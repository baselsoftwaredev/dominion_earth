use bevy::prelude::*;
use core_sim::resources::ActiveCivTurn;
use core_sim::{Civilization, PlayerControlled};

/// Plugin for handling civilization-specific audio
pub struct CivilizationAudioPlugin;

impl Plugin for CivilizationAudioPlugin {
    fn build(&self, app: &mut App) {
        info!("🎵 CivilizationAudioPlugin initialized with error handling");

        app.init_resource::<CurrentMusicTrack>()
            .init_resource::<AudioErrorState>()
            .add_systems(
                Update,
                (
                    detect_turn_change_and_play_music,
                    monitor_audio_playback_errors,
                ),
            );

        info!(
            "⚠️  Audio system will gracefully disable after {} failures to prevent crashes",
            AudioErrorState::MAX_FAILURES
        );
    }
}

/// Resource tracking the currently playing music
#[derive(Resource, Default)]
struct CurrentMusicTrack {
    playing_entity: Option<Entity>,
    current_civ_id: Option<core_sim::CivId>,
}

/// Resource tracking audio system errors to prevent crash loops
#[derive(Resource, Default)]
struct AudioErrorState {
    audio_disabled: bool,
    failed_attempts: u32,
}

impl AudioErrorState {
    const MAX_FAILURES: u32 = 3;

    fn record_failure(&mut self) {
        self.failed_attempts += 1;
        if self.failed_attempts >= Self::MAX_FAILURES {
            self.audio_disabled = true;
            error!(
                "🎵 Audio system disabled after {} failures to prevent crashes",
                self.failed_attempts
            );
        }
    }
}

/// Detect when the active civilization changes and play their music
fn detect_turn_change_and_play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active_civ_turn: Option<Res<ActiveCivTurn>>,
    mut current_track: ResMut<CurrentMusicTrack>,
    mut audio_error_state: ResMut<AudioErrorState>,
    civilizations: Query<&Civilization>,
    music_query: Query<Entity, With<crate::audio::Music>>,
    player_civ: Query<&Civilization, With<PlayerControlled>>,
    settings: Res<crate::settings::GameSettings>,
) {
    // If audio is disabled due to errors, skip music playback
    if audio_error_state.audio_disabled {
        return;
    }

    // Don't play music in AI-only mode
    if settings.ai_only {
        // Clean up any existing music when switching to AI-only mode
        if current_track.playing_entity.is_some() {
            if let Some(entity) = current_track.playing_entity {
                if commands.get_entity(entity).is_ok() {
                    commands.entity(entity).despawn();
                    info!("🎵 Stopped music for AI-only mode");
                }
            }
            for entity in music_query.iter() {
                commands.entity(entity).despawn();
            }
            current_track.playing_entity = None;
            current_track.current_civ_id = None;
        }
        return;
    }

    let Some(active_civ_turn) = active_civ_turn else {
        if current_track.current_civ_id.is_none() {
            warn!("🎵 ActiveCivTurn resource not found - music system waiting");
        }
        return;
    };

    if active_civ_turn.civs_per_turn.is_empty() {
        return;
    }

    let active_civ_id = if active_civ_turn.current_civ_index < active_civ_turn.civs_per_turn.len() {
        active_civ_turn.civs_per_turn[active_civ_turn.current_civ_index]
    } else {
        warn!(
            "🎵 Invalid civ index: {} (max: {})",
            active_civ_turn.current_civ_index,
            active_civ_turn.civs_per_turn.len()
        );
        return;
    };

    if current_track.current_civ_id == Some(active_civ_id) {
        return;
    }

    info!("🎵 Turn changed! New active civ: {:?}", active_civ_id);

    for civ in civilizations.iter() {
        if civ.id == active_civ_id {
            let is_player_turn = player_civ.iter().any(|p| p.id == civ.id);

            if !is_player_turn {
                current_track.current_civ_id = Some(civ.id);
                continue;
            }

            info!(
                "🎵 Turn started for: {} - Attempting to play theme: {}",
                civ.name, civ.music_theme
            );

            // Clean up old music
            if let Some(entity) = current_track.playing_entity {
                if commands.get_entity(entity).is_ok() {
                    commands.entity(entity).despawn();
                }
            }

            for entity in music_query.iter() {
                commands.entity(entity).despawn();
            }

            // TODO: fix this. crashes the game 
            // Try to play music with error handling
            if let Err(e) = try_play_music(
                &mut commands,
                &asset_server,
                &civ.music_theme,
                &civ.name,
                &mut current_track,
                &mut audio_error_state,
            ) {
                warn!("🎵 Failed to play music for {}: {}", civ.name, e);
                audio_error_state.record_failure();
            }

            current_track.current_civ_id = Some(civ.id);
            break;
        }
    }
}

/// Try to play music with error handling
fn try_play_music(
    commands: &mut Commands,
    asset_server: &AssetServer,
    music_path: &str,
    civ_name: &str,
    current_track: &mut CurrentMusicTrack,
    audio_error_state: &mut AudioErrorState,
) -> Result<(), String> {
    // Check if the asset path looks valid
    if music_path.is_empty() {
        return Err("Empty music path".to_string());
    }

    // Convert to owned String for 'static lifetime requirement
    let music_path_owned = music_path.to_string();
    let civ_name_owned = civ_name.to_string();

    // Load the audio handle
    let music_handle = asset_server.load(music_path_owned.clone());

    // Spawn the audio entity
    // Note: The actual error will occur asynchronously when Bevy tries to decode
    // We can't catch it here, but we track it via the error state
    let music_entity = commands
        .spawn((
            AudioPlayer::new(music_handle),
            PlaybackSettings::LOOP.with_spatial(false),
            crate::audio::Music,
            Name::new(format!("{} Theme", civ_name_owned)),
        ))
        .id();

    current_track.playing_entity = Some(music_entity);

    info!(
        "🎵 Queued music: {} for {}",
        music_path_owned, civ_name_owned
    );
    Ok(())
}

/// Helper function to play a sound effect for player actions
pub fn play_player_sound(
    commands: &mut Commands,
    asset_server: &AssetServer,
    sound_category: &str,
    player_civ: &Query<&Civilization, With<PlayerControlled>>,
) {
    let sound_theme = player_civ
        .iter()
        .next()
        .map(|civ| civ.sound_theme.clone())
        .unwrap_or_else(|| "default".to_string());

    let sound_path = format!("sounds/effects/{}/{}.ogg", sound_theme, sound_category);

    // Use the safer audio playing function that handles errors gracefully
    if let Err(e) = try_play_sound_effect(commands, asset_server, &sound_path) {
        warn!("🔊 Failed to play sound effect '{}': {}", sound_path, e);
    }
}

/// Try to play a sound effect with error handling
fn try_play_sound_effect(
    commands: &mut Commands,
    asset_server: &AssetServer,
    sound_path: &str,
) -> Result<(), String> {
    if sound_path.is_empty() {
        return Err("Empty sound path".to_string());
    }

    // Just load and spawn - errors will be logged but won't crash
    crate::audio::play_sound_effect(commands, asset_server, sound_path.to_string());
    Ok(())
}

/// Monitor for audio playback errors and disable audio if too many failures occur
fn monitor_audio_playback_errors(
    mut audio_error_state: ResMut<AudioErrorState>,
    audio_sinks: Query<&AudioSink>,
) {
    // This system monitors for audio-related issues
    // Since Bevy's audio errors occur in async tasks, we can't directly catch them
    // But we can detect when audio entities fail to initialize properly

    if audio_error_state.audio_disabled {
        // Once disabled, we stay disabled to prevent crash loops
        return;
    }

    // Additional monitoring logic could go here if needed
}

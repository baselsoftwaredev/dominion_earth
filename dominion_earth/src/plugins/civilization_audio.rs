use bevy::prelude::*;
use core_sim::resources::ActiveCivTurn;
use core_sim::{Civilization, PlayerControlled};

/// Plugin for handling civilization-specific audio
pub struct CivilizationAudioPlugin;

impl Plugin for CivilizationAudioPlugin {
    fn build(&self, app: &mut App) {
        info!("🎵 CivilizationAudioPlugin initialized");
        app.init_resource::<CurrentMusicTrack>()
            .add_systems(Update, (detect_turn_change_and_play_music,));
    }
}

/// Resource tracking the currently playing music
#[derive(Resource, Default)]
struct CurrentMusicTrack {
    playing_entity: Option<Entity>,
    current_civ_id: Option<core_sim::CivId>,
}

/// Detect when the active civilization changes and play their music
fn detect_turn_change_and_play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active_civ_turn: Option<Res<ActiveCivTurn>>,
    mut current_track: ResMut<CurrentMusicTrack>,
    civilizations: Query<&Civilization>,
    music_query: Query<Entity, With<crate::audio::Music>>,
    player_civ: Query<&Civilization, With<PlayerControlled>>,
) {
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
                "🎵 Turn started for: {} - Playing theme: {}",
                civ.name, civ.music_theme
            );

            if let Some(entity) = current_track.playing_entity {
                if commands.get_entity(entity).is_ok() {
                    commands.entity(entity).despawn();
                }
            }

            for entity in music_query.iter() {
                commands.entity(entity).despawn();
            }

            let music_handle = asset_server.load(&civ.music_theme);
            let music_entity = commands
                .spawn((
                    AudioPlayer::new(music_handle),
                    PlaybackSettings::LOOP,
                    crate::audio::Music,
                    Name::new(format!("{} Theme", civ.name)),
                ))
                .id();

            current_track.playing_entity = Some(music_entity);
            current_track.current_civ_id = Some(civ.id);

            info!("🎵 Now playing: {} theme for {}", civ.music_theme, civ.name);
            break;
        }
    }
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

    crate::audio::play_sound_effect(commands, asset_server, sound_path);
}

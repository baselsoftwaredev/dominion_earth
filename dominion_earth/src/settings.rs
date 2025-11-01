use bevy::audio::GlobalVolume;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const SETTINGS_FILENAME: &str = "settings.ron";
const SAVES_DIRECTORY: &str = "saves";

#[derive(Debug, Clone, Serialize, Deserialize, Resource, Reflect)]
#[reflect(Resource)]
pub struct GameSettings {
    pub volume: f32,
    pub seed: Option<u64>,
    pub ai_only: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            volume: crate::constants::audio::DEFAULT_MUSIC_VOLUME,
            seed: None,
            ai_only: false,
        }
    }
}

impl GameSettings {
    pub fn get_settings_path() -> PathBuf {
        PathBuf::from(format!("{}/{}", SAVES_DIRECTORY, SETTINGS_FILENAME))
    }

    pub fn load() -> Self {
        let path = Self::get_settings_path();

        match fs::read_to_string(&path) {
            Ok(contents) => match ron::from_str::<GameSettings>(&contents) {
                Ok(settings) => {
                    info!("✅ Loaded settings from {:?}", path);
                    settings
                }
                Err(e) => {
                    warn!(
                        "⚠️ Failed to parse settings file {:?}: {}. Using defaults.",
                        path, e
                    );
                    Self::default()
                }
            },
            Err(_) => {
                info!("ℹ️ Settings file not found at {:?}. Using defaults.", path);
                Self::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::get_settings_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create saves directory: {}", e))?;
        }

        let ron_string = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(&path, ron_string)
            .map_err(|e| format!("Failed to write settings file: {}", e))?;

        info!("💾 Saved settings to {:?}", path);
        Ok(())
    }
}

/// Game settings persistence plugin
pub struct SettingsPersistencePlugin;

impl Plugin for SettingsPersistencePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameSettings>()
            .add_systems(
                Startup,
                (load_settings_on_startup, apply_settings_on_startup).chain(),
            )
            .add_systems(Update, sync_volume_to_settings);
    }
}

/// Load settings on startup
fn load_settings_on_startup(mut commands: Commands) {
    crate::debug_println!("🔧 Loading game settings...");

    let settings = GameSettings::load();

    crate::debug_println!(
        "🔊 Loaded volume setting: {:.0}%",
        settings.volume * crate::constants::settings::PERCENTAGE_MULTIPLIER
    );

    if let Some(seed_value) = settings.seed {
        crate::debug_println!("🎲 Loaded seed setting: {}", seed_value);
    }

    crate::debug_println!(
        "🤖 AI-only mode: {}",
        if settings.ai_only {
            "enabled"
        } else {
            "disabled"
        }
    );

    commands.insert_resource(settings);
}

/// Apply loaded settings to the game
fn apply_settings_on_startup(settings: Res<GameSettings>, mut global_volume: ResMut<GlobalVolume>) {
    global_volume.volume = bevy::audio::Volume::Linear(settings.volume);
}

/// System to sync current global volume to the settings resource
pub fn sync_volume_to_settings(
    global_volume: Res<GlobalVolume>,
    mut settings: ResMut<GameSettings>,
) {
    if global_volume.is_changed() {
        settings.volume = global_volume.volume.to_linear();
    }
}

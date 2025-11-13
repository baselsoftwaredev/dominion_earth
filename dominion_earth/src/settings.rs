use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

const SETTINGS_KEY: &str = "game_settings";

#[derive(Debug, Clone, Serialize, Deserialize, Resource, Reflect)]
#[reflect(Resource)]
pub struct GameSettings {
    pub volume: f32,
    pub seed: Option<u64>,
    pub ai_only: bool,
    pub map_size: MapSize,
    pub num_civilizations: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect, PartialEq)]
pub enum MapSize {
    Small,
    Medium,
    Large,
    Huge,
}

impl Default for MapSize {
    fn default() -> Self {
        MapSize::Medium
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            volume: crate::constants::audio::DEFAULT_MUSIC_VOLUME,
            seed: None,
            ai_only: false,
            map_size: MapSize::Medium,
            num_civilizations: 2,
        }
    }
}

impl GameSettings {
    /// Minimum number of civilizations
    pub const MIN_CIVILIZATIONS: u32 = 1;
    /// Maximum number of civilizations
    pub const MAX_CIVILIZATIONS: u32 = 50;

    /// Clamp num_civilizations to valid range
    pub fn clamp_civilizations(&mut self) {
        self.num_civilizations = self
            .num_civilizations
            .clamp(Self::MIN_CIVILIZATIONS, Self::MAX_CIVILIZATIONS);
    }

    /// Load settings early (before Bevy app is created)
    /// This is used in main() to configure the initial app setup
    pub fn load_early() -> Self {
        // Create a temporary PkvStore for early loading
        match PkvStore::new("BaselSoftwareDev", "DominionEarth").get::<GameSettings>(SETTINGS_KEY) {
            Ok(mut settings) => {
                settings.clamp_civilizations();
                settings
            }
            Err(_) => Self::default(),
        }
    }

    pub fn load(pkv: &PkvStore) -> Self {
        match pkv.get::<GameSettings>(SETTINGS_KEY) {
            Ok(mut settings) => {
                info!("✅ Loaded settings from PkvStore");
                settings.clamp_civilizations();
                settings
            }
            Err(_) => {
                info!("ℹ️ Settings not found in PkvStore. Using defaults.");
                Self::default()
            }
        }
    }

    pub fn save(&self, pkv: &mut PkvStore) -> Result<(), String> {
        pkv.set(SETTINGS_KEY, self)
            .map_err(|e| format!("Failed to save settings: {}", e))?;

        info!("💾 Saved settings to PkvStore");
        Ok(())
    }
}

/// Game settings persistence plugin
pub struct SettingsPersistencePlugin;

impl Plugin for SettingsPersistencePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameSettings>()
            .register_type::<MapSize>()
            .insert_resource(PkvStore::new("BaselSoftwareDev", "DominionEarth"))
            .add_systems(
                Startup,
                (load_settings_on_startup, apply_settings_on_startup).chain(),
            );
    }
}

/// Load settings on startup
fn load_settings_on_startup(mut commands: Commands, pkv: Res<PkvStore>) {
    crate::debug_println!("🔧 Loading game settings...");

    let settings = GameSettings::load(&pkv);

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

    crate::debug_println!("🗺️ Map size: {:?}", settings.map_size);
    crate::debug_println!("👥 Number of civilizations: {}", settings.num_civilizations);

    commands.insert_resource(settings);
}

/// Apply loaded settings to the game
fn apply_settings_on_startup(settings: Res<GameSettings>, audio: Res<Audio>) {
    // bevy_kira_audio volume uses 0.0-1.0 range
    audio.set_volume(settings.volume);
}

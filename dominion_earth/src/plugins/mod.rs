// Plugin system for organizing Dominion Earth systems

pub mod audio;
pub mod camera;
pub mod core_simulation;
pub mod input_handling;
pub mod menu;
pub mod rendering;
pub mod resources;
pub mod save_load;
pub mod ui_integration;

pub use audio::AudioPlugin;
pub use camera::CameraPlugin;
pub use core_simulation::CoreSimulationPlugin;
pub use input_handling::InputHandlingPlugin;
pub use menu::MenuPlugin;
pub use rendering::RenderingPlugin;
pub use resources::{ResourcesPlugin, ResourcesPluginWithConfig};
pub use save_load::SaveLoadPlugin;
pub use ui_integration::UiIntegrationPlugin;

/// Collection of all game plugins for easy registration
pub struct DominionEarthPlugins;

impl bevy::app::PluginGroup for DominionEarthPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(ResourcesPlugin)
            .add(MenuPlugin)
            .add(CoreSimulationPlugin)
            .add(CameraPlugin)
            .add(RenderingPlugin)
            .add(InputHandlingPlugin)
            .add(AudioPlugin)
            .add(SaveLoadPlugin::default())
            .add(UiIntegrationPlugin)
    }
}

impl DominionEarthPlugins {
    /// Create plugins with specific configuration
    pub fn with_config(config: resources::ResourceConfig) -> DominionEarthPluginsWithConfig {
        DominionEarthPluginsWithConfig {
            config,
            save_directory: None,
        }
    }
}

/// DominionEarthPlugins configured with specific settings
pub struct DominionEarthPluginsWithConfig {
    config: resources::ResourceConfig,
    save_directory: Option<String>,
}

impl DominionEarthPluginsWithConfig {
    pub fn with_save_directory(mut self, save_dir: String) -> Self {
        self.save_directory = Some(save_dir);
        self
    }
}

impl bevy::app::PluginGroup for DominionEarthPluginsWithConfig {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let save_load_plugin = if let Some(save_dir) = self.save_directory {
            SaveLoadPlugin::with_save_directory(save_dir)
        } else {
            SaveLoadPlugin::default()
        };

        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(ResourcesPlugin::with_config(self.config))
            .add(MenuPlugin)
            .add(CoreSimulationPlugin)
            .add(CameraPlugin)
            .add(RenderingPlugin)
            .add(InputHandlingPlugin)
            .add(AudioPlugin)
            .add(save_load_plugin)
            .add(UiIntegrationPlugin)
    }
}

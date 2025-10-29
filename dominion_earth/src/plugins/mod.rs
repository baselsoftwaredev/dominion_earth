pub mod audio;
pub mod camera;
pub mod civilization_audio;
pub mod core_simulation;
pub mod input_handling;
pub mod inspector;
pub mod menu;
pub mod rendering;
pub mod resources;
pub mod save_load;
pub mod ui_integration;

pub use audio::AudioPlugin;
pub use camera::CameraPlugin;
pub use civilization_audio::CivilizationAudioPlugin;
pub use core_simulation::CoreSimulationPlugin;
pub use input_handling::InputHandlingPlugin;
pub use inspector::InspectorPlugin;
pub use menu::MenuPlugin;
pub use rendering::RenderingPlugin;
pub use resources::{ResourcesPlugin, ResourcesPluginWithConfig};
pub use save_load::SaveLoadPlugin;
pub use ui_integration::UiIntegrationPlugin;

pub struct DominionEarthPlugins;

impl bevy::app::PluginGroup for DominionEarthPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(crate::settings::SettingsPersistencePlugin)
            .add(ResourcesPlugin)
            .add(MenuPlugin)
            .add(CoreSimulationPlugin)
            .add(CameraPlugin)
            .add(RenderingPlugin)
            .add(InputHandlingPlugin)
            .add(AudioPlugin)
            .add(CivilizationAudioPlugin)
            .add(SaveLoadPlugin::default())
            .add(UiIntegrationPlugin)
    }
}

impl DominionEarthPlugins {
    pub fn with_config(config: resources::ResourceConfig) -> DominionEarthPluginsWithConfig {
        DominionEarthPluginsWithConfig { config }
    }
}

pub struct DominionEarthPluginsWithConfig {
    config: resources::ResourceConfig,
}

impl bevy::app::PluginGroup for DominionEarthPluginsWithConfig {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut builder = bevy::app::PluginGroupBuilder::start::<Self>()
            .add(crate::settings::SettingsPersistencePlugin)
            .add(ResourcesPlugin::with_config(self.config))
            .add(MenuPlugin)
            .add(CoreSimulationPlugin)
            .add(CameraPlugin)
            .add(RenderingPlugin)
            .add(InputHandlingPlugin)
            .add(AudioPlugin)
            .add(CivilizationAudioPlugin)
            .add(SaveLoadPlugin::default())
            .add(UiIntegrationPlugin);

        #[cfg(debug_assertions)]
        {
            builder = builder.add(InspectorPlugin);
        }

        builder
    }
}

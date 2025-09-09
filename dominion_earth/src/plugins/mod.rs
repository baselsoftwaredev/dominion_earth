// Plugin system for organizing Dominion Earth systems

pub mod camera;
pub mod core_simulation;
pub mod input_handling;
pub mod rendering;
pub mod resources;
pub mod ui_integration;

pub use camera::CameraPlugin;
pub use core_simulation::CoreSimulationPlugin;
pub use input_handling::InputHandlingPlugin;
pub use rendering::RenderingPlugin;
pub use resources::{ResourcesPlugin, ResourcesPluginWithConfig};
pub use ui_integration::UiIntegrationPlugin;

/// Collection of all game plugins for easy registration
pub struct DominionEarthPlugins;

impl bevy::app::PluginGroup for DominionEarthPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(ResourcesPlugin)
            .add(CoreSimulationPlugin)
            .add(CameraPlugin)
            .add(RenderingPlugin)
            .add(InputHandlingPlugin)
            .add(UiIntegrationPlugin)
    }
}

impl DominionEarthPlugins {
    /// Create plugins with specific configuration
    pub fn with_config(config: resources::ResourceConfig) -> DominionEarthPluginsWithConfig {
        DominionEarthPluginsWithConfig { config }
    }
}

/// DominionEarthPlugins configured with specific settings
pub struct DominionEarthPluginsWithConfig {
    config: resources::ResourceConfig,
}

impl bevy::app::PluginGroup for DominionEarthPluginsWithConfig {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(ResourcesPlugin::with_config(self.config))
            .add(CoreSimulationPlugin)
            .add(CameraPlugin)
            .add(RenderingPlugin)
            .add(InputHandlingPlugin)
            .add(UiIntegrationPlugin)
    }
}

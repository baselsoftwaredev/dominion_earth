use bevy::prelude::*;

#[derive(Resource)]
pub struct UnitAssets {
    pub ancient_infantry: Handle<Image>,
    // Add more unit types as needed
}

pub fn setup_unit_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let unit_assets = UnitAssets {
        ancient_infantry: asset_server.load("tiles/units/ancient_infantry.png"),
        // Add more unit types as needed
    };
    commands.insert_resource(unit_assets);
}

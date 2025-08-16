use bevy::prelude::*;

#[derive(Resource)]
pub struct TileAssets {
    pub sprite_sheet: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    // Sprite indices for different tile types
    pub plains_index: usize,
    pub ocean_index: usize,
    pub capital_ancient_index: usize,
    pub ancient_infantry_index: usize,
}

/// Load tile assets
pub fn setup_tile_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the sprite sheet
    let sprite_sheet = asset_server.load("tiles/sprite-sheet.png");

    // Create texture atlas layout
    // Assuming the sprite sheet has a grid layout - adjust these values based on your actual sprite sheet
    // For now, assuming 4x4 grid with 64x64 pixel tiles
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let tile_assets = TileAssets {
        sprite_sheet,
        texture_atlas_layout,
        // Define sprite indices - adjust these based on your actual sprite sheet layout
        plains_index: 0,           // First sprite in the sheet
        ocean_index: 16,           // Ocean sprite index (update as needed)
        capital_ancient_index: 2,  // Third sprite in the sheet
        ancient_infantry_index: 3, // Fourth sprite in the sheet
    };
    commands.insert_resource(tile_assets);
}

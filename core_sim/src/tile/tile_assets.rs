use crate::components::TerrainType;
use crate::tile::tile_components::TileAssetProvider;
use bevy::prelude::*;

#[derive(Resource)]
pub struct TileAssets {
    pub sprite_sheet: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    // Sprite indices for different tile types
    pub plains_index: usize,
    pub hills_index: usize,
    pub mountains_index: usize,
    pub forest_index: usize,
    pub desert_index: usize,
    pub coast_index: usize,
    pub shallow_coast_index: usize,
    pub ocean_index: usize,
    pub river_index: usize,

    // Simple coast variations (will be rotated/flipped as needed)
    pub coast_1_side_index: usize, // Index 8 - 1 side coast (ocean to south)
    pub coast_2_side_index: usize, // Index 9 - 2 side coast (ocean to east and south)
    pub coast_3_side_index: usize, // Index 1 - 3 side coast (ocean to north, east, south)
    pub island_index: usize,       // Island (ocean on all 4 sides)

    pub capital_ancient_index: usize,
    pub ancient_infantry_index: usize,
}

impl TileAssetProvider for TileAssets {
    fn get_index_for_terrain(&self, terrain: &TerrainType) -> u32 {
        match terrain {
            TerrainType::Plains => self.plains_index as u32,
            TerrainType::Hills => self.hills_index as u32,
            TerrainType::Mountains => self.mountains_index as u32,
            TerrainType::Forest => self.forest_index as u32,
            TerrainType::Desert => self.desert_index as u32,
            TerrainType::Coast => self.coast_index as u32,
            TerrainType::ShallowCoast => self.shallow_coast_index as u32,
            TerrainType::Ocean => self.ocean_index as u32,
            TerrainType::River => self.river_index as u32,
        }
    }

    fn get_coast_index(&self) -> u32 {
        self.coast_index as u32
    }
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
    // Texture atlas is 3 rows x 8 columns (24 total sprites)
    // For now, assuming 64x64 pixel tiles
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 8, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let tile_assets = TileAssets {
        sprite_sheet,
        texture_atlas_layout,
        // TODO: Update these indices to match your sprite sheet!
        plains_index: 0,         // Plains sprite index
        hills_index: 0,          // Hills sprite index
        mountains_index: 0,      // Mountains sprite index
        forest_index: 0,         // Forest sprite index
        desert_index: 0,         // Desert sprite index
        coast_index: 8,          // Coast sprite index (fallback)
        shallow_coast_index: 17, // Shallow coast sprite index
        ocean_index: 16,         // Ocean sprite index
        river_index: 0,          // River sprite index

        // Simple coast variations (rotated/flipped as needed)
        coast_1_side_index: 8, // 1 side coast (ocean to south pattern)
        coast_2_side_index: 9, // 2 side coast (ocean to east and south pattern)
        coast_3_side_index: 1, // 3 side coast (ocean to north, east, south pattern)
        island_index: 2,       // Island (ocean on all 4 sides) - TODO: Set actual index

        capital_ancient_index: 3,   // Capital sprite index
        ancient_infantry_index: 10, // Infantry sprite index - trying index 10
    };
    commands.insert_resource(tile_assets);
}

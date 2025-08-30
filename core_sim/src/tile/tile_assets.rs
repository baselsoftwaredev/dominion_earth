use crate::components::TerrainType;
use crate::constants::{sprite_indices, texture_atlas};
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
    let sprite_sheet = asset_server.load(texture_atlas::SPRITE_SHEET_PATH);

    // Create texture atlas layout
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(texture_atlas::TILE_SIZE_PIXELS, texture_atlas::TILE_SIZE_PIXELS),
        texture_atlas::ATLAS_COLUMNS,
        texture_atlas::ATLAS_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let tile_assets = TileAssets {
        sprite_sheet,
        texture_atlas_layout,
        plains_index: sprite_indices::PLAINS,
        hills_index: sprite_indices::HILLS,
        mountains_index: sprite_indices::MOUNTAINS,
        forest_index: sprite_indices::FOREST,
        desert_index: sprite_indices::DESERT,
        coast_index: sprite_indices::COAST_FALLBACK,
        shallow_coast_index: sprite_indices::SHALLOW_COAST,
        ocean_index: sprite_indices::OCEAN,
        river_index: sprite_indices::RIVER,

        // Coast variations
        coast_1_side_index: sprite_indices::COAST_1_SIDE,
        coast_2_side_index: sprite_indices::COAST_2_SIDE,
        coast_3_side_index: sprite_indices::COAST_3_SIDE,
        island_index: sprite_indices::ISLAND,

        capital_ancient_index: sprite_indices::CAPITAL_ANCIENT,
        ancient_infantry_index: sprite_indices::ANCIENT_INFANTRY,
    };
    commands.insert_resource(tile_assets);
}

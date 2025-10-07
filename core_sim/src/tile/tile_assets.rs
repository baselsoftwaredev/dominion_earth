use crate::components::TerrainType;
use crate::constants::{sprite_indices, texture_atlas};
use crate::tile::tile_components::TileAssetProvider;
use bevy::asset::Handle;
use bevy::prelude::*;

#[derive(Resource, Clone)]
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

/// Marker resource to track that we've started loading
#[derive(Resource)]
pub struct TileAssetsLoading {
    sprite_sheet: Handle<Image>,
}

/// System to load tile assets and insert as resource when dependencies are ready
pub fn load_tile_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    tile_assets: Option<Res<TileAssets>>,
    loading: Option<Res<TileAssetsLoading>>,
) {
    // Only run once - if TileAssets already exists, skip
    if tile_assets.is_some() {
        return;
    }

    // If not loading yet, start loading
    if loading.is_none() {
        println!(
            "Starting to load sprite sheet: {}",
            texture_atlas::SPRITE_SHEET_PATH
        );
        let sprite_sheet = asset_server.load(texture_atlas::SPRITE_SHEET_PATH);
        commands.insert_resource(TileAssetsLoading { sprite_sheet });
        return;
    }

    // Check if loaded
    let loading = loading.unwrap();
    if !asset_server.is_loaded_with_dependencies(&loading.sprite_sheet) {
        return; // Not ready yet, try again next frame
    }

    println!("Sprite sheet loaded! Creating TileAssets resource");

    // Create texture atlas layout
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(
            texture_atlas::TILE_SIZE_PIXELS,
            texture_atlas::TILE_SIZE_PIXELS,
        ),
        texture_atlas::ATLAS_COLUMNS,
        texture_atlas::ATLAS_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let tile_assets = TileAssets {
        sprite_sheet: loading.sprite_sheet.clone(),
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
    commands.remove_resource::<TileAssetsLoading>();
    println!("TileAssets resource inserted!");
}

impl FromWorld for TileAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let sprite_sheet = asset_server.load(texture_atlas::SPRITE_SHEET_PATH);

        let mut texture_atlas_layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(
                texture_atlas::TILE_SIZE_PIXELS,
                texture_atlas::TILE_SIZE_PIXELS,
            ),
            texture_atlas::ATLAS_COLUMNS,
            texture_atlas::ATLAS_ROWS,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        TileAssets {
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
        }
    }
}

/// Load tile assets (deprecated - use load_resource in App instead)
///
/// This function is kept for backward compatibility but the preferred approach
/// is to use `app.load_resource::<TileAssets>()` which ensures assets are fully
/// loaded before the resource is inserted.
#[deprecated(note = "Use app.load_resource::<TileAssets>() instead")]
pub fn setup_tile_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the sprite sheet
    let sprite_sheet = asset_server.load(texture_atlas::SPRITE_SHEET_PATH);

    // Create texture atlas layout
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(
            texture_atlas::TILE_SIZE_PIXELS,
            texture_atlas::TILE_SIZE_PIXELS,
        ),
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

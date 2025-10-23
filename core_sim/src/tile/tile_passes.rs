use crate::components::direction_names;
use crate::constants::{coordinates, tile_passes};
use crate::debug_utils::CoreDebugUtils;
use crate::tile::tile_components::{
    TileAssetProvider, TileCapabilities, TileContents, TileNeighbors, WorldTile,
};
use crate::{Position, TerrainType};
use bevy::prelude::{Commands, Entity, InheritedVisibility, Transform, ViewVisibility, Visibility};
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::TileFlip;

//=============================================================================
// TILE FLIP CONSTANTS
//=============================================================================
/// Geometric flip constants for coast tiles
/// These represent the specific transformations needed for tile sprites
pub const TILE_FLIP_NO_CHANGE: TileFlip = TileFlip {
    x: false,
    y: false,
    d: false,
};
pub const TILE_FLIP_VERTICAL_ONLY: TileFlip = TileFlip {
    x: false,
    y: true,
    d: false,
};
pub const TILE_FLIP_HORIZONTAL_ONLY: TileFlip = TileFlip {
    x: true,
    y: false,
    d: false,
};
pub const TILE_FLIP_BOTH_AXES: TileFlip = TileFlip {
    x: true,
    y: true,
    d: false,
};
pub const TILE_FLIP_DIAGONAL_ONLY: TileFlip = TileFlip {
    x: false,
    y: false,
    d: true,
};
pub const TILE_FLIP_DIAGONAL_AND_VERTICAL: TileFlip = TileFlip {
    x: false,
    y: true,
    d: true,
};
pub const TILE_FLIP_DIAGONAL_AND_HORIZONTAL: TileFlip = TileFlip {
    x: true,
    y: false,
    d: true,
};
pub const TILE_FLIP_ALL_AXES: TileFlip = TileFlip {
    x: true,
    y: true,
    d: true,
};

//=============================================================================
// WORLD TILE GENERATION - THREE-PASS SYSTEM
//=============================================================================
// This module handles world tile generation in three distinct passes:
// 1. SPAWN PASS: Create tile entities with basic terrain
// 2. NEIGHBOR PASS: Link tiles to their adjacent neighbors
// 3. COAST PASS: Convert land tiles to coast tiles when adjacent to ocean
//=============================================================================

/// **PASS 1: SPAWN TILES**
///
/// Creates the initial tile entities across the world map grid.
/// Each tile gets its base terrain type (Plains, Hills, Ocean, etc.)
/// and is positioned in the ECS tilemap system.
///
/// **What this does:**
/// - Iterates through every position on the world map
/// - Creates a tile entity for each position  
/// - Assigns the terrain type from the world map data
/// - Sets up visual components (texture, position, visibility)
/// - Stores tile entities in a 2D grid for later reference
pub fn spawn_world_tiles_pass(
    commands: &mut Commands,
    tilemap_id: TilemapId,
    tile_assets: &impl TileAssetProvider,
    world_map: &crate::resources::WorldMap,
    tile_storage: &mut TileStorage,
    tile_entities: &mut Vec<Vec<Entity>>, // 2D grid: [x][y] -> Entity
    terrain_types: &mut Vec<Vec<TerrainType>>, // 2D grid: [x][y] -> TerrainType
) {
    let map_dimensions = TilemapSize {
        x: world_map.width,
        y: world_map.height,
    };

    // Iterate through every position on the map grid
    for x_coord in 0..map_dimensions.x {
        for y_coord in 0..map_dimensions.y {
            let tile_position = TilePos {
                x: x_coord,
                y: y_coord,
            };
            let world_position = Position::new(x_coord as i32, y_coord as i32);

            // Get the terrain type for this position from the world map
            let terrain_at_position = world_map
                .get_tile(world_position)
                .map(|tile| tile.terrain.clone())
                .unwrap_or(TerrainType::Ocean); // Default to ocean if no data

            // Store terrain type in our working array
            terrain_types[x_coord as usize][y_coord as usize] = terrain_at_position.clone();

            // Get the sprite index for this terrain type
            let sprite_index = tile_assets.get_index_for_terrain(&terrain_at_position);

            // Create the tile entity with all necessary components
            let tile_entity = commands
                .spawn((
                    // Core tilemap components
                    TileBundle {
                        position: tile_position,
                        tilemap_id,
                        texture_index: TileTextureIndex(sprite_index),
                        ..Default::default()
                    },
                    // Game-specific components
                    WorldTile {
                        grid_pos: world_position,
                        terrain_type: terrain_at_position.clone(),
                        capabilities: TileCapabilities::from_terrain(&terrain_at_position),
                    },
                    // Track what entities are on this tile
                    TileContents::default(),
                    // Bevy required components for rendering and transforms
                    Transform::default(),
                    Visibility::Inherited,
                    InheritedVisibility::VISIBLE,
                    ViewVisibility::default(),
                ))
                .id();

            // Store the entity in our 2D grid for neighbor linking
            tile_entities[x_coord as usize][y_coord as usize] = tile_entity;
            tile_storage.set(&tile_position, tile_entity);
        }
    }
}

/// **PASS 2: LINK NEIGHBORS**
///
/// Connects each tile to its adjacent neighbors (North, South, East, West).
/// This creates a navigation graph that allows systems to easily traverse
/// between adjacent tiles.
///
/// **Coordinate System:**
/// - X increases going East (→)
/// - Y increases going North (↑)
/// - Map boundaries are handled safely (no out-of-bounds neighbors)
pub fn assign_tile_neighbors_pass(
    commands: &mut Commands,
    tile_entities: &Vec<Vec<Entity>>, // 2D grid of tile entities [x][y]
    map_dimensions: &TilemapSize,
) {
    // Process every tile position
    for x_coord in 0..map_dimensions.x {
        for y_coord in 0..map_dimensions.y {
            let current_tile = tile_entities[x_coord as usize][y_coord as usize];

            // Find neighbor entities in each direction (with boundary checks)
            let neighbor_to_north = if (y_coord + tile_passes::NEIGHBOR_OFFSET) < map_dimensions.y {
                Some(tile_entities[x_coord as usize][(y_coord + 1) as usize])
            } else {
                None // No neighbor beyond north edge
            };

            let neighbor_to_south = if y_coord > coordinates::MIN_COORDINATE as u32 {
                Some(tile_entities[x_coord as usize][(y_coord - 1) as usize])
            } else {
                None // No neighbor beyond south edge
            };

            let neighbor_to_east = if (x_coord + tile_passes::NEIGHBOR_OFFSET) < map_dimensions.x {
                Some(tile_entities[(x_coord + 1) as usize][y_coord as usize])
            } else {
                None // No neighbor beyond east edge
            };

            let neighbor_to_west = if x_coord > coordinates::MIN_COORDINATE as u32 {
                Some(tile_entities[(x_coord - 1) as usize][y_coord as usize])
            } else {
                None // No neighbor beyond west edge
            };

            // Attach the neighbor links to this tile
            commands.entity(current_tile).insert(TileNeighbors {
                north: neighbor_to_north,
                south: neighbor_to_south,
                east: neighbor_to_east,
                west: neighbor_to_west,
            });
        }
    }
}

/// **PASS 3: COAST CONVERSION**
///
/// Goes through land tiles and finds their ocean neighbors.
/// Depending on ocean neighbors, replaces with relevant coast tiles.
/// Follows user's exact specification: "go through land tiles and find its neighbours,
/// depending on its neighbour to ocean replace with with relevant coast tiles"
///
/// **Coast Tile Types:**
/// - 1-side coast (index 8): Land with ocean to the north only  
/// - 2-side coast (index 9): Land with ocean to both south and east  
/// - 3-side coast (index 1): Land with ocean to north, east, and south
/// - Island: Land completely surrounded by ocean
/// - Other patterns: Fall back to basic coast tile (can be rotated/flipped)
pub fn update_coast_tiles_pass(
    commands: &mut Commands,
    tile_assets: &impl TileAssetProvider,
    tile_entities: &Vec<Vec<Entity>>,
    terrain_types: &mut Vec<Vec<TerrainType>>,
    map_dimensions: &TilemapSize,
    world_map: &mut crate::resources::WorldMap,
) {
    // Collect all coast conversions first to avoid borrowing conflicts
    let mut coast_conversions = Vec::new();

    // Process every land tile one by one as requested by user
    for x_coord in 0..map_dimensions.x {
        for y_coord in 0..map_dimensions.y {
            let current_terrain = &terrain_types[x_coord as usize][y_coord as usize];

            // Only process land tiles (skip ocean)
            if is_land_tile(current_terrain) {
                // Find its ocean neighbors
                let ocean_neighbors =
                    detect_ocean_neighbors(x_coord, y_coord, terrain_types, map_dimensions);

                // If it has ocean neighbors, collect for coast conversion
                if ocean_neighbors.has_any_ocean() {
                    coast_conversions.push((
                        x_coord,
                        y_coord,
                        ocean_neighbors,
                        current_terrain.clone(),
                    ));
                }
            }
        }
    }

    // Now apply all the coast conversions
    for (x_coord, y_coord, ocean_neighbors, original_terrain) in coast_conversions {
        let current_tile_entity = tile_entities[x_coord as usize][y_coord as usize];

        convert_land_to_coast_tile(
            commands,
            tile_assets,
            current_tile_entity,
            x_coord,
            y_coord,
            &original_terrain,
            &ocean_neighbors,
            terrain_types,
            world_map,
        );
    }
}

/// **PASS 4: SHALLOW COAST CONVERSION**
///
/// Goes through ocean tiles and converts those with coast neighbors to shallow coast tiles.
/// Uses sprite index 17 as specified by the user.
pub fn update_shallow_coast_tiles_pass(
    commands: &mut Commands,
    tile_entities: &Vec<Vec<Entity>>,
    terrain_types: &mut Vec<Vec<TerrainType>>,
    map_dimensions: &TilemapSize,
    world_map: &mut crate::resources::WorldMap,
) {
    // Collect all shallow coast conversions first to avoid borrowing conflicts
    let mut shallow_coast_conversions = Vec::new();

    for x_coord in 0..map_dimensions.x {
        for y_coord in 0..map_dimensions.y {
            let current_terrain = &terrain_types[x_coord as usize][y_coord as usize];

            // Only process ocean tiles
            if matches!(current_terrain, TerrainType::Ocean) {
                // Check if this ocean tile has any coast neighbors
                let coast_neighbors =
                    detect_coast_neighbors(x_coord, y_coord, terrain_types, map_dimensions);

                // If ocean tile has coast neighbors, convert to shallow coast
                if coast_neighbors.has_any_coast() {
                    shallow_coast_conversions.push((x_coord, y_coord, current_terrain.clone()));
                }
            }
        }
    }

    // Now apply all the shallow coast conversions
    for (x_coord, y_coord, _original_terrain) in shallow_coast_conversions {
        let current_tile_entity = tile_entities[x_coord as usize][y_coord as usize];

        convert_ocean_to_shallow_coast_tile(
            commands,
            current_tile_entity,
            x_coord,
            y_coord,
            terrain_types,
            world_map,
        );
    }
}
/// Converts an ocean tile to a shallow coast tile with sprite index 17
fn convert_ocean_to_shallow_coast_tile(
    commands: &mut Commands,
    tile_entity: Entity,
    x_coord: u32,
    y_coord: u32,
    terrain_grid: &mut Vec<Vec<TerrainType>>,
    world_map: &mut crate::resources::WorldMap,
) {
    // Debug log the conversion using core debug utilities
    CoreDebugUtils::log_shallow_coast_conversion(x_coord, y_coord);

    // Update the tile entity with shallow coast components
    // Using sprite index 17 as specified by the user
    commands
        .entity(tile_entity)
        .insert(TileTextureIndex(17))
        .insert(WorldTile {
            grid_pos: Position::new(x_coord as i32, y_coord as i32),
            terrain_type: TerrainType::ShallowCoast,
            capabilities: TileCapabilities::water(), // Shallow coast is water, not buildable
        });

    // Keep our terrain grid synchronized
    terrain_grid[x_coord as usize][y_coord as usize] = TerrainType::ShallowCoast;

    // Keep the world map resource synchronized for UI display
    let world_position = Position::new(x_coord as i32, y_coord as i32);
    if let Some(map_tile) = world_map.get_tile_mut(world_position) {
        map_tile.terrain = TerrainType::ShallowCoast;
    }
}

//=============================================================================
// HELPER FUNCTIONS - Coast Generation Support
//=============================================================================

/// Container for coast neighbor detection results
#[derive(Debug)]
struct CoastNeighbors {
    north: bool, // Is there coast to the north?
    south: bool, // Is there coast to the south?
    east: bool,  // Is there coast to the east?
    west: bool,  // Is there coast to the west?
}

impl CoastNeighbors {
    /// Returns true if this tile has any coast neighbors
    fn has_any_coast(&self) -> bool {
        self.north || self.south || self.east || self.west
    }

    /// Returns a list of direction names where coast exists
    fn get_coast_direction_names(&self) -> Vec<&'static str> {
        let mut directions = Vec::new();
        if self.north {
            directions.push(direction_names::NORTH);
        }
        if self.south {
            directions.push(direction_names::SOUTH);
        }
        if self.east {
            directions.push(direction_names::EAST);
        }
        if self.west {
            directions.push(direction_names::WEST);
        }
        directions
    }

    /// Counts how many coast neighbors this tile has (1, 2, 3, or 4)
    fn count_coast_sides(&self) -> u32 {
        let mut count = 0;
        if self.north {
            count += 1;
        }
        if self.south {
            count += 1;
        }
        if self.east {
            count += 1;
        }
        if self.west {
            count += 1;
        }
        count
    }
}

/// Container for ocean neighbor detection results
#[derive(Debug)]
struct OceanNeighbors {
    north: bool, // Is there ocean to the north?
    south: bool, // Is there ocean to the south?
    east: bool,  // Is there ocean to the east?
    west: bool,  // Is there ocean to the west?
}

impl OceanNeighbors {
    /// Returns true if this tile has any ocean neighbors
    fn has_any_ocean(&self) -> bool {
        self.north || self.south || self.east || self.west
    }

    /// Returns a list of direction names where ocean exists
    fn get_ocean_direction_names(&self) -> Vec<&'static str> {
        let mut directions = Vec::new();
        if self.north {
            directions.push(direction_names::NORTH);
        }
        if self.south {
            directions.push(direction_names::SOUTH);
        }
        if self.east {
            directions.push(direction_names::EAST);
        }
        if self.west {
            directions.push(direction_names::WEST);
        }
        directions
    }

    /// Counts how many ocean neighbors this tile has (1, 2, 3, or 4)
    fn count_ocean_sides(&self) -> u32 {
        let mut count = 0;
        if self.north {
            count += 1;
        }
        if self.south {
            count += 1;
        }
        if self.east {
            count += 1;
        }
        if self.west {
            count += 1;
        }
        count
    }
}

/// Helper function to check if a terrain type is land (not ocean)
fn is_land_tile(terrain: &TerrainType) -> bool {
    !matches!(terrain, TerrainType::Ocean)
}

/// Checks all four cardinal directions around a tile position to detect coast neighbors
///
/// **Coordinate System Reminder:**
/// - North: y + 1 (up on screen)
/// - South: y - 1 (down on screen)  
/// - East: x + 1 (right on screen)
/// - West: x - 1 (left on screen)
fn detect_coast_neighbors(
    x_coord: u32,
    y_coord: u32,
    terrain_grid: &Vec<Vec<TerrainType>>,
    map_dimensions: &TilemapSize,
) -> CoastNeighbors {
    CoastNeighbors {
        north: is_coast_at_position(x_coord, y_coord + 1, terrain_grid, map_dimensions),
        south: is_coast_at_position(
            x_coord,
            y_coord.saturating_sub(1),
            terrain_grid,
            map_dimensions,
        ),
        east: is_coast_at_position(x_coord + 1, y_coord, terrain_grid, map_dimensions),
        west: is_coast_at_position(
            x_coord.saturating_sub(1),
            y_coord,
            terrain_grid,
            map_dimensions,
        ),
    }
}

/// Safely checks if there's coast at a specific position
/// Returns false for out-of-bounds positions (treats map edges as non-coast)
fn is_coast_at_position(
    x: u32,
    y: u32,
    terrain_grid: &Vec<Vec<TerrainType>>,
    map_dimensions: &TilemapSize,
) -> bool {
    // Check bounds first
    if x >= map_dimensions.x || y >= map_dimensions.y {
        return false; // Out of bounds = no coast
    }

    // Special case: y can underflow to very large number when subtracting
    if y > 1000 {
        // Reasonable check for underflow
        return false;
    }

    // Check if the terrain at this position is coast
    terrain_grid[x as usize][y as usize] == TerrainType::Coast
}

/// Checks all four cardinal directions around a tile position to detect ocean neighbors
///
/// **Coordinate System Reminder:**
/// - North: y + 1 (up on screen)
/// - South: y - 1 (down on screen)  
/// - East: x + 1 (right on screen)
/// - West: x - 1 (left on screen)
fn detect_ocean_neighbors(
    x_coord: u32,
    y_coord: u32,
    terrain_grid: &Vec<Vec<TerrainType>>,
    map_dimensions: &TilemapSize,
) -> OceanNeighbors {
    OceanNeighbors {
        north: is_ocean_at_position(x_coord, y_coord + 1, terrain_grid, map_dimensions),
        south: is_ocean_at_position(
            x_coord,
            y_coord.saturating_sub(1),
            terrain_grid,
            map_dimensions,
        ),
        east: is_ocean_at_position(x_coord + 1, y_coord, terrain_grid, map_dimensions),
        west: is_ocean_at_position(
            x_coord.saturating_sub(1),
            y_coord,
            terrain_grid,
            map_dimensions,
        ),
    }
}

/// Safely checks if there's ocean at a specific position
/// Returns false for out-of-bounds positions (treats map edges as non-ocean)
fn is_ocean_at_position(
    x: u32,
    y: u32,
    terrain_grid: &Vec<Vec<TerrainType>>,
    map_dimensions: &TilemapSize,
) -> bool {
    // Check bounds first
    if x >= map_dimensions.x || y >= map_dimensions.y {
        return false; // Out of bounds = no ocean
    }

    // Special case: y can underflow to very large number when subtracting
    if y > 1000 {
        // Reasonable check for underflow
        return false;
    }

    // Check if the terrain at this position is ocean
    terrain_grid[x as usize][y as usize] == TerrainType::Ocean
}

/// Combined function that determines both the coast sprite index and tile flip
/// based on ocean neighbor patterns. Uses ocean count as primary logic:
/// - 4 ocean sides = island sprite
/// - 3 ocean sides = 1-side coast sprite
/// - 2 ocean sides = 2-side coast sprite
/// - 1 ocean side = 1-side coast sprite
fn determine_coast_sprite_and_flip(
    ocean_neighbors: &OceanNeighbors,
    tile_assets: &impl TileAssetProvider,
) -> (u32, TileFlip) {
    let ocean_count = ocean_neighbors.count_ocean_sides();

    match ocean_count {
        4 => {
            // Island: Completely surrounded by ocean
            let island_sprite_index = tile_assets.get_index_for_terrain(&TerrainType::Plains); // Use island sprite if available
            (island_sprite_index, TILE_FLIP_NO_CHANGE)
        }
        3 => {
            // 3-side coast: Land with ocean on three sides
            let three_side_coast_index = 1; // Based on comment: "3-side coast (index 1)"

            // Determine flip based on which side is NOT ocean (the land connection)
            let tile_flip = if !ocean_neighbors.north {
                TILE_FLIP_DIAGONAL_ONLY // Land connection to north
            } else if !ocean_neighbors.east {
                TILE_FLIP_BOTH_AXES // Land connection to east
            } else if !ocean_neighbors.south {
                TILE_FLIP_ALL_AXES // Land connection to south
            } else if !ocean_neighbors.west {
                TILE_FLIP_NO_CHANGE // Land connection to west
            } else {
                TILE_FLIP_NO_CHANGE // Fallback
            };

            (three_side_coast_index, tile_flip)
        }
        2 => {
            // 2-side coast: Land with ocean on two sides
            // Sprite 9 is asymmetric/uneven, so we use specialized flip constants
            // that work better with asymmetric tiles instead of standard rotations
            let two_side_coast_index = 9; // Based on comment: "2-side coast (index 9)"

            let tile_flip = if ocean_neighbors.north && ocean_neighbors.east {
                // Northeast corner: vertical flip moves South border to North
                TILE_FLIP_VERTICAL_ONLY
            } else if ocean_neighbors.east && ocean_neighbors.south {
                // Southeast corner: base orientation (matches sprite 9 design)
                TILE_FLIP_NO_CHANGE
            } else if ocean_neighbors.south && ocean_neighbors.west {
                // Southwest corner: horizontal flip moves East border to West
                TILE_FLIP_HORIZONTAL_ONLY
            } else if ocean_neighbors.west && ocean_neighbors.north {
                // Northwest corner: both flips move East→West and South→North
                TILE_FLIP_BOTH_AXES
            } else if ocean_neighbors.north && ocean_neighbors.south {
                // North-South strait: symmetric case, use no flip
                TILE_FLIP_NO_CHANGE
            } else if ocean_neighbors.east && ocean_neighbors.west {
                // East-West strait: symmetric case, use diagonal flip
                TILE_FLIP_DIAGONAL_ONLY
            } else {
                TILE_FLIP_NO_CHANGE // Fallback for unexpected patterns
            };

            (two_side_coast_index, tile_flip)
        }
        1 => {
            // 1-side coast: Land with ocean on one side
            let one_side_coast_index = 8; // Based on comment: "1-side coast (index 8)"

            // Determine flip based on which direction has ocean
            let tile_flip = if ocean_neighbors.north {
                TILE_FLIP_BOTH_AXES // Ocean to north, coast faces north
            } else if ocean_neighbors.east {
                TILE_FLIP_DIAGONAL_ONLY // Ocean to east, coast faces east
            } else if ocean_neighbors.south {
                TILE_FLIP_NO_CHANGE // Ocean to south, coast faces south (default)
            } else if ocean_neighbors.west {
                TILE_FLIP_ALL_AXES // Ocean to west, coast faces west
            } else {
                TILE_FLIP_NO_CHANGE // Fallback
            };

            (one_side_coast_index, tile_flip)
        }
        _ => {
            // Fallback: No ocean neighbors (shouldn't happen in coast conversion)
            let default_coast_index = 8;
            (default_coast_index, TILE_FLIP_NO_CHANGE)
        }
    }
}

/// Converts a land tile to a coast tile with the appropriate sprite
fn convert_land_to_coast_tile(
    commands: &mut Commands,
    tile_assets: &impl TileAssetProvider,
    tile_entity: Entity,
    x_coord: u32,
    y_coord: u32,
    original_terrain: &TerrainType,
    ocean_neighbors: &OceanNeighbors,
    terrain_grid: &mut Vec<Vec<TerrainType>>,
    world_map: &mut crate::resources::WorldMap,
) {
    // Debug log the conversion using core debug utilities
    let ocean_direction_names = ocean_neighbors.get_ocean_direction_names();
    CoreDebugUtils::log_coast_conversion(
        x_coord,
        y_coord,
        original_terrain,
        &ocean_direction_names,
    );

    // Determine both coast sprite index and flip settings in one call
    let (coast_sprite_index, tile_flip) =
        determine_coast_sprite_and_flip(ocean_neighbors, tile_assets);

    CoreDebugUtils::log_coast_sprite_selection(coast_sprite_index);
    CoreDebugUtils::log_tile_flip(tile_flip.x, tile_flip.y, tile_flip.d);

    // Update the tile entity with coast components
    commands
        .entity(tile_entity)
        .insert(TileTextureIndex(coast_sprite_index))
        .insert(tile_flip)
        .insert(WorldTile {
            grid_pos: Position::new(x_coord as i32, y_coord as i32),
            terrain_type: TerrainType::Coast,
            capabilities: TileCapabilities::coastal(), // Coast tiles are buildable (converted from land)
        });

    // Keep our terrain grid synchronized
    terrain_grid[x_coord as usize][y_coord as usize] = TerrainType::Coast;

    // Keep the world map resource synchronized for UI display
    let world_position = Position::new(x_coord as i32, y_coord as i32);
    if let Some(map_tile) = world_map.get_tile_mut(world_position) {
        map_tile.terrain = TerrainType::Coast;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultViewPoint {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// Component that defines what capabilities a tile has
/// This is set based on the original terrain before any conversions (like coast conversion)
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct TileCapabilities {
    pub buildable: bool,      // Can build cities, capitals, improvements
    pub walkable: bool,       // Can units move through this tile
    pub naval: bool,          // Can naval units move through this tile
    pub resource_yield: bool, // Can produce resources
}

impl TileCapabilities {
    /// Creates capabilities for land-based terrain types
    pub fn land() -> Self {
        Self {
            buildable: true,
            walkable: true,
            naval: false,
            resource_yield: true,
        }
    }

    /// Creates capabilities for water-based terrain types
    pub fn water() -> Self {
        Self {
            buildable: false,
            walkable: false,
            naval: true,
            resource_yield: false,
        }
    }

    /// Creates capabilities for coastal terrain (converted from land)
    pub fn coastal() -> Self {
        Self {
            buildable: true,      // Still buildable since it was originally land
            walkable: true,       // Still walkable for land units
            naval: true,          // Also accessible to naval units
            resource_yield: true, // Can still produce resources
        }
    }

    /// Creates capabilities for mountainous terrain
    pub fn mountains() -> Self {
        Self {
            buildable: false, // Can't build on mountains
            walkable: true,   // Can move through but with movement penalty
            naval: false,
            resource_yield: true, // Mountains can have resources
        }
    }

    /// Determines capabilities based on the original terrain type (before any conversions)
    pub fn from_terrain(terrain: &TerrainType) -> Self {
        use crate::TerrainType::*;
        match terrain {
            Plains | Hills | Forest | Desert => Self::land(),
            Mountains => Self::mountains(),
            Ocean | ShallowCoast => Self::water(),
            Coast => Self::coastal(), // Coast tiles were originally land, so they remain buildable
            River => Self::water(),   // Rivers are not buildable
        }
    }
}
/// System to update tile asset index when terrain changes
pub fn update_tile_asset_on_terrain_change(
    mut events: EventReader<TileTerrainChanged>,
    mut query: Query<(&mut TileTextureIndex, &mut WorldTile)>,
    tile_assets: Option<Res<TileAssets>>,
) {
    // Wait for TileAssets to be loaded
    let Some(tile_assets) = tile_assets else {
        return;
    };

    for event in events.read() {
        if let Ok((mut texture_index, mut world_tile)) = query.get_mut(event.entity) {
            let new_index = tile_assets.get_index_for_terrain(&event.new_terrain);
            texture_index.0 = new_index;
            world_tile.terrain_type = event.new_terrain.clone();
        }
    }
}
use bevy::prelude::*;

/// Event emitted when a tile's terrain type changes
#[derive(Debug, Clone, Event)]
pub struct TileTerrainChanged {
    pub entity: Entity,
    pub new_terrain: TerrainType,
}
use crate::resources::WorldMap;
use crate::tile::tile_assets::TileAssets;
use crate::tile::tile_passes::{
    assign_tile_neighbors_pass, spawn_world_tiles_pass, update_coast_tiles_pass,
    update_shallow_coast_tiles_pass,
};
use bevy::prelude::{Component, Entity};
use bevy_ecs_tilemap::prelude::*;

/// System to setup world tiles, assign terrain, and link neighbors
pub fn setup_world_tiles(
    commands: &mut Commands,
    tilemap_id: TilemapId,
    tile_assets: &impl TileAssetProvider,
    world_map: &mut WorldMap,
) -> TileStorage {
    let map_size = TilemapSize {
        x: world_map.width,
        y: world_map.height,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let mut tile_entities =
        vec![vec![Entity::PLACEHOLDER; map_size.y as usize]; map_size.x as usize];
    let mut terrain_types =
        vec![vec![TerrainType::Ocean; map_size.y as usize]; map_size.x as usize];

    spawn_world_tiles_pass(
        commands,
        tilemap_id,
        tile_assets,
        world_map,
        &mut tile_storage,
        &mut tile_entities,
        &mut terrain_types,
    );
    assign_tile_neighbors_pass(commands, &tile_entities, &map_size);
    update_coast_tiles_pass(
        commands,
        tile_assets,
        &tile_entities,
        &mut terrain_types,
        &map_size,
        world_map,
    );
    update_shallow_coast_tiles_pass(
        commands,
        &tile_entities,
        &mut terrain_types,
        &map_size,
        world_map,
    );

    tile_storage
}

/// Trait to abstract asset index lookup for core_sim
pub trait TileAssetProvider {
    fn get_index_for_terrain(&self, terrain: &TerrainType) -> u32;
    fn get_coast_index(&self) -> u32;
}
use crate::{CivId, Position, TerrainType};

#[derive(Component)]
pub struct WorldTile {
    pub grid_pos: Position,
    pub terrain_type: TerrainType,
    pub capabilities: TileCapabilities, // Added capabilities based on original terrain
}

#[derive(Component, Clone)]
pub struct TileNeighbors {
    pub north: Option<Entity>,
    pub south: Option<Entity>,
    pub east: Option<Entity>,
    pub west: Option<Entity>,
}

/// Component to track what entities are currently on this tile
#[derive(Component, Default)]
pub struct TileContents {
    pub units: Vec<Entity>,
    pub buildings: Vec<Entity>,
    pub capitals: Vec<Entity>,
    pub cities: Vec<Entity>,
}

impl TileContents {
    pub fn add_unit(&mut self, entity: Entity) {
        if !self.units.contains(&entity) {
            self.units.push(entity);
        }
    }

    pub fn add_building(&mut self, entity: Entity) {
        if !self.buildings.contains(&entity) {
            self.buildings.push(entity);
        }
    }

    pub fn add_capital(&mut self, entity: Entity) {
        if !self.capitals.contains(&entity) {
            self.capitals.push(entity);
        }
    }

    pub fn add_city(&mut self, entity: Entity) {
        if !self.cities.contains(&entity) {
            self.cities.push(entity);
        }
    }

    pub fn remove_unit(&mut self, entity: Entity) {
        self.units.retain(|&e| e != entity);
    }

    pub fn remove_building(&mut self, entity: Entity) {
        self.buildings.retain(|&e| e != entity);
    }

    pub fn remove_capital(&mut self, entity: Entity) {
        self.capitals.retain(|&e| e != entity);
    }

    pub fn remove_city(&mut self, entity: Entity) {
        self.cities.retain(|&e| e != entity);
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
            && self.buildings.is_empty()
            && self.capitals.is_empty()
            && self.cities.is_empty()
    }

    pub fn has_capital(&self) -> bool {
        !self.capitals.is_empty()
    }

    pub fn has_city(&self) -> bool {
        !self.cities.is_empty()
    }

    pub fn has_units(&self) -> bool {
        !self.units.is_empty()
    }
}

#[derive(Component)]
pub struct UnitSprite {
    pub unit_entity: Entity,
}

#[derive(Component)]
pub struct CapitalSprite {
    pub civ_id: CivId,
}

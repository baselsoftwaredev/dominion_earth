use bevy::prelude::*;
use core_sim::*;

#[derive(Component)]
pub struct WorldTile {
    pub grid_pos: Position,
    pub terrain_type: TerrainType,
}

#[derive(Component, Clone)]
pub struct TileNeighbors {
    pub north: Option<Entity>,
    pub south: Option<Entity>,
    pub east: Option<Entity>,
    pub west: Option<Entity>,
}

#[derive(Component)]
pub struct UnitSprite {
    pub unit_entity: Entity,
}

#[derive(Component)]
pub struct CapitalSprite {
    pub civ_id: CivId,
}

// TODO: Replace with bevy_hui implementation
use bevy::prelude::*;
use core_sim::components::{Position, TerrainType};

#[derive(Resource, Default, Clone)]
pub struct SelectedTile {
    pub position: Option<Position>,
}

#[derive(Resource, Default, Clone)]
pub struct HoveredTile {
    pub position: Option<Position>,
    pub terrain_type: Option<TerrainType>,
}

#[derive(Resource, Default, Clone)]
pub struct LastLoggedTile {
    pub position: Option<Position>,
}

#[derive(Resource, Default, Clone)]
pub struct TerrainCounts {
    pub plains: usize,
    pub hills: usize,
    pub forest: usize,
    pub ocean: usize,
    pub coast: usize,
    pub mountains: usize,
    pub desert: usize,
    pub river: usize,
}

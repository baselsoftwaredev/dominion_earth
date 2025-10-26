use bevy::prelude::Reflect;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use moonshine_save::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{CivId, Position};

/// Visibility state of a tile for a specific civilization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum VisibilityState {
    /// Never seen before - completely hidden
    Unexplored,
    /// Seen before but no units nearby - dimmed/desaturated
    Explored,
    /// Currently visible - full brightness
    Visible,
}

impl Default for VisibilityState {
    fn default() -> Self {
        VisibilityState::Unexplored
    }
}

/// Component that marks an entity as providing vision (units, cities)
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[require(Save)]
pub struct ProvidesVision {
    pub range: i32,
}

impl ProvidesVision {
    /// Standard vision range for military units (2 tiles)
    pub fn unit_vision() -> Self {
        Self { range: 2 }
    }

    /// Extended vision range for cities (3 tiles)
    pub fn city_vision() -> Self {
        Self { range: 3 }
    }
}

/// Per-civilization visibility map
/// Tracks which tiles are Unexplored, Explored, or Visible
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct VisibilityMap {
    /// Map dimensions
    pub width: u32,
    pub height: u32,
    /// Visibility state for each tile (stored as [x][y])
    pub tiles: Vec<Vec<VisibilityState>>,
}

impl VisibilityMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![VisibilityState::Unexplored; height as usize]; width as usize],
        }
    }

    /// Get visibility state at a position
    pub fn get(&self, pos: Position) -> Option<VisibilityState> {
        if pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height {
            Some(self.tiles[pos.x as usize][pos.y as usize])
        } else {
            None
        }
    }

    /// Set visibility state at a position
    pub fn set(&mut self, pos: Position, state: VisibilityState) {
        if pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height {
            self.tiles[pos.x as usize][pos.y as usize] = state;
        }
    }

    /// Check if a position is visible (for filtering AI decisions)
    pub fn is_visible(&self, pos: Position) -> bool {
        matches!(self.get(pos), Some(VisibilityState::Visible))
    }

    /// Check if a position has been explored (visible or explored)
    pub fn is_explored(&self, pos: Position) -> bool {
        matches!(
            self.get(pos),
            Some(VisibilityState::Visible) | Some(VisibilityState::Explored)
        )
    }

    /// Reset all Visible tiles to Explored (called at turn start before recalculating)
    pub fn reset_visibility(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.tiles[x as usize][y as usize] == VisibilityState::Visible {
                    self.tiles[x as usize][y as usize] = VisibilityState::Explored;
                }
            }
        }
    }

    /// Mark a position and surrounding tiles as visible (circular range using Chebyshev distance)
    pub fn mark_visible(&mut self, center: Position, range: i32) {
        for dx in -range..=range {
            for dy in -range..=range {
                let pos = Position::new(center.x + dx, center.y + dy);
                // Chebyshev distance: max(|dx|, |dy|)
                if dx.abs().max(dy.abs()) <= range {
                    if pos.x >= 0
                        && pos.y >= 0
                        && (pos.x as u32) < self.width
                        && (pos.y as u32) < self.height
                    {
                        self.tiles[pos.x as usize][pos.y as usize] = VisibilityState::Visible;
                    }
                }
            }
        }
    }
}

/// Resource that stores visibility maps for all civilizations
/// Uses Vec instead of HashMap to support reflection-based serialization
#[derive(Debug, Clone, Resource, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct FogOfWarMaps {
    pub maps: Vec<(CivId, VisibilityMap)>,
}

impl FogOfWarMaps {
    pub fn new() -> Self {
        Self { maps: Vec::new() }
    }

    /// Initialize visibility map for a civilization
    pub fn init_for_civ(&mut self, civ_id: CivId, width: u32, height: u32) {
        // Remove existing entry if present
        self.maps.retain(|(id, _)| *id != civ_id);
        // Add new entry
        self.maps.push((civ_id, VisibilityMap::new(width, height)));
    }

    /// Get visibility map for a civilization
    pub fn get(&self, civ_id: CivId) -> Option<&VisibilityMap> {
        self.maps
            .iter()
            .find(|(id, _)| *id == civ_id)
            .map(|(_, map)| map)
    }

    /// Get mutable visibility map for a civilization
    pub fn get_mut(&mut self, civ_id: CivId) -> Option<&mut VisibilityMap> {
        self.maps
            .iter_mut()
            .find(|(id, _)| *id == civ_id)
            .map(|(_, map)| map)
    }

    /// Check if a position is visible to a civilization
    pub fn is_visible_to(&self, civ_id: CivId, pos: Position) -> bool {
        self.get(civ_id)
            .map(|map| map.is_visible(pos))
            .unwrap_or(false)
    }

    /// Check if a position has been explored by a civilization
    pub fn is_explored_by(&self, civ_id: CivId, pos: Position) -> bool {
        self.get(civ_id)
            .map(|map| map.is_explored(pos))
            .unwrap_or(false)
    }
}

impl Default for FogOfWarMaps {
    fn default() -> Self {
        Self::new()
    }
}

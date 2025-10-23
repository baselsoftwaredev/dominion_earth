use bevy::prelude::Reflect;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Position component for entities on the world map
///
/// This is a "Model" component - critical game state that should be saved.
/// Note: Add the `Save` component manually when spawning positioned entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// Manual Component implementation to avoid proc macro issues
impl Component for Position {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Manhattan distance for 4-directional movement
    pub fn manhattan_distance_to(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Get adjacent positions in 4 directions (North, South, East, West)
    pub fn adjacent_positions(&self) -> [Position; 4] {
        use crate::components::direction_offsets;
        [
            Position::new(
                self.x + direction_offsets::NORTH.0,
                self.y + direction_offsets::NORTH.1,
            ), // North
            Position::new(
                self.x + direction_offsets::SOUTH.0,
                self.y + direction_offsets::SOUTH.1,
            ), // South
            Position::new(
                self.x + direction_offsets::EAST.0,
                self.y + direction_offsets::EAST.1,
            ), // East
            Position::new(
                self.x + direction_offsets::WEST.0,
                self.y + direction_offsets::WEST.1,
            ), // West
        ]
    }

    /// Get position in a specific direction
    pub fn in_direction(&self, direction: Direction) -> Position {
        use crate::components::direction_offsets;
        match direction {
            Direction::North => Position::new(
                self.x + direction_offsets::NORTH.0,
                self.y + direction_offsets::NORTH.1,
            ),
            Direction::South => Position::new(
                self.x + direction_offsets::SOUTH.0,
                self.y + direction_offsets::SOUTH.1,
            ),
            Direction::East => Position::new(
                self.x + direction_offsets::EAST.0,
                self.y + direction_offsets::EAST.1,
            ),
            Direction::West => Position::new(
                self.x + direction_offsets::WEST.0,
                self.y + direction_offsets::WEST.1,
            ),
        }
    }
}

/// Cardinal directions for movement and positioning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

// Manual Component implementation to avoid proc macro issues
impl Component for Direction {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Direction constants for easy access
pub mod directions {
    use super::Direction;

    pub const NORTH: Direction = Direction::North;
    pub const SOUTH: Direction = Direction::South;
    pub const EAST: Direction = Direction::East;
    pub const WEST: Direction = Direction::West;

    /// All directions in array form
    pub const ALL: [Direction; 4] = [NORTH, SOUTH, EAST, WEST];
}

/// Direction names as strings
pub mod direction_names {
    pub const NORTH: &str = "North";
    pub const SOUTH: &str = "South";
    pub const EAST: &str = "East";
    pub const WEST: &str = "West";

    /// All direction names in array form
    pub const ALL: [&str; 4] = [NORTH, SOUTH, EAST, WEST];
}

/// Direction offset coordinates
pub mod direction_offsets {
    /// North: y increases (up on screen)
    pub const NORTH: (i32, i32) = (0, 1);
    /// South: y decreases (down on screen)
    pub const SOUTH: (i32, i32) = (0, -1);
    /// East: x increases (right on screen)
    pub const EAST: (i32, i32) = (1, 0);
    /// West: x decreases (left on screen)
    pub const WEST: (i32, i32) = (-1, 0);

    /// All direction offsets in array form
    pub const ALL: [(i32, i32); 4] = [NORTH, SOUTH, EAST, WEST];
}

/// Movement order component for pathfinding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementOrder {
    /// Path to follow (sequence of positions)
    pub path: Vec<Position>,
    /// Current index in the path
    pub path_index: usize,
    /// Target destination
    pub destination: Position,
    /// Movement points required
    pub movement_cost: f32,
}

// Manual Component implementation to avoid proc macro issues
impl Component for MovementOrder {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl MovementOrder {
    pub fn new(path: Vec<Position>, destination: Position) -> Self {
        Self {
            path,
            path_index: 0,
            destination,
            movement_cost: 1.0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.path_index >= self.path.len()
    }

    pub fn next_position(&self) -> Option<Position> {
        self.path.get(self.path_index).copied()
    }

    pub fn advance(&mut self) {
        if self.path_index < self.path.len() {
            self.path_index += 1;
        }
    }
}

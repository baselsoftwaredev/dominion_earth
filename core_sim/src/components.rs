use crate::CivId;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Position component for entities on the world map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    /// Check if another position is adjacent (4-directional)
    pub fn is_adjacent_to(&self, other: &Position) -> bool {
        self.manhattan_distance_to(other) == 1
    }

    /// Get direction to another position (if adjacent)
    pub fn direction_to(&self, other: &Position) -> Option<Direction> {
        if !self.is_adjacent_to(other) {
            return None;
        }

        match (other.x - self.x, other.y - self.y) {
            (0, 1) => Some(Direction::North),
            (0, -1) => Some(Direction::South),
            (1, 0) => Some(Direction::East),
            (-1, 0) => Some(Direction::West),
            _ => None,
        }
    }

    /// Move in a specific direction
    pub fn move_in_direction(&self, direction: Direction) -> Position {
        use crate::components::direction_offsets;
        let offset = match direction {
            Direction::North => direction_offsets::NORTH,
            Direction::South => direction_offsets::SOUTH,
            Direction::East => direction_offsets::EAST,
            Direction::West => direction_offsets::WEST,
        };
        Position::new(self.x + offset.0, self.y + offset.1)
    }
}

/// Four cardinal directions for movement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Direction constants for easy access
pub mod directions {
    use super::Direction;

    pub const NORTH: Direction = Direction::North;
    pub const SOUTH: Direction = Direction::South;
    pub const EAST: Direction = Direction::East;
    pub const WEST: Direction = Direction::West;

    /// All directions as an array
    pub const ALL: [Direction; 4] = [NORTH, SOUTH, EAST, WEST];
}

/// Direction name constants for consistent string representation
pub mod direction_names {
    pub const NORTH: &str = "North";
    pub const SOUTH: &str = "South";
    pub const EAST: &str = "East";
    pub const WEST: &str = "West";

    /// All direction names as an array
    pub const ALL: [&str; 4] = [NORTH, SOUTH, EAST, WEST];
}

/// Coordinate offset constants for directional calculations
pub mod direction_offsets {
    /// (x, y) offset for North direction
    pub const NORTH: (i32, i32) = (0, 1);
    /// (x, y) offset for South direction
    pub const SOUTH: (i32, i32) = (0, -1);
    /// (x, y) offset for East direction
    pub const EAST: (i32, i32) = (1, 0);
    /// (x, y) offset for West direction
    pub const WEST: (i32, i32) = (-1, 0);

    /// All direction offsets as an array
    pub const ALL: [(i32, i32); 4] = [NORTH, SOUTH, EAST, WEST];
}

impl Direction {
    /// Get all 4 directions
    pub fn all() -> [Direction; 4] {
        directions::ALL
    }

    /// Get the string name for this direction
    pub fn name(&self) -> &'static str {
        match self {
            Direction::North => direction_names::NORTH,
            Direction::South => direction_names::SOUTH,
            Direction::East => direction_names::EAST,
            Direction::West => direction_names::WEST,
        }
    }

    /// Get opposite direction
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

/// Civilization component containing core data
#[derive(Debug, Clone)]
pub struct Civilization {
    pub id: CivId,
    pub name: String,
    pub color: [f32; 3],
    pub capital: Option<Position>,
    pub personality: CivPersonality,
    pub technologies: Technologies,
    pub economy: Economy,
    pub military: Military,
}

// Manual Component implementation
impl Component for Civilization {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// AI personality traits that drive decision making
#[derive(Debug, Clone)]
pub struct CivPersonality {
    pub land_hunger: f32,     // 0.0 - 1.0, desire to expand territory
    pub industry_focus: f32,  // 0.0 - 1.0, focus on economic development
    pub tech_focus: f32,      // 0.0 - 1.0, investment in research
    pub interventionism: f32, // 0.0 - 1.0, willingness to interfere abroad
    pub risk_tolerance: f32,  // 0.0 - 1.0, willingness to take risks
    pub honor_treaties: f32,  // 0.0 - 1.0, diplomatic reliability
    pub militarism: f32,      // 0.0 - 1.0, focus on military strength
    pub isolationism: f32,    // 0.0 - 1.0, preference for isolation
}

// Manual Component implementation
impl Component for CivPersonality {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl Default for CivPersonality {
    fn default() -> Self {
        Self {
            land_hunger: 0.5,
            industry_focus: 0.5,
            tech_focus: 0.5,
            interventionism: 0.5,
            risk_tolerance: 0.5,
            honor_treaties: 0.5,
            militarism: 0.5,
            isolationism: 0.5,
        }
    }
}

/// Technology research state
#[derive(Debug, Clone)]
pub struct Technologies {
    pub known: HashMap<String, bool>,
    pub research_points: f32,
    pub current_research: Option<String>,
}

impl Default for Technologies {
    fn default() -> Self {
        Self {
            known: HashMap::new(),
            research_points: 0.0,
            current_research: None,
        }
    }
}

/// Economic state of a civilization
#[derive(Debug, Clone)]
pub struct Economy {
    pub gold: f32,
    pub income: f32,
    pub expenses: f32,
    pub production: f32,
    pub trade_routes: Vec<TradeRoute>,
}

// Manual Component implementation
impl Component for Economy {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            gold: 100.0,
            income: 10.0,
            expenses: 5.0,
            production: 8.0,
            trade_routes: Vec::new(),
        }
    }
}

/// Trade route between cities/regions
#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub from: Position,
    pub to: Position,
    pub value: f32,
    pub security: f32,
}

// Manual Component implementation
impl Component for TradeRoute {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Military forces and capabilities
#[derive(Debug, Clone)]
pub struct Military {
    pub units: Vec<MilitaryUnit>,
    pub total_strength: f32,
    pub maintenance_cost: f32,
}

impl Default for Military {
    fn default() -> Self {
        Self {
            units: Vec::new(),
            total_strength: 0.0,
            maintenance_cost: 0.0,
        }
    }
}

/// Individual military unit
#[derive(Debug, Clone)]
pub struct MilitaryUnit {
    pub id: u32,
    pub owner: CivId,
    pub unit_type: UnitType,
    pub position: Position,
    pub strength: f32,
    pub movement_remaining: u32,
    pub experience: f32,
}

// Manual Component implementation
impl Component for MilitaryUnit {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitType {
    Infantry,
    Cavalry,
    Archer,
    Siege,
    Naval,
}

/// City component
#[derive(Debug, Clone)]
pub struct City {
    pub name: String,
    pub owner: CivId,
    pub population: u32,
    pub production: f32,
    pub defense: f32,
    pub buildings: Vec<Building>,
}

// Manual Component implementation
impl Component for City {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Capital component for tracking civilization capitals through different ages
#[derive(Debug, Clone)]
pub struct Capital {
    pub owner: CivId,
    pub age: CapitalAge,
    pub sprite_index: u32,
    pub established_turn: u32,
}

// Manual Component implementation
impl Component for Capital {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Ages that a capital can evolve through
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapitalAge {
    Neolithic,
    Bronze,
    Iron,
    Classical,
    Medieval,
    Renaissance,
    Industrial,
    Modern,
    Information,
    Future,
}

impl CapitalAge {
    /// Get the sprite index for this capital age
    pub fn sprite_index(&self) -> u32 {
        match self {
            CapitalAge::Neolithic => 3, // Use index 3 as requested
            CapitalAge::Bronze => 5,    // You can assign these as you add more sprites
            CapitalAge::Iron => 6,
            CapitalAge::Classical => 7,
            CapitalAge::Medieval => 8,
            CapitalAge::Renaissance => 9,
            CapitalAge::Industrial => 10,
            CapitalAge::Modern => 11,
            CapitalAge::Information => 12,
            CapitalAge::Future => 13,
        }
    }

    /// Get the next age for capital evolution
    pub fn next_age(&self) -> Option<CapitalAge> {
        match self {
            CapitalAge::Neolithic => Some(CapitalAge::Bronze),
            CapitalAge::Bronze => Some(CapitalAge::Iron),
            CapitalAge::Iron => Some(CapitalAge::Classical),
            CapitalAge::Classical => Some(CapitalAge::Medieval),
            CapitalAge::Medieval => Some(CapitalAge::Renaissance),
            CapitalAge::Renaissance => Some(CapitalAge::Industrial),
            CapitalAge::Industrial => Some(CapitalAge::Modern),
            CapitalAge::Modern => Some(CapitalAge::Information),
            CapitalAge::Information => Some(CapitalAge::Future),
            CapitalAge::Future => None, // Max evolution
        }
    }

    /// Get the requirements for evolving to the next age
    pub fn evolution_requirements(&self) -> CapitalEvolutionRequirements {
        match self {
            CapitalAge::Neolithic => CapitalEvolutionRequirements {
                min_population: 2000,
                required_technologies: vec!["Bronze Working".to_string()],
                min_buildings: 2,
                min_turn: 10,
            },
            CapitalAge::Bronze => CapitalEvolutionRequirements {
                min_population: 4000,
                required_technologies: vec!["Iron Working".to_string()],
                min_buildings: 3,
                min_turn: 25,
            },
            CapitalAge::Iron => CapitalEvolutionRequirements {
                min_population: 8000,
                required_technologies: vec!["Currency".to_string(), "Writing".to_string()],
                min_buildings: 4,
                min_turn: 50,
            },
            // Add more requirements as needed for other ages
            _ => CapitalEvolutionRequirements {
                min_population: 10000,
                required_technologies: vec![],
                min_buildings: 5,
                min_turn: 100,
            },
        }
    }
}

/// Requirements for capital evolution
#[derive(Debug, Clone)]
pub struct CapitalEvolutionRequirements {
    pub min_population: u32,
    pub required_technologies: Vec<String>,
    pub min_buildings: usize,
    pub min_turn: u32,
}

/// Building in a city
#[derive(Debug, Clone)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: u32,
}

// Manual Component implementation
impl Component for Building {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

#[derive(Debug, Clone)]
pub enum BuildingType {
    Granary,
    Barracks,
    Workshop,
    Library,
    Walls,
    Market,
    Temple,
}

/// Territory control component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Territory {
    pub owner: CivId,
    pub control_strength: f32,
    pub terrain_type: TerrainType,
}

// Manual Component implementation
impl Component for Territory {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Plains,
    Hills,
    Mountains,
    Forest,
    Desert,
    Coast,
    ShallowCoast,
    Ocean,
    River,
}

// Manual Component implementation
impl Component for TerrainType {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

// Manual Serialize/Deserialize implementation
impl Serialize for TerrainType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TerrainType::Plains => serializer.serialize_str("Plains"),
            TerrainType::Hills => serializer.serialize_str("Hills"),
            TerrainType::Mountains => serializer.serialize_str("Mountains"),
            TerrainType::Forest => serializer.serialize_str("Forest"),
            TerrainType::Desert => serializer.serialize_str("Desert"),
            TerrainType::Coast => serializer.serialize_str("Coast"),
            TerrainType::ShallowCoast => serializer.serialize_str("ShallowCoast"),
            TerrainType::Ocean => serializer.serialize_str("Ocean"),
            TerrainType::River => serializer.serialize_str("River"),
        }
    }
}

impl<'de> Deserialize<'de> for TerrainType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Plains" => Ok(TerrainType::Plains),
            "Hills" => Ok(TerrainType::Hills),
            "Mountains" => Ok(TerrainType::Mountains),
            "Forest" => Ok(TerrainType::Forest),
            "Desert" => Ok(TerrainType::Desert),
            "Coast" => Ok(TerrainType::Coast),
            "ShallowCoast" => Ok(TerrainType::ShallowCoast),
            "Ocean" => Ok(TerrainType::Ocean),
            "River" => Ok(TerrainType::River),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &[
                    "Plains",
                    "Hills",
                    "Mountains",
                    "Forest",
                    "Desert",
                    "Coast",
                    "ShallowCoast",
                    "Ocean",
                    "River",
                ],
            )),
        }
    }
}

/// Diplomatic relationship component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticRelation {
    pub civ_a: CivId,
    pub civ_b: CivId,
    pub relation_value: f32, // -100 to 100
    pub treaties: Vec<Treaty>,
    pub trade_agreement: bool,
}

// Manual Component implementation
impl Component for DiplomaticRelation {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Treaty {
    NonAggression { turns_remaining: u32 },
    Alliance { turns_remaining: u32 },
    TradePact { turns_remaining: u32 },
    War { started_turn: u32 },
}

/// Diplomatic actions that civilizations can take
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiplomaticAction {
    ProposeAlliance,
    ProposeNonAggression,
    ProposeTradePact,
    DeclareWar,
    MakePeace,
    BreakTreaty,
}

/// Movement order for units
#[derive(Debug, Clone)]
pub struct MovementOrder {
    pub unit_entity: bevy_ecs::entity::Entity,
    pub target: Position,
    pub path: Vec<Position>,
    pub path_index: usize,
}

// Manual Component implementation
impl Component for MovementOrder {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Turn marker for entities that should act this turn
#[derive(Debug, Clone)]
pub struct ActiveThisTurn;

// Manual Component implementation
impl Component for ActiveThisTurn {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// AI decision component
#[derive(Debug, Clone)]
pub struct AIDecision {
    pub decision_type: DecisionType,
    pub priority: f32,
    pub target: Option<Position>,
}

// Manual Component implementation
impl Component for AIDecision {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

#[derive(Debug, Clone)]
pub enum DecisionType {
    Expand,
    Research(String),
    BuildUnit(UnitType),
    BuildBuilding(BuildingType),
    Attack(CivId),
    MakePeace(CivId),
    Trade(CivId),
}

/// Data structure for serialization
#[derive(Debug, Clone)]
pub struct CivilizationData {
    pub civilization: Civilization,
    pub cities: Vec<City>,
    pub territories: Vec<(Position, Territory)>,
    pub diplomatic_relations: Vec<DiplomaticRelation>,
}

/// AI actions that can be taken by civilizations (GOAP-based)
#[derive(Debug, Clone)]
pub enum AIAction {
    Expand {
        target_position: Position,
        priority: f32,
    },
    Research {
        technology: String,
        priority: f32,
    },
    BuildUnit {
        unit_type: UnitType,
        position: Position,
        priority: f32,
    },
    BuildBuilding {
        building_type: BuildingType,
        position: Position,
        priority: f32,
    },
    Trade {
        partner: CivId,
        resource: crate::GameResource,
        priority: f32,
    },
    Attack {
        target: CivId,
        target_position: Position,
        priority: f32,
    },
    Diplomacy {
        target: CivId,
        action: DiplomaticAction,
        priority: f32,
    },
    Defend {
        position: Position,
        priority: f32,
    },
}

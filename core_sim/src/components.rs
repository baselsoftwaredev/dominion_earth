use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{CivId, SimResult};

/// Position component for entities on the world map
#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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
}

/// Civilization component containing core data
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
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

/// AI personality traits that drive decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivPersonality {
    pub land_hunger: f32,      // 0.0 - 1.0, desire to expand territory
    pub industry_focus: f32,   // 0.0 - 1.0, focus on economic development
    pub tech_focus: f32,       // 0.0 - 1.0, investment in research
    pub interventionism: f32,  // 0.0 - 1.0, willingness to interfere abroad
    pub risk_tolerance: f32,   // 0.0 - 1.0, willingness to take risks
    pub honor_treaties: f32,   // 0.0 - 1.0, diplomatic reliability
    pub militarism: f32,       // 0.0 - 1.0, focus on military strength
    pub isolationism: f32,     // 0.0 - 1.0, preference for isolation
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Economy {
    pub gold: f32,
    pub income: f32,
    pub expenses: f32,
    pub production: f32,
    pub trade_routes: Vec<TradeRoute>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRoute {
    pub from: Position,
    pub to: Position,
    pub value: f32,
    pub security: f32,
}

/// Military forces and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilitaryUnit {
    pub id: u32,
    pub unit_type: UnitType,
    pub position: Position,
    pub strength: f32,
    pub movement_remaining: u32,
    pub experience: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitType {
    Infantry,
    Cavalry,
    Archer,
    Siege,
    Naval,
}

/// City component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub owner: CivId,
    pub population: u32,
    pub production: f32,
    pub defense: f32,
    pub buildings: Vec<Building>,
}

/// Building in a city
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Territory {
    pub owner: CivId,
    pub control_strength: f32,
    pub terrain_type: TerrainType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Plains,
    Hills,
    Mountains,
    Forest,
    Desert,
    Coast,
    Ocean,
    River,
}

/// Diplomatic relationship component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticRelation {
    pub civ_a: CivId,
    pub civ_b: CivId,
    pub relation_value: f32, // -100 to 100
    pub treaties: Vec<Treaty>,
    pub trade_agreement: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Treaty {
    NonAggression { turns_remaining: u32 },
    Alliance { turns_remaining: u32 },
    TradePact { turns_remaining: u32 },
    War { started_turn: u32 },
}

/// Movement order for units
#[derive(Component, Debug, Clone)]
pub struct MovementOrder {
    pub target: Position,
    pub path: Vec<Position>,
    pub path_index: usize,
}

/// Turn marker for entities that should act this turn
#[derive(Component, Debug, Clone)]
pub struct ActiveThisTurn;

/// AI decision component
#[derive(Component, Debug, Clone)]
pub struct AIDecision {
    pub decision_type: DecisionType,
    pub priority: f32,
    pub target: Option<Position>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationData {
    pub civilization: Civilization,
    pub cities: Vec<City>,
    pub territories: Vec<(Position, Territory)>,
    pub diplomatic_relations: Vec<DiplomaticRelation>,
}

use crate::{CivId, DiplomaticRelation, Position, TerrainType};
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Global world map resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<MapTile>>,
}

impl Default for WorldMap {
    fn default() -> Self {
        Self {
            width: 100,
            height: 50,
            tiles: vec![vec![MapTile::default(); 50]; 100],
        }
    }
}

impl WorldMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![MapTile::default(); height as usize]; width as usize],
        }
    }

    pub fn get_tile(&self, pos: Position) -> Option<&MapTile> {
        if pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height {
            Some(&self.tiles[pos.x as usize][pos.y as usize])
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, pos: Position) -> Option<&mut MapTile> {
        if pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height {
            Some(&mut self.tiles[pos.x as usize][pos.y as usize])
        } else {
            None
        }
    }

    pub fn neighbors(&self, pos: Position) -> Vec<Position> {
        // 4-directional movement only: North, South, East, West
        let directions = [
            (0, 1),  // North
            (0, -1), // South
            (1, 0),  // East
            (-1, 0), // West
        ];

        directions
            .iter()
            .map(|(dx, dy)| Position::new(pos.x + dx, pos.y + dy))
            .filter(|p| {
                p.x >= 0 && p.y >= 0 && (p.x as u32) < self.width && (p.y as u32) < self.height
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapTile {
    pub terrain: TerrainType,
    pub owner: Option<CivId>,
    pub city: Option<String>,
    pub resource: Option<Resource>,
    pub movement_cost: f32,
    pub defense_bonus: f32,
}

impl Default for MapTile {
    fn default() -> Self {
        Self {
            terrain: TerrainType::Plains,
            owner: None,
            city: None,
            resource: None,
            movement_cost: 1.0,
            defense_bonus: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resource {
    Iron,
    Gold,
    Horses,
    Wheat,
    Fish,
    Stone,
    Wood,
    Spices,
}

/// Global economy resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEconomy {
    pub trade_routes: Vec<GlobalTradeRoute>,
    pub resource_prices: HashMap<Resource, f32>,
    pub economic_events: Vec<EconomicEvent>,
}

impl Default for GlobalEconomy {
    fn default() -> Self {
        let mut resource_prices = HashMap::new();
        resource_prices.insert(Resource::Iron, 10.0);
        resource_prices.insert(Resource::Gold, 50.0);
        resource_prices.insert(Resource::Horses, 20.0);
        resource_prices.insert(Resource::Wheat, 5.0);
        resource_prices.insert(Resource::Fish, 3.0);
        resource_prices.insert(Resource::Stone, 8.0);
        resource_prices.insert(Resource::Wood, 6.0);
        resource_prices.insert(Resource::Spices, 25.0);

        Self {
            trade_routes: Vec::new(),
            resource_prices,
            economic_events: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTradeRoute {
    pub from_civ: CivId,
    pub to_civ: CivId,
    pub resource: Resource,
    pub volume: f32,
    pub security_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicEvent {
    pub event_type: EconomicEventType,
    pub affected_resource: Option<Resource>,
    pub magnitude: f32,
    pub duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EconomicEventType {
    ResourceBoom,
    ResourceCrash,
    TradeDisruption,
    TechnologicalAdvancement,
}

/// Diplomatic state resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticState {
    pub relations: HashMap<(CivId, CivId), DiplomaticRelation>,
    pub ongoing_negotiations: Vec<Negotiation>,
    pub diplomatic_events: Vec<DiplomaticEvent>,
}

impl Default for DiplomaticState {
    fn default() -> Self {
        Self {
            relations: HashMap::new(),
            ongoing_negotiations: Vec::new(),
            diplomatic_events: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Negotiation {
    pub initiator: CivId,
    pub target: CivId,
    pub proposal: DiplomaticProposal,
    pub turns_remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiplomaticProposal {
    TradePact,
    NonAggressionPact,
    Alliance,
    PeaceTreaty,
    TechnologyExchange(String),
    ResourceTrade(Resource, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticEvent {
    pub event_type: DiplomaticEventType,
    pub involved_civs: Vec<CivId>,
    pub turn: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiplomaticEventType {
    WarDeclared,
    PeaceSigned,
    AllianceFormed,
    AllianceBroken,
    TradeAgreementSigned,
    DiplomaticInsult,
}

/// Current turn resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct CurrentTurn(pub u32);

impl Default for CurrentTurn {
    fn default() -> Self {
        CurrentTurn(1)
    }
}

/// Active civilization turn tracker
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct ActiveCivTurn {
    pub current_civ_index: usize,
    pub civs_per_turn: Vec<CivId>,
    pub turn_phase: TurnPhase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TurnPhase {
    Planning,  // AI is making decisions
    Execution, // Actions are being executed
    Complete,  // Turn is finished, ready to advance
}

impl Default for ActiveCivTurn {
    fn default() -> Self {
        Self {
            current_civ_index: 0,
            civs_per_turn: Vec::new(),
            turn_phase: TurnPhase::Planning,
        }
    }
}

/// Game configuration
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub max_turns: u32,
    pub turn_time_limit: Option<f32>,
    pub ai_difficulty: AIDifficulty,
    pub world_size: WorldSize,
    pub random_seed: u64,
    pub debug_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIDifficulty {
    Easy,
    Normal,
    Hard,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldSize {
    Small,
    Medium,
    Large,
    Huge,
}

impl Default for GameConfig {
    fn default() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let random_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(42))
            .as_secs();

        Self {
            max_turns: 500,
            turn_time_limit: None,
            ai_difficulty: AIDifficulty::Normal,
            world_size: WorldSize::Medium,
            random_seed,
            debug_logging: false,
        }
    }
}

/// Random number generator resource
#[derive(Resource)]
pub struct GameRng(pub rand_pcg::Pcg64);

impl Default for GameRng {
    fn default() -> Self {
        use rand::SeedableRng;
        use std::time::{SystemTime, UNIX_EPOCH};
        let random_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(42))
            .as_secs();
        Self(rand_pcg::Pcg64::seed_from_u64(random_seed))
    }
}

/// Resource to request a turn advance (set by UI or timer)
#[derive(Default, Resource)]
pub struct TurnAdvanceRequest(pub bool);

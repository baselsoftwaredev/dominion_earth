use crate::{
    components::{CivPersonality, Position, TerrainType},
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationDefinition {
    pub name: String,
    pub capital_name: String,
    pub starting_position: PositionData,
    pub color: (f32, f32, f32),
    pub personality: PersonalityData,
    pub starting_technologies: Vec<String>,
    pub starting_units: Vec<UnitCount>,
    pub starting_buildings: Vec<BuildingCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionData {
    pub x: i32,
    pub y: i32,
}

impl From<PositionData> for Position {
    fn from(pos: PositionData) -> Self {
        Position::new(pos.x, pos.y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityData {
    pub land_hunger: f32,
    pub industry_focus: f32,
    pub tech_focus: f32,
    pub interventionism: f32,
    pub risk_tolerance: f32,
    pub honor_treaties: f32,
    pub militarism: f32,
    pub isolationism: f32,
}

impl From<PersonalityData> for CivPersonality {
    fn from(data: PersonalityData) -> Self {
        CivPersonality {
            land_hunger: data.land_hunger,
            industry_focus: data.industry_focus,
            tech_focus: data.tech_focus,
            interventionism: data.interventionism,
            risk_tolerance: data.risk_tolerance,
            honor_treaties: data.honor_treaties,
            militarism: data.militarism,
            isolationism: data.isolationism,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitCount {
    pub unit_type: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingCount {
    pub building_type: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGenerationData {
    pub width: u32,
    pub height: u32,
    pub continent_count: u32,
    pub mountain_density: f32,
    pub forest_density: f32,
    pub desert_density: f32,
    pub river_count: u32,
    pub resource_density: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRulesData {
    pub max_turns: u32,
    pub turn_time_limit_seconds: f32,
    pub starting_gold: f32,
    pub research_cost_base: f32,
    pub unit_cost_base: f32,
    pub building_cost_base: f32,
    pub movement_points_per_turn: u32,
    pub combat_experience_gain: f32,
    pub diplomatic_relation_decay: f32,
    pub trade_route_max_distance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationDataCollection {
    pub civilizations: Vec<CivilizationDefinition>,
    pub world_generation: WorldGenerationData,
    pub game_rules: GameRulesData,
}

pub struct CivilizationDataLoader;

impl CivilizationDataLoader {
    pub fn load_from_ron(path: &str) -> Result<CivilizationDataCollection, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let data: CivilizationDataCollection = ron::from_str(&content)?;
        Ok(data)
    }

    /// Generate random starting positions for civilizations on suitable land tiles
    pub fn generate_random_starting_positions(
        civilizations: &[CivilizationDefinition],
        world_map: &crate::WorldMap,
        rng: &mut impl Rng,
        min_distance_between_civs: u32,
    ) -> HashMap<String, Position> {
        let mut positions = HashMap::new();
        let mut used_positions = Vec::new();

        for civ in civilizations {
            let mut attempts = 0;
            const MAX_PLACEMENT_ATTEMPTS: u32 = 1000;

            loop {
                attempts += 1;
                if attempts > MAX_PLACEMENT_ATTEMPTS {
                    // Fallback to original position if we can't find a suitable spot
                    let fallback_position = Position::from(civ.starting_position.clone());
                    positions.insert(civ.name.clone(), fallback_position);
                    break;
                }

                // Generate random position
                let x = rng.gen_range(0..world_map.width as i32);
                let y = rng.gen_range(0..world_map.height as i32);
                let candidate_position = Position::new(x, y);

                // Check if position is suitable for settlement
                if !Self::is_suitable_starting_position(world_map, candidate_position) {
                    continue;
                }

                // Check distance from other civilizations
                if Self::is_too_close_to_existing_civs(&used_positions, candidate_position, min_distance_between_civs) {
                    continue;
                }

                // Position is valid
                positions.insert(civ.name.clone(), candidate_position);
                used_positions.push(candidate_position);
                break;
            }
        }

        positions
    }

    fn is_suitable_starting_position(world_map: &crate::WorldMap, position: Position) -> bool {
        if let Some(tile) = world_map.get_tile(position) {
            match tile.terrain {
                TerrainType::Plains | TerrainType::Coast | TerrainType::Forest => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_too_close_to_existing_civs(
        used_positions: &[Position],
        candidate: Position,
        min_distance: u32,
    ) -> bool {
        for &existing_pos in used_positions {
            let distance = Self::calculate_distance(existing_pos, candidate);
            if distance < min_distance {
                return true;
            }
        }
        false
    }

    fn calculate_distance(pos1: Position, pos2: Position) -> u32 {
        let dx = (pos1.x - pos2.x).abs();
        let dy = (pos1.y - pos2.y).abs();
        ((dx * dx + dy * dy) as f32).sqrt() as u32
    }
}

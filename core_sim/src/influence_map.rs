use crate::{Position, WorldMap, CivId};
use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};

/// Influence map for strategic AI decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluenceMap {
    pub width: u32,
    pub height: u32,
    pub layers: HashMap<InfluenceType, Vec<Vec<f32>>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfluenceType {
    Military(CivId),
    Economic(CivId),
    Cultural(CivId),
    Control(CivId),
    Strategic, // General strategic value
    Threat,    // Combined threat assessment
}

impl InfluenceMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            layers: HashMap::new(),
        }
    }

    pub fn add_layer(&mut self, influence_type: InfluenceType) {
        let layer = vec![vec![0.0; self.height as usize]; self.width as usize];
        self.layers.insert(influence_type, layer);
    }

    pub fn get_influence(&self, influence_type: &InfluenceType, pos: Position) -> f32 {
        if let Some(layer) = self.layers.get(influence_type) {
            if pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height {
                return layer[pos.x as usize][pos.y as usize];
            }
        }
        0.0
    }

    pub fn set_influence(&mut self, influence_type: &InfluenceType, pos: Position, value: f32) {
        if let Some(layer) = self.layers.get_mut(influence_type) {
            if pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height {
                layer[pos.x as usize][pos.y as usize] = value;
            }
        }
    }

    pub fn add_influence(&mut self, influence_type: &InfluenceType, pos: Position, value: f32) {
        if let Some(layer) = self.layers.get_mut(influence_type) {
            if pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height {
                layer[pos.x as usize][pos.y as usize] += value;
            }
        }
    }

    pub fn decay_influence(&mut self, influence_type: &InfluenceType, decay_rate: f32) {
        if let Some(layer) = self.layers.get_mut(influence_type) {
            for row in layer.iter_mut() {
                for cell in row.iter_mut() {
                    *cell *= decay_rate;
                }
            }
        }
    }

    pub fn project_influence(&mut self, influence_type: &InfluenceType, center: Position, strength: f32, max_distance: f32) {
        let max_dist_i = max_distance as i32;
        
        for dx in -max_dist_i..=max_dist_i {
            for dy in -max_dist_i..=max_dist_i {
                let pos = Position::new(center.x + dx, center.y + dy);
                let distance = center.distance_to(&pos);
                
                if distance <= max_distance && distance > 0.0 {
                    let influence_value = strength * (1.0 - distance / max_distance).max(0.0);
                    self.add_influence(influence_type, pos, influence_value);
                }
            }
        }
    }

    pub fn get_combined_influence(&self, pos: Position, civs: &[CivId]) -> HashMap<CivId, f32> {
        let mut combined = HashMap::new();

        for &civ in civs {
            let mut total_influence = 0.0;
            
            // Combine military, economic, and cultural influence
            total_influence += self.get_influence(&InfluenceType::Military(civ), pos) * 0.4;
            total_influence += self.get_influence(&InfluenceType::Economic(civ), pos) * 0.3;
            total_influence += self.get_influence(&InfluenceType::Cultural(civ), pos) * 0.2;
            total_influence += self.get_influence(&InfluenceType::Control(civ), pos) * 0.1;
            
            combined.insert(civ, total_influence);
        }

        combined
    }

    pub fn find_strategic_positions(&self, influence_type: &InfluenceType, threshold: f32) -> Vec<Position> {
        let mut positions = Vec::new();

        if let Some(layer) = self.layers.get(influence_type) {
            for (x, row) in layer.iter().enumerate() {
                for (y, &value) in row.iter().enumerate() {
                    if value >= threshold {
                        positions.push(Position::new(x as i32, y as i32));
                    }
                }
            }
        }

        positions
    }

    pub fn update_strategic_layer(&mut self, world_map: &WorldMap) {
        // Clear existing strategic layer
        if !self.layers.contains_key(&InfluenceType::Strategic) {
            self.add_layer(InfluenceType::Strategic);
        }

        // Reset strategic values
        if let Some(layer) = self.layers.get_mut(&InfluenceType::Strategic) {
            for row in layer.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = 0.0;
                }
            }
        }

        // Calculate strategic value based on terrain and resources
        for x in 0..self.width {
            for y in 0..self.height {
                let pos = Position::new(x as i32, y as i32);
                if let Some(tile) = world_map.get_tile(pos) {
                    let mut strategic_value = 0.0;

                    // Base terrain value
                    strategic_value += match tile.terrain {
                        crate::TerrainType::Plains => 1.0,
                        crate::TerrainType::Hills => 1.5,
                        crate::TerrainType::Mountains => 0.5,
                        crate::TerrainType::Forest => 1.2,
                        crate::TerrainType::Desert => 0.3,
                        crate::TerrainType::Coast => 2.0,
                        crate::TerrainType::Ocean => 0.1,
                        crate::TerrainType::River => 1.8,
                    };

                    // Resource bonus
                    if tile.resource.is_some() {
                        strategic_value += 2.0;
                    }

                    // Defensive bonus
                    strategic_value += tile.defense_bonus;

                    // Movement accessibility (inverse of movement cost)
                    strategic_value += 2.0 / tile.movement_cost;

                    self.set_influence(&InfluenceType::Strategic, pos, strategic_value);
                }
            }
        }
    }

    pub fn update_threat_assessment(&mut self, civs: &[CivId]) {
        // Clear existing threat layer
        if !self.layers.contains_key(&InfluenceType::Threat) {
            self.add_layer(InfluenceType::Threat);
        }

        // Reset threat values
        if let Some(layer) = self.layers.get_mut(&InfluenceType::Threat) {
            for row in layer.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = 0.0;
                }
            }
        }

        // Calculate threat level for each position
        for x in 0..self.width {
            for y in 0..self.height {
                let pos = Position::new(x as i32, y as i32);
                let mut threat_level = 0.0;

                for &civ in civs {
                    let military_influence = self.get_influence(&InfluenceType::Military(civ), pos);
                    threat_level += military_influence;
                }

                self.set_influence(&InfluenceType::Threat, pos, threat_level);
            }
        }
    }
}

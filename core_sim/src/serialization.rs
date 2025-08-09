use serde::{Deserialize, Serialize};
use crate::{GameState, SimResult, CivilizationData};
use std::path::Path;

/// Serialization utilities for saving/loading game state
pub struct GameSerializer;

impl GameSerializer {
    /// Save game state to RON format
    pub fn save_to_ron<P: AsRef<Path>>(game_state: &GameState, path: P) -> SimResult<()> {
        let ron_string = ron::ser::to_string_pretty(game_state, ron::ser::PrettyConfig::default())?;
        std::fs::write(path, ron_string)?;
        Ok(())
    }

    /// Load game state from RON format
    pub fn load_from_ron<P: AsRef<Path>>(path: P) -> SimResult<GameState> {
        let content = std::fs::read_to_string(path)?;
        let game_state: GameState = ron::from_str(&content)?;
        Ok(game_state)
    }

    /// Save game state to JSON format
    pub fn save_to_json<P: AsRef<Path>>(game_state: &GameState, path: P) -> SimResult<()> {
        let json_string = serde_json::to_string_pretty(game_state)?;
        std::fs::write(path, json_string)?;
        Ok(())
    }

    /// Load game state from JSON format
    pub fn load_from_json<P: AsRef<Path>>(path: P) -> SimResult<GameState> {
        let content = std::fs::read_to_string(path)?;
        let game_state: GameState = serde_json::from_str(&content)?;
        Ok(game_state)
    }

    /// Create a compact save containing only essential data
    pub fn create_compact_save(game_state: &GameState) -> CompactGameState {
        CompactGameState {
            turn: game_state.turn,
            civ_count: game_state.civilizations.len() as u32,
            world_checksum: Self::calculate_world_checksum(&game_state.world_map),
            major_events: Self::extract_major_events(game_state),
        }
    }

    fn calculate_world_checksum(world_map: &crate::WorldMap) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        world_map.width.hash(&mut hasher);
        world_map.height.hash(&mut hasher);
        
        // Sample a few tiles for checksum
        for x in (0..world_map.width).step_by(10) {
            for y in (0..world_map.height).step_by(10) {
                if let Some(tile) = world_map.get_tile(crate::Position::new(x as i32, y as i32)) {
                    format!("{:?}", tile.terrain).hash(&mut hasher);
                    tile.owner.map(|o| o.0).hash(&mut hasher);
                }
            }
        }
        
        hasher.finish()
    }

    fn extract_major_events(game_state: &GameState) -> Vec<String> {
        let mut events = Vec::new();
        
        // Extract diplomatic events
        for event in &game_state.diplomatic_state.diplomatic_events {
            events.push(format!("Turn {}: {:?} involving {:?}", 
                event.turn, event.event_type, event.involved_civs));
        }
        
        // Extract economic events
        for event in &game_state.global_economy.economic_events {
            if event.duration > 0 {
                events.push(format!("Economic event: {:?} affecting {:?}", 
                    event.event_type, event.affected_resource));
            }
        }
        
        events
    }

    /// Validate game state integrity
    pub fn validate_game_state(game_state: &GameState) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        // Check civilization data consistency
        for (civ_id, civ_data) in &game_state.civilizations {
            if civ_data.civilization.id != *civ_id {
                errors.push(ValidationError::CivIdMismatch(*civ_id));
            }
            
            // Check if capital exists on the map
            if let Some(capital_pos) = civ_data.civilization.capital {
                if let Some(tile) = game_state.world_map.get_tile(capital_pos) {
                    if tile.owner != Some(*civ_id) {
                        errors.push(ValidationError::CapitalNotOwnedByOwner(*civ_id));
                    }
                } else {
                    errors.push(ValidationError::CapitalOutsideMap(*civ_id));
                }
            }
        }
        
        // Check for orphaned territories
        for x in 0..game_state.world_map.width {
            for y in 0..game_state.world_map.height {
                let pos = crate::Position::new(x as i32, y as i32);
                if let Some(tile) = game_state.world_map.get_tile(pos) {
                    if let Some(owner) = tile.owner {
                        if !game_state.civilizations.contains_key(&owner) {
                            errors.push(ValidationError::OrphanedTerritory(pos, owner));
                        }
                    }
                }
            }
        }
        
        errors
    }
}

/// Compact representation for quick saves/autosaves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactGameState {
    pub turn: u32,
    pub civ_count: u32,
    pub world_checksum: u64,
    pub major_events: Vec<String>,
}

/// Validation errors that can occur in game state
#[derive(Debug, Clone)]
pub enum ValidationError {
    CivIdMismatch(crate::CivId),
    CapitalNotOwnedByOwner(crate::CivId),
    CapitalOutsideMap(crate::CivId),
    OrphanedTerritory(crate::Position, crate::CivId),
}

/// ECS world serialization helper
pub struct EcsSerializer;

impl EcsSerializer {
    /// Extract game state from ECS world
    pub fn extract_from_world(world: &bevy_ecs::world::World) -> GameState {
        let mut game_state = GameState::default();
        
        // Extract current turn
        if let Some(current_turn) = world.get_resource::<crate::CurrentTurn>() {
            game_state.turn = current_turn.0;
        }
        
        // Extract world map
        if let Some(world_map) = world.get_resource::<crate::WorldMap>() {
            game_state.world_map = world_map.clone();
        }
        
        // Extract global economy
        if let Some(global_economy) = world.get_resource::<crate::GlobalEconomy>() {
            game_state.global_economy = global_economy.clone();
        }
        
        // Extract diplomatic state
        if let Some(diplomatic_state) = world.get_resource::<crate::DiplomaticState>() {
            game_state.diplomatic_state = diplomatic_state.clone();
        }
        
        // Extract civilizations and related data
        let mut civs_query = world.query::<(&crate::Civilization, &crate::Position)>();
        let mut cities_query = world.query::<(&crate::City, &crate::Position)>();
        let mut territories_query = world.query::<(&crate::Territory, &crate::Position)>();
        let mut diplomatic_relations_query = world.query::<&crate::DiplomaticRelation>();
        
        // Group data by civilization
        for (civilization, _position) in civs_query.iter(world) {
            let civ_id = civilization.id;
            
            // Collect cities for this civ
            let cities: Vec<crate::City> = cities_query.iter(world)
                .filter(|(city, _)| city.owner == civ_id)
                .map(|(city, _)| city.clone())
                .collect();
            
            // Collect territories for this civ
            let territories: Vec<(crate::Position, crate::Territory)> = territories_query.iter(world)
                .filter(|(territory, _)| territory.owner == civ_id)
                .map(|(territory, position)| (*position, territory.clone()))
                .collect();
            
            // Collect diplomatic relations involving this civ
            let diplomatic_relations: Vec<crate::DiplomaticRelation> = diplomatic_relations_query.iter(world)
                .filter(|relation| relation.civ_a == civ_id || relation.civ_b == civ_id)
                .cloned()
                .collect();
            
            let civ_data = CivilizationData {
                civilization: civilization.clone(),
                cities,
                territories,
                diplomatic_relations,
            };
            
            game_state.civilizations.insert(civ_id, civ_data);
        }
        
        game_state
    }

    /// Apply game state to ECS world
    pub fn apply_to_world(game_state: &GameState, world: &mut bevy_ecs::world::World) {
        // Clear existing entities
        world.clear_entities();
        
        // Insert resources
        world.insert_resource(crate::CurrentTurn(game_state.turn));
        world.insert_resource(game_state.world_map.clone());
        world.insert_resource(game_state.global_economy.clone());
        world.insert_resource(game_state.diplomatic_state.clone());
        
        // Spawn civilizations and related entities
        for (civ_id, civ_data) in &game_state.civilizations {
            // Spawn civilization
            if let Some(capital) = civ_data.civilization.capital {
                world.spawn((civ_data.civilization.clone(), capital));
            }
            
            // Spawn cities
            for city in &civ_data.cities {
                // Find city position from world map
                if let Some(position) = Self::find_city_position(&game_state.world_map, &city.name) {
                    world.spawn((city.clone(), position));
                }
            }
            
            // Spawn territories
            for (position, territory) in &civ_data.territories {
                world.spawn((territory.clone(), *position));
            }
            
            // Spawn diplomatic relations
            for relation in &civ_data.diplomatic_relations {
                world.spawn(relation.clone());
            }
        }
    }

    fn find_city_position(world_map: &crate::WorldMap, city_name: &str) -> Option<crate::Position> {
        for x in 0..world_map.width {
            for y in 0..world_map.height {
                let pos = crate::Position::new(x as i32, y as i32);
                if let Some(tile) = world_map.get_tile(pos) {
                    if let Some(ref tile_city) = tile.city {
                        if tile_city.contains(city_name) {
                            return Some(pos);
                        }
                    }
                }
            }
        }
        None
    }
}

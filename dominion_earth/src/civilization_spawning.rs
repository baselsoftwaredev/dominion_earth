//! Civilization spawning and world setup for Dominion Earth
//!
//! This module handles the initialization of civilizations, cities, and units
//! at the start of the game.

use bevy::prelude::*;
use core_sim::{
    CivId, Civilization, CivPersonality, City, Building, BuildingType,
    Capital, CapitalAge, MilitaryUnit, UnitType, ProductionQueue, PlayerControlled, Position,
    ActiveThisTurn, Economy, Military, Technologies,
    resources::{WorldMap, GameRng},
    CivilizationDataLoader, CivilizationDefinition,
};
use crate::debug_utils::{DebugLogging, DebugUtils};

/// Spawn initial civilizations on the world map
pub fn spawn_initial_civilizations(
    commands: &mut Commands,
    world_map: &mut WorldMap,
    rng: &mut rand_pcg::Pcg64,
    debug_logging: &DebugLogging,
    ai_only: bool,
    total_civilizations: u32,
) {
    use rand::seq::SliceRandom;

    let civilization_data = load_civilization_data(debug_logging);
    if civilization_data.is_none() {
        return;
    }

    let selected_civs = select_random_civilizations(
        civilization_data.unwrap(),
        total_civilizations,
        rng
    );

    let random_positions = generate_starting_positions(
        &selected_civs,
        world_map,
        rng
    );

    let mut spawned_count = 0;
    for (civ_index, civ_def) in selected_civs.into_iter().enumerate() {
        if spawn_civilization(
            commands,
            world_map,
            &civ_def,
            civ_index,
            &random_positions,
            ai_only,
            debug_logging,
        ) {
            spawned_count += 1;
        }
    }

    DebugUtils::log_civilization_spawn(debug_logging, spawned_count);
}

/// Load civilization data from RON file
fn load_civilization_data(
    debug_logging: &DebugLogging
) -> Option<core_sim::CivilizationDataCollection> {
    match CivilizationDataLoader::load_from_ron("dominion_earth/assets/data/civilizations.ron") {
        Ok(data) => {
            println!(
                "Successfully loaded {} civilizations from RON file",
                data.civilizations.len()
            );
            Some(data)
        }
        Err(e) => {
            println!("Failed to load civilization data: {}", e);
            DebugUtils::log_info(
                debug_logging,
                &format!("Failed to load civilization data: {}", e),
            );
            None
        }
    }
}

/// Select random civilizations for the game
fn select_random_civilizations(
    civilization_data: core_sim::CivilizationDataCollection,
    total_civilizations: u32,
    rng: &mut rand_pcg::Pcg64,
) -> Vec<CivilizationDefinition> {
    use rand::seq::SliceRandom;
    
    let mut available_civs = civilization_data.civilizations.clone();
    available_civs.shuffle(rng);
    available_civs
        .into_iter()
        .take(total_civilizations as usize)
        .collect()
}

/// Generate random starting positions for civilizations
fn generate_starting_positions(
    selected_civs: &[CivilizationDefinition],
    world_map: &WorldMap,
    rng: &mut rand_pcg::Pcg64,
) -> std::collections::HashMap<String, Position> {
    const MIN_DISTANCE_BETWEEN_CIVS: u32 = 5;
    
    CivilizationDataLoader::generate_random_starting_positions(
        selected_civs,
        world_map,
        rng,
        MIN_DISTANCE_BETWEEN_CIVS,
    )
}

/// Spawn a single civilization with its capital and starting unit
fn spawn_civilization(
    commands: &mut Commands,
    world_map: &mut WorldMap,
    civ_def: &CivilizationDefinition,
    civ_index: usize,
    random_positions: &std::collections::HashMap<String, Position>,
    ai_only: bool,
    debug_logging: &DebugLogging,
) -> bool {
    let position = match random_positions.get(&civ_def.name) {
        Some(&pos) => pos,
        None => {
            DebugUtils::log_capital_spawn_skip(debug_logging, &civ_def.name, 0, 0);
            return false;
        }
    };

    if !is_buildable_position(world_map, position) {
        DebugUtils::log_capital_spawn_skip(
            debug_logging,
            &civ_def.name,
            position.x,
            position.y,
        );
        return false;
    }

    let civ_id = CivId(civ_index as u32);
    let is_player = !ai_only && civ_index == 0;

    spawn_civilization_entity(commands, civ_def, civ_id, position, is_player, debug_logging);
    spawn_capital_city(commands, civ_def, civ_id, position, is_player);
    spawn_starting_unit(commands, civ_id, position, civ_index, is_player);
    claim_starting_territory(world_map, civ_id, position, &civ_def.capital_name);

    true
}

/// Check if a position is suitable for building
fn is_buildable_position(world_map: &WorldMap, position: Position) -> bool {
    if let Some(tile) = world_map.get_tile(position) {
        match tile.terrain {
            core_sim::TerrainType::Plains
            | core_sim::TerrainType::Coast
            | core_sim::TerrainType::Forest => true,
            _ => false,
        }
    } else {
        false
    }
}

/// Spawn the civilization entity
fn spawn_civilization_entity(
    commands: &mut Commands,
    civ_def: &CivilizationDefinition,
    civ_id: CivId,
    position: Position,
    is_player: bool,
    debug_logging: &DebugLogging,
) {
    let color = [civ_def.color.0, civ_def.color.1, civ_def.color.2];
    let personality = CivPersonality::from(civ_def.personality.clone());

    let civilization = Civilization {
        id: civ_id,
        name: civ_def.name.clone(),
        color,
        capital: Some(position),
        personality,
        technologies: Technologies::default(),
        economy: Economy::default(),
        military: Military::default(),
    };

    let mut civ_entity_commands = commands.spawn((civilization, position, ActiveThisTurn));

    if is_player {
        civ_entity_commands.insert(PlayerControlled);
        DebugUtils::log_info(
            debug_logging,
            &format!("Marking {} as player-controlled civilization", civ_def.name),
        );
    } else {
        DebugUtils::log_info(
            debug_logging,
            &format!("Marking {} as AI-controlled civilization", civ_def.name),
        );
    }
}

/// Spawn the capital city
fn spawn_capital_city(
    commands: &mut Commands,
    civ_def: &CivilizationDefinition,
    civ_id: CivId,
    position: Position,
    is_player: bool,
) {
    let city = City {
        name: civ_def.capital_name.clone(),
        owner: civ_id,
        population: 1000,
        production: 5.0,
        defense: 10.0,
        buildings: vec![Building {
            building_type: BuildingType::Granary,
            level: 1,
        }],
    };

    let capital = Capital {
        owner: civ_id,
        age: CapitalAge::Neolithic,
        sprite_index: CapitalAge::Neolithic.sprite_index(),
        established_turn: 0,
    };

    let mut capital_commands = commands.spawn((city, capital, position));
    capital_commands.insert(ProductionQueue::new(civ_id));

    if is_player {
        capital_commands.insert(PlayerControlled);
    }
}

/// Spawn the starting military unit
fn spawn_starting_unit(
    commands: &mut Commands,
    civ_id: CivId,
    position: Position,
    civ_index: usize,
    is_player: bool,
) {
    let initial_unit = MilitaryUnit {
        id: civ_index as u32,
        owner: civ_id,
        unit_type: UnitType::Infantry,
        position,
        strength: 10.0,
        movement_remaining: 2,
        experience: 0.0,
    };

    let mut unit_commands = commands.spawn((initial_unit, position));

    if is_player {
        unit_commands.insert(PlayerControlled);
    }
}

/// Claim starting territory for the civilization
fn claim_starting_territory(
    world_map: &mut WorldMap,
    civ_id: CivId,
    position: Position,
    capital_name: &str,
) {
    if let Some(tile) = world_map.get_tile_mut(position) {
        tile.owner = Some(civ_id);
        tile.city = Some(capital_name.to_string());
    }
}
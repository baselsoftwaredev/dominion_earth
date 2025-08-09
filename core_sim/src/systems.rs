use bevy_ecs::prelude::*;
use crate::{
    Civilization, Position, City, Territory, DiplomaticRelation, MovementOrder,
    CivId, WorldMap, CurrentTurn, GameRng, AIDecision, ActiveThisTurn,
    MilitaryUnit, CivPersonality, InfluenceMap, InfluenceType,
};
use std::collections::HashMap;
use rand::Rng;

/// Core systems for game simulation
pub mod turn_management;
pub mod movement;
pub mod ai_decision;
pub mod combat_resolution;
pub mod economic_update;

/// System for advancing the game turn
pub fn advance_turn(
    mut current_turn: ResMut<CurrentTurn>,
    mut query: Query<Entity, With<ActiveThisTurn>>,
    mut commands: Commands,
) {
    // Remove all active this turn markers
    for entity in query.iter_mut() {
        commands.entity(entity).remove::<ActiveThisTurn>();
    }

    // Advance turn counter
    current_turn.0 += 1;

    println!("Advanced to turn {}", current_turn.0);
}

/// System to mark civilizations as active for the current turn
pub fn activate_civilizations(
    mut commands: Commands,
    query: Query<Entity, With<Civilization>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(ActiveThisTurn);
    }
}

/// System for processing unit movement orders
pub fn process_movement_orders(
    mut query: Query<(&mut Position, &mut MovementOrder), With<MilitaryUnit>>,
    world_map: Res<WorldMap>,
) {
    for (mut position, mut movement_order) in query.iter_mut() {
        if movement_order.path_index < movement_order.path.len() {
            let next_position = movement_order.path[movement_order.path_index];
            
            // Check if movement is valid
            if let Some(tile) = world_map.get_tile(next_position) {
                if tile.movement_cost <= 2.0 { // Can move if cost is reasonable
                    *position = next_position;
                    movement_order.path_index += 1;
                }
            }
        }
    }
}

/// System for basic AI decision making
pub fn ai_decision_system(
    mut commands: Commands,
    mut civs: Query<(Entity, &Civilization, &Position), With<ActiveThisTurn>>,
    world_map: Res<WorldMap>,
    mut rng: ResMut<GameRng>,
    current_turn: Res<CurrentTurn>,
) {
    for (entity, civilization, position) in civs.iter_mut() {
        // Simple AI decision making based on personality
        let decision = make_simple_ai_decision(civilization, position, &world_map, &mut rng.0, current_turn.0);
        
        if let Some(decision) = decision {
            commands.entity(entity).insert(decision);
        }
    }
}

fn make_simple_ai_decision(
    civilization: &Civilization,
    position: &Position,
    world_map: &WorldMap,
    rng: &mut impl Rng,
    turn: u32,
) -> Option<AIDecision> {
    let personality = &civilization.personality;
    
    // Early game focuses on expansion
    if turn < 50 && personality.land_hunger > 0.5 {
        // Look for nearby empty land
        for neighbor in world_map.neighbors(*position) {
            if let Some(tile) = world_map.get_tile(neighbor) {
                if tile.owner.is_none() && !matches!(tile.terrain, crate::TerrainType::Ocean) {
                    return Some(AIDecision {
                        decision_type: crate::DecisionType::Expand,
                        priority: personality.land_hunger,
                        target: Some(neighbor),
                    });
                }
            }
        }
    }
    
    // Technology focus
    if personality.tech_focus > 0.6 && rng.gen_bool(0.3) {
        let techs = ["Agriculture", "Bronze Working", "Writing", "Mathematics", "Iron Working"];
        let tech = techs[rng.gen_range(0..techs.len())];
        
        if !civilization.technologies.known.get(tech).unwrap_or(&false) {
            return Some(AIDecision {
                decision_type: crate::DecisionType::Research(tech.to_string()),
                priority: personality.tech_focus,
                target: None,
            });
        }
    }
    
    // Military buildup
    if personality.militarism > 0.5 && rng.gen_bool(0.2) {
        return Some(AIDecision {
            decision_type: crate::DecisionType::BuildUnit(crate::UnitType::Infantry),
            priority: personality.militarism,
            target: Some(*position),
        });
    }
    
    None
}

/// System for updating influence maps
pub fn update_influence_maps(
    mut influence_map: ResMut<InfluenceMap>,
    civs: Query<(&Civilization, &Position)>,
    cities: Query<(&City, &Position)>,
    world_map: Res<WorldMap>,
) {
    // Clear existing influence
    for layer in influence_map.layers.values_mut() {
        for row in layer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = 0.0;
            }
        }
    }
    
    // Project influence from civilizations
    for (civilization, position) in civs.iter() {
        let civ_id = civilization.id;
        
        // Ensure influence layers exist
        let military_type = InfluenceType::Military(civ_id);
        let economic_type = InfluenceType::Economic(civ_id);
        let cultural_type = InfluenceType::Cultural(civ_id);
        let control_type = InfluenceType::Control(civ_id);
        
        if !influence_map.layers.contains_key(&military_type) {
            influence_map.add_layer(military_type.clone());
        }
        if !influence_map.layers.contains_key(&economic_type) {
            influence_map.add_layer(economic_type.clone());
        }
        if !influence_map.layers.contains_key(&cultural_type) {
            influence_map.add_layer(cultural_type.clone());
        }
        if !influence_map.layers.contains_key(&control_type) {
            influence_map.add_layer(control_type.clone());
        }
        
        // Project military influence
        let military_strength = civilization.military.total_strength;
        influence_map.project_influence(&military_type, *position, military_strength, 10.0);
        
        // Project economic influence
        let economic_strength = civilization.economy.gold + civilization.economy.income * 10.0;
        influence_map.project_influence(&economic_type, *position, economic_strength, 8.0);
        
        // Project cultural influence (based on cities and buildings)
        let cultural_strength = 5.0; // Base cultural influence
        influence_map.project_influence(&cultural_type, *position, cultural_strength, 6.0);
        
        // Project control influence
        influence_map.project_influence(&control_type, *position, 10.0, 5.0);
    }
    
    // Additional influence from cities
    for (city, position) in cities.iter() {
        let civ_id = city.owner;
        let economic_type = InfluenceType::Economic(civ_id);
        let cultural_type = InfluenceType::Cultural(civ_id);
        
        let city_economic_influence = city.population as f32 * 0.5;
        let city_cultural_influence = city.population as f32 * 0.3 + city.buildings.len() as f32 * 2.0;
        
        influence_map.project_influence(&economic_type, *position, city_economic_influence, 6.0);
        influence_map.project_influence(&cultural_type, *position, city_cultural_influence, 8.0);
    }
    
    // Update strategic and threat assessments
    influence_map.update_strategic_layer(&world_map);
    
    let civ_ids: Vec<_> = civs.iter().map(|(civ, _)| civ.id).collect();
    influence_map.update_threat_assessment(&civ_ids);
}

/// System for territorial expansion
pub fn territorial_expansion_system(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    civs: Query<(Entity, &Civilization, &Position), With<AIDecision>>,
) {
    for (entity, civilization, position) in civs.iter() {
        // Check if this civ has an expansion decision
        // For now, just expand to adjacent empty tiles randomly
        let neighbors = world_map.neighbors(*position);
        
        for neighbor_pos in neighbors {
            if let Some(tile) = world_map.get_tile_mut(neighbor_pos) {
                if tile.owner.is_none() && !matches!(tile.terrain, crate::TerrainType::Ocean) {
                    tile.owner = Some(civilization.id);
                    
                    // Create territory component
                    let territory = Territory {
                        owner: civilization.id,
                        control_strength: 1.0,
                        terrain_type: tile.terrain.clone(),
                    };
                    
                    commands.spawn((territory, Position::new(neighbor_pos.x, neighbor_pos.y)));
                    break; // Only expand to one tile per turn
                }
            }
        }
        
        // Remove the decision after processing
        commands.entity(entity).remove::<AIDecision>();
    }
}

/// System for spawning initial civilizations
pub fn spawn_civilizations_system(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    current_turn: Res<CurrentTurn>,
) {
    if current_turn.0 != 1 {
        return; // Only spawn on first turn
    }
    
    let starting_positions = crate::world_gen::get_starting_positions();
    
    for (i, (name, position, color)) in starting_positions.into_iter().take(20).enumerate() {
        let civ_id = CivId(i as u32);
        
        // Create civilization with random personality
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let personality = CivPersonality {
            land_hunger: rng.gen_range(0.2..0.8),
            industry_focus: rng.gen_range(0.2..0.8),
            tech_focus: rng.gen_range(0.2..0.8),
            interventionism: rng.gen_range(0.1..0.7),
            risk_tolerance: rng.gen_range(0.2..0.8),
            honor_treaties: rng.gen_range(0.3..0.9),
            militarism: rng.gen_range(0.2..0.8),
            isolationism: rng.gen_range(0.1..0.6),
        };
        
        let civilization = Civilization {
            id: civ_id,
            name: name.clone(),
            color,
            capital: Some(position),
            personality,
            technologies: crate::Technologies::default(),
            economy: crate::Economy::default(),
            military: crate::Military::default(),
        };
        
        // Spawn civilization entity
        let civ_entity = commands.spawn((
            civilization,
            position,
            ActiveThisTurn,
        )).id();
        
        // Spawn capital city
        let city = City {
            name: format!("{} Capital", name),
            owner: civ_id,
            population: 1000,
            production: 5.0,
            defense: 10.0,
            buildings: vec![
                crate::Building { building_type: crate::BuildingType::Granary, level: 1 }
            ],
        };
        
        commands.spawn((city, position));
        
        // Claim starting territory
        if let Some(tile) = world_map.get_tile_mut(position) {
            tile.owner = Some(civ_id);
            tile.city = Some(name);
        }
        
        // Spawn initial military unit
        let initial_unit = MilitaryUnit {
            id: i as u32,
            unit_type: crate::UnitType::Infantry,
            position,
            strength: 10.0,
            movement_remaining: 2,
            experience: 0.0,
        };
        
        commands.spawn((initial_unit, position));
    }
}

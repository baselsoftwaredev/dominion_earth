use crate::{
    debug_utils::CoreDebugUtils,
    influence_map::{InfluenceMap, InfluenceType},
    resources::{ActiveCivTurn, CurrentTurn, GameRng, TurnPhase},
    AIDecision, ActiveThisTurn, Capital, CapitalAge, City, CivId, CivPersonality, Civilization, MilitaryUnit,
    MovementOrder, Position, Territory, WorldMap,
};
use bevy_ecs::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

/// Core systems for game simulation
pub mod turn_management;
// pub mod movement; // Disabled - module not implemented yet
// Temporarily disabled due to build issues
// pub mod ai_decision;
// pub mod combat_resolution;
// pub mod economic_update;

/// System for managing turn-based gameplay
pub fn turn_based_system(
    mut current_turn: ResMut<CurrentTurn>,
    mut active_civ_turn: ResMut<ActiveCivTurn>,
    mut commands: Commands,
    civs: Query<(Entity, &Civilization), With<Civilization>>,
    mut units: Query<(&mut Position, &mut MilitaryUnit)>,
    world_map: Res<WorldMap>,
    mut rng: ResMut<GameRng>,
    mut turn_advance: ResMut<crate::resources::TurnAdvanceRequest>,
) {
    if !turn_advance.0 {
        return;
    }

    // Initialize civilization list if empty
    if active_civ_turn.civs_per_turn.is_empty() {
        active_civ_turn.civs_per_turn = civs.iter().map(|(_, civ)| civ.id).collect();
        CoreDebugUtils::log_turn_order_init(active_civ_turn.civs_per_turn.len());
    }

    if active_civ_turn.civs_per_turn.is_empty() {
        return; // No civilizations to process
    }

    let current_civ_id = active_civ_turn.civs_per_turn[active_civ_turn.current_civ_index];

    match active_civ_turn.turn_phase {
        TurnPhase::Planning => {
            // Remove all active markers first
            for (entity, _) in civs.iter() {
                commands.entity(entity).remove::<ActiveThisTurn>();
            }

            // Mark only the current civilization as active
            for (entity, civ) in civs.iter() {
                if civ.id == current_civ_id {
                    commands.entity(entity).insert(ActiveThisTurn);
                    CoreDebugUtils::log_civ_turn_active(current_turn.0, &civ.name, &format!("{:?}", civ.id));
                    break;
                }
            }

            active_civ_turn.turn_phase = TurnPhase::Execution;
        }

        TurnPhase::Execution => {
            // Move units belonging to the current civilization
            for (mut pos, mut unit) in units.iter_mut() {
                // Use the actual owner field to check if this unit belongs to the current civilization
                if unit.owner == current_civ_id {
                    // Only move if on a land tile (not Ocean)
                    if let Some(tile) = world_map.get_tile(*pos) {
                        if !matches!(tile.terrain, crate::TerrainType::Ocean) {
                            // Get all adjacent positions
                            let adj = pos.adjacent_positions();
                            // Filter to valid land (non-Ocean) tiles
                            let mut valid_moves = vec![];
                            for p in adj.iter() {
                                if let Some(adj_tile) = world_map.get_tile(*p) {
                                    if !matches!(adj_tile.terrain, crate::TerrainType::Ocean) {
                                        valid_moves.push(*p);
                                    }
                                }
                            }
                            // Move to a random valid adjacent tile if possible
                            if let Some(new_pos) = valid_moves.choose(&mut rng.0) {
                                *pos = *new_pos;
                                unit.position = *new_pos;
                                CoreDebugUtils::log_unit_movement(unit.id, unit.owner.0, new_pos.x, new_pos.y);
                            }
                        }
                    }
                }
            }

            active_civ_turn.turn_phase = TurnPhase::Complete;
        }

        TurnPhase::Complete => {
            // Advance to next civilization
            active_civ_turn.current_civ_index += 1;

            if active_civ_turn.current_civ_index >= active_civ_turn.civs_per_turn.len() {
                // All civilizations have had their turn, advance the game turn
                active_civ_turn.current_civ_index = 0;
                current_turn.0 += 1;
                CoreDebugUtils::log_turn_complete(current_turn.0 - 1, current_turn.0);
            }

            active_civ_turn.turn_phase = TurnPhase::Planning;
        }
    }

    // At the end of the function, reset the flag so the system only runs once per request
    turn_advance.0 = false;
}

/// System for advancing the game turn (legacy - replaced by turn_based_system)
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

    CoreDebugUtils::log_turn_advance(current_turn.0);
}

/// System to mark civilizations as active for the current turn
pub fn activate_civilizations(mut commands: Commands, query: Query<Entity, With<Civilization>>) {
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
                if tile.movement_cost <= 2.0 {
                    // Can move if cost is reasonable
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
        let decision = make_simple_ai_decision(
            civilization,
            position,
            &world_map,
            &mut rng.0,
            current_turn.0,
        );

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
        let techs = [
            "Agriculture",
            "Bronze Working",
            "Writing",
            "Mathematics",
            "Iron Working",
        ];
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
        let city_cultural_influence =
            city.population as f32 * 0.3 + city.buildings.len() as f32 * 2.0;

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
        let _civ_entity = commands
            .spawn((civilization, position, ActiveThisTurn))
            .id();

        // Spawn capital city
        let city = City {
            name: format!("{} Capital", name),
            owner: civ_id,
            population: 1000,
            production: 5.0,
            defense: 10.0,
            buildings: vec![crate::Building {
                building_type: crate::BuildingType::Granary,
                level: 1,
            }],
        };

        let capital = Capital {
            owner: civ_id,
            age: CapitalAge::Neolithic,
            sprite_index: CapitalAge::Neolithic.sprite_index(),
            established_turn: 0, // Starting at turn 0
        };

        // Spawn capital entity with both City and Capital components
        commands.spawn((city, capital, position));

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
            owner: civ_id,
        };

        commands.spawn((initial_unit, position));
    }
}

/// System for managing capital evolution through different ages
pub fn capital_evolution_system(
    mut capitals: Query<(&mut Capital, &City), With<Capital>>,
    civilizations: Query<&Civilization>,
    current_turn: Res<CurrentTurn>,
) {
    for (mut capital, city) in capitals.iter_mut() {
        // Find the owning civilization to check technologies
        if let Ok(civilization) = civilizations
            .iter()
            .find(|civ| civ.id == capital.owner)
            .ok_or("Capital owner not found")
        {
            let requirements = capital.age.evolution_requirements();
            
            // Check if capital can evolve to next age
            if let Some(next_age) = capital.age.next_age() {
                let can_evolve = 
                    // Population requirement
                    city.population >= requirements.min_population &&
                    // Turn requirement
                    current_turn.0 >= capital.established_turn + requirements.min_turn &&
                    // Buildings requirement
                    city.buildings.len() >= requirements.min_buildings &&
                    // Technology requirements
                    requirements.required_technologies.iter().all(|tech| {
                        *civilization.technologies.known.get(tech).unwrap_or(&false)
                    });

                if can_evolve {
                    CoreDebugUtils::log_capital_evolution(
                        &civilization.name, 
                        &format!("{:?}", capital.age), 
                        &format!("{:?}", next_age), 
                        current_turn.0
                    );
                    
                    capital.age = next_age.clone();
                    capital.sprite_index = next_age.sprite_index();
                    capital.established_turn = current_turn.0;
                }
            }
        }
    }
}

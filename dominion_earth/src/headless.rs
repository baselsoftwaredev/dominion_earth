use core_sim::*;
use ai_planner::{ai_coordinator::AICoordinatorSystem};
use rand::SeedableRng;
use std::time::Instant;

/// Run a headless simulation for performance testing and AI validation
pub fn run_headless_simulation() {
    println!("Starting headless simulation...");
    let start_time = Instant::now();

    // Initialize game state
    let mut game_state = initialize_headless_game();
    let mut ai_coordinator = AICoordinatorSystem::new();
    
    let target_turns = 200;
    let mut performance_metrics = PerformanceMetrics::new();

    // Main simulation loop
    for turn in 1..=target_turns {
        let turn_start = Instant::now();

        // Generate AI decisions
        let ai_decisions = ai_coordinator.generate_turn_decisions(&game_state);
        
        // Execute AI decisions
        let mut modified_state = game_state.clone();
        let execution_results = ai_coordinator.execute_decisions(&ai_decisions, &mut modified_state);
        game_state = modified_state;

        // Update turn
        game_state.turn = turn;

        // Update systems (simplified)
        update_economies(&mut game_state);
        update_diplomacy(&mut game_state);
        update_military(&mut game_state);

        let turn_duration = turn_start.elapsed();
        performance_metrics.record_turn(turn, turn_duration, ai_decisions.len(), execution_results.len());

        // Progress reporting
        if turn % 20 == 0 {
            let progress = (turn as f32 / target_turns as f32) * 100.0;
            println!("Turn {}/{} ({:.1}%) - {:.2}ms", turn, target_turns, progress, turn_duration.as_millis());
        }

        // Early termination check (if all civs destroyed)
        if game_state.civilizations.is_empty() {
            println!("Simulation ended early - no civilizations remaining");
            break;
        }
    }

    let total_duration = start_time.elapsed();
    
    // Final statistics
    println!("\n=== Simulation Complete ===");
    println!("Total time: {:?}", total_duration);
    println!("Average turn time: {:.2}ms", total_duration.as_millis() as f32 / target_turns as f32);
    println!("Target met: {}", if total_duration.as_secs() < 2 { "YES" } else { "NO" });
    
    performance_metrics.print_summary();
    print_final_state(&game_state);

    // Save final state for analysis (simplified - no serialization for now)
    println!("Final state: Turn {}, {} civilizations", game_state.turn, game_state.civilizations.len());
    
    // TODO: Implement proper serialization when all types support Serialize/Deserialize
    /*
    if let Err(e) = ron::ser::to_writer_pretty(
        std::fs::File::create("headless_final_state.ron").unwrap(),
        &game_state,
        ron::ser::PrettyConfig::default()
    ) {
        println!("Failed to save final state: {}", e);
    } else {
        println!("Final state saved to headless_final_state.ron");
    }
    */
}

fn initialize_headless_game() -> GameState {
    let mut rng = rand_pcg::Pcg64::seed_from_u64(42); // Deterministic seed
    
    // Generate world
    let _world_map = world_gen::generate_island_map(100, 50, &mut rng);
    
    // Create civilizations
    let starting_positions = world_gen::get_starting_positions();
    let mut civilizations = std::collections::HashMap::new();
    
    for (i, (name, position, color)) in starting_positions.into_iter().take(20).enumerate() {
        let civ_id = CivId(i as u32);
        
        use rand::Rng;
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
            technologies: Technologies::default(),
            economy: Economy::default(),
            military: Military::default(),
        };

        let city = City {
            name: format!("{} Capital", name),
            owner: civ_id,
            population: 1000,
            production: 5.0,
            defense: 10.0,
            buildings: vec![
                Building { building_type: BuildingType::Granary, level: 1 }
            ],
        };

        let _initial_unit = MilitaryUnit {
            id: i as u32,
            owner: civ_id,
            unit_type: UnitType::Infantry,
            position,
            strength: 10.0,
            movement_remaining: 2,
            experience: 0.0,
        };

        let civ_data = CivilizationData {
            civilization,
            cities: vec![city],
            territories: vec![(position, Territory {
                owner: civ_id,
                control_strength: 1.0,
                terrain_type: TerrainType::Plains,
            })],
            diplomatic_relations: vec![],
        };

        civilizations.insert(civ_id, civ_data);
    }

    GameState {
        turn: 1,
        civilizations,
        current_player: Some(CivId(0)),
    }
}

fn update_economies(game_state: &mut GameState) {
    // Simplified economy update - just increase gold over time
    for civ_data in game_state.civilizations.values_mut() {
        civ_data.civilization.economy.gold += civ_data.civilization.economy.income;
        // Simple growth
        civ_data.civilization.economy.income *= 1.001; // 0.1% growth per turn
    }
}

fn update_diplomacy(game_state: &mut GameState) {
    // Simplified diplomacy update - just modify relations over time
    for civ_data in game_state.civilizations.values_mut() {
        // Simple diplomacy: occasionally adjust relations with other civs
        if game_state.turn % 10 == 0 {
            // Every 10 turns, slightly improve relations (simplified)
            for relation in &mut civ_data.diplomatic_relations {
                relation.relation_value += 0.1;
            }
        }
    }
}

fn update_military(_game_state: &mut GameState) {
    // Simplified military updates
    // In a full implementation, this would handle combat, unit movement, etc.
}

struct PerformanceMetrics {
    turn_times: Vec<std::time::Duration>,
    ai_decision_counts: Vec<usize>,
    execution_counts: Vec<usize>,
}

impl PerformanceMetrics {
    fn new() -> Self {
        Self {
            turn_times: Vec::new(),
            ai_decision_counts: Vec::new(),
            execution_counts: Vec::new(),
        }
    }

    fn record_turn(&mut self, _turn: u32, duration: std::time::Duration, decisions: usize, executions: usize) {
        self.turn_times.push(duration);
        self.ai_decision_counts.push(decisions);
        self.execution_counts.push(executions);
    }

    fn print_summary(&self) {
        if self.turn_times.is_empty() {
            return;
        }

        let total_time: std::time::Duration = self.turn_times.iter().sum();
        let avg_time = total_time / self.turn_times.len() as u32;
        let max_time = self.turn_times.iter().max().unwrap();
        let min_time = self.turn_times.iter().min().unwrap();
        
        let avg_decisions = self.ai_decision_counts.iter().sum::<usize>() as f32 / self.ai_decision_counts.len() as f32;
        let avg_executions = self.execution_counts.iter().sum::<usize>() as f32 / self.execution_counts.len() as f32;

        println!("\n=== Performance Metrics ===");
        println!("Turn time - Avg: {:?}, Min: {:?}, Max: {:?}", avg_time, min_time, max_time);
        println!("AI decisions per turn - Avg: {:.1}", avg_decisions);
        println!("AI executions per turn - Avg: {:.1}", avg_executions);
        
        // Check performance target
        let target_met = total_time.as_secs() < 2 && avg_time.as_millis() < 10;
        println!("Performance target (200 turns < 2s, avg < 10ms): {}", 
                if target_met { "MET" } else { "MISSED" });
    }
}

fn print_final_state(game_state: &GameState) {
    println!("\n=== Final Game State ===");
    println!("Turn: {}", game_state.turn);
    println!("Civilizations remaining: {}", game_state.civilizations.len());
    
    // Sort civilizations by score (simplified: gold + military strength)
    let mut scored_civs: Vec<_> = game_state.civilizations.iter()
        .map(|(id, data)| {
            let score = data.civilization.economy.gold + data.civilization.military.total_strength;
            (*id, &data.civilization.name, score)
        })
        .collect();
    
    scored_civs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    println!("\nTop Civilizations:");
    for (i, (civ_id, name, score)) in scored_civs.iter().take(5).enumerate() {
        println!("{}. {} (ID: {:?}) - Score: {:.0}", i + 1, name, civ_id, score);
    }
    
    // Economic statistics
    let total_gold: f32 = game_state.civilizations.values()
        .map(|data| data.civilization.economy.gold)
        .sum();
    
    let total_military: f32 = game_state.civilizations.values()
        .map(|data| data.civilization.military.total_strength)
        .sum();
    
    println!("\nGlobal Statistics:");
    println!("Total gold in world: {:.0}", total_gold);
    println!("Total military strength: {:.0}", total_military);
    println!("Trade routes: 0"); // Simplified - no global economy system yet
    println!("Diplomatic events: 0"); // Simplified - no diplomatic state system yet
}

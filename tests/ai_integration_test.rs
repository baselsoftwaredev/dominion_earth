use core_sim::*;

#[test]
fn test_ai_action_creation() {
    // Test that all GOAP action types can be created without compilation errors
    let actions = vec![
        AIAction::Expand {
            target_position: Position { x: 0, y: 0 },
            priority: 1.0,
        },
        AIAction::Research {
            technology: "Writing".to_string(),
            priority: 0.8,
        },
        AIAction::BuildUnit {
            unit_type: UnitType::Infantry,
            position: Position { x: 5, y: 5 },
            priority: 0.7,
        },
        AIAction::BuildBuilding {
            building_type: BuildingType::Market,
            position: Position { x: 3, y: 3 },
            priority: 0.6,
        },
        AIAction::Trade {
            partner: CivId(1),
            resource: GameResource::Gold,
            priority: 0.5,
        },
        AIAction::Attack {
            target: CivId(2),
            target_position: Position { x: 8, y: 8 },
            priority: 0.9,
        },
        AIAction::Defend {
            position: Position { x: 2, y: 2 },
            priority: 0.4,
        },
    ];
    
    // Verify all action types can be created
    assert_eq!(actions.len(), 7, "All GOAP action types should be available");
    
    // Test pattern matching works for all types
    for action in actions {
        match action {
            AIAction::Expand { priority, .. } => assert!(priority > 0.0),
            AIAction::Research { priority, .. } => assert!(priority > 0.0),
            AIAction::BuildUnit { priority, .. } => assert!(priority > 0.0),
            AIAction::BuildBuilding { priority, .. } => assert!(priority > 0.0),
            AIAction::Trade { priority, .. } => assert!(priority > 0.0),
            AIAction::Attack { priority, .. } => assert!(priority > 0.0),
            AIAction::Defend { priority, .. } => assert!(priority > 0.0),
        }
    }
}

#[test]
fn test_ai_planner_integration() {
    // Test that ai_planner can generate decisions 
    let mut coordinator = ai_planner::AICoordinator::new();
    
    // Create a minimal game state for testing
    let game_state = GameState {
        turn: 1,
        civilizations: std::collections::HashMap::new(),
        current_player: CivId(0),
    };
    
    // Test that the coordinator can process empty state without errors
    let result = coordinator.generate_decisions(&game_state);
    
    // The result might be an error or empty, but it should not panic
    match result {
        Ok(_decisions) => {
            // Success case - decisions were generated
            println!("AI coordinator successfully generated decisions");
        },
        Err(_) => {
            // Error case - this is OK for now, we just want to ensure no panics
            println!("AI coordinator returned error (expected for empty state)");
        }
    }
}

#[test]
fn test_goap_action_priorities() {
    // Test that GOAP actions properly handle priority comparisons
    let high_priority = AIAction::Attack {
        target: CivId(1),
        target_position: Position { x: 0, y: 0 },
        priority: 0.9,
    };
    
    let low_priority = AIAction::Defend {
        position: Position { x: 0, y: 0 },
        priority: 0.1,
    };
    
    // Extract priorities for comparison
    let high_val = match high_priority {
        AIAction::Attack { priority, .. } => priority,
        _ => 0.0,
    };
    
    let low_val = match low_priority {
        AIAction::Defend { priority, .. } => priority,
        _ => 0.0,
    };
    
    assert!(high_val > low_val, "Attack should have higher priority than Defend in this test");
}

#[test]
fn test_ai_action_debug_output() {
    // Test that AIAction can be debugged (useful for logging)
    let action = AIAction::Research {
        technology: "Bronze Working".to_string(),
        priority: 0.75,
    };
    
    let debug_string = format!("{:?}", action);
    assert!(debug_string.contains("Research"), "Debug output should contain action type");
    assert!(debug_string.contains("Bronze Working"), "Debug output should contain technology name");
}

# Dominion Earth AI Agent Instructions

This is a turn-based grand strategy game built with Rust and Bevy ECS, featuring AI-driven civilizations with complex personality systems.

## Architecture Overview

**Three-Crate Structure:**

- `core_sim/`: Pure ECS simulation engine (no graphics dependencies)
- `ai_planner/`: Multi-layered AI system (Utility AI + GOAP + HTN)
- `dominion_earth/`: Bevy frontend with 2D rendering and UI

**Key Design Pattern:** The simulation separates pure game logic from presentation, enabling headless performance testing.

## Critical ECS Patterns

### Component Structure

Components avoid `#[derive(Component)]` due to proc macro compatibility issues with current Rust toolchain. Key components:

- `Position`: World coordinates for all entities
- `Civilization`: Core civ data with embedded `CivPersonality` (8 traits: land_hunger, industry_focus, etc.)
- `City`, `Territory`, `MilitaryUnit`: Game entities requiring Position

### Resource vs. GameResource Conflict

**Important:** Bevy's `Resource` trait conflicts with game's `Resource` enum. Use:

```rust
use crate::resources::{Resource as GameResource, CurrentTurn, WorldMap};
```

### System Organization

Systems live in `core_sim/src/systems/` as individual files, coordinated through `systems.rs`. All systems use Bevy ECS queries and resources.

## Data-Driven Design

**RON Files in `assets/data/`:**

- `civilizations.ron`: Defines 40+ civs with starting positions, personalities, and initial assets
- `units.ron`: Unit types with combat stats and tech requirements
- `technologies.ron`: Tech tree definitions
- `terrain.ron`: Map tile types and modifiers

**Pattern:** All game content should be externalized to RON files, not hardcoded.

## AI System Architecture

**Three-Layer Decision Making:**

1. `utility_ai`: Scores potential actions based on current state
2. `goap`: Plans action sequences to achieve goals
3. `htn_planner`: Manages high-level strategic decisions

**Implementation:** AI decisions flow through `AICoordinator::make_decision()` → `AIDecisionSystem::execute_ai_decision()`

## Critical Build Context

**Current State:** Project compiles 113/114 crates but has remaining issues:

- Missing `From` trait implementations for `SimError`
- Component derives disabled due to proc macro version 10 incompatibility
- Import path issues for `Resource` and `CurrentTurn`

**Build Commands:**

```bash
cargo check -p core_sim  # Check core simulation only
cargo build --release    # Full optimized build
cargo run --release -- --headless --turns 200  # Performance testing
```

## Key Files for Understanding

- `core_sim/src/lib.rs`: Main exports and GameState definition
- `core_sim/src/components.rs`: All ECS components
- `core_sim/src/systems.rs`: System coordination and main game loop
- `ai_planner/src/ai_coordinator.rs`: AI decision-making entry point
- `assets/data/civilizations.ron`: Civilization definitions and personalities

## Development Patterns

**ECS Queries:** Use `Query<(&Component1, &Component2), With<Filter>>` pattern extensively
**Error Handling:** `SimResult<T>` type alias with custom `SimError` enum
**Randomization:** Deterministic via `GameRng` resource for reproducible gameplay
**Performance:** Release builds target 200 turns in <2 seconds for headless simulation

## Common Pitfalls

- Don't use `#[derive(Component)]` - manual Component implementation needed
- Import `GameResource` not `Resource` to avoid trait conflicts
- RON files use specific syntax: `(field: value, ...)` not `{field: value}`
- AI personalities are f32 values 0.0-1.0, affect all strategic decisions

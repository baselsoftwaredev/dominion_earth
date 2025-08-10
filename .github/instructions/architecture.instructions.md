# Dominion Earth AI Agent Instructions

This is a turn-based, 4-directional tile-based grand strategy game built with Rust and Bevy ECS, featuring AI-driven civilizations with complex personality systems.

## Architecture Overview

**Three-Crate Workspace Structure:**

- `core_sim/`: Pure ECS simulation engine (no graphics dependencies)
- `ai_planner/`: Multi-layered AI system (Utility AI + GOAP + HTN)
- `dominion_earth/`: Bevy frontend with 2D rendering and UI

**Key Design Pattern:** Clean separation between game logic and presentation enables headless performance testing and modular development.

## Current Build Status

**Build Commands:**

```bash
cargo check -p core_sim      # ✅ Builds successfully
cargo check -p ai_planner    # ✅ Builds successfully
cargo check                  # ✅ Builds successfully (with warnings)
```

**Build Status:** All crates now build successfully. Only minor warnings remain for unused variables and dead code.

## Critical ECS Patterns

### Manual Component Implementation Required

**CRITICAL:** All components use manual `Component` trait implementation (no `#[derive(Component)]`):

```rust
impl Component for YourComponent {
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}
```

### Resource Naming Conflict

**CRITICAL:** Bevy's `Resource` trait conflicts with game's `Resource` enum:

```rust
// Always use this import pattern
use crate::resources::{Resource as GameResource, CurrentTurn, WorldMap};
```

### System Organization Pattern

- Systems organized in `core_sim/src/systems/` as individual modules
- Central coordination through `systems.rs`
- Standard ECS query patterns: `Query<(&Component1, &Component2), With<Filter>>`

## 4-Directional Movement System

**CRITICAL CONSTRAINT:** All movement restricted to cardinal directions only:

```rust
// Position provides 4-directional methods
impl Position {
    pub fn adjacent_positions(&self) -> [Position; 4] {
        [
            Position::new(self.x, self.y + 1), // North
            Position::new(self.x, self.y - 1), // South
            Position::new(self.x + 1, self.y), // East
            Position::new(self.x - 1, self.y), // West
        ]
    }
    pub fn manhattan_distance_to(&self, other: &Position) -> i32
    pub fn is_adjacent_to(&self, other: &Position) -> bool
}
```

- Pathfinding uses Manhattan distance heuristic
- No diagonal movement allowed
- Direction enum: `North/South/East/West` only

## Data-Driven Configuration

**RON File Locations:** `dominion_earth/assets/data/`

- `civilizations.ron`: 40+ civs with positions, personalities (8 f32 traits 0.0-1.0), starting assets
- `units.ron`, `technologies.ron`, `terrain.ron`: Game content definitions

**RON Syntax Pattern:** `(field: value, ...)` not `{field: value}`

**CivPersonality Traits:**

```ron
personality: (
    land_hunger: 0.6,      // Expansion aggressiveness
    industry_focus: 0.7,   // Production preference
    tech_focus: 0.8,       // Research priority
    interventionism: 0.4,  // Foreign engagement
    risk_tolerance: 0.5,   // Decision boldness
    honor_treaties: 0.8,   // Diplomatic reliability
    militarism: 0.5,       // Military preference
    isolationism: 0.3,     // Isolation tendency
),
```

## AI Architecture

**Three-Layer Decision System:**

1. **Utility AI**: Scores actions based on current state
2. **GOAP**: Plans action sequences for goal achievement
3. **HTN**: High-level strategic decision management

**Decision Flow:** `AICoordinator::generate_decisions()` → `AIDecisionSystem::execute_ai_decision()`

**AIAction Types:** `Expand`, `Research`, `BuildUnit`, `BuildBuilding`, `Trade`, `Attack`, `Diplomacy`, `Defend`

## Performance & Testing

**Headless Mode:**

```bash
cargo run --release -- --headless --turns 200
```

**Performance Target:** 200 turns in <2 seconds (release mode)

**Deterministic Simulation:** Uses `GameRng` resource with seeded `rand_pcg` for reproducible results

## Key Files for Understanding

- `core_sim/src/lib.rs`: Core exports, GameState, SimError definitions
- `core_sim/src/components.rs`: All ECS components with manual implementations
- `core_sim/src/systems.rs`: System coordination and game loop
- `ai_planner/src/ai_coordinator.rs`: Multi-layer AI decision making
- `dominion_earth/src/main.rs`: Bevy app setup with headless/GUI modes
- `dominion_earth/assets/data/civilizations.ron`: Civ definitions and personalities

## Common Development Patterns

**Error Handling:** `SimResult<T>` type alias with custom `SimError` enum

**Serde Integration:** Mix of `#[derive(Serialize, Deserialize)]` and manual implementations for complex types

**Release Optimization:** LTO enabled, single codegen unit, panic=abort for performance

## Critical Pitfalls to Avoid

- **Never** use `#[derive(Component)]` - manual implementation required
- **Always** import `Resource as GameResource` to avoid trait conflicts
- **Remember** RON syntax uses parentheses, not braces
- **Ensure** all movement logic respects 4-directional constraint
- **Verify** `From` trait implementations exist for `SimError` variants
- **Test** changes with both `cargo run` and `cargo run -- --headless`

## Current Technical Debt

1. **Frontend Type Issues**: CivId string conversion needed in rendering system
2. **System Module Organization**: Some systems temporarily disabled in `systems.rs`
3. **Manual Component Maintenance**: Overhead from avoiding derive macros

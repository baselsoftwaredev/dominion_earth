# Dominion Earth AI Agent Instructions

This is a turn-based, 4-directional tile-based grand strategy game built with Rust and Bevy ECS, featuring AI-driven civilizations with complex personality systems.

## Architecture Overview

**Three-Crate Structure:**

- `core_sim/`: Pure ECS simulation engine (no graphics dependencies)
- `ai_planner/`: Multi-layered AI system (Utility AI + GOAP + HTN)
- `dominion_earth/`: Bevy frontend with 2D rendering and UI

**Key Design Pattern:** The simulation separates pure game logic from presentation, enabling headless performance testing.

## Movement System & Game Mechanics

**4-Directional Tile-Based Movement:**

- Movement restricted to cardinal directions only: North, South, East, West
- No diagonal movement allowed
- `Position` struct enhanced with movement methods: `adjacent_positions()`, `manhattan_distance_to()`, `is_adjacent_to()`, `direction_to()`, `move_in_direction()`
- `Direction` enum provides `North/South/East/West` with utility methods like `opposite()` and `all()`
- Pathfinding uses Manhattan distance heuristic for optimal 4-directional paths

**CRITICAL:** Update `WorldMap::neighbors()` to use 4-directional only:
```rust
// WRONG (8-directional - current implementation)
let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

// CORRECT (4-directional - needed fix)
let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // North, South, East, West
```

## Critical ECS Patterns

### Component Structure

Components avoid `#[derive(Component)]` due to proc macro metadata version 10 incompatibility. All components require manual implementation:

```rust
impl Component for YourComponent {
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}
```

**Key Components:**
- `Position`: World coordinates with 4-directional movement methods
- `Direction`: Cardinal directions enum (North/South/East/West only)
- `Civilization`: Core civ data with embedded `CivPersonality` (8 traits: land_hunger, industry_focus, etc.)
- `City`, `Territory`, `MilitaryUnit`: Game entities requiring Position
- `MovementOrder`: For unit movement with pathfinding
- `AIAction`: GOAP-based AI decisions (moved from ai_planner to core_sim)

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

**GOAP Integration:** `AIAction` enum moved from ai_planner to core_sim to resolve circular dependencies. AI actions include:
- `Expand`, `Research`, `BuildUnit`, `BuildBuilding`, `Trade`, `Attack`, `Diplomacy`, `Defend`
- Each action has priority scoring and position targeting for spatial decision-making

## Critical Build Context

**Current State:** Project has mixed build status:

- ✅ **core_sim crate**: Builds successfully with warnings only
- ❌ **ai_planner crate**: Multiple build errors requiring fixes
- ❌ **dominion_earth crate**: Depends on ai_planner fixes

**Core Issues Identified:**

1. **Manual Component trait implementations** for all ECS components (no `#[derive(Component)]`)
2. **Proc macro metadata version 10 incompatibility** with current Rust toolchain  
3. **Import path management** for `Resource` vs `GameResource` conflicts
4. **4-directional movement system** requires `WorldMap::neighbors()` update (✅ FIXED)
5. **AI planner architectural mismatches** - missing fields in GameState, outdated module references
6. **Missing serde implementations** for serialization/deserialization (✅ PARTIALLY FIXED)

**Build Commands:**

```bash
cargo check -p core_sim  # ✅ Builds successfully  
cargo check -p ai_planner # ❌ Multiple errors requiring fixes
cargo build --release    # ❌ Blocked by ai_planner issues
cargo run --release -- --headless --turns 200  # ❌ Blocked by build issues
```

**Immediate Fix Requirements:**

```rust
// 1. Add missing From trait implementations (✅ COMPLETED)
impl From<ron::Error> for SimError { /* ... */ }
impl From<serde_json::Error> for SimError { /* ... */ }
impl From<std::io::Error> for SimError { /* ... */ }

// 2. Update GameState structure to include missing fields
pub struct GameState {
    pub turn: u32,
    pub civilizations: HashMap<CivId, CivilizationData>,
    pub current_player: Option<CivId>,
    // TODO: Add these back when implementing full features:
    // pub world_map: WorldMap,
    // pub diplomatic_state: DiplomaticState,
}

// 3. Fix ai_planner module references 
use core_sim::{DiplomaticAction, GameResource as Resource};
// instead of core_sim::diplomacy::DiplomaticAction
```

## Key Files for Understanding

- `core_sim/src/lib.rs`: Main exports and GameState definition with SimError enum
- `core_sim/src/components.rs`: All ECS components with manual Component implementations and 4-directional movement
- `core_sim/src/systems.rs`: System coordination and main game loop
- `core_sim/src/pathfinding.rs`: A* pathfinding with Manhattan distance heuristic for 4-directional movement
- `core_sim/src/resources.rs`: WorldMap with neighbors() method (needs 4-directional fix)
- `ai_planner/src/ai_coordinator.rs`: AI decision-making entry point
- `assets/data/civilizations.ron`: Civilization definitions and personalities

## Development Patterns

**ECS Queries:** Use `Query<(&Component1, &Component2), With<Filter>>` pattern extensively
**Error Handling:** `SimResult<T>` type alias with custom `SimError` enum
**Randomization:** Deterministic via `GameRng` resource for reproducible gameplay
**Performance:** Release builds target 200 turns in <2 seconds for headless simulation
**Movement System:** 4-directional pathfinding with `Position` methods and `Direction` enum

## Common Pitfalls

- Don't use `#[derive(Component)]` - manual Component implementation needed
- Import `GameResource` not `Resource` to avoid trait conflicts
- RON files use specific syntax: `(field: value, ...)` not `{field: value}`
- AI personalities are f32 values 0.0-1.0, affect all strategic decisions
- Movement system MUST use 4-directional only - ✅ `WorldMap::neighbors()` method fixed
- Remember to add `From` trait implementations for `SimError` variants (✅ COMPLETED)
- Add `#[derive(Serialize, Deserialize)]` to types used in RON files - manual implementation required for complex types
- AI planner crate needs significant refactoring to match current GameState structure
- GameState currently simplified - missing world_map and diplomatic_state fields that ai_planner expects

## Known Technical Debt

1. **AI Planner Integration**: ai_planner expects GameState fields not yet implemented:
   - `game_state.world_map` - needs to be added as Resource or field
   - `game_state.diplomatic_state` - diplomatic system needs implementation
   
2. **Component Architecture**: Manual Component implementations work but create maintenance overhead

3. **Serde Integration**: Mix of manual and derived implementations needs standardization

4. **Module Organization**: Some cross-crate dependencies assume modules that don't exist (e.g., `core_sim::diplomacy`)

## Development Priority Order

1. ✅ **Core ECS Foundation** - Components, basic simulation
2. ✅ **4-Directional Movement** - Tile-based movement system  
3. ✅ **Manual Component Traits** - Work around proc macro issues
4. 🔄 **AI Planner Fixes** - Update to match current GameState
5. ❌ **Full Diplomatic System** - Implement missing diplomatic features
6. ❌ **World Map Integration** - Add WorldMap to GameState or as Resource
7. ❌ **Performance Optimization** - Headless simulation targets

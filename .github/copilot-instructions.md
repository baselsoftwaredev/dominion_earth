# Dominion Earth - AI Coding Instructions

## Architecture

- **core_sim/**: Pure ECS simulation engine using `bevy_ecs` (no graphics dependencies)
- **ai_planner/**: Multi-layered AI system (Utility AI + GOAP + HTN planning)
- **dominion_earth/**: Bevy frontend with 2D rendering and UI
- **assets/data/**: Game content defined in RON files

## Essential Commands

**Always use debug seed for consistency: `--seed 1756118413`**

```bash
cargo build                                              # Debug build
cargo run -- --seed 1756118413                         # Run game
cargo run -- --seed 1756118413 --debug-logging        # With debug output
```

## Data-Driven Design

Game content lives in `dominion_earth/assets/data/*.ron` - modify RON files rather than hardcoding in Rust.

## Core Principles

- **core_sim** is pure ECS - no graphics dependencies
- Components in modular structure: `core_sim/src/components/`
- Systems follow turn-based patterns: AI planning → execution → world update
- Use `bevy_ecs::Resource` for global state

## Coding Standards

### NEVER use comments - make code self-documenting:

```rust
// BAD: Comments explain unclear code
// Calculate distance between two points
fn calc(a: Vec2, b: Vec2) -> f32 { ... }

// GOOD: Function name explains itself
fn calculate_euclidean_distance_between_points(first_point: Vec2, second_point: Vec2) -> f32 { ... }
```

### ALL magic values must be constants:

```rust
// BAD: Magic numbers
if unit.hp < 10 { unit.retreat(); }
sprite_index = 3;

// GOOD: Named constants
if unit.health_points < CRITICAL_HEALTH_THRESHOLD { initiate_unit_retreat(&mut unit); }
sprite_index = CAPITAL_SPRITE_INDEX;
```

### Constants organization:

```rust
pub mod constants {
    pub mod combat {
        pub const CRITICAL_HEALTH_THRESHOLD: i32 = 10;
    }
    pub mod rendering {
        pub const CAPITAL_SPRITE_INDEX: usize = 3;
    }
}
```

### Naming conventions:

- Functions: `verb_noun_context()` - `calculate_unit_movement_cost()`
- Variables: Full descriptive names - `remaining_movement_points` not `mp`
- Constants: `SCREAMING_SNAKE_CASE` - `MAXIMUM_UNITS_PER_CIVILIZATION`

### Prefer many small functions over large complex ones

**Code structure:**

- Use early returns and guard clauses
- Keep functions under 20 lines when possible
- Each function does one thing well
- Extract complex logic into well-named functions

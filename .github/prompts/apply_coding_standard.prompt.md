---
mode: agent
---

# Apply Coding Standard

## Workflow

1. **First, run `git status` to identify changed files**
2. **Apply the coding standards below to all modified Rust files**
3. **Focus on files with `.rs` extension that have been modified or added**

Apply the below coding standards to the provided Rust code. Ensure all code adheres to these standards.

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

### Debug Logging Standards:

**NEVER use println!, eprintln!, or dbg! directly**. Always use the debug logging system:

```rust
// BAD: Direct terminal output
println!("Unit moved to position {}", position);
eprintln!("Error occurred");

// GOOD: Use debug logging system
use crate::debug_println;
debug_println!(debug_logging, "Unit moved to position {}", position);

// Or use specific debug utility functions
DebugUtils::log_unit_movement(&debug_logging, unit_id, position);
```

Debug logging rules:

- Import `use crate::debug_println;` macro for formatted debug output
- All debug output must respect the `--debug-logging` flag
- System messages (like "Using custom random seed") can use direct println! as they're important config info
- Create specific debug logging functions in `debug_utils.rs` for common patterns
- Debug output only appears when `--debug-logging` flag is used

### Prefer many small functions over large complex ones

**Code structure:**

- Use early returns and guard clauses
- Keep functions under 20 lines when possible
- Each function does one thing well
- Extract complex logic into well-named functions
- Files should not exceed 500 lines; split into modules if necessary

### Code Reusability

**Design code to be reusable across multiple contexts:**

```rust
// BAD: Hardcoded, single-use function
fn spawn_warrior_at_capital() {
    let position = Vec2::new(10.0, 10.0);
    spawn_unit(position, UnitType::Warrior);
}

// GOOD: Parameterized, reusable function
fn spawn_unit_at_position(position: Vec2, unit_type: UnitType) {
    spawn_unit(position, unit_type);
}

// BETTER: Generic, composable function
fn spawn_entity_at_position<T: Bundle>(position: Vec2, entity_bundle: T, commands: &mut Commands) {
    commands.spawn((entity_bundle, Position(position)));
}
```

**Reusability principles:**

- Design functions to accept parameters rather than hardcoding values
- Create utility modules for common operations (e.g., `math_utils`, `spawn_utils`)
- Use traits to enable polymorphic behavior across different types
- Build composable functions that can be combined in different ways
- Extract business logic from Bevy systems so it can be tested and reused independently
- Prefer returning values over mutating global state when possible
- Design APIs that work in multiple contexts, not just the immediate use case

**Example of reusable utility:**

```rust
// In utilities module
pub mod position_utils {
    pub fn calculate_distance_between_entities(pos_a: Vec2, pos_b: Vec2) -> f32 {
        pos_a.distance(pos_b)
    }

    pub fn is_entity_within_range(entity_pos: Vec2, target_pos: Vec2, range: f32) -> bool {
        calculate_distance_between_entities(entity_pos, target_pos) <= range
    }

    pub fn find_nearest_position(origin: Vec2, candidates: &[Vec2]) -> Option<Vec2> {
        candidates.iter()
            .min_by(|a, b| {
                calculate_distance_between_entities(origin, **a)
                    .partial_cmp(&calculate_distance_between_entities(origin, **b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }
}
```

### DRY Principle (Don't Repeat Yourself)

**Extract repeated code into functions:**

```rust
// BAD: Duplicated logic
fn process_infantry_order(commands: &mut Commands, gold: &mut u32) {
    if *gold >= INFANTRY_COST {
        *gold -= INFANTRY_COST;
        spawn_unit(commands, UnitType::Infantry);
        update_ui_gold_display(commands, *gold);
        log_unit_production("Infantry", *gold);
    }
}

fn process_archer_order(commands: &mut Commands, gold: &mut u32) {
    if *gold >= ARCHER_COST {
        *gold -= ARCHER_COST;
        spawn_unit(commands, UnitType::Archer);
        update_ui_gold_display(commands, *gold);
        log_unit_production("Archer", *gold);
    }
}

// GOOD: Extract common pattern
fn process_unit_production_order(
    commands: &mut Commands,
    gold: &mut u32,
    unit_type: UnitType,
    unit_cost: u32
) {
    if !can_afford_unit_production(*gold, unit_cost) {
        return;
    }

    deduct_unit_production_cost(gold, unit_cost);
    spawn_unit(commands, unit_type);
    update_ui_gold_display(commands, *gold);
    log_unit_production(&format!("{:?}", unit_type), *gold);
}

fn process_infantry_order(commands: &mut Commands, gold: &mut u32) {
    process_unit_production_order(commands, gold, UnitType::Infantry, INFANTRY_COST);
}

fn process_archer_order(commands: &mut Commands, gold: &mut u32) {
    process_unit_production_order(commands, gold, UnitType::Archer, ARCHER_COST);
}
```

**DRY Rules:**

- If you copy-paste code more than once, extract it into a function
- Look for similar patterns with small variations - parameterize the differences
- Extract complex condition checks into well-named boolean functions
- Use generic functions and traits when appropriate to avoid type-specific duplication
- Shared constants should be defined once in a constants module

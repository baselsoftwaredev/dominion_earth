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

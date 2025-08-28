# Dominion Earth - AI Coding Instructions

## Architecture Overview

This is a modular Rust + Bevy 0.16 grand strategy game with a performance-optimized, data-driven architecture:

- **core_sim/**: Pure ECS simulation engine using `bevy_ecs` (no graphics dependencies)
- **ai_planner/**: Multi-layered AI system (Utility AI + GOAP + HTN planning)
- **dominion_earth/**: Bevy frontend with 2D rendering and UI
- **assets/data/**: Game content defined in RON files (civilizations, units, technologies)

## Key Development Patterns

### Essential CLI Commands

**Always use the default debug seed** for consistent debugging: `--seed 1756118413`

```bash
# Standard development build (use debug for faster compilation)
cargo build

# Run with GUI and debug seed
cargo run -- --seed 1756118413

# Enable debug logging for detailed output
RUST_LOG=debug cargo run -- --seed 1756118413 --debug-logging

# Check available CLI options
cargo run -- --help
```

### Reading Debug Outputs

Enable debug logging to see detailed information:

# Enable all debug logging for comprehensive output

```RUST
RUST_LOG=debug cargo run -- --seed 1756118413 --debug-logging
```

### Data-Driven Design

All game content lives in `dominion_earth/assets/data/*.ron`:

- **civilizations.ron**: Starting positions, personalities (8 traits: land_hunger, tech_focus, etc.), initial units/buildings
- **units.ron**: Combat stats, movement, required technologies
- **technologies.ron**: Tech tree dependencies and effects
- **terrain.ron**: Tile types and resource yields

When adding game content, modify RON files rather than hardcoding in Rust.

### ECS & Tilemap Architecture

This project uses **bevy_ecs_tilemap** for efficient 2D tile rendering. Key documentation:

- [bevy_ecs_tilemap docs](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/)
- Tilemap entities are separate from game logic entities
- Tile indices correspond to sprite sheet positions in `assets/tiles/sprite-sheet.png`

**Core ECS Principles**:

- **core_sim** is pure ECS - no graphics dependencies, designed for performance
- Components live in `core_sim/src/components.rs` with clear separation (Position, CivId, Unit types)
- Systems in `core_sim/src/systems/` follow turn-based patterns (AI planning → execution → world update)
- Use `bevy_ecs::Resource` for global state, avoid direct field access

### AI System Integration

Three-layer AI approach in `ai_planner/`:

- **Utility AI**: Scores potential actions based on game state
- **GOAP**: Plans action sequences to achieve goals
- **HTN**: High-level strategic decision making

When extending AI, add new actions to the `AIAction` enum and implement scoring in all three layers.

## Gameplay Research & Inspiration

### 4X Game Repositories for Reference

When implementing gameplay features, study these open-source 4X games with similar mechanics to Civilization:

- **[Unciv](https://github.com/yairm210/Unciv)** - Kotlin-based Civilization V clone with excellent turn-based mechanics
- **[C7](https://github.com/C7-Game/Prototype)** - Modern 4X game prototype with innovative design patterns
- **[OpenCiv1](https://github.com/rajko-horvat/OpenCiv1)** - Open source implementation of original Civilization
- **[FreeOrion](https://github.com/freeorion/freeorion)** - Space-based 4X with complex AI and game systems
- **[Freeciv21](https://github.com/longturn/freeciv21)** - Modern Qt-based Freeciv implementation

### Repository Analysis Tools

Use these DeepWiki MCP tools to study the referenced codebases:

```bash
# Get repository structure and documentation topics
mcp_deepwiki_read_wiki_structure("yairm210/Unciv")

# Read specific documentation about gameplay systems
mcp_deepwiki_read_wiki_contents("yairm210/Unciv")

# Ask targeted questions about implementation details
mcp_deepwiki_ask_question("yairm210/Unciv", "How does the AI system make tactical combat decisions?")
mcp_deepwiki_ask_question("C7-Game/Prototype", "What data structures are used for tile-based world representation?")
mcp_deepwiki_ask_question("freeorion/freeorion", "How is turn-based progression implemented with multiple players?")
```

**Research Workflow:**

1. Use `read_wiki_structure` to understand repository organization
2. Use `read_wiki_contents` to study architectural decisions
3. Use `ask_question` to get specific implementation guidance
4. Adapt successful patterns to Dominion Earth's ECS architecture

## VS Code Development Tools

**Development Workflow:**

1. Use `get_workspace_symbols` to find existing ECS components/systems
2. Use `find_usages` to understand how game mechanics are implemented
3. Use `get_call_hierarchy` to trace system execution flow
4. Use `rename` for safe refactoring across the modular architecture

### Code Organization

- **Avoid** putting graphics code in `core_sim` - keep it pure ECS
- **Use** workspace dependencies in `Cargo.toml` for version consistency
- **Follow** turn-based system ordering: AI decisions → execution → world updates
- **Always use** the debug seed `--seed 1756118413` for reproducible testing

### Common Gotchas

- RON file syntax requires trailing commas in arrays/tuples
- `core_sim::Resource` conflicts with `bevy_ecs::Resource` - use qualified imports
- Tilemap rendering requires proper sprite sheet tile indices (see coast logic for examples)

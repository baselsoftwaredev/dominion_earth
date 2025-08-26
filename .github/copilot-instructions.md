# Dominion Earth - AI Coding Instructions

## Architecture Overview

This is a modular Rust + Bevy 0.16 grand strategy game with a performance-optimized, data-driven architecture:

- **core_sim/**: Pure ECS simulation engine using `bevy_ecs` (no graphics dependencies)
- **ai_planner/**: Multi-layered AI system (Utility AI + GOAP + HTN planning)  
- **dominion_earth/**: Bevy frontend with 2D rendering, UI, and BRP debugging support
- **assets/data/**: Game content defined in RON files (civilizations, units, technologies)

## Key Development Patterns

### Essential CLI Commands

**Always use the default debug seed** for consistent debugging: `--seed 1756118413`

```bash
# Standard development build
cargo build --release

# Run with GUI and debug seed
cargo run --release -- --seed 1756118413

# Run headless simulation for performance testing
cargo run --release -- --headless --turns 200 --seed 1756118413

# Enable debug logging for detailed output
RUST_LOG=debug cargo run --release -- --seed 1756118413 --debug-logging

# Check available CLI options
cargo run -- --help
```

### BRP Debugging Workflow (CRITICAL)

**MANDATORY PATTERN**: BRP commands require the app to be running first with a sleep delay:

```bash
# Start app with BRP enabled in background, wait for startup, then query
cargo run -- --enable-remote --seed 1756118413 & sleep 10 && curl -X POST http://localhost:15702/brp_extras/screenshot -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "brp_extras/screenshot", "params": {"path": "/tmp/coast_rotation_verification.png"}}'

# Query entity count
cargo run -- --enable-remote --seed 1756118413 & sleep 10 && curl -X POST http://localhost:15702/bevy/list -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "bevy/list"}' | jq '.result | length'

# Query specific components
cargo run -- --enable-remote --seed 1756118413 & sleep 10 && curl -X POST http://localhost:15702/bevy/query -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "bevy/query", "params": {"data": {"components": ["bevy_transform::components::transform::Transform"]}}}' | jq '.result | length'
```

**Why the sleep is required**: The Bevy app needs time to fully initialize the BRP server before accepting connections.

### Reading Debug Outputs

Enable debug logging to see detailed information:

```bash
# See coast generation and tile neighbor logic
RUST_LOG=debug cargo run -- --seed 1756118413 --debug-logging

# View AI decision making process
RUST_LOG=ai_planner=debug cargo run -- --seed 1756118413

# Monitor turn progression and system execution
RUST_LOG=core_sim::systems=debug cargo run -- --seed 1756118413
```

Key debug output patterns to watch for:
- Coast tile conversions and tile index assignments
- AI coordinator decision generation and execution
- Turn advancement and civilization state changes
- Entity spawning and component assignments

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
- Coast tiles use specific indices (8, 9, 1, 2) based on ocean neighbor patterns

**Core ECS Principles**:
- **core_sim** is pure ECS - no graphics dependencies, designed for headless performance
- Components live in `core_sim/src/components.rs` with clear separation (Position, CivId, Unit types)
- Systems in `core_sim/src/systems/` follow turn-based patterns (AI planning → execution → world update)
- Use `bevy_ecs::Resource` for global state, avoid direct field access

### AI System Integration

Three-layer AI approach in `ai_planner/`:
- **Utility AI**: Scores potential actions based on game state
- **GOAP**: Plans action sequences to achieve goals
- **HTN**: High-level strategic decision making

When extending AI, add new actions to the `AIAction` enum and implement scoring in all three layers.

### Performance & Testing

- Release mode targets 200 turns in <2 seconds for headless simulation
- Use `test_coast_logic.rs` pattern for isolated algorithm testing
- Integration tests in `tests/` verify AI action compilation and execution
- Profile with headless mode: `cargo run --release -- --headless --turns 200 --seed 1756118413`

### Bevy Remote Protocol (BRP) Development Workflow

**CRITICAL**: Always use the background + sleep pattern for BRP operations:

1. **Start app in background**: `cargo run -- --enable-remote --seed 1756118413 &`
2. **Wait for initialization**: `sleep 10` (app needs time to start BRP server)
3. **Execute BRP commands**: Use curl with proper JSON-RPC format

Essential BRP operations:
```bash
# Take screenshot for debugging
curl -X POST http://localhost:15702/brp_extras/screenshot -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "brp_extras/screenshot", "params": {"path": "/tmp/debug_screenshot.png"}}'

# List all registered components
curl -X POST http://localhost:15702/bevy/list -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "bevy/list"}' | jq

# Discover component format for spawning/modifying
curl -X POST http://localhost:15702/brp_extras/discover_format -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "brp_extras/discover_format", "params": {"types": ["bevy_transform::components::transform::Transform"]}}'
```

### Code Organization

- **Avoid** putting graphics code in `core_sim` - keep it pure ECS
- **Use** workspace dependencies in `Cargo.toml` for version consistency
- **Follow** turn-based system ordering: AI decisions → execution → world updates
- **Test** new algorithms in standalone files (like `test_coast_logic.rs`) before integration
- **Always use** the debug seed `--seed 1756118413` for reproducible testing

### Common Gotchas

- RON file syntax requires trailing commas in arrays/tuples
- `core_sim::Resource` conflicts with `bevy_ecs::Resource` - use qualified imports
- Headless mode requires different plugin setup than GUI mode
- BRP requires `"bevy_remote"` and `"png"` features enabled in Bevy
- **BRP commands fail without the sleep delay** - app startup takes time
- Tilemap rendering requires proper sprite sheet tile indices (see coast logic for examples)

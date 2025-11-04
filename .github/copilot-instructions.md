# Dominion Earth - AI Coding Instructions

If you don't understand something, ask for clarification!
If you don't know how to do something, ask for help!
If you don't understad my requirements, ask questions!

## Architecture

- **core_sim/**: Pure ECS simulation engine using `bevy_ecs` (no graphics dependencies)
- **ai_planner/**: Multi-layered AI system (Utility AI + GOAP + HTN planning)
- **dominion_earth/**: Bevy frontend with 2D rendering and UI
- **assets/data/**: Game content defined in RON files
- **core_sim** is pure ECS - no graphics dependencies
- Components in modular structure: `core_sim/src/components/`
- Systems follow turn-based patterns: AI planning → execution → world update
- Use `bevy_ecs::Resource` for global state

## Essential Commands

Always use these commands for building and running the game. Don't use the `--release` flag unless instructed.

```bash
cargo run -- --debug-logging        # With debug output
```

## Data-Driven Design

Game content lives in `dominion_earth/assets/data/*.ron` - modify RON files rather than hardcoding in Rust.

## Cargo

Don't install new dependencies without approval.

## Documentation

When making changes to systems or architecture:

- **DO** update relevant documentation in `docs/` folder (e.g., `event_system.md`, `fog_of_war.md`, etc.)
- **DON'T** modify anything in `docs/bevy_examples/` folder - those are reference examples only
- Keep docs synchronized with code changes to maintain accuracy

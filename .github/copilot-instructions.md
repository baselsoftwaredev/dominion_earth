# Dominion Earth - AI Coding Instructions

## Architecture

- **core_sim/**: Pure ECS simulation engine using `bevy_ecs` (no graphics dependencies)
- **ai_planner/**: Multi-layered AI system (Utility AI + GOAP + HTN planning)
- **dominion_earth/**: Bevy frontend with 2D rendering and UI
- **assets/data/**: Game content defined in RON files

## Essential Commands

**Always use debug seed for consistency: `--seed 1756118413`**

```bash
cargo run -- --seed 1756118413 --debug-logging        # With debug output
```

## Data-Driven Design

Game content lives in `dominion_earth/assets/data/*.ron` - modify RON files rather than hardcoding in Rust.

## Core Principles

- **core_sim** is pure ECS - no graphics dependencies
- Components in modular structure: `core_sim/src/components/`
- Systems follow turn-based patterns: AI planning → execution → world update
- Use `bevy_ecs::Resource` for global state


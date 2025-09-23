# Dominion Earth Save System

This directory contains saved game states for Dominion Earth.

## Save Files

Save files are stored in RON (Rust Object Notation) format with the `.scn.ron` extension.
Each save file contains a serialized Bevy scene with all the game entities and resources.

## Quick Save/Load

- **F5**: Quick Save (saves to `quicksave.scn.ron`)
- **F9**: Quick Load (loads from `quicksave.scn.ron`)

## What Gets Saved

The save system captures:

### Components

- Position: Entity locations on the world map
- Civilization: Civilization data, personalities, technologies, economy
- City: City information, buildings, population
- Military Unit: Unit types, positions, experience
- Terrain Type: Terrain information for tiles

### Resources

- Current Turn: The current turn number
- Active Civ Turn: Which civilization is currently active
- World Map: Map dimensions and basic tile data
- Turn Phase: Current phase of the turn (Planning/Execution/Complete)

### What's NOT Saved (Currently)

Some complex data structures are skipped in the current implementation:

- HashMap-based data (trade routes, resource prices)
- Complex diplomatic relations
- Full tile details (resources are skipped)

This ensures the save system is stable and fast, while capturing the essential game state.

## Technical Details

The save system uses Bevy's reflection system to automatically serialize/deserialize components.
All saved components must implement the `Reflect` trait with `#[reflect(Component)]` or `#[reflect(Resource)]`.

Save files are created asynchronously to avoid blocking the main game thread.

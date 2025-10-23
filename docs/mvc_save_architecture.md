# MVC Save/Load Architecture (moonshine-save Philosophy)

## Overview

This document explains the Model-View-Controller (MVC) architecture applied to the save/load system, following the philosophy of the [moonshine-save](https://github.com/Zeenobit/moonshine_save) crate.

## MVC Philosophy

The core principle is to separate game logic (Model) from visual presentation (View):

- **Model**: Core game state that should be saved/loaded
- **View**: Visual and aesthetic elements that should NOT be saved
- **Controller**: Systems that handle save/load events

This separation ensures that:

1. Only essential game state is serialized (smaller save files)
2. Visual elements are recreated from model data during load
3. Save data becomes the single source of truth
4. Game state can be tested independently of visuals

## Component Classification

### Model Components (Saved)

These components contain core game state and should be included in save files:

- `City` - Settlement data (population, buildings, production)
- `Civilization` - Faction state (name, color, technologies, economy)
- `CivId` - Civilization identifiers
- `CivPersonality` - AI behavior traits
- `Capital` - Capital city metadata
- `MilitaryUnit` - Unit state (health, position, experience)
- `Position` - Entity locations on the map
- `PlayerControlled` - Player ownership markers
- `Technologies`, `Economy`, `Military` - Civilization sub-systems

### View Components (Not Saved)

These components represent visual/aesthetic data and should be despawned before loading:

- `SpriteEntityReference` - Links to sprite entities
- UI components (HtmlNode, etc.)
- Camera components
- Rendering-specific state
- Audio source entities
- Visual effects

### Resources (Context-Dependent)

Resources are saved selectively based on their role:

**Saved Resources:**

- `WorldMap` - Terrain and tile data
- `CurrentTurn` - Game turn counter
- `ActiveCivTurn` - Turn order state
- `TurnPhase` - Current phase of turn
- `GameConfig` - Game settings
- `FogOfWarMaps` - Visibility state

**Not Saved:**

- Input state
- AssetServer references
- Window/rendering configuration

## Current Implementation

The codebase uses `bevy_save` with the `DominionEarthPipeline` that implements this MVC separation:

```rust
// In SaveLoadPlugin
impl Pipeline for DominionEarthPipeline {
    fn capture(&self, builder: BuilderRef) -> Snapshot {
        builder
            .extract_entities_matching(|e| {
                e.contains::<Position>()
                    || e.contains::<Civilization>()
                    || e.contains::<City>()
                    || e.contains::<MilitaryUnit>()
            })
            .deny::<SpriteEntityReference>()  // Exclude view components
            .extract_resource::<WorldMap>()
            .extract_resource::<CurrentTurn>()
            // ... more resources
            .build()
    }
}
```

### Save Process

1. Filter entities to only include those with Model components
2. Explicitly exclude View components (`SpriteEntityReference`)
3. Extract necessary resources
4. Serialize to RON format

### Load Process

1. Despawn sprite entities (View)
2. Despawn UI panels (View)
3. Load saved entities (Model)
4. Restore player control markers
5. Refresh fog of war
6. Respawn UI panels

## Future Integration with moonshine-save

When `moonshine-save` becomes compatible with Bevy 0.16+, the codebase is prepared for migration:

### Marking Components

Model components would use the `Save` component:

```rust
// When #[require] is available in compatible bevy_ecs version
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Save)]  // Marks for saving
pub struct City { /* ... */ }
```

View components would use the `Unload` component:

```rust
#[derive(Component, Reflect)]
#[require(Unload)]  // Marks for despawn before load
pub struct SpriteEntityReference { /* ... */ }
```

### Event-Based Save/Load

Instead of manual pipeline setup:

```rust
// Save
commands.trigger_save(SaveWorld::default_into_file("save.ron"));

// Load
commands.trigger_load(LoadWorld::default_from_file("save.ron"));
```

### Observers

Register observers to handle save/load events:

```rust
app.add_observer(save_on_default_event)
   .add_observer(load_on_default_event);
```

## Benefits

1. **Smaller Save Files**: Only essential state is serialized
2. **Flexibility**: Visual presentation can change without breaking saves
3. **Testing**: Game logic can be tested independently
4. **Clarity**: Clear separation between data and presentation
5. **Maintainability**: Easier to understand what gets saved

## Documentation References

- [moonshine-save README](https://github.com/Zeenobit/moonshine_save/blob/main/README.md)
- [moonshine-save Philosophy](https://github.com/Zeenobit/moonshine_save/blob/main/README.md#philosophy)
- Current implementation: `dominion_earth/src/plugins/save_load.rs`
- Serialization utilities: `core_sim/src/serialization.rs`

## Component Guidelines

When creating new components, ask:

1. **Is this core game state?** → Model (should be saved)
2. **Is this visual/aesthetic?** → View (should not be saved)
3. **Does it reference visual elements?** → View
4. **Would a headless simulation need it?** → If yes, Model; if no, View

## Migration Path

To fully adopt moonshine-save when compatible:

1. Add `moonshine-save` dependency (when Bevy 0.16+ compatible version available)
2. Mark Model components with `#[require(Save)]`
3. Mark View components with `#[require(Unload)]`
4. Replace `DominionEarthPipeline` with moonshine-save observers
5. Use `trigger_save` and `trigger_load` instead of manual pipeline
6. Remove manual sprite/UI despawning (handled by `Unload`)

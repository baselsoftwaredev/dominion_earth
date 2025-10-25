# MVC Save/Load Architecture Guide

## Overview

This document explains the Model-View-Controller (MVC) architecture applied to Dominion Earth's save/load system, following the philosophy of the [moonshine-save](https://github.com/Zeenobit/moonshine_save) crate.

The Dominion Earth project uses the MVC philosophy to ensure clean separation between game logic and visual presentation, making saves smaller, more maintainable, and future-proof.

## MVC Philosophy

The core principle is to separate game logic (Model) from visual presentation (View):

- **Model**: Core game state that should be saved/loaded - the "single source of truth"
- **View**: Visual and aesthetic elements that should NOT be saved - recreated from Model data
- **Controller**: Systems that handle save/load events and coordinate between Model and View

This separation ensures that:

1. Only essential game state is serialized (smaller save files)
2. Visual elements are recreated from model data during load
3. Save data becomes the single source of truth
4. Game state can be tested independently of visuals
5. Visual changes don't break existing save files

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
- `CapitalLabel` - World-space text labels for capitals
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
- Camera transform/zoom state

## Implementation Status

### Code Documentation

The following files have been enhanced with MVC documentation:

**Component Files:**

- `core_sim/src/components/city.rs` - City and Capital marked as Model components
- `core_sim/src/components/civilization.rs` - Civilization and CivId marked as Model
- `core_sim/src/components/military.rs` - MilitaryUnit marked as Model
- `core_sim/src/components/position.rs` - Position marked as Model
- `core_sim/src/components/rendering.rs` - SpriteEntityReference documented as View

Each component has clear documentation explaining:

- Whether it's a Model (game state) or View (visual) component
- Why it should/shouldn't be saved
- Its role in the MVC architecture

**System Files:**

- `core_sim/src/serialization.rs` - Extensive MVC philosophy documentation

  - Explains Model/View/Controller separation
  - Documents placeholder methods for future moonshine-save integration
  - Maintains backward compatibility with existing save system

- `dominion_earth/src/plugins/save_load.rs` - MVC architecture comments
  - Documents which components are Model vs View
  - Includes commented examples for future moonshine-save integration
  - Maintains current `bevy_save` implementation

### Verification

✅ Project compiles successfully with all changes
✅ MVC philosophy documented throughout codebase
✅ Component classification clearly defined
✅ Migration path prepared for future integration
✅ Existing save/load functionality maintained

## Current Implementation

The codebase uses `bevy_save` with the `DominionEarthPipeline` that implements MVC separation:

```rust
// In dominion_earth/src/plugins/save_load.rs
impl Pipeline for DominionEarthPipeline {
    fn capture(&self, builder: BuilderRef) -> Snapshot {
        builder
            // Extract only Model entities
            .extract_entities_matching(|e| {
                e.contains::<Position>()
                    || e.contains::<Civilization>()
                    || e.contains::<City>()
                    || e.contains::<MilitaryUnit>()
            })
            // Explicitly exclude View components
            .deny::<SpriteEntityReference>()

            // Extract Model resources
            .extract_resource::<WorldMap>()
            .extract_resource::<CurrentTurn>()
            .extract_resource::<ActiveCivTurn>()
            .extract_resource::<TurnPhase>()
            .extract_resource::<GameConfig>()
            .extract_resource::<FogOfWarMaps>()
            .extract_resource::<TurnAdvanceRequest>()
            .extract_resource::<PlayerActionsComplete>()

            .build()
    }
}
```

### Save Process (F5)

1. **Capture Model entities** - Filter entities to only include those with Model components
2. **Exclude View components** - Explicitly deny `SpriteEntityReference` and other View components
3. **Extract Model resources** - Capture game state resources
4. **Serialize to RON** - Write to file in human-readable format

### Load Process (F9)

1. **Despawn View entities**
   - Despawn sprite entities (View)
   - Despawn UI panels (View)
   - Despawn capital labels (View)
2. **Load Model data**
   - Load saved entities (positions, civilizations, cities, units)
   - Restore resources (world map, turn state, fog of war)
3. **Restore gameplay state**
   - Restore player control markers
   - Refresh fog of war rendering
4. **Recreate View**
   - Respawn UI panels with loaded data
   - Sprites and labels will be recreated by normal spawn systems

## Future Integration with moonshine-save

When `moonshine-save` becomes compatible with Bevy 0.16+, the codebase is prepared for migration.

### Migration Path

To fully adopt moonshine-save when compatible:

1. **Add dependency** - `moonshine-save = "0.6.0+"` (when Bevy 0.16+ compatible version available)
2. **Mark Model components** - Add `#[require(Save)]` to components that should be saved
3. **Mark View components** - Add `#[require(Unload)]` to components that should be despawned before load
4. **Replace pipeline** - Remove `DominionEarthPipeline` and use moonshine-save observers
5. **Use event API** - Replace manual save/load with `trigger_save` and `trigger_load`
6. **Remove manual cleanup** - Sprite/UI despawning will be handled automatically by `Unload`

### Marking Components

Model components would use the `Save` component:

```rust
// When #[require] is available in compatible bevy_ecs version
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Save)]  // Marks for saving
pub struct City { /* ... */ }

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Save)]
pub struct Civilization { /* ... */ }
```

View components would use the `Unload` component:

```rust
#[derive(Component, Reflect)]
#[require(Unload)]  // Marks for despawn before load
pub struct SpriteEntityReference { /* ... */ }

#[derive(Component, Reflect)]
#[require(Unload)]
pub struct CapitalLabel { /* ... */ }
```

### Event-Based Save/Load

Instead of manual pipeline setup:

```rust
// Save (replaces F5 handler)
commands.trigger_save(SaveWorld::default_into_file("save.ron"));

// Load (replaces F9 handler)
commands.trigger_load(LoadWorld::default_from_file("save.ron"));
```

### Observers

Register observers to handle save/load events:

```rust
app.add_observer(save_on_default_event)
   .add_observer(load_on_default_event);
```

### When to Migrate

The codebase is ready to adopt `moonshine-save` when:

1. A compatible version for Bevy 0.16+ is released
2. The `#[require]` attribute syntax is available
3. Team decides to migrate from `bevy_save` to `moonshine-save`

Until then, the MVC principles are applied through documentation and code organization, ensuring the save/load system follows best practices for separation of concerns.

## Component Guidelines

When creating new components, ask:

1. **Is this core game state?** → Model (should be saved)
2. **Is this visual/aesthetic?** → View (should not be saved)
3. **Does it reference visual elements?** → View
4. **Would a headless simulation need it?** → If yes, Model; if no, View

### Examples

**Model Components:**

- `Position` - Where an entity is on the game map (game state)
- `City` - Population, buildings, production queue (game state)
- `MilitaryUnit` - Unit stats, health, experience (game state)
- `PlayerControlled` - Ownership marker (game state)

**View Components:**

- `SpriteEntityReference` - Link to rendering sprite (visual)
- `CapitalLabel` - Text label for capitals (visual)
- `HtmlNode` - UI panel elements (visual)
- Transform/visibility of non-gameplay entities (visual)

## Benefits

1. **Smaller Save Files**: Only essential state is serialized (typically 10-20% of runtime entities)
2. **Flexibility**: Visual presentation can change without breaking saves
3. **Testing**: Game logic can be tested independently in headless mode
4. **Clarity**: Clear separation between data and presentation
5. **Maintainability**: Easier to understand what gets saved and why
6. **Future-Proof**: Ready for moonshine-save integration when available

## Troubleshooting

### Save File Too Large

- Check if View components are being saved
- Verify `.deny::<ViewComponent>()` is used in pipeline
- Review which resources are being extracted

### Data Lost After Load

- Ensure component has Model classification
- Check if component is in `.extract_entities_matching()` filter
- Verify component has `Reflect` derive

### Visual Elements Persist After Load

- Add component to despawn systems in `save_load.rs`
- Consider marking as View component with `Unload` (future)
- Check cleanup systems in `despawn_referenced_sprites()` and `despawn_ui_panels()`

### Save/Load Fails

- Check RON file syntax if manually edited
- Verify all saved components have `Reflect` implementation
- Ensure serialization registration in `core_sim/src/serialization.rs`

## References

- [moonshine-save Repository](https://github.com/Zeenobit/moonshine_save)
- [moonshine-save Philosophy](https://github.com/Zeenobit/moonshine_save/blob/main/README.md#philosophy)
- Current implementation: `dominion_earth/src/plugins/save_load.rs`
- Serialization utilities: `core_sim/src/serialization.rs`
- Component classification: `core_sim/src/components/*.rs`

## Related Documentation

- `docs/ui_labels.md` - UI labels system (View components)
- `docs/menu_system.md` - Menu and screen state management
- `docs/fog_of_war.md` - Visibility system (Model with View representation)

# moonshine-save Philosophy Integration Summary

## What Was Done

The Dominion Earth project has been updated to follow the MVC (Model-View-Controller) philosophy from the `moonshine-save` crate for save/load architecture.

## Changes Made

### 1. Documentation Created

- **`docs/mvc_save_architecture.md`** - Comprehensive guide to the MVC save/load architecture
  - Explains Model vs View component classification
  - Documents current implementation with `bevy_save`
  - Provides migration path for future `moonshine-save` integration

### 2. Code Documentation Updated

#### Component Files Enhanced with MVC Comments:

- `core_sim/src/components/city.rs` - City and Capital marked as Model components
- `core_sim/src/components/civilization.rs` - Civilization and CivId marked as Model
- `core_sim/src/components/military.rs` - MilitaryUnit marked as Model
- `core_sim/src/components/position.rs` - Position marked as Model
- `core_sim/src/components/rendering.rs` - SpriteEntityReference documented as View

Each component now has clear documentation explaining:

- Whether it's a Model (game state) or View (visual) component
- Why it should/shouldn't be saved
- Its role in the MVC architecture

#### Serialization Module Updated:

- `core_sim/src/serialization.rs` - Added extensive MVC philosophy documentation
  - Explains Model/View/Controller separation
  - Documents placeholder methods for future moonshine-save integration
  - Maintains backward compatibility with existing save system

#### SaveLoadPlugin Enhanced:

- `dominion_earth/src/plugins/save_load.rs` - Added MVC architecture comments
  - Documents which components are Model vs View
  - Includes commented examples for future moonshine-save integration
  - Maintains current `bevy_save` implementation

## Architecture Principles Applied

### Model Components (Saved)

Game logic and state that forms the "single source of truth":

- City, Civilization, CivId, Capital
- MilitaryUnit, Position
- Technologies, Economy, Military
- PlayerControlled markers
- Resources: WorldMap, CurrentTurn, GameConfig, FogOfWarMaps

### View Components (Not Saved)

Visual and aesthetic elements recreated from Model data:

- SpriteEntityReference
- UI components (HtmlNode, etc.)
- Camera, audio, and rendering state
- Visual effects

### Benefits

1. **Smaller save files** - Only essential state serialized
2. **Flexibility** - Visual changes don't break saves
3. **Testability** - Game logic testable without graphics
4. **Maintainability** - Clear separation of concerns
5. **Future-proof** - Ready for moonshine-save integration

## Current Implementation

The project uses `bevy_save` with a `DominionEarthPipeline` that implements MVC principles:

```rust
fn capture(&self, builder: BuilderRef) -> Snapshot {
    builder
        .extract_entities_matching(|e| {
            e.contains::<Position>()
                || e.contains::<Civilization>()
                || e.contains::<City>()
                || e.contains::<MilitaryUnit>()
        })
        .deny::<SpriteEntityReference>()  // Explicitly exclude View
        .extract_resource::<WorldMap>()
        // ... other resources
        .build()
}
```

## Future Integration Path

When `moonshine-save` becomes compatible with Bevy 0.16+:

1. Add dependency: `moonshine-save = "0.6.0+"`
2. Mark Model components: `#[require(Save)]`
3. Mark View components: `#[require(Unload)]`
4. Add observers: `.add_observer(save_on_default_event)` `.add_observer(load_on_default_event)`
5. Use event-based API:
   ```rust
   commands.trigger_save(SaveWorld::default_into_file("save.ron"));
   commands.trigger_load(LoadWorld::default_from_file("save.ron"));
   ```

## Verification

✅ Project compiles successfully with all changes
✅ MVC philosophy documented throughout codebase
✅ Component classification clearly defined
✅ Migration path prepared for future integration
✅ Existing save/load functionality maintained

## References

- [moonshine-save Repository](https://github.com/Zeenobit/moonshine_save)
- [moonshine-save Philosophy](https://github.com/Zeenobit/moonshine_save/blob/main/README.md#philosophy)
- `docs/mvc_save_architecture.md` - Full architecture documentation
- `core_sim/src/serialization.rs` - Serialization utilities with MVC comments
- `dominion_earth/src/plugins/save_load.rs` - Current save/load implementation

## Next Steps

The codebase is now prepared to adopt `moonshine-save` when:

1. A compatible version for Bevy 0.16+ is released
2. The `#[require]` attribute syntax is available
3. Team decides to migrate from `bevy_save` to `moonshine-save`

Until then, the MVC principles are applied through documentation and code organization, ensuring the save/load system follows best practices for separation of concerns.

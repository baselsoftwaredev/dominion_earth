# Save/Load System Troubleshooting Guide

## Overview

This guide documents common issues encountered with the moonshine-save-based save/load system and their solutions. Use this when debugging save/load problems to avoid repeating past mistakes.

## Common Issues and Solutions

### Issue 1: Duplicate Sprites After Loading

**Symptoms:**

- After loading a saved game, visual sprites (units, capitals, etc.) appear duplicated
- Multiple sprites at the same position

**Root Cause:**

- `game::setup_game` runs every time entering `Screen::Gameplay` state
- This happens both on new game start AND when loading
- Old sprite entities aren't cleaned up before load
- New sprites created on top of loaded entities

**Solution:**

```rust
// In SaveLoadState resource:
pub is_loading_from_save: bool,

// In game::setup_game system:
pub fn setup_game(
    // ... params
    save_load_state: Res<SaveLoadState>,
) {
    // Skip setup if we're loading from a save file
    if save_load_state.is_loading_from_save {
        println!("Skipping game setup - loading from save");
        return;
    }
    // ... rest of setup
}
```

**Files Modified:**

- `dominion_earth/src/plugins/save_load.rs`
- `dominion_earth/src/game.rs`

---

### Issue 2: Game Crashes During Load

**Symptoms:**

```
Encountered an error in command: The entity with ID XXvX was despawned
Encountered a panic when applying buffers for system `recreate_missing_unit_sprites`
```

**Root Cause:**

- Sprite rendering systems run during moonshine-save's load process
- These systems try to create `SpriteEntityReference` components on entities that moonshine is currently loading/unloading
- Race condition between rendering systems and save/load system

**Solution:**

1. **Clean up sprites BEFORE triggering load** (three-phase approach):

```rust
// Phase 1: Handle load request - set pending flag
fn handle_load_requests(mut save_state: ResMut<SaveLoadState>) {
    if let Some(load_name) = save_state.load_requested.take() {
        save_state.pending_load_name = Some(load_name);
        save_state.is_loading_from_save = true;
    }
}

// Phase 2: Clean up visual entities
fn cleanup_world_before_load(
    mut commands: Commands,
    mut save_state: ResMut<SaveLoadState>,
    sprite_entities: Query<Entity, (With<Sprite>, Without<Position>)>,
    label_entities: Query<Entity, Or<(With<CapitalLabel>, With<UnitLabel>)>>,
) {
    if save_state.pending_load_name.is_none() {
        return;
    }

    // Despawn all sprites and labels
    for sprite_entity in sprite_entities.iter() {
        commands.entity(sprite_entity).despawn();
    }
    for label_entity in label_entities.iter() {
        commands.entity(label_entity).despawn();
    }
}

// Phase 3: Trigger the load
fn trigger_pending_load(mut commands: Commands, mut save_state: ResMut<SaveLoadState>) {
    if let Some(load_name) = save_state.pending_load_name.take() {
        let file_path = format!("saves/{}.ron", load_name);
        commands.trigger_load(LoadWorld::default_from_file(file_path));
        save_state.frames_since_load_triggered = 0;
    }

    if save_state.is_loading_from_save {
        save_state.frames_since_load_triggered += 1;
    }
}
```

2. **Prevent sprite systems from running during load**:

```rust
// In rendering plugin:
fn not_loading_from_save(
    save_state: Option<Res<SaveLoadState>>
) -> bool {
    save_state.map_or(true, |state| !state.is_loading_from_save)
}

// Apply to all sprite systems:
.add_systems(
    Update,
    (
        rendering::units::spawn_unit_sprites,
        rendering::units::recreate_missing_unit_sprites,
        // ... other sprite systems
    )
    .run_if(in_state(Screen::Gameplay))
    .run_if(not_loading_from_save),
)
```

3. **Add frame delay before clearing loading flag**:

```rust
fn clear_loading_flag(mut save_state: ResMut<SaveLoadState>) {
    const MIN_FRAMES_AFTER_LOAD: u32 = 3;

    if save_state.is_loading_from_save
        && !save_state.needs_player_restore
        && !save_state.fog_of_war_needs_refresh
        && !save_state.ui_needs_respawn
        && save_state.frames_since_load_triggered >= MIN_FRAMES_AFTER_LOAD
    {
        save_state.is_loading_from_save = false;
        save_state.frames_since_load_triggered = 0;
    }
}
```

**Files Modified:**

- `dominion_earth/src/plugins/save_load.rs`
- `dominion_earth/src/plugins/rendering.rs`

---

### Issue 3: Component Registration Errors

**Symptoms:**

```
Encountered an error: cannot find type `SpriteEntityReference` in this scope
```

**Root Cause:**

- Registering View components (like `SpriteEntityReference`) with the save/load system
- These components are marked `#[require(Unload)]` and should NOT be saved
- Moonshine-save tries to manage them, causing conflicts

**Solution:**

- **DO NOT** register View components with `register_type`
- **DO NOT** import them in `save_load.rs`
- Let moonshine-save handle `Unload` components automatically

```rust
// ❌ WRONG - Don't do this:
use core_sim::components::rendering::SpriteEntityReference;
.register_type::<SpriteEntityReference>()

// ✅ CORRECT - Don't register View components at all
```

**Files Modified:**

- `dominion_earth/src/plugins/save_load.rs`

---

### Issue 4: Entity Does Not Exist Warnings

**Symptoms:**

```
WARN: The entity with ID XXvX does not exist (index has been reused or was never spawned)
```

**Root Cause:**

- Systems trying to remove components from entities that were despawned during load
- Using `commands.entity(entity).remove::<Component>()` without checking if entity exists

**Solution:**
Use `get_entity()` with error handling:

```rust
// ❌ WRONG:
if let Some(prev_entity) = selected_unit.unit_entity {
    commands
        .entity(prev_entity)
        .remove::<UnitSelected>();
}

// ✅ CORRECT:
if let Some(prev_entity) = selected_unit.unit_entity {
    if let Some(mut entity_commands) = commands.get_entity(prev_entity).ok() {
        entity_commands.remove::<UnitSelected>();
    }
}
```

**Files Modified:**

- `dominion_earth/src/input/tile_selection.rs`
- `dominion_earth/src/input/unit_interaction.rs`

---

### Issue 5: Wrong Component Marked for Save

**Symptoms:**

- Transient UI state persists after load
- Components on entities that shouldn't have them

**Root Cause:**

- UI/View components marked with `#[require(Save)]` instead of `#[require(Unload)]`
- Example: `UnitSelected` is UI state, not game state

**Solution:**
Properly classify components according to MVC pattern:

```rust
// ❌ WRONG - UnitSelected is UI state:
#[derive(Component, Debug, Clone, Reflect)]
#[require(Save)]
pub struct UnitSelected;

// ✅ CORRECT - Mark as Unload:
#[derive(Component, Debug, Clone, Reflect)]
#[require(Unload)]
pub struct UnitSelected;
```

Then remove from save/load registration.

**Files Modified:**

- `core_sim/src/components/player.rs`
- `dominion_earth/src/plugins/save_load.rs`

---

## MVC Component Classification Guide

### Model Components (Save)

Mark with `#[require(Save)]` and register in save/load plugin:

- `City` - Settlement data
- `Civilization` - Faction state
- `Capital` - Capital city metadata
- `MilitaryUnit` - Unit state
- `Position` - Entity locations
- `PlayerControlled` - Ownership markers
- `PlayerMovementOrder` - Pending orders
- `Technologies`, `Economy`, `Military` - Civ subsystems

### View Components (Unload)

Mark with `#[require(Unload)]` and DON'T register:

- `SpriteEntityReference` - Links to sprite entities
- `UnitSelected` - UI selection state
- `CapitalLabel` - World-space labels
- Any rendering/UI specific components

### Resources

**Saved:**

- `WorldMap`, `CurrentTurn`, `ActiveCivTurn`
- `TurnPhase`, `GameConfig`, `FogOfWarMaps`

**Not Saved:**

- Input state, rendering config, camera state
- Asset references, window settings

---

## System Execution Order

Proper order for save/load systems:

```
Update Schedule:
1. handle_save_requests          // Process save requests
2. handle_load_requests          // Set pending load flag
3. cleanup_world_before_load     // Despawn sprites/labels
4. trigger_pending_load          // Trigger moonshine load
   [moonshine load happens here]
5. restore_player_control_after_load
6. refresh_fog_of_war_after_load
7. respawn_ui_after_load
8. restore_music_volume_after_load
9. clear_loading_flag            // After MIN_FRAMES delay
```

---

## Testing Checklist

When modifying save/load system, test:

- [ ] Start new game - no errors
- [ ] Save game (F6) - no errors
- [ ] Load game (F7) - no crashes
- [ ] No duplicate sprites after load
- [ ] No entity warnings in console
- [ ] UI panels respawn correctly
- [ ] Fog of war state preserved
- [ ] Player control restored to first civ
- [ ] Units can be selected after load
- [ ] Turn state preserved

---

## Debug Logging

Add to save/load systems for troubleshooting:

```rust
info!("Load requested: {}", load_name);
info!("Cleaning up visual entities before load");
info!("Despawned {} sprite entities", sprite_count);
info!("Triggering load for: {}", load_name);
info!("All load restoration complete, clearing loading flag");
```

Enable with: `cargo run -- --debug-logging`

---

## Key Principles

1. **Never run game initialization when loading** - Check `is_loading_from_save` flag
2. **Clean up View before loading Model** - Despawn sprites/UI before triggering load
3. **Prevent systems during load** - Use run conditions on sprite/rendering systems
4. **Proper MVC separation** - Save = Model, Unload = View
5. **Graceful entity handling** - Always check entity exists before operating on it
6. **Frame delay for safety** - Wait a few frames after load before resuming normal systems

---

## Related Documentation

- [MVC Save Architecture](./mvc_save_architecture.md) - Overall architecture philosophy
- [Moonshine-Save Crate](https://github.com/Zeenobit/moonshine_save) - External documentation
- [Bevy ECS Commands](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html) - Entity command handling

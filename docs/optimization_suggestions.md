# Save/Load System Optimization Suggestions

Based on Bevy's ECS examples, here are concrete improvements for the save/load system.

## 1. Use StateScoped Entities (High Priority)

### Current Problem

Manual cleanup of sprites and UI in multiple places:

- `cleanup_world_before_load` - manually despawns sprites
- `despawn_all_game_entities` - manually despawns game entities
- Menu cleanup systems - manually despawn menu UI

### Solution: StateScoped Components

```rust
// In rendering/units/mod.rs - when spawning unit sprites
fn spawn_unit_sprite(...) {
    if let Some(sprite_entity) = spawn_entity_on_tile(...) {
        commands.entity(sprite_entity).insert((
            StateScoped(Screen::Gameplay),  // Auto-despawn when leaving Gameplay
        ));

        commands.entity(unit_entity)
            .insert(SpriteEntityReference { sprite_entity });
    }
}

// In UI spawning
commands.spawn((
    TopPanel,
    StateScoped(Screen::Gameplay),  // Auto-cleanup
    Node { ... },
));

// For loading-specific entities
commands.spawn((
    LoadingSpinner,
    StateScoped(LoadingPhase::Active),  // Only exists during load
));
```

### Benefits

- **Eliminate** `cleanup_world_before_load` entirely
- **Eliminate** manual sprite despawning in screen transitions
- Guaranteed cleanup - no memory leaks
- Simpler code, fewer bugs

### Implementation Steps

1. Add `StateScoped(Screen::Gameplay)` to all sprites when spawned
2. Add `StateScoped(Menu::X)` to all menu UI entities
3. Remove manual cleanup code from:
   - `cleanup_world_before_load`
   - `despawn_all_game_entities`
   - Menu cleanup systems
4. Test that entities properly despawn on state changes

---

## 2. System Piping for Error Handling (Medium Priority)

### Current Problem

No error handling in save/load operations - failures are silent or panic.

### Solution: Result-Based Systems

```rust
#[derive(Debug)]
enum SaveLoadError {
    IOError(std::io::Error),
    SerializationError(String),
    NoSaveRequested,
}

// Separate logic from side effects
fn validate_save_request(save_state: Res<SaveLoadState>) -> Result<String, SaveLoadError> {
    save_state.save_requested
        .as_ref()
        .map(|name| name.clone())
        .ok_or(SaveLoadError::NoSaveRequested)
}

fn execute_save(
    In(result): In<Result<String, SaveLoadError>>,
    mut commands: Commands,
    global_volume: Res<GlobalVolume>,
    mut saved_volume: ResMut<SavedMusicVolume>,
) {
    match result {
        Ok(save_name) => {
            saved_volume.volume = global_volume.volume.to_linear();
            let file_path = format!("saves/{}.ron", save_name);

            commands.trigger_save(
                SaveWorld::default_into_file(file_path)
                    .include_resource::<WorldMap>()
                    // ... other resources
            );

            info!("✅ Game saved successfully: {}", save_name);
        }
        Err(SaveLoadError::IOError(e)) => {
            error!("❌ Save failed - IO error: {}", e);
            // Could trigger a UI notification here
        }
        Err(SaveLoadError::SerializationError(e)) => {
            error!("❌ Save failed - serialization error: {}", e);
        }
        Err(SaveLoadError::NoSaveRequested) => {} // Normal case
    }
}

// Setup
.add_systems(Update, validate_save_request.pipe(execute_save))
```

### Benefits

- Explicit error handling
- Better user feedback on failures
- Easier to add UI notifications for errors
- Testable logic separation

---

## 3. Create a LoadingPhase SubState (Medium Priority)

### Current Problem

Manual flags like `is_loading_from_save`, `needs_player_restore`, etc.

### Solution: Dedicated Loading State

```rust
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum LoadingPhase {
    Idle,
    CleaningUp,      // Despawning old sprites
    Loading,         // Moonshine load in progress
    RestorePlayer,   // Restoring player control
    RestoreFog,      // Refreshing fog of war
    RestoreUI,       // Respawning UI
    Complete,
}

// Systems automatically run based on state
.add_systems(OnEnter(LoadingPhase::CleaningUp), cleanup_before_load)
.add_systems(OnEnter(LoadingPhase::Loading), trigger_load)
.add_systems(OnEnter(LoadingPhase::RestorePlayer), restore_player_control)
.add_systems(OnEnter(LoadingPhase::RestoreFog), refresh_fog_of_war)
.add_systems(OnEnter(LoadingPhase::RestoreUI), respawn_ui)
.add_systems(OnEnter(LoadingPhase::Complete), clear_loading_state)

// State transitions
fn cleanup_before_load(mut next_state: ResMut<NextState<LoadingPhase>>) {
    // Cleanup happens via StateScoped
    info!("✅ Cleanup complete");
    next_state.set(LoadingPhase::Loading);
}
```

### Benefits

- Self-documenting load process
- Clear state machine
- Easier to debug (can see current loading phase)
- Each system only runs when needed
- No manual flag management

---

## 4. Resource Observers for Cleaner Integration

### Current Pattern

```rust
pub struct SaveLoadState {
    pub save_requested: Option<String>,
    pub load_requested: Option<String>,
    // ... many flags
}
```

### Better Pattern with Observers

```rust
// Use events + observers instead of polling resources
app.add_observer(on_save_requested)
   .add_observer(on_load_requested);

fn on_save_requested(
    trigger: Trigger<SaveRequested>,
    mut commands: Commands,
    // ... other params
) {
    let save_name = trigger.event().name.clone();
    commands.trigger_save(...);
}
```

This is already partially used, but could be expanded to replace the polling pattern.

---

## Priority Ranking

### High Priority (Immediate Win)

1. **StateScoped entities** - Eliminates most manual cleanup code
   - Effort: Medium (mark entities, remove cleanup code)
   - Benefit: High (simpler, more reliable)

### Medium Priority (Nice to Have)

2. **System piping** - Better error handling

   - Effort: Low (refactor existing systems)
   - Benefit: Medium (better UX, easier debugging)

3. **LoadingPhase state** - Replace manual flags
   - Effort: Medium (create state, update systems)
   - Benefit: Medium (clearer code, easier to extend)

### Low Priority (Future Enhancement)

4. **Full observer pattern** - Replace polling entirely
   - Effort: High (architectural change)
   - Benefit: Low (marginal improvement over current)

---

## Example: Minimal StateScoped Implementation

Here's the smallest change with biggest impact:

```rust
// In rendering/units/mod.rs
fn spawn_unit_sprite(...) {
    // ... existing code ...

    if let Some(sprite_entity) = spawn_entity_on_tile(...) {
        // ADD THIS LINE
        commands.entity(sprite_entity).insert(StateScoped(Screen::Gameplay));

        // ... rest of existing code ...
    }
}

// In rendering/capitals/mod.rs
fn spawn_animated_capital_sprite(...) {
    let sprite_entity = commands.spawn((
        // ... existing sprite components ...
        StateScoped(Screen::Gameplay),  // ADD THIS
    )).id();
}

// In ui/system_setup.rs
fn spawn_ui_panels(...) {
    commands.spawn((
        TopPanel,
        StateScoped(Screen::Gameplay),  // ADD THIS
        // ... existing UI components ...
    ));
}

// THEN DELETE cleanup_world_before_load entirely!
// Sprites will auto-despawn when:
// - Loading a game (StateScoped handles it)
// - Exiting to menu (StateScoped handles it)
```

This single change could eliminate ~50 lines of manual cleanup code!

# Fog of War System - Complete Documentation

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Implementation Guide](#implementation-guide)
4. [Troubleshooting & Fixes](#troubleshooting--fixes)
5. [Usage Reference](#usage-reference)
6. [Performance & Future Enhancements](#performance--future-enhancements)

---

## Overview

This document covers the complete implementation of a turn-based Civilization-style fog of war system for Dominion Earth. The system replaces the previously incorrect `bevy_fog_of_war` dependency (designed for real-time RTS games) with a custom turn-based solution.

### Key Features

- ✅ Turn-based visibility updates
- ✅ Per-civilization independent fog of war
- ✅ Permanent exploration (tiles stay explored)
- ✅ Circular vision using Chebyshev distance
- ✅ AI helper functions to filter visible entities
- ✅ Save/load support (fully serializable)
- ✅ Clean separation: core_sim (pure ECS) vs rendering

### Visibility States

| State        | Description                    | Tile Color        | Entity Visibility                 |
| ------------ | ------------------------------ | ----------------- | --------------------------------- |
| `Unexplored` | Never seen                     | Black             | Hidden                            |
| `Explored`   | Seen before, no current vision | Dimmed gray (40%) | Hidden (except player-controlled) |
| `Visible`    | Current vision range           | Full brightness   | Visible                           |

---

## Architecture

### Core Simulation Layer (`core_sim`)

Pure ECS components and systems with no graphics dependencies.

#### Components (`core_sim/src/components/fog_of_war.rs`)

**`VisibilityState`** - Enum with three states:

```rust
pub enum VisibilityState {
    Unexplored,  // Never seen before
    Explored,    // Seen before but no units nearby
    Visible,     // Currently visible
}
```

**`ProvidesVision`** - Component for entities that provide vision:

```rust
pub struct ProvidesVision {
    pub range: i32,
}

impl ProvidesVision {
    pub fn unit_vision() -> Self { Self { range: 2 } }  // 2-tile range
    pub fn city_vision() -> Self { Self { range: 3 } }  // 3-tile range
}
```

**`VisibilityMap`** - Per-civilization visibility tracking:

```rust
pub struct VisibilityMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<VisibilityState>>,
}
```

Methods:

- `get(pos)` - Get visibility state at position
- `set(pos, state)` - Set visibility state
- `is_visible(pos)` - Check if currently visible
- `is_explored(pos)` - Check if explored or visible
- `reset_visibility()` - Convert all Visible → Explored
- `mark_visible(center, range)` - Mark area visible using Chebyshev distance

**`FogOfWarMaps`** - Resource storing all civilization visibility maps:

```rust
pub struct FogOfWarMaps {
    pub maps: HashMap<CivId, VisibilityMap>,
}
```

#### Systems (`core_sim/src/systems/fog_of_war.rs`)

**`update_fog_of_war`** - Main visibility update system:

1. Initializes maps for new civilizations
2. Resets all Visible tiles to Explored
3. Recalculates visibility from units (2-tile range)
4. Recalculates visibility from cities (3-tile range)
5. Runs every frame independently

**Helper Functions**:

- `filter_visible_units()` - Filter units visible to a civilization
- `filter_visible_cities()` - Filter cities visible to a civilization
- `is_position_visible()` - Check if position is visible
- `is_position_explored()` - Check if position has been explored
- `get_visible_positions()` - Get all visible positions
- `get_explored_positions()` - Get all explored positions

### Rendering Layer (`dominion_earth`)

#### Components (`dominion_earth/src/rendering/fog_of_war.rs`)

**`TileSprite`** - Links tile sprite entities to grid positions:

```rust
pub struct TileSprite {
    pub position: Position,
}
```

#### Systems

**`attach_tile_sprite_components`** - Startup system:

- Runs after tiles are spawned
- Attaches `TileSprite` to all tile entities

**`apply_fog_of_war_to_tiles`** - Runtime system (every frame):

- Reads player civilization's visibility map
- Updates tile colors:
  - Unexplored: `Color::srgba(0.0, 0.0, 0.0, 1.0)` (black)
  - Explored: `Color::srgba(0.4, 0.4, 0.4, 1.0)` (dimmed gray)
  - Visible: `Color::WHITE` (full brightness)

**`hide_entities_in_fog`** - Runtime system (every frame):

- Hides units/cities/buildings on non-visible tiles
- Always shows player-controlled entities
- Sets `Visibility::Hidden` for entities in fog

### Turn-Based Flow

```
Every Frame:
    ↓
update_fog_of_war runs:
    ↓
    reset_visibility() - Visible → Explored
    ↓
    For each civilization:
        ↓
        For each unit → mark_visible(position, 2)
        ↓
        For each city → mark_visible(position, 3)
    ↓
Rendering systems apply visibility:
    ↓
    apply_fog_of_war_to_tiles (update tile colors)
    ↓
    hide_entities_in_fog (show/hide entities)
```

---

## Implementation Guide

### Files Created

1. `core_sim/src/components/fog_of_war.rs` - Core fog of war components
2. `core_sim/src/systems/fog_of_war.rs` - Visibility update systems
3. `dominion_earth/src/rendering/fog_of_war.rs` - Rendering systems

### Files Modified

1. `core_sim/src/components/mod.rs` - Added fog_of_war module
2. `core_sim/src/systems/mod.rs` - Added fog_of_war module
3. `core_sim/src/lib.rs` - Exported fog of war types
4. `dominion_earth/src/rendering/mod.rs` - Added fog_of_war module
5. `dominion_earth/src/rendering/tilemap/mod.rs` - Added TileSprite attachment
6. `dominion_earth/src/plugins/rendering.rs` - Added fog of war systems
7. `dominion_earth/src/plugins/core_simulation.rs` - Integrated fog of war updates
8. `dominion_earth/src/game.rs` - Added initialization system
9. `dominion_earth/src/civilization_spawning.rs` - Added ProvidesVision and CivId to spawns

### Plugin Integration

**CoreSimulationPlugin** (`dominion_earth/src/plugins/core_simulation.rs`):

```rust
.init_resource::<core_sim::FogOfWarMaps>()
.add_systems(Startup, (
    game::setup_game,
    game::initialize_fog_of_war.after(game::setup_game),
))
// Fog of war updates independently every frame (not chained)
.add_systems(Update, core_sim::update_fog_of_war)
```

**RenderingPlugin** (`dominion_earth/src/plugins/rendering.rs`):

```rust
.add_systems(Startup,
    rendering::tilemap::attach_tile_sprite_components
        .after(rendering::tilemap::setup_tilemap),
)
.add_systems(Update, (
    rendering::fog_of_war::apply_fog_of_war_to_tiles,
    rendering::fog_of_war::hide_entities_in_fog,
))
```

### Spawning Entities with Vision

**Units** (`dominion_earth/src/civilization_spawning.rs`):

```rust
let mut unit_commands = commands.spawn((
    initial_unit,
    position,
    civ_id,  // ⚠️ CRITICAL: Must include CivId component
));
unit_commands.insert(core_sim::ProvidesVision::unit_vision());
```

**Cities**:

```rust
let mut capital_commands = commands.spawn((
    city,
    capital,
    position,
    civ_id,  // ⚠️ CRITICAL: Must include CivId component
));
capital_commands.insert(core_sim::ProvidesVision::city_vision());
```

---

## Troubleshooting & Fixes

### Issue 1: All Tiles Remain Dark (Unexplored)

**Symptom**: Everything is black, no tiles are revealed.

**Root Cause**: Units and cities were spawned without the `CivId` component. The fog of war system queries for entities with `CivId`:

```rust
units: Query<(&Position, &CivId, &ProvidesVision), With<MilitaryUnit>>,
```

But entities were spawned as:

```rust
commands.spawn((initial_unit, position)); // ❌ Missing CivId component!
```

**Solution**: Add `CivId` as a separate component:

```rust
commands.spawn((initial_unit, position, civ_id)); // ✅ Includes CivId
```

**Why This Happens**: ECS queries cannot access struct fields (like `MilitaryUnit.owner`), only components attached to entities.

**Verification**:

```bash
cargo run -- --seed 1756118413 2>&1 | grep FOG_OF_WAR
```

Expected output:

```
FOG_OF_WAR: Spawned capital for civ CivId(0) at (21, 5) with vision range 3
FOG_OF_WAR: Updated civ CivId(0) - 1 units, 1 cities providing vision
```

### Issue 2: Tiles Not Revealed When Units Move

**Symptom**: Initial vision works, but moving units doesn't reveal new tiles.

**Root Cause**: `update_fog_of_war` was in a `.chain()` with turn management systems, so it only ran when the entire chain executed (not every frame).

```rust
// WRONG:
.add_systems(Update, (
    core_sim::execute_movement_orders,
    core_sim::update_fog_of_war, // Only runs when chain runs
    // ... other systems
).chain())
```

**Solution**: Make `update_fog_of_war` run independently every frame:

```rust
// CORRECT:
.add_systems(Update, (
    core_sim::execute_movement_orders,
    // ... other systems
).chain())
.add_systems(Update, core_sim::update_fog_of_war) // Runs every frame
```

**How It Works**:

1. Unit moves (position changes)
2. Next frame: `update_fog_of_war` detects new position
3. Visibility recalculated
4. Rendering systems apply updates

### ECS Design Pattern Note

**❌ DON'T** store queryable data inside struct fields:

```rust
struct MilitaryUnit {
    owner: CivId, // Can't query for this
}
```

**✅ DO** add it as a separate component:

```rust
commands.spawn((
    MilitaryUnit { ... },
    civ_id, // Can query for this
));
```

This enables efficient queries:

```rust
Query<&Position, With<CivId>>
```

---

## Usage Reference

### Adding Vision to Entities

**Standard Units**:

```rust
commands.spawn((
    MilitaryUnit { /* ... */ },
    Position::new(x, y),
    civ_id,
    core_sim::ProvidesVision::unit_vision(), // 2-tile range
));
```

**Cities/Buildings**:

```rust
commands.spawn((
    City { /* ... */ },
    Position::new(x, y),
    civ_id,
    core_sim::ProvidesVision::city_vision(), // 3-tile range
));
```

**Custom Vision Range**:

```rust
core_sim::ProvidesVision { range: 5 } // 5-tile range
```

### Querying Visibility

**Check if Position is Visible**:

```rust
use core_sim::{is_position_visible, FogOfWarMaps, CivId, Position};

fn my_system(fog_of_war: Res<FogOfWarMaps>) {
    let civ_id = CivId(0);
    let pos = Position::new(10, 15);

    if is_position_visible(pos, civ_id, &fog_of_war) {
        // Position is currently visible
    }
}
```

**Check if Position is Explored**:

```rust
if is_position_explored(pos, civ_id, &fog_of_war) {
    // Position has been seen before (explored or visible)
}
```

**Filter Visible Units**:

```rust
use core_sim::{filter_visible_units, MilitaryUnit};

fn ai_decision_system(
    fog_of_war: Res<FogOfWarMaps>,
    units: Query<(&Position, Entity), With<MilitaryUnit>>,
) {
    let civ_id = CivId(0);
    let visible_units = filter_visible_units(
        units.iter(),
        civ_id,
        &fog_of_war,
    );
    // visible_units contains only entities this civ can see
}
```

**Get All Visible Positions**:

```rust
use core_sim::get_visible_positions;

let visible_positions = get_visible_positions(civ_id, &fog_of_war);
for pos in visible_positions {
    // Process each visible tile
}
```

### Direct Map Access

**Get Visibility State**:

```rust
if let Some(vis_map) = fog_of_war.get(civ_id) {
    let state = vis_map.get(Position::new(10, 15));
    match state {
        Some(VisibilityState::Unexplored) => { /* Never seen */ },
        Some(VisibilityState::Explored) => { /* Seen before */ },
        Some(VisibilityState::Visible) => { /* Currently visible */ },
        None => { /* Invalid position */ },
    }
}
```

**Modify Visibility** (Advanced):

```rust
if let Some(vis_map) = fog_of_war.get_mut(civ_id) {
    // Mark a specific tile as visible
    vis_map.set(Position::new(10, 15), VisibilityState::Visible);

    // Mark area around position (e.g., 3 tile range)
    vis_map.mark_visible(Position::new(10, 15), 3);
}
```

### Common Patterns

**AI Only Targets Visible Enemies**:

```rust
fn ai_target_selection(
    fog_of_war: Res<FogOfWarMaps>,
    ai_civ: Query<&CivId, Without<PlayerControlled>>,
    enemy_units: Query<(&Position, Entity, &MilitaryUnit)>,
) {
    for ai_civ_id in ai_civ.iter() {
        let visible_enemies = filter_visible_units(
            enemy_units.iter().map(|(pos, entity, _)| (pos, entity)),
            *ai_civ_id,
            &fog_of_war,
        );
        // AI can only target visible enemies
    }
}
```

**Reveal Quest Area**:

```rust
fn reveal_quest_area(
    mut fog_of_war: ResMut<FogOfWarMaps>,
    player_civ: Query<&CivId, With<PlayerControlled>>,
) {
    if let Ok(civ_id) = player_civ.single() {
        if let Some(vis_map) = fog_of_war.get_mut(*civ_id) {
            // Reveal 5x5 area around quest location
            vis_map.mark_visible(Position::new(25, 15), 2);
        }
    }
}
```

**Initialize New Civilization**:

```rust
use core_sim::initialize_fog_of_war_for_civ;

fn spawn_new_civ(
    mut fog_of_war: ResMut<FogOfWarMaps>,
    world_map: Res<WorldMap>,
) {
    let new_civ_id = CivId(5);
    initialize_fog_of_war_for_civ(new_civ_id, &mut fog_of_war, &world_map);
}
```

### Debugging

**Check Fog of War State**:

```rust
fn debug_fog_of_war(fog_of_war: Res<FogOfWarMaps>) {
    for (civ_id, vis_map) in fog_of_war.maps.iter() {
        println!("Civilization {:?} visibility:", civ_id);
        println!("  Map size: {}x{}", vis_map.width, vis_map.height);

        let mut visible_count = 0;
        let mut explored_count = 0;
        let mut unexplored_count = 0;

        for x in 0..vis_map.width {
            for y in 0..vis_map.height {
                match vis_map.tiles[x as usize][y as usize] {
                    VisibilityState::Visible => visible_count += 1,
                    VisibilityState::Explored => explored_count += 1,
                    VisibilityState::Unexplored => unexplored_count += 1,
                }
            }
        }

        println!("  Visible: {}, Explored: {}, Unexplored: {}",
                 visible_count, explored_count, unexplored_count);
    }
}
```

---

## Performance & Future Enhancements

### Current Performance

**System Characteristics**:

- Updates run every frame (60 FPS)
- Quick calculations: simple iteration over units/cities
- Chebyshev distance calculation is O(1)
- Marking tiles visible is direct array access
- Memory: ~1 byte per tile per civilization
- For 50x25 map with 8 civs: ~10KB total

**Why Every Frame is Acceptable**:

- Typical game: ~10-20 units per civilization
- 50x25 map = 1,250 tiles
- Modern systems handle this easily at 60 FPS

### Optimization Options (If Needed)

**Option 1: Change Detection**:

```rust
.add_systems(Update,
    core_sim::update_fog_of_war
        .run_if(any_with_component::<MovementOrder>()
            .or_else(any_with_component::<PlayerMovementOrder>()))
)
```

**Option 2: Incremental Updates**:

```rust
pub fn update_fog_of_war_on_change(
    mut fog_of_war: ResMut<FogOfWarMaps>,
    changed_units: Query<(&Position, &CivId, &ProvidesVision),
        (Changed<Position>, With<MilitaryUnit>)>,
    // ... only recalculate for civs with changed units
)
```

### Future Enhancements

**Enhanced Visibility Rules**:

- Terrain-based vision blocking (mountains block line of sight)
- Watchtowers with extended range
- Scout units with bonus vision
- Height-based vision advantage

**Visual Polish**:

- Smooth fade transitions between visibility states
- Particle effects at fog boundaries
- Mini-map fog of war representation
- Edge-of-fog shader effects

**AI Integration**:

- AI uses visibility helpers for all decision-making
- AI only scouts/attacks visible tiles
- AI remembers last-seen enemy positions
- Fog-of-war aware pathfinding

**Multiplayer Support**:

- Already supports per-civ fog of war
- Would need network synchronization
- Client-side fog of war rendering
- Spectator mode with full visibility

### Serialization

Fog of war state automatically saves/loads:

```rust
// All types implement Serialize/Deserialize
// Fog of war state is preserved in save files
// No manual intervention needed
```

All components and resources derive `Serialize` and `Deserialize`, ensuring the complete fog of war state (including what each civilization has explored) persists across game sessions.

---

## Testing

Successfully tested with:

```bash
cargo run -- --seed 1756118413 --debug-logging
```

**Verification Checklist**:

- ✅ 3 civilizations spawned
- ✅ Fog of war resource initialized
- ✅ Visibility maps created for each civilization
- ✅ Initial vision around capitals and units
- ✅ Tiles reveal when units move
- ✅ Previously visible tiles become explored (dimmed)
- ✅ Rendering systems operational
- ✅ Debug logging confirms vision updates

---

## Conclusion

The fog of war system is fully implemented and operational. It:

- Follows the project's architecture guidelines
- Maintains separation between core simulation and rendering
- Supports serialization for save/load
- Provides a solid foundation for strategic gameplay
- Works correctly with turn-based movement
- Enables future AI and multiplayer features

The system has been tested and debugged to handle both initial implementation issues (missing CivId components) and dynamic update issues (movement not triggering updates), resulting in a robust and maintainable fog of war implementation.

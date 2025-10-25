# UI Labels System Documentation

## Overview

The UI labels system provides world-space text labels that display information about game entities. Labels are rendered using Bevy's `Text2d` component, which allows them to exist in world space and scale naturally with camera zoom.

## Capital Labels

Capital labels display the name of capital cities and their owning civilization directly on the game map.

### Architecture

**Location**: `dominion_earth/src/ui/bevy_hui/capital_labels.rs`

**Components**:

- `CapitalLabel` - Marker component attached to label entities
  - `capital_entity: Entity` - Reference to the capital entity being labeled
  - `capital_position: Position` - Cached position for change detection

**Systems**:

1. `spawn_capital_labels` - Creates labels for new capitals
2. `update_capital_labels` - Updates label positions and removes orphaned labels

### System Integration

Capital label systems are integrated into the UI plugin with state scoping:

```rust
// In ui/bevy_hui/mod.rs
pub fn setup_plugins_for_screen<S: States>(app: &mut App, screen: S) {
    app.add_plugins((HuiPlugin, HuiAutoLoadPlugin::new(&["ui"])))
        .add_systems(OnEnter(screen.clone()), setup_main_ui)
        .add_systems(OnExit(screen.clone()), cleanup_ui)
        .add_systems(
            Update,
            (
                update_ui_properties_system.run_if(should_update_ui_this_frame),
                spawn_capital_labels,
                update_capital_labels,
            )
                .run_if(in_state(screen)),
        );
}
```

The systems only run during the `Screen::Gameplay` state.

### Label Positioning

Labels are positioned at the north neighboring tile of the capital city with vertical offset:

```rust
// Calculate position of tile north of capital
let north_tile_pos = TilePos {
    x: capital_position.x as u32,
    y: (capital_position.y as i32 + 1) as u32,
};

// Convert to world space and apply offset
let mut world_pos = north_tile_pos.center_in_world(
    map_size, grid_size, tile_size, map_type, anchor
);
world_pos.y += CAPITAL_LABEL_VERTICAL_OFFSET_PIXELS;
```

### Styling Constants

Located in `capital_labels::constants`:

```rust
pub const CAPITAL_LABEL_FONT_SIZE: f32 = 16.0;
pub const CAPITAL_LABEL_Z_INDEX: f32 = 100.0;
pub const CAPITAL_LABEL_NORTH_OFFSET_TILES: f32 = 1.0;
pub const CAPITAL_LABEL_VERTICAL_OFFSET_PIXELS: f32 = -40.0;
pub const CAPITAL_LABEL_BACKGROUND_ALPHA: f32 = 0.7;
```

- **Font Size**: 16px for readability at typical zoom levels
- **Z-Index**: 100.0 to render above tiles and units
- **Vertical Offset**: -40px to position above the tile center
- **North Offset**: 1 tile to avoid overlapping the city sprite

### Label Format

Labels display two lines of text:

```
{city_name}
({civilization_name})
```

Example:

```
Alexandria
(Egyptian Empire)
```

### Lifecycle Management

#### Creation

Labels are spawned when:

1. A new `Capital` component is added to an entity (`Added<Capital>`)
2. An existing capital doesn't have a corresponding label

The system queries for both cases to ensure all capitals have labels:

```rust
// New capitals
capitals_query: Query<(Entity, &Position, &Capital, &City), Added<Capital>>

// Capitals without labels (backup/recovery)
capitals_without_labels: Query<(Entity, &Position, &Capital, &City), Without<CapitalLabel>>
```

#### Updates

Labels track the position of their associated capital. When a capital moves:

```rust
if *capital_position != capital_label.capital_position {
    // Recalculate world position
    let new_world_position = calculate_north_tile_world_position(...);
    label_transform.translation = new_world_position.extend(CAPITAL_LABEL_Z_INDEX);
}
```

#### Cleanup

Labels are automatically despawned in two scenarios:

1. **Capital destroyed**: When the associated capital entity no longer exists

   ```rust
   if capitals_query.get(capital_label.capital_entity).is_err() {
       commands.entity(label_entity).despawn();
   }
   ```

2. **Exiting gameplay**: When transitioning from `Screen::Gameplay` to another screen

   ```rust
   // In screens/gameplay.rs - OnExit(Screen::Gameplay)
   capital_label_entities: Query<Entity, With<crate::ui::bevy_hui::CapitalLabel>>

   for label_entity in &capital_label_entities {
       commands.entity(label_entity).despawn();
   }
   ```

### Debug Logging

When `--debug-logging` is enabled, the system logs:

```
Spawned Text2d capital label for {city_name} ({civ_name}) at world position (x, y)
Despawning {count} capital label entities
```

## Future Label Types

The label system is designed to be extensible. Potential future additions:

### City Labels

- Show city names for all cities (not just capitals)
- Display city population or production
- Color-coded by civilization

### Unit Labels

- Show unit type and strength
- Health bars
- Movement points remaining

### Terrain Labels

- Resource indicators
- Strategic position markers
- Improvement labels

### Implementation Pattern

Future labels should follow the capital label pattern:

1. **Create a marker component** with entity reference and cached data
2. **Implement spawn system** that queries for `Added<T>` and entities without labels
3. **Implement update system** for position tracking and orphan cleanup
4. **Add to cleanup system** in `screens/gameplay.rs` for proper state management
5. **Use constants module** for consistent styling
6. **Scope to Screen::Gameplay** using `run_if(in_state(...))`

## Best Practices

### Performance

- Use change detection (`Added<T>`, `Changed<T>`) to minimize unnecessary updates
- Cache entity references to avoid repeated lookups
- Only update positions when they actually change

### State Management

- Always add new label types to the `despawn_all_game_entities` system
- Ensure systems only run during appropriate game states
- Use marker components for efficient querying

### Positioning

- Use tilemap utility functions for world space calculations
- Apply consistent z-index ordering (labels above gameplay elements)
- Consider camera zoom levels when choosing font sizes

### Visual Consistency

- Define styling constants in dedicated module
- Use consistent text formatting across label types
- Consider colorblind-friendly color schemes

## Troubleshooting

### Labels Not Appearing

1. Check that the spawn system is running during correct game state
2. Verify z-index is high enough to render above other elements
3. Ensure world position calculation uses correct tilemap parameters
4. Check debug logging for spawn confirmation

### Labels Persisting Across Screens

1. Verify label entities are included in `despawn_all_game_entities`
2. Check that cleanup runs `OnExit(Screen::Gameplay)`
3. Confirm label component is in the cleanup query

### Labels Not Updating Position

1. Verify the update system is running each frame
2. Check that position change detection is working
3. Ensure capital entity reference is valid

### Label Positioning Issues

1. Verify tilemap configuration (size, tile size, anchor)
2. Check offset constants are appropriate for your tile size
3. Test with different camera zoom levels

## Related Systems

- **bevy_ecs_tilemap**: Provides tilemap utilities for world position calculations
- **Fog of War**: May need to hide labels in unexplored areas (future enhancement)
- **Camera System**: Zoom affects label visibility and readability
- **Menu System**: Screen states control when labels are active

## API Reference

### Components

#### `CapitalLabel`

```rust
pub struct CapitalLabel {
    pub capital_entity: Entity,
    pub capital_position: Position,
}
```

### Systems

#### `spawn_capital_labels`

Spawns `Text2d` labels for capitals that don't have labels.

**Queries**:

- `capitals_query`: New capitals with `Added<Capital>`
- `capitals_without_labels`: Existing capitals missing labels
- `existing_labels`: Check for duplicate prevention
- `civilizations_query`: Look up civilization names
- `tilemap_query`: Get tilemap configuration

#### `update_capital_labels`

Updates label positions and removes orphaned labels.

**Queries**:

- `label_query`: All capital labels with transform
- `capitals_query`: All capitals for position lookup
- `tilemap_query`: Get tilemap configuration for position calculations

### Helper Functions

#### `get_civilization_name`

```rust
fn get_civilization_name(
    civilizations_query: &Query<&Civilization>,
    civ_id: core_sim::CivId,
) -> String
```

#### `calculate_north_tile_world_position`

```rust
fn calculate_north_tile_world_position(
    capital_position: &Position,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Vec2
```

#### `spawn_capital_label_text2d`

```rust
fn spawn_capital_label_text2d(
    commands: &mut Commands,
    capital_entity: Entity,
    capital_position: Position,
    city_name: &str,
    civilization_name: &str,
    world_position: Vec2,
)
```

# UI Panel Implementation Guide

## Overview

This document covers best practices and patterns for implementing conditional UI panels in Dominion Earth using native Bevy UI. While this guide uses the unit stats panel as the primary example, these patterns apply to all UI panels (production menus, city panels, diplomacy screens, etc.).

## General Panel Architecture

All conditional panels follow this structure:

### 1. Panel Component (Rust)

Location: `dominion_earth/src/ui/[panel_name].rs`

- Defines the panel as a native Bevy UI component hierarchy
- Uses `NodeBundle`, `TextBundle`, `ButtonBundle` for layout
- Includes marker components for querying specific panels

### 2. Spawn Function

Location: `dominion_earth/src/ui/[panel_name].rs`

- Function named `spawn_[panel_name]()`
- Creates the panel entity hierarchy with native Bevy UI components
- Applies styling using `Style`, `BackgroundColor`, etc.

### 3. Update Functions

Location: `dominion_earth/src/ui/[panel_name].rs`

- Functions that query game state and update panel text/visibility
- Use `Query<&mut Text>` or `Query<&mut Visibility>` to modify panels
- Called during UI update cycle

### 4. State Management (Selection Logic)

Location: `dominion_earth/src/input/` or relevant systems

- Systems that manage when panel should show
- Resource/component to track selection state
- Clear/update logic for mutual exclusivity

## Example: UI Panels

### Implementation Locations

- **Left Panel**: `dominion_earth/src/ui/left_panel.rs` - Production menu and unit info
- **Right Panel**: `dominion_earth/src/ui/right_panel.rs` - Statistics and tile/civilization info
- **Top Panel**: `dominion_earth/src/ui/top_panel.rs` - Turn counter and player resources
- **Selection Logic**:
  - `dominion_earth/src/input/unit_interaction.rs` (unit selection)
  - `dominion_earth/src/input/tile_selection.rs` (clearing selections on empty tiles)

## Example Panel Data: Unit Stats

### Core Combat Stats

- **Attack**: Base attack value with effective attack (after modifiers)
- **Defense**: Base defense value with effective defense (after modifiers)
- **Health**: Current health / maximum health
- **Range**: Unit's attack range

### Movement Stats

- **Movement Remaining**: Current movement / maximum movement range
- Shows how many tiles the unit can still move this turn

### Condition Stats (Contextual)

- **Fatigue**: 0-100% (higher = worse performance, affects combat effectiveness)
- **Supply**: 0-100% (lower = worse performance, affects combat effectiveness)
- **Decay**: 0-100% (equipment deterioration over time)

### Experience

- **Experience**: 0-100% progress toward next level

## Panel Visibility Patterns

### General Visibility Logic

All panels should implement clear show/hide conditions based on game state:

**Show Panel When:**

- Required selection/state is active (e.g., unit selected, city selected, diplomacy screen opened)
- Selection belongs to player (if applicable)
- No higher-priority panel is showing (mutual exclusivity)

**Hide Panel When:**

- Selection is cleared or changed to different type
- Player clicks on empty/irrelevant tile
- Higher-priority panel takes precedence
- Panel is explicitly closed by user action

### Example: Left Panel Visibility

**Shows When:**

- Player clicks on their own capital (production menu)
- Player clicks on their own unit (unit info)
- Panel sections use `Visibility::Visible` or `Visibility::Hidden` to show/hide

**Hides When:**

- Player clicks on empty tile → both sections hidden
- Player clicks on enemy unit (not implemented to select)
- Selection is cleared

### Mutual Exclusivity Pattern

Multiple panels on the same sidebar should be mutually exclusive:

**Example: Left Sidebar (Production Menu vs Unit Info)**

- When capital selected: production menu shows, unit info hidden
- When unit selected: unit info shows, production menu hidden
- When empty tile selected: both hidden

**Implementation Guidelines:**

- Each panel's update function checks selection state
- Selection systems clear conflicting selections
- Bidirectional clearing: If A shows, clear B's state; if B shows, clear A's state
- All related panels should be cleared when clicking empty space

## Critical Implementation Rules for Native Bevy UI

### Rule #1: Use Visibility Component

**CRITICAL**: For ALL conditional panels, use Bevy's `Visibility` component to show/hide elements.

- ✅ **Use**: `Visibility::Visible` to show panels
- ✅ **Use**: `Visibility::Hidden` to hide panels
- ✅ **Query**: `Query<&mut Visibility>` to toggle panel visibility

### Why This Matters

Bevy's `Visibility` component is the standard way to control whether entities are rendered. It integrates properly with Bevy's rendering pipeline and inherited visibility system.

This applies to:

- Unit info panels
- Production menus
- City information panels
- Diplomacy screens
- Any conditional UI element

### Standard Code Pattern for Any Panel

**Panel Spawning**:

```rust
pub fn spawn_panel(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Percent(100.0),
                // ... other style properties
                ..default()
            },
            background_color: Color::srgb(0.18, 0.18, 0.18).into(),
            visibility: Visibility::Hidden, // Start hidden
            ..default()
        },
        PanelMarker, // Component to identify this panel
    ))
    .with_children(|parent| {
        // Spawn child UI elements
    });
}
```

**Panel Update System**:

```rust
pub fn update_panel_visibility(
    mut panel_query: Query<&mut Visibility, With<PanelMarker>>,
    selection: Res<SomeSelection>,
) {
    for mut visibility in &mut panel_query {
        if selection.is_active() {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
```

### Rule #2: Use Marker Components

Always use marker components to identify specific panels for querying:

```rust
#[derive(Component)]
pub struct LeftPanel;

#[derive(Component)]
pub struct ProductionMenu;

#[derive(Component)]
pub struct UnitInfoPanel;
```

This allows precise queries without relying on entity hierarchy traversal.

## Generic Selection Clearing Pattern

### Principle: Clear All Competing States

When implementing any selection system that shows/hides panels, always clear competing selection states. This prevents multiple panels from appearing simultaneously or old panels from staying visible.

### Example Implementation in tile_selection.rs

When any tile is clicked, check in priority order and clear competing states:

1. **Check highest priority** (e.g., capital) → show its panel, clear other selections
2. **Check medium priority** (e.g., unit, city) → check if present at position
3. **If empty** (no special tile) → clear ALL panel selections

```rust
// Generic pattern for any tile-based selection
let primary_selection_made = check_primary_selection(...);
let secondary_at_position = check_secondary_query.iter()
    .any(|(_, _, pos)| pos == clicked_position);

if !primary_selection_made && !secondary_at_position {
    // Clear ALL competing panel selections
    clear_primary_selection();
    clear_secondary_selection();
    clear_tertiary_selection();
    // ... clear any other panel states
}
```

### Bidirectional Clearing Pattern

When implementing selection handlers, BOTH systems must clear each other:

**System A (e.g., unit_interaction.rs):**

```rust
// If unit found → select it, clear capital/city/etc
if unit_found {
    select_unit();
    clear_capital_selection();
    clear_city_selection();
}
// If no unit → clear unit AND other selections
else {
    clear_unit_selection();
    clear_capital_selection();
    clear_city_selection();
}
```

**System B (e.g., tile_selection.rs for capitals):**

```rust
// If capital found → select it, clear unit/city/etc
if capital_found {
    select_capital();
    clear_unit_selection();
    clear_city_selection();
}
```

This bidirectional clearing ensures panels never conflict.

## Panel-Specific Implementation Notes

### Scrollable Panels

When a panel container may overflow (e.g., left/right sidebars with multiple components), enable scrolling using the `overflow` style property.

**In Native Bevy UI:**

```rust
NodeBundle {
    style: Style {
        width: Val::Px(300.0),
        height: Val::Percent(100.0),
        overflow: Overflow {
            x: OverflowAxis::Clip,     // Horizontal clipping
            y: OverflowAxis::Scroll,   // Vertical scrolling
        },
        // ... other properties
        ..default()
    },
    // ... other bundle fields
    ..default()
}
```

**Overflow options:**

- `OverflowAxis::Visible`: Content not clipped (default)
- `OverflowAxis::Clip`: Content clipped at boundary
- `OverflowAxis::Scroll`: Scrollable when content exceeds container

**Scrolling behavior:**

- Panels automatically get scroll behavior when content exceeds container size
- Mouse wheel scrolling works automatically when hovering over scrollable areas
- Scroll position is maintained by Bevy's built-in scroll system

**IMPORTANT - Preventing Camera Zoom on UI Scroll:**

To prevent the camera from zooming when scrolling over UI panels, the mouse input system must check if the cursor is over a UI panel before processing zoom events:

```rust
// In handle_mouse_input system
pub fn handle_mouse_input(
    mut mouse_wheel: MessageReader<MouseWheel>,
    // ... other params
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let ui_bounds = window_query
        .single()
        .ok()
        .map(UiPanelBounds::from_window);

    handle_camera_zoom_controls(&mut mouse_wheel, &mut camera_query, ui_bounds.as_ref(), &last_cursor_pos);
}

// In handle_camera_zoom_controls
fn handle_camera_zoom_controls(
    mouse_wheel: &mut MessageReader<MouseWheel>,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
    ui_bounds: Option<&UiPanelBounds>,
    last_cursor_pos: &Local<Option<Vec2>>,
) {
    for wheel_event in mouse_wheel.read() {
        // Skip camera zoom if cursor is over UI panel
        if let (Some(cursor_pos), Some(bounds)) = (**last_cursor_pos, ui_bounds) {
            if is_cursor_over_ui_panel(cursor_pos, bounds) {
                continue; // Let UI handle the scroll event
            }
        }

        // Apply camera zoom
        apply_camera_zoom_from_wheel_event(wheel_event, &mut camera_transform);
    }
}
```

This ensures that:

- Mouse wheel events over UI panels are ignored by the camera system
- UI panels can handle scroll events independently
- Camera zoom only occurs when scrolling over the game world

### Unit Stats Panel: Combat Effectiveness

Effective stats are calculated in `core_sim/src/components/military.rs`:

```rust
// Fatigue and supply reduce effectiveness
effective_attack = base_attack * (1.0 - fatigue * 0.3) * supply
effective_defense = base_defense * (1.0 - fatigue * 0.3) * supply
```

### Future Panel Types

As you add more panels, follow the same patterns:

- **City Panel**: City stats, building queue, garrison units
- **Diplomacy Screen**: Civ relations, trade offers, declarations
- **Technology Tree**: Research progress, available techs
- **Event Panels**: Random events, quests, notifications

Each should follow the structure outlined above with:

- Data struct with `is_visible: String`
- Builder function checking game state
- Property update function
- Selection/state management systems
- Proper clearing of competing selections

## Common Issues & Solutions (Any Panel)

### Issue: Panel doesn't hide when it should

**Root Cause**: Selection state not being cleared properly.

**Solution**:

- Ensure selection system has access to ALL relevant queries
- Check that empty tile clicks clear ALL competing panel states
- Verify bidirectional clearing (both systems clear each other)

### Issue: Text not updating in UI

**Root Cause**: Text component not being queried or updated properly.

**Solution**: Verify:

- Text components have marker components for querying
- Update systems are running in the correct schedule
- Query filters are correct (e.g., `With<SomeMarker>`)
- Text updates use `text.sections[0].value = new_value.to_string()`

### Issue: Panel shows old/stale data

**Root Cause**: Selection state persists when it shouldn't.

**Solution**:

- ALL selection handlers must clear competing states
- Use bidirectional clearing pattern
- When in doubt, clear everything when clicking empty space

### Issue: Multiple panels appear simultaneously

**Root Cause**: Missing mutual exclusivity checks.

**Solution**:

- Each panel's builder function checks other panels' visibility state
- Selection handlers clear competing selections before setting new ones
- Implement priority system (higher priority panels hide lower priority)

### Issue: Panel visibility not toggling

**Root Cause**: Visibility component not being updated or queried incorrectly.

**Solution**:

- Ensure you're querying `Query<&mut Visibility>`
- Set visibility to `Visibility::Visible` or `Visibility::Hidden`
- Check that the visibility update system is running
- Verify marker components are correct

## Related Documentation

- [Action Queue System](./action_queue_system.md)
- [Fog of War](./fog_of_war.md)
- [Menu System](./menu_system.md)

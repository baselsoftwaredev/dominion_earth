# UI Scrolling Implementation Guide

## Overview

This guide documents the implementation of mouse wheel scrolling for Bevy UI panels, including the technical challenges encountered, solutions applied, and key learnings about Bevy's event system and UI components.

## Problem Statement

The left panel in the game UI needed to support vertical scrolling when content exceeded the available viewport height. Without scrolling, UI sections (GamePanel, ProductionMenuPanel, UnitInfoPanel) would be clipped or hidden when the window height was insufficient to display all content.

## Bevy UI Scroll Components

### ScrollPosition Component

Bevy provides a built-in `ScrollPosition` component that tracks the current scroll offset:

```rust
ScrollPosition::default()  // Initializes with x: 0.0, y: 0.0
```

**Key Properties:**

- `x`: Horizontal scroll offset (not used in our implementation)
- `y`: Vertical scroll offset (0.0 = top, increases downward)

### Overflow Property

The `Overflow` property on a `Node` determines whether content can scroll:

```rust
Node {
    overflow: Overflow::scroll_y(),  // Enable vertical scrolling
    ..default()
}
```

**Options:**

- `Overflow::visible()`: Content extends beyond bounds (default)
- `Overflow::clip()`: Content is clipped at bounds
- `Overflow::scroll_y()`: Enables vertical scrolling
- `Overflow::scroll_x()`: Enables horizontal scrolling
- `Overflow::scroll()`: Enables both axes

## Implementation Architecture

### Required Components on Scrollable Container

```rust
commands.spawn((
    LeftPanel,
    Node {
        overflow: Overflow::scroll_y(),  // REQUIRED: Enable scroll
        ..default()
    },
    ScrollPosition::default(),           // REQUIRED: Track scroll offset
    BackgroundColor(...),
))
```

### Child Panel Configuration

All child panels must have `flex_shrink: 0.0` to prevent compression:

```rust
Node {
    flex_shrink: 0.0,  // CRITICAL: Prevents panel from shrinking
    ..default()
}
```

**Why This Matters:**
Without `flex_shrink: 0.0`, Bevy's flexbox layout will compress child panels to fit the container, defeating the purpose of scrolling. The content must be allowed to exceed container bounds for scrolling to activate.

## Event System Architecture

### MessageReader vs EventReader

Bevy 0.17+ uses `MessageReader` for event handling. Key characteristics:

**Each MessageReader maintains its own cursor:**

```rust
fn system_a(mut events: MessageReader<MouseWheel>) {
    for event in events.read() {
        // This system's cursor advances
    }
}

fn system_b(mut events: MessageReader<MouseWheel>) {
    for event in events.read() {
        // Independent cursor - sees the same events
    }
}
```

**Critical Insight:**
Multiple systems can read the same events independently. This is different from older Bevy versions where `EventReader` would consume events for all systems.

### System Ordering

The scroll handler must run BEFORE input systems that might consume mouse wheel events:

```rust
.add_systems(
    Update,
    handle_ui_scroll
        .before(crate::input::handle_mouse_input)
        .run_if(in_state(Screen::Gameplay))
)
```

**Why Ordering Matters:**

- Scroll events should be processed when cursor is over UI
- Camera zoom/pan should be skipped when scrolling UI
- System ordering ensures UI gets priority over game world input

## Mouse Wheel Event Handling

### Event Structure

```rust
pub struct MouseWheel {
    pub unit: MouseScrollUnit,  // Line or Pixel
    pub y: f32,                  // Vertical scroll delta
    pub x: f32,                  // Horizontal scroll delta (unused)
}
```

### Converting Scroll Units to Pixels

Different platforms report scroll events differently:

```rust
const MOUSE_WHEEL_PIXELS_PER_LINE: f32 = 20.0;

let scroll_delta_in_pixels = match scroll_event.unit {
    MouseScrollUnit::Line => {
        scroll_event.y * MOUSE_WHEEL_PIXELS_PER_LINE
    }
    MouseScrollUnit::Pixel => scroll_event.y,
};
```

**Platform Differences:**

- **macOS/trackpads**: Usually send `Pixel` events (smooth scrolling)
- **Windows/mice**: Usually send `Line` events (discrete scrolling)
- **Linux**: Varies by distribution and device

## Bounds Checking and Cursor Position

### Screen Coordinate System

Bevy's window coordinates:

- Origin (0, 0) is at **top-left corner**
- X increases rightward
- Y increases downward

### Determining Cursor Over UI Panel

```rust
let cursor_position = window.cursor_position()?;  // Returns Option<Vec2>

let left_panel_width = 300.0;
let header_height = 80.0;

let is_cursor_over_left_panel =
    cursor_position.x <= left_panel_width
    && cursor_position.y >= header_height;
```

**Design Decision:**
We use simple screen coordinate checking rather than complex GlobalTransform intersection because:

1. Our panels are absolutely positioned with known dimensions
2. Screen coordinates are simpler and more reliable
3. No need for transform hierarchy calculations

## Scroll Position Clamping

### Calculating Maximum Scroll Offset

```rust
let content_size = computed_node.content_size();  // Total content height
let node_size = computed_node.size();             // Visible viewport height

let maximum_scroll_offset = (content_size.y - node_size.y).max(0.0);
```

**Key Formula:**

```
max_scroll = max(content_height - viewport_height, 0)
```

**Why `.max(0.0)`:**
When content is smaller than viewport, we don't want negative scroll values. The content should stay at the top (scroll = 0).

### Applying Scroll Delta

```rust
scroll_position.y = (scroll_position.y - scroll_delta_in_pixels)
    .clamp(0.0, maximum_scroll_offset);
```

**Direction Note:**

- Scroll wheel "up" (positive delta) → decrease scroll_position.y (scroll toward top)
- Scroll wheel "down" (negative delta) → increase scroll_position.y (scroll toward bottom)

## Debugging Journey and Lessons Learned

### Issue 1: ScrollPosition Component Missing

**Symptom:** Scroll didn't work at all, no visual response to mouse wheel.

**Cause:** Forgot to add `ScrollPosition::default()` component to the scrollable container.

**Lesson:** Both `Overflow::scroll_y()` AND `ScrollPosition` are required. Bevy won't create `ScrollPosition` automatically.

### Issue 2: System Not Receiving Events

**Symptom:** Debug logs showed system wasn't executing.

**Investigation Steps:**

1. Added verbose debug logging to confirm system execution
2. Discovered system wasn't registered with proper state filter
3. System was only running in generic `Update` schedule

**Solution:** Add `.run_if(in_state(Screen::Gameplay))` to system registration:

```rust
.add_systems(
    Update,
    handle_ui_scroll
        .before(crate::input::handle_mouse_input)
        .run_if(in_state(screen.clone()))
)
```

**Lesson:** Systems must be registered in the correct state/schedule to execute. Generic `Update` systems won't run in screen-specific contexts.

### Issue 3: Panels Compressed Instead of Scrolling

**Symptom:** Content was squished to fit container instead of overflowing.

**Cause:** Bevy's flexbox layout defaults to shrinking children to fit container.

**Solution:** Add `flex_shrink: 0.0` to all child panels:

```rust
Node {
    flex_shrink: 0.0,  // Maintain natural size
    ..default()
}
```

**Lesson:** Understanding CSS flexbox behavior is critical for Bevy UI. Default `flex_shrink: 1.0` allows compression.

### Issue 4: Bounds Checking with Wrong Coordinates

**Symptom:** Scroll worked inconsistently or in wrong areas.

**Initial Attempt:** Tried using `GlobalTransform` to check if cursor was within panel bounds.

**Problem:** Complex transform calculations, hierarchy dependencies, margin of error.

**Final Solution:** Use simple screen coordinates since panels are absolutely positioned:

```rust
let is_cursor_over_left_panel = cursor_position.x <= left_panel_width
    && cursor_position.y >= header_height;
```

**Lesson:** Choose the simplest solution that works. Don't over-engineer bounds checking when simple math suffices.

## State Management Considerations

### State-Specific System Registration

When working with Bevy states (e.g., `Screen::Gameplay`, `Screen::MainMenu`), ensure systems are registered appropriately:

**Option 1: State-Filtered Systems**

```rust
.add_systems(
    Update,
    my_system.run_if(in_state(Screen::Gameplay))
)
```

**Option 2: OnEnter/OnExit**

```rust
.add_systems(OnEnter(Screen::Gameplay), setup_system)
.add_systems(OnExit(Screen::Gameplay), cleanup_system)
```

### UI Cleanup on State Transitions

When transitioning between screens, clean up UI entities:

```rust
fn cleanup_ui(
    mut commands: Commands,
    panel_query: Query<Entity, With<LeftPanel>>,
    children_query: Query<&Children>,
) {
    for entity in &panel_query {
        recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
        );
    }
}
```

## Performance Considerations

### Early Returns

Use early returns to minimize computation when scroll isn't needed:

```rust
let Ok(window) = window_query.single() else {
    return;  // No window, skip processing
};

let Some(cursor_position) = window.cursor_position() else {
    return;  // Cursor not in window, skip processing
};

if !is_cursor_over_left_panel {
    return;  // Not hovering UI, skip processing
}
```

### Single Scrollable Element

Break after processing the first scrollable element to avoid unnecessary iterations:

```rust
for (mut scroll_position, node, computed_node) in &mut scrollable_query {
    // ... process scroll ...
    break;  // Only scroll first match
}
```

## Testing Strategy

### Runtime Testing Checklist

- [ ] Scroll activates when cursor is over left panel
- [ ] Scroll doesn't activate when cursor is over game world
- [ ] Content scrolls smoothly without jumps
- [ ] Scroll stops at top (position = 0)
- [ ] Scroll stops at bottom (position = max_offset)
- [ ] All panels remain visible when scrolling
- [ ] Window resize recalculates scroll bounds correctly
- [ ] Debug logging shows correct events and calculations (with `--debug-logging`)

### Edge Cases to Test

1. **Window smaller than content:** Should allow full scrolling
2. **Window larger than content:** Should disable scrolling (max_offset = 0)
3. **Dynamic content changes:** Content grows/shrinks during gameplay
4. **Multiple scrollable panels:** Only scroll the one under cursor
5. **Rapid scroll events:** No stuttering or event loss

## References and Resources

### Example Code

Bevy's official scroll example:

```
https://github.com/bevyengine/bevy/blob/release-0.17.2/examples/ui/scroll.rs
```

This example demonstrates:

- Multiple scrollable containers
- Nested scroll areas
- Different scroll configurations

### Related Components

- `ScrollPosition`: Tracks scroll offset
- `Node`: UI layout properties including `overflow`
- `ComputedNode`: Runtime-calculated size and layout info
- `Interaction`: Button/element interaction states

## Future Enhancements

### Potential Improvements

1. **Scroll Bar Visualization**

   - Add visual scroll bar indicator
   - Show current position within scrollable area

2. **Smooth Scrolling Animation**

   - Interpolate scroll position over time
   - Ease in/out curves for natural feel

3. **Touch/Drag Scrolling**

   - Support touch input on mobile/tablets
   - Click-and-drag scroll on desktop

4. **Keyboard Navigation**

   - Arrow keys for scrolling
   - Page Up/Down for larger jumps
   - Home/End for top/bottom

5. **Scroll Velocity**
   - Configurable scroll speed multiplier
   - Different speeds for different input devices

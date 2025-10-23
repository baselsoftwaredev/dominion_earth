# UI Panel Implementation Guide

## Overview

This document covers best practices and patterns for implementing conditional UI panels in Dominion Earth using bevy_hui. While this guide uses the unit stats panel as the primary example, these patterns apply to all UI panels (production menus, city panels, diplomacy screens, etc.).

## General Panel Architecture

All conditional panels follow this structure:

### 1. UI Template (HTML)

Location: `dominion_earth/assets/ui/components/[sidebar]/[panel_name].html`

- Defines visual layout and styling
- Contains property definitions with default values
- Uses `display="{visibility_property}"` for conditional rendering

### 2. Data Structure (Rust)

Location: `dominion_earth/src/ui/bevy_hui/property_updates.rs`

- Struct holding all panel data as Strings
- Includes visibility property (String: "flex" or "none")
- One struct per panel type

### 3. Builder Function

Location: `dominion_earth/src/ui/bevy_hui/property_updates.rs`

- Function named `build_[panel_name]_data()`
- Queries game state and populates data structure
- Returns data struct with appropriate visibility value

### 4. Property Update Function

Location: `dominion_earth/src/ui/bevy_hui/property_updates.rs`

- Function named `update_[panel_name]_properties()`
- Maps struct fields to template properties
- Called during UI update cycle

### 5. State Management (Selection Logic)

Location: `dominion_earth/src/input/` or relevant systems

- Systems that manage when panel should show
- Resource/component to track selection state
- Clear/update logic for mutual exclusivity

## Example: Unit Stats Panel

### Specific Implementation Locations

- **UI Template**: `dominion_earth/assets/ui/components/right_side_panel/unit_info.html`
- **Data Structure**: `UnitInformation` struct in `property_updates.rs`
- **Builder**: `build_unit_info_data()` in `property_updates.rs`
- **Updater**: `update_unit_info_properties()` in `property_updates.rs`
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

### Example: Unit Stats Panel Visibility

**Shows When:**

- Player clicks on their own unit
- `selected_unit.unit_entity` is Some(entity)
- Unit belongs to player civilization (`unit.owner == player_civ_id`)
- Production menu is NOT showing

**Hides When:**

- Player clicks on capital → production menu takes precedence
- Player clicks on empty tile (no unit, no capital)
- Player clicks on enemy unit (not implemented to select)
- `selected_unit.unit_entity` is None

### Mutual Exclusivity Pattern

Multiple panels on the same sidebar should be mutually exclusive:

**Example: Left Sidebar (Production Menu vs Unit Stats)**

- When capital selected: production menu shows, unit stats hidden
- When unit selected: unit stats show, production menu hidden
- When empty tile selected: both hidden

**Implementation Guidelines:**

- Each panel's builder function checks other panels' state
- Selection systems clear conflicting selections
- Bidirectional clearing: If A shows, clear B's state; if B shows, clear A's state
- All related panels should be cleared when clicking empty space

## Critical Implementation Rules for bevy_hui

### Rule #1: Use Display Property, Not s:if

**CRITICAL**: For ALL conditional panels in bevy_hui, use `display="{property}"` instead of `s:if="{property}"`.

- ❌ **NEVER use**: `s:if="{is_visible}"` with boolean values
- ✅ **ALWAYS use**: `display="{visibility_property}"` with string values "flex" or "none"

### Why This Matters

The `s:if` directive in bevy_hui doesn't work reliably with boolean string values ("true"/"false"). Using the `display` CSS property with "flex"/"none" values works consistently with bevy_hui's templating system and properly shows/hides panels.

This applies to:

- Unit panels
- Production menus
- City information panels
- Diplomacy screens
- Any conditional UI element

### Standard Code Pattern for Any Panel

**In Rust (property_updates.rs)**:

```rust
// Data structure for ANY panel
pub struct PanelInformation {
    pub is_visible: String,  // ALWAYS String, NEVER bool!
    pub field1: String,
    pub field2: String,
    // ... other fields
}

// When panel should be visible
PanelInformation {
    is_visible: "flex".to_string(),
    field1: actual_value.to_string(),
    // ...
}

// When panel should be hidden
PanelInformation {
    is_visible: "none".to_string(),
    field1: "default".to_string(),  // Still need default values
    // ...
}
```

**In HTML Template** (any panel):

```html
<property name="is_visible">none</property>
<property name="field1">Default</property>
<property name="field2">Default</property>

<node
  display="{is_visible}"
  width="100%"
  background="#2d2d2d"
  border="2px solid #444444"
  ...>
  <text>{field1}</text>
  <text>{field2}</text>
  <!-- panel content -->
</node>
```

### Rule #2: Property Name Consistency

Properties must match EXACTLY across three locations:

1. **HTML Template**: Property definitions in component (`<property name="field_name">`)
2. **Rust Struct**: Field names in data structure (`pub field_name: String`)
3. **Parent Binding**: Property bindings when including component (`field_name="{field_name}"`)

**Important**: No prefixes! Pass property names directly without panel-specific prefixes (e.g., `attack` not `unit_attack` in the component itself, though parent may use prefixes for disambiguation).

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

### Issue: Properties not updating in UI

**Root Cause**: Property name mismatch or wrong data type.

**Solution**: Verify exact matches across:
- HTML template property definitions (`<property name="field">`)
- Parent component property bindings (`field="{field}"`)
- Rust struct field names (`pub field: String`)
- Property insertion (`template_properties.insert("field", value)`)

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

### Issue: Panel uses s:if and doesn't work

**Root Cause**: Using `s:if` instead of `display` property.

**Solution**: 
- Change HTML from `s:if="{is_visible}"` to `display="{is_visible}"`
- Change Rust struct field from `pub is_visible: bool` to `pub is_visible: String`
- Set values to `"flex".to_string()` or `"none".to_string()`

## Related Documentation

- [Action Queue System](./action_queue_system.md)
- [Fog of War](./fog_of_war.md)
- [Menu System](./menu_system.md)

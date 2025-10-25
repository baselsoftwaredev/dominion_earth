# Bevy HUI to Native Bevy UI Migration Guide

## Overview

This guide documents the process of migrating UI components from bevy_hui's HTML-based system to native Bevy UI. This migration improves maintainability, IDE support, and makes the codebase easier for AI assistants to understand and modify.

## Why Migrate?

### Benefits of Native Bevy UI:

- **Better maintainability**: Pure Rust code vs HTML templates
- **Superior IDE support**: Full autocomplete, type checking, and refactoring
- **Easier AI assistance**: Standard Bevy patterns are better understood
- **Better performance**: No HTML parsing overhead
- **More flexible**: Direct access to Bevy's ECS for complex interactions
- **Type safety**: Compile-time guarantees instead of runtime string-based properties

### Drawbacks of bevy_hui:

- Requires understanding both HTML-like syntax and Bevy
- Limited tooling support for HTML templates
- Harder to debug (template compilation errors)
- Property binding can be fragile
- Difficult to refactor across template boundaries

## Migration Process

### Step 1: Analyze the Existing HTML Component

**Example: Top Panel HTML**

```html
<!-- top_panel.html -->
<template>
  <property name="game_title">Dominion Earth</property>
  <property name="current_turn">1</property>
  <property name="player_gold">0</property>
  <property name="player_production">0</property>
  <node position="absolute" width="100%" height="100%">
    <left_side_top
      game_title="{game_title}"
      player_gold="{player_gold}"
      player_production="{player_production}"
      current_turn="{current_turn}" />
  </node>
</template>
```

**Identify:**

1. Layout structure (nodes, containers)
2. Styling (colors, sizes, positioning)
3. Dynamic properties (data that changes)
4. Child components
5. Event handlers

### Step 2: Create Marker Components

Create marker components for each UI element that needs to be queried or updated.

```rust
// Marker component for the container
#[derive(Component)]
pub struct TopPanel;

// Marker components for dynamic text elements
#[derive(Component)]
pub struct GameTitleText;

#[derive(Component)]
pub struct GoldDisplayText;

#[derive(Component)]
pub struct ProductionDisplayText;

#[derive(Component)]
pub struct TurnDisplayText;
```

**Best Practices:**

- Use descriptive names that indicate purpose
- Suffix with `Text` for text elements
- Keep components simple (no data needed for markers)
- Group related markers in the same module

### Step 3: Create the Spawn Function

Build the UI hierarchy using Bevy's native components.

```rust
/// Spawn the top panel UI hierarchy
pub fn spawn_top_panel(mut commands: Commands) {
    commands
        .spawn((
            TopPanel,  // Marker for querying the root
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(HEADER_HEIGHT),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.165, 0.165, 0.165, 1.0)),
            Name::new("Top Panel"),
        ))
        .with_children(|parent| {
            // Spawn child elements
            parent.spawn((
                GameTitleText,
                Text::new("Dominion Earth"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::horizontal(Val::Px(20.0)),
                    ..default()
                },
                Name::new("Game Title"),
            ));

            // More children...
        });
}
```

**Key Mappings:**

| HTML Attribute                    | Bevy Component                                               | Notes               |
| --------------------------------- | ------------------------------------------------------------ | ------------------- |
| `position="absolute"`             | `Node { position_type: PositionType::Absolute, .. }`         |                     |
| `width="100%"`                    | `Node { width: Val::Percent(100.0), .. }`                    |                     |
| `height="80px"`                   | `Node { height: Val::Px(80.0), .. }`                         |                     |
| `background="#2a2a2a"`            | `BackgroundColor(Color::srgba(r, g, b, a))`                  | Convert hex to RGBA |
| `border="2px solid #444"`         | `BorderColor`, `Node { border: UiRect::all(Val::Px(2.0)) }`  |                     |
| `border_radius="8px"`             | `BorderRadius::all(Val::Px(8.0))`                            |                     |
| `flex_direction="row"`            | `Node { flex_direction: FlexDirection::Row, .. }`            |                     |
| `justify_content="space_between"` | `Node { justify_content: JustifyContent::SpaceBetween, .. }` |                     |
| `align_items="center"`            | `Node { align_items: AlignItems::Center, .. }`               |                     |
| `padding="10px"`                  | `Node { padding: UiRect::all(Val::Px(10.0)), .. }`           |                     |
| `margin="5px"`                    | `Node { margin: UiRect::all(Val::Px(5.0)), .. }`             |                     |
| `<text>Content</text>`            | `Text::new("Content")`                                       |                     |
| `font_size="18"`                  | `TextFont { font_size: 18.0, .. }`                           |                     |
| `font_color="#ffcc00"`            | `TextColor(Color::srgba(1.0, 0.8, 0.0, 1.0))`                |                     |

**Color Conversion:**

```rust
// HTML: #2a2a2a
// Bevy: Color::srgba(0.165, 0.165, 0.165, 1.0)
// Calculation: hex / 255.0 (e.g., 0x2a = 42, 42/255 ≈ 0.165)
```

### Step 4: Create Update Systems

Replace property binding functions with ECS systems that query and update components.

**Old bevy_hui approach:**

```rust
html_functions.register(
    "update_player_gold",
    |In(entity): In<Entity>,
     mut commands: Commands,
     mut template_properties: Query<&mut TemplateProperties>,
     player_civs: Query<&Civilization, With<PlayerControlled>>| {
        // Complex property update logic
    },
);
```

**New native Bevy approach:**

```rust
/// Update gold and production displays from player civilization
pub fn update_player_resources(
    player_query: Query<&Civilization, With<PlayerControlled>>,
    mut gold_text: Query<&mut Text, (With<GoldDisplayText>, Without<ProductionDisplayText>)>,
    mut production_text: Query<&mut Text, With<ProductionDisplayText>>,
) {
    if let Some(player_civ) = player_query.iter().next() {
        // Update gold display
        if let Some(mut text) = gold_text.iter_mut().next() {
            **text = format!("Gold: {}", player_civ.economy.gold);
        }

        // Update production display
        if let Some(mut text) = production_text.iter_mut().next() {
            **text = format!("Production: {}", player_civ.economy.production);
        }
    }
}

/// Update turn display from CurrentTurn resource
pub fn update_turn_display(
    current_turn: Res<CurrentTurn>,
    mut turn_text: Query<&mut Text, With<TurnDisplayText>>,
) {
    if current_turn.is_changed() {
        if let Some(mut text) = turn_text.iter_mut().next() {
            **text = format!("Turn: {}", current_turn.0);
        }
    }
}
```

**Best Practices:**

- Use `is_changed()` to avoid unnecessary updates
- Query only what you need (use `Without` filters to disambiguate)
- Use `iter().next()` for optional single-entity queries
- Format text directly instead of managing string properties

### Step 5: Register Systems

Add the spawn and update systems to your app.

```rust
impl BevyHuiSystem {
    pub fn setup_plugins_for_screen<S: States>(app: &mut App, screen: S) {
        app.add_plugins((HuiPlugin, HuiAutoLoadPlugin::new(&["ui"])))
            // Add native spawn alongside HTML spawn
            .add_systems(
                OnEnter(screen.clone()),
                (setup_main_ui, crate::ui::top_panel::spawn_top_panel)
            )
            .add_systems(OnExit(screen.clone()), cleanup_ui)
            .add_systems(
                Update,
                (
                    // HTML update systems
                    setup_scrollable_panels,
                    update_ui_properties_system.run_if(should_update_ui_this_frame),

                    // Native update systems
                    crate::ui::top_panel::update_player_resources,
                    crate::ui::top_panel::update_turn_display,
                )
                    .run_if(in_state(screen)),
            );
    }
}
```

### Step 6: Update Cleanup Logic

Ensure both HTML and native panels are cleaned up properly.

```rust
fn cleanup_ui(
    mut commands: Commands,
    ui_panels: Query<Entity, With<HtmlNode>>,
    top_panel: Query<Entity, With<crate::ui::top_panel::TopPanel>>,
    children_query: Query<&Children>,
) {
    let mut despawned = std::collections::HashSet::new();

    // Despawn HtmlNode-based panels
    for entity in &ui_panels {
        recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
            &mut despawned,
        );
    }

    // Despawn native panels
    for entity in &top_panel {
        recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
            &mut despawned,
        );
    }
}
```

### Step 7: Update Visibility Management

If your UI has visibility toggling (e.g., during menu transitions), update those systems.

```rust
pub fn hide_gameplay_ui_panels(
    mut html_ui_panels: Query<&mut Visibility, With<HtmlNode>>,
    mut top_panel: Query<&mut Visibility, (With<TopPanel>, Without<HtmlNode>)>,
    debug_logging: Res<DebugLogging>,
) {
    // Hide HTML-based panels
    for mut panel_visibility in &mut html_ui_panels {
        *panel_visibility = Visibility::Hidden;
    }

    // Hide native panels
    for mut panel_visibility in &mut top_panel {
        *panel_visibility = Visibility::Hidden;
    }
}

pub fn show_gameplay_ui_panels(
    mut html_ui_panels: Query<&mut Visibility, With<HtmlNode>>,
    mut top_panel: Query<&mut Visibility, (With<TopPanel>, Without<HtmlNode>)>,
    debug_logging: Res<DebugLogging>,
) {
    // Show HTML-based panels
    for mut panel_visibility in &mut html_ui_panels {
        *panel_visibility = Visibility::Visible;
    }

    // Show native panels
    for mut panel_visibility in &mut top_panel {
        *panel_visibility = Visibility::Visible;
    }
}
```

### Step 8: Deprecate Old Code

Mark the old bevy_hui spawn function as deprecated but keep it temporarily.

```rust
/// DEPRECATED: Now using native Bevy UI implementation in top_panel.rs
#[allow(dead_code)]
fn spawn_top_panel(commands: &mut Commands, asset_server: &AssetServer) {
    // Old implementation...
}
```

Comment out the call in the spawn function:

```rust
fn spawn_main_ui_layout_panels(commands: &mut Commands, asset_server: &AssetServer) {
    // Top panel is now handled by native Bevy UI in top_panel.rs
    // spawn_top_panel(commands, asset_server);
    spawn_right_side_panel(commands, asset_server);
    spawn_left_side_panel(commands, asset_server);
}
```

### Step 9: Test Thoroughly

**Test Checklist:**

- [ ] Panel displays correctly on startup
- [ ] Dynamic values update properly
- [ ] Panel hides when menus open
- [ ] Panel shows when menus close
- [ ] Panel is cleaned up when exiting screen
- [ ] No duplicate panels after screen transitions
- [ ] No console errors or warnings
- [ ] Performance is acceptable (no frame drops)

**Test with seed for consistency:**

```bash
cargo run -- --seed 1756118413
```

### Step 10: Clean Up (Optional)

Once confirmed working:

1. Remove the old HTML files (if not used elsewhere)
2. Remove deprecated functions
3. Remove unused property constants
4. Update documentation

## File Structure Template

```
dominion_earth/src/ui/
├── mod.rs                    # Module exports
├── top_panel.rs             # Native Bevy UI (NEW)
├── bevy_hui/
│   ├── mod.rs               # bevy_hui setup
│   ├── main_ui.rs           # HTML panel spawning (DEPRECATED spawn_top_panel)
│   └── property_updates.rs # HTML property updates
└── constants.rs             # Shared constants
```

## Common Pitfalls

### 1. Query Ambiguity

**Problem:** Multiple queries match the same entity.

```rust
// ❌ Ambiguous - both queries match same entities
mut gold_text: Query<&mut Text>,
mut production_text: Query<&mut Text>,
```

**Solution:** Use marker components and `Without` filters.

```rust
// ✅ Unambiguous
mut gold_text: Query<&mut Text, (With<GoldDisplayText>, Without<ProductionDisplayText>)>,
mut production_text: Query<&mut Text, With<ProductionDisplayText>>,
```

### 2. Wrong Query Methods

**Problem:** Using `get_single()` which doesn't exist.

```rust
// ❌ Error: no method named `get_single`
if let Ok(player_civ) = player_query.get_single() { }
```

**Solution:** Use `iter().next()` for optional single results.

```rust
// ✅ Correct
if let Some(player_civ) = player_query.iter().next() { }
```

### 3. Component Construction Errors

**Problem:** Wrong syntax for component initialization.

```rust
// ❌ Error: expected function, found struct
BorderColor(Color::srgba(0.267, 0.267, 0.267, 1.0))
```

**Solution:** Check component API - some use `From` trait.

```rust
// ✅ Correct
BorderColor::from(Color::srgba(0.267, 0.267, 0.267, 1.0))
```

### 4. Text Dereferencing

**Problem:** Not properly dereferencing `Text` component.

```rust
// ❌ Type error
text = format!("Gold: {}", value);
```

**Solution:** Use double dereference for `Text`.

```rust
// ✅ Correct
**text = format!("Gold: {}", value);
```

### 5. Forgetting Cleanup

**Problem:** Native panels not cleaned up, causing duplicates.

**Solution:** Always update cleanup systems to handle native panels.

### 6. Visibility Not Managed

**Problem:** Native panels stay visible during menus.

**Solution:** Update visibility management systems.

## Migration Priority

Recommended order for migrating panels:

1. **Top Panel** - Simple, static layout with few dynamic elements
2. **Side Panels** - More complex, but still primarily containers
3. **Menus** - Self-contained, clear boundaries
4. **Complex Widgets** - Last, as they may have intricate state management

## Coexistence Strategy

You can run both bevy_hui and native Bevy UI side-by-side:

1. Migrate one panel at a time
2. Test each migration thoroughly before proceeding
3. Keep old code deprecated but functional
4. Update integration points (cleanup, visibility) for both
5. Remove old code only after full migration and testing

## Performance Considerations

**Native Bevy UI is generally faster:**

- No HTML parsing at runtime
- Direct ECS queries (no property lookup)
- Better cache locality
- Fewer intermediate data structures

**Monitor:**

- Frame time during UI updates
- Memory usage (especially with many panels)
- Spawn/despawn overhead during screen transitions

## Future Migrations

When migrating other panels, follow this checklist:

- [ ] Analyze HTML structure and properties
- [ ] Create marker components
- [ ] Implement spawn function with proper styling
- [ ] Create update systems (replace property functions)
- [ ] Register systems in plugin
- [ ] Update cleanup logic
- [ ] Update visibility management
- [ ] Deprecate old code
- [ ] Test thoroughly
- [ ] Document any panel-specific quirks

## Example: Complete Migration

See `/dominion_earth/src/ui/top_panel.rs` for a complete reference implementation.

## References

- [Bevy UI Documentation](https://bevyengine.org/learn/book/migration-guides/)
- [Bevy UI Examples](https://github.com/bevyengine/bevy/tree/main/examples/ui)
- Project: `/docs/ui_panel_implementation_guide.md` (if exists)

---

**Last Updated:** October 25, 2025  
**Migrated Panels:** Top Panel  
**Remaining Panels:** Left Side Panel, Right Side Panel, Production Menu, Statistics Panel, Tile Info, Minimap

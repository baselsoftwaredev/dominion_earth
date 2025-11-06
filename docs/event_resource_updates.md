# Event Resource Updates - Top Panel

How to update top panel resources when events are triggered.

## Overview

When an event is processed, its effects modify the civilization's economy. The top panel automatically reflects these changes.

## Flow

1. Event triggered → GameEvent entity created
2. Player selects choice → `selected_choice` stored on GameEvent
3. `apply_event_effects` system runs → effects applied to `civilization.economy`
4. `update_player_resources` system runs → UI text updated
5. Popup despawned

## Example: Gold Change Event

### 1. Define Event in RON

`dominion_earth/assets/data/events.ron`:

```ron
EventDefinition(
    id: "gold_discovery",
    title: "Gold Discovery!",
    description: "Your miners found gold!",
    trigger: Random { probability: 0.01 },
    effects: [],  // Base effects (if no choice made)
    choices: [
        EventChoice(
            text: "Keep it",
            effects: [GoldChange(50.0)],
        ),
        EventChoice(
            text: "Share with people",
            effects: [GoldChange(25.0)],
        ),
    ],
)
```

### 2. Effect is Applied

`core_sim/src/systems/events.rs` in `apply_event_effects()`:

```rust
// Gets choice effects based on player selection
let effects_to_apply = if let Some(choice_index) = event.selected_choice {
    if let Some(choice) = event.choices.get(choice_index) {
        &choice.effects  // Use selected choice's effects
    } else {
        &event.effects   // Fallback
    }
} else {
    &event.effects       // No choice = base effects
};

// Apply each effect
for effect in effects_to_apply {
    apply_effect(effect, &mut civ, &mut modifiers, &mut commands);
}
```

### 3. Gold is Modified

`core_sim/src/systems/events.rs` in `apply_effect()`:

```rust
match effect {
    EventEffect::GoldChange(amount) => {
        civilization.economy.gold += amount;  // Modifies Civilization component
        info!("Applied gold change: {}", amount);
    }
    // ... other effects
}
```

### 4. UI Updates Automatically

`dominion_earth/src/ui/top_panel/resources_section.rs`:

```rust
pub fn update_player_resources(
    player_query: Query<&Civilization, With<PlayerControlled>>,
    mut gold_text: Query<&mut Text, With<GoldDisplayText>>,
) {
    let Ok(player_civ) = player_query.single() else { return; };

    for mut text in gold_text.iter_mut() {
        let new_text = format!("Gold: {:.0}", player_civ.economy.gold);
        if **text != new_text {
            info!("Gold text updated");  // Only logs on change
            **text = new_text;
        }
    }
}
```

## Adding New Resource Types

### 1. Add to EventEffect enum

`core_sim/src/components/events.rs`:

```rust
pub enum EventEffect {
    GoldChange(f32),
    ScienceChange(f32),  // New
    // ...
}
```

### 2. Handle in apply_effect

`core_sim/src/systems/events.rs`:

```rust
EventEffect::ScienceChange(amount) => {
    civilization.economy.science += amount;
}
```

### 3. Add to top panel

Follow the 5-step guide in `ui_resource_implementation.md`

### 4. Use in events

`dominion_earth/assets/data/events.ron`:

```ron
EventChoice(
    text: "Pursue knowledge",
    effects: [
        GoldChange(-20.0),
        ScienceChange(40.0),
    ],
)
```

## Key Architecture

- Event effects modify `civilization.economy.field`
- UI system queries `Civilization` component
- Change detection: only update text if value changed
- System execution order: effects → UI updates (via `.chain()`)

## Related Files

- `core_sim/src/components/events.rs` - EventEffect enum
- `core_sim/src/systems/events.rs` - apply_event_effects system
- `dominion_earth/src/ui/top_panel/resources_section.rs` - UI updates
- `dominion_earth/assets/data/events.ron` - Event definitions

# Event System

## Overview

The event system provides dynamic, turn-based events that trigger based on various conditions (random chance, specific game states, or milestone achievements). Events can affect civilizations with various effects like gold changes, happiness modifiers, unit grants, and temporary bonuses.

## Architecture

### Components (`core_sim/src/components/events.rs`)

#### `GameEvent`

The main event component attached to event entities.

- `event_id`: Unique identifier for the event type
- `affected_civ`: Which civilization is affected
- `title`: Display name of the event
- `description`: Narrative text explaining what happened
- `effects`: List of effects to apply when acknowledged
- `choices`: Player choice options (for interactive events)
- `has_been_shown`: Legacy field (currently unused)
- `turn_triggered`: When the event was triggered

#### `PendingEvent`

Marker component indicating an event needs to be shown to the player or processed by AI. Removed when:

- For player civs: UI popup is created
- For AI civs: Event is logged to console

#### `EventHistory`

Tracks which events have been triggered for each civilization to prevent duplicates.

- Stores a set of event IDs that have already fired
- Prevents "first-time" events from triggering multiple times

#### `ActiveEventModifiers`

Tracks temporary modifiers from events (e.g., production bonuses).

- Contains list of active modifiers with duration tracking
- Automatically decrements duration each turn

#### `EventTriggerType`

Enum defining when events should trigger:

- `Random { probability }`: Each turn, roll against probability (0.0-1.0)
- `ResourceThreshold`: When gold/production/etc reaches a value
- `FirstUnitCreated`: When civ creates their first unit
- `FirstCivContact`: When meeting another civilization
- `TechnologyResearched`: When a specific tech is discovered
- `CityFounded`: When a new city is founded
- `SpecificTurn`: Triggers on an exact turn number

#### `EventEffect`

Enum defining what happens when an event is acknowledged:

- `GoldChange(amount)`: Add/subtract gold
- `HappinessChange(amount)`: Modify happiness (not yet implemented)
- `GrantUnit { unit_type }`: Spawn a unit (not yet implemented)
- `GrantTechnology { tech_name }`: Unlock a technology (not yet implemented)
- `ProductionModifier { multiplier, duration_turns }`: Temporary production boost
- `TemporaryTraitModifier { trait_name, modifier, duration_turns }`: Temporary stat change

### Systems (`core_sim/src/systems/events.rs`)

#### `check_event_triggers`

**Schedule**: Update (but runs only once per turn via `Local<u32>` tracking)

**Purpose**: Evaluate all event definitions and spawn events when conditions are met.

**Flow**:

1. Check if already ran this turn (using `last_turn_checked` local state)
2. For each civilization:
   - For each event definition:
     - Skip if event already triggered (check `EventHistory`)
     - Evaluate trigger condition
     - If triggered, spawn `GameEvent` entity with `PendingEvent` marker
     - Update `EventHistory` to record the event

**Important**: Uses `Local<u32>` to ensure it only runs once per turn, not every frame.

#### `apply_event_effects`

**Schedule**: Update

**Purpose**: Apply effects to civilizations after events are acknowledged.

**Flow**:

1. Query for events `Without<PendingEvent>` (means they've been acknowledged)
2. Find the affected civilization
3. Apply each effect (gold changes, modifiers, etc.)
4. Despawn the event entity

#### `update_event_modifiers`

**Schedule**: Update

**Purpose**: Decrement duration of temporary event modifiers each turn.

**Flow**:

1. For civilizations with `ActiveEventModifiers`:
   - Decrement remaining turns for each modifier
   - Remove expired modifiers

### UI System (`dominion_earth/src/ui/event_popup.rs`)

#### Components

**`EventPopup { event_entity }`**
Marker linking a UI popup to its corresponding event entity. This ensures only the correct popup is despawned when the player responds.

**`EventAcknowledged`**
Marker added to event entities when the player clicks a choice button, signaling the event should be processed.

#### Systems

**`spawn_event_popup`**
**Schedule**: Update

**Purpose**: Create UI popups for player civilization events.

**Flow**:

1. Query for events with `PendingEvent` marker
2. Filter to only player-controlled civilizations
3. For AI civs: Log to console and remove `PendingEvent`
4. For player civs:
   - Create modal overlay with event details
   - Create choice buttons (or single "Acknowledge" button)
   - Link popup to event via `EventPopup { event_entity }`
   - **Remove `PendingEvent` immediately** to prevent duplicate popups

**`despawn_completed_event_popups`**
**Schedule**: Update

**Purpose**: Remove popups after player responds.

**Flow**:

1. Query all popups with their linked event entities
2. Check if the linked event has `EventAcknowledged`
3. If acknowledged, despawn the popup

**`acknowledge_event`**
Called when player clicks a choice button.

- Removes `PendingEvent` from event entity
- Adds `EventAcknowledged` marker
- Triggers popup despawn and effect application

## Data Definition (`dominion_earth/assets/data/events.ron`)

Events are defined in RON format:

```ron
EventDefinitionList(
    events: [
        EventDefinition(
            id: "bountiful_harvest",
            title: "Bountiful Harvest",
            description: "Your farmers have produced an exceptional crop this season!",
            trigger: Random(probability: 0.05), // 5% chance per turn
            effects: [
                GoldChange(50.0),
            ],
            choices: [], // Empty = single "Acknowledge" button
        ),
        EventDefinition(
            id: "merchant_caravan",
            title: "Merchant Caravan Arrives",
            description: "Traveling merchants offer goods and opportunities.",
            trigger: Random(probability: 0.03),
            effects: [
                GoldChange(30.0),
            ],
            choices: [
                EventChoice(
                    text: "Trade with them",
                    effects: [GoldChange(-20.0), ProductionModifier(multiplier: 1.2, duration_turns: 3)],
                ),
                EventChoice(
                    text: "Send them away",
                    effects: [],
                ),
            ],
        ),
    ],
)
```

## Event Flow

### Turn Start

1. `check_event_triggers` runs once when turn number changes
2. Evaluates all event definitions against all civilizations
3. Spawns `GameEvent` entities with `PendingEvent` marker

### Player Event Processing

1. `spawn_event_popup` detects `PendingEvent` for player civ
2. Creates modal UI popup
3. Removes `PendingEvent` immediately (prevents spam)
4. Popup stays open until player clicks

### Player Response

1. Player clicks choice button
2. `acknowledge_event` adds `EventAcknowledged` marker
3. `despawn_completed_event_popups` removes the popup
4. `apply_event_effects` processes effects and despawns event entity

### AI Event Processing

1. `spawn_event_popup` detects `PendingEvent` for AI civ
2. Logs event to console (placeholder for AI decision logic)
3. Removes `PendingEvent` immediately
4. `apply_event_effects` processes effects automatically

## Key Design Decisions

### Once-Per-Turn Checking

The `check_event_triggers` system uses `Local<u32>` to track the last turn it ran. This prevents it from spawning events every frame while still being in the Update schedule for flexibility.

```rust
mut last_turn_checked: Local<u32>,
```

### Immediate PendingEvent Removal

When a popup is created, `PendingEvent` is removed immediately. This prevents the system from creating duplicate popups in subsequent frames. The popup stays open because `despawn_completed_event_popups` only removes popups for acknowledged events.

### Popup-Event Linking

Each popup stores the entity ID of its event (`EventPopup { event_entity }`). This ensures that when a player responds to one event, only that specific popup is closed, not all popups.

### Event History

Each civilization maintains an `EventHistory` component to track which events have already triggered. This prevents "first-time" events from firing multiple times.

## Testing

### Test Events

The system includes test events for development:

```ron
EventDefinition(
    id: "test_welcome",
    title: "A New Beginning",
    description: "Your civilization begins its journey!",
    trigger: SpecificTurn(turn: 1), // Always triggers turn 1
    effects: [GoldChange(100.0)],
    choices: [],
),
```

### Manual Testing

1. Set event probability to 1.0 (100%) to guarantee triggers
2. Use `SpecificTurn` trigger for exact timing
3. Enable `--debug-logging` to see event system logs
4. Look for logs: `🎭 Checking event triggers for turn X`

## Future Enhancements

### Not Yet Implemented

- **Happiness effects**: Framework exists but no happiness system yet
- **Unit granting**: Need to spawn units at capital location
- **Technology granting**: Need to integrate with tech tree
- **Choice-specific effects**: Currently all events use first effect set
- **AI decision logic**: Currently just logs to console

### Potential Features

- **Event chains**: One event can trigger another
- **Prerequisites**: Events that require certain conditions
- **Weighted random selection**: Multiple events compete for same slot
- **Localization**: Multi-language support for text
- **Event categories**: Natural disasters, diplomatic, military, economic, etc.
- **Visual effects**: Animations or particles when events trigger
- **Sound effects**: Audio feedback for different event types

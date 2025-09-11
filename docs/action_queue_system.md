# Action Queue System Implementation

## Overview

We've successfully implemented a comprehensive action queue system for Dominion Earth where **each civilization has its own AI queue**. This system allows each AI civilization to queue up actions that will be processed systematically each turn.

## Key Components

### 1. ActionQueue Component (`core_sim/src/components/action_queue.rs`)

**Purpose**: Each civilization gets its own action queue to manage AI decisions.

**Key Features**:

- **Per-Civilization Queues**: Each civ maintains its own independent action queue
- **Priority System**: Actions are automatically sorted by priority (Defense > Attack > Units > Diplomacy > Research > Expansion > Buildings > Trade)
- **Turn-Based Processing**: Configurable actions per turn limit (default: 3 actions/turn)
- **Retry Logic**: Failed actions can be retried up to a maximum number of times
- **Delayed Execution**: Actions can be scheduled for future turns
- **Queue Management**: Automatic capacity management and overflow protection

**Configuration Constants**:

```rust
pub const DEFAULT_MAX_QUEUE_SIZE: usize = 20;           // Max actions in queue
pub const DEFAULT_ACTIONS_PER_TURN: usize = 3;         // Actions processed per turn
pub const DEFAULT_MAX_RETRIES: u8 = 2;                 // Retry attempts for failed actions
```

**Priority Bonuses**:

- Defense: +10.0 (highest priority - defending territory)
- Attack: +8.0 (military actions)
- Diplomacy: +6.0 (diplomatic relations)
- Units: +5.0 (military unit construction)
- Expansion: +4.0 (territory expansion)
- Research: +3.0 (technology development)
- Buildings: +2.0 (infrastructure)
- Trade: +1.0 (economic actions)

### 2. Queue Management Systems (`core_sim/src/systems/action_queue.rs`)

**spawn_action_queues_for_new_civilizations**:

- Automatically creates action queues for newly spawned civilizations
- Ensures every civ has its own independent queue

**process_civilization_action_queues**:

- Processes queued actions for all civilizations each turn
- Respects per-turn action limits
- Handles action execution and retry logic
- Manages failed action requeuing

**populate_action_queues_from_ai_decisions**:

- Takes AI-generated decisions and adds them to appropriate civilization queues
- Handles queue capacity checking and overflow

### 3. Integration with Game Systems

**Core Simulation Plugin** (`dominion_earth/src/plugins/core_simulation.rs`):

```rust
// Action Queue Systems (run first in the update cycle)
core_sim::spawn_action_queues_for_new_civilizations,
core_sim::process_civilization_action_queues,
```

## How It Works

### Civilization-Specific AI Processing

1. **Queue Creation**: When a civilization is spawned, it automatically gets its own `ActionQueue` component
2. **AI Decision Making**: The AI coordinator generates decisions for each civilization
3. **Queue Population**: AI decisions are added to the specific civilization's queue with appropriate priorities
4. **Turn Processing**: Each turn, every civilization's queue is processed independently:
   - Up to 3 actions per civilization per turn (configurable)
   - Actions executed in priority order
   - Failed actions retried on subsequent turns
   - Queue maintains state between turns

### Action Priority System

Actions are automatically prioritized to ensure sensible AI behavior:

**Defensive Actions First**: If a civilization is under threat, defensive actions get highest priority

**Military Actions**: Attack and unit building get high priority for aggressive civilizations

**Diplomatic Actions**: Treaties, trade agreements, and diplomatic relations are prioritized over economic development

**Economic Development**: Research, expansion, and building construction happen when no immediate threats exist

### Example Workflow

```
Turn 1: AI Coordinator generates decisions for Civilization A
├── Attack enemy position (Priority: 8.0 + base = 15.0)
├── Build defensive unit (Priority: 5.0 + base = 12.0)
├── Research new technology (Priority: 3.0 + base = 8.0)
└── Expand to new territory (Priority: 4.0 + base = 7.0)

Queue Processing Order:
1. Attack enemy position (executed)
2. Build defensive unit (executed)
3. Research new technology (executed)
4. Expand to new territory (queued for next turn - action limit reached)

Turn 2: Continue processing remaining actions + new AI decisions
```

### Key Benefits

✅ **Independent AI Behavior**: Each civilization acts autonomously with its own decision queue

✅ **Realistic Turn-Based Play**: Actions are spread across multiple turns, preventing unrealistic "everything happens at once" behavior

✅ **Smart Prioritization**: Critical actions (defense, military) take precedence over economic development

✅ **Robust Error Handling**: Failed actions are retried rather than lost

✅ **Scalable Design**: System can handle any number of civilizations, each with their own queue

✅ **Configurable Behavior**: Action limits, queue sizes, and priorities can be easily tuned

## Future Enhancements

The system is designed to be extensible:

- **Dynamic Priority Adjustment**: Priorities could be modified based on game state (e.g., higher defense priority when at war)
- **Queue Inspection**: UI could display each civilization's action queue for debugging
- **Action Dependencies**: Actions could depend on other actions completing first
- **Resource-Aware Queuing**: Actions could be automatically delayed if insufficient resources
- **Diplomatic Queue Coordination**: Civilizations could coordinate actions through diplomatic channels

## Testing

The system has been integrated into the main game loop and will be active when civilizations are spawned. Each civilization will automatically get its own action queue and begin processing AI decisions independently.

To observe the system in action:

1. Run the game with multiple AI civilizations
2. Each civ will process up to 3 actions per turn
3. Actions will be executed in priority order
4. Failed actions will be retried on subsequent turns

The queue system provides the foundation for sophisticated, realistic AI behavior where each civilization acts independently and strategically manages its actions over time.

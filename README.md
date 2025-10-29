# Dominion Earth

A turn-based, 2D grand strategy game prototype built with Rust and Bevy 0.16. Experience AI-driven gameplay with ~40 civilizations placed in real-world locations, each with distinct human-like personalities and strategic decision-making capabilities.

## 🎮 Features

- **Configurable Player Count**: Control multiple civilizations or let AI handle them all
- **Multiplayer Support**: Up to N player-controlled civilizations with AI opponents
- **Scalable Game Size**: Choose from 2 to 40+ civilizations per game
- **AI-Driven Opponents**: Civilizations with unique personalities using Utility AI + GOAP/HTN planning
- **Real Earth Map**: Civilizations placed in historically accurate starting locations
- **Interactive Gameplay**: Click to select units, right-click to move, intuitive controls
- **Data-Driven Design**: All game content defined in RON/JSON asset files
- **Performance Optimized**: Efficient real-time simulation with GUI rendering
- **Deterministic Simulation**: Reproducible gameplay with seeded random number generation
- **Modular Architecture**: Separate crates for core simulation, AI planning, and Bevy frontend

## 🏗️ Architecture

```
dominion_earth/          # Main Bevy application (GUI)
├── core_sim/            # Pure ECS simulation engine
├── ai_planner/          # AI decision-making system
└── assets/data/         # Game content (RON files)
```

### Core Components

- **core_sim**: Pure ECS simulation using `bevy_ecs` with no graphics dependencies
- **ai_planner**: Multi-layered AI system combining Utility AI, GOAP, and HTN planning
- **dominion_earth**: Bevy frontend with 2D rendering, UI, and input handling

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable) - Install from [rustup.rs](https://rustup.rs/)
- Git

### Building

```bash
# Clone the repository
git clone <repository-url>
cd dominion_earth

# Build the project (debug mode)
cargo build

# Build optimized release version
cargo build --release
```

### Running

```bash
# Run with default settings (loaded from saves/settings.ron if available)
cargo run

# Run with debug logging
cargo run -- --debug-logging
```

**Note**: Seed and AI-only mode settings are controlled from the main menu settings. Set these in the Settings menu on the main screen before starting a game.

### Command Line Options

- `--debug-logging`: Enable detailed debug output

### Game Settings

Game settings (volume, random seed, AI-only mode) are saved in `saves/settings.ron` and can be configured from the main menu:

1. Launch the game
2. Click "Settings" from the main menu
3. Adjust volume, set random seed, or enable AI-only mode
4. Click "Save Settings"
5. Click "Play" to start a game with your configured settings

**Settings Available in Main Menu Only:**

- **Random Seed**: Set a specific seed for reproducible games, or leave as "None" for random generation
- **AI-Only Mode**: When enabled, all civilizations are controlled by AI (no player control)

**Settings Available Everywhere:**

- **Master Volume**: Adjust game audio volume

### Controls

#### Player Mode (Default)

- **Left Click**: Select your units
- **Right Click**: Move selected unit to target tile
- **Space**: Skip turn for selected unit
- **Next Turn Button**: Advance to next turn (manual mode)

#### Camera Controls

- **Arrow Keys**: Move camera
- **Q/E**: Zoom in/out
- **Mouse Drag**: Pan camera
- **Mouse Wheel**: Zoom

#### Game Controls

- **P**: Pause/Resume
- **A**: Toggle auto-advance mode

### Development

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Check code formatting
cargo fmt --check

# Run clippy linting
cargo clippy -- -D warnings
```

## 🎯 Game Design

### Civilizations

Each civilization has a unique personality profile affecting their strategic decisions:

- **Land Hunger**: Expansion aggressiveness (0.0-1.0)
- **Industry Focus**: Preference for production vs. other yields
- **Tech Focus**: Research priority and innovation rate
- **Interventionism**: Willingness to engage in foreign affairs
- **Risk Tolerance**: Conservative vs. aggressive decision-making
- **Honor Treaties**: Diplomatic reliability
- **Militarism**: Military buildup and warfare preference
- **Isolationism**: Preference for isolation vs. engagement

### AI Decision Making

The AI system uses a three-layer approach:

1. **Utility AI**: Evaluates current game state and scores potential actions
2. **GOAP (Goal-Oriented Action Planning)**: Plans sequences of actions to achieve goals
3. **HTN (Hierarchical Task Networks)**: Manages high-level strategic decisions

### Turn Flow

1. **AI Planning Phase**: All civilizations plan their actions
2. **Action Resolution**: Simultaneous execution of planned actions
3. **World Update**: Economics, diplomacy, and territorial changes
4. **Victory Check**: Evaluate win/loss conditions
5. **Next Turn**: Advance to next turn or end game

## 📈 Performance Characteristics

### Memory Usage

- **Baseline**: ~50MB for 40 civilizations
- **Peak**: ~100MB during intensive AI planning
- **Per Civilization**: ~1-2MB average memory footprint

### CPU Usage

- **Turn Processing**: 10-50ms per turn (40 civs)
- **AI Planning**: 60-80% of total CPU time
- **Rendering**: 10-20% of total CPU time (GUI mode)

### Scalability

The engine supports:

- **Civilizations**: 1-100+ (performance scales roughly linearly)
- **Map Size**: 50x25 to 200x100 tiles
- **Turns**: Unlimited (with save/load support)

### Performance Targets

- [ ] 500 turns in <5 seconds (40 civs)
- [ ] 100 civilizations support
- [ ] <1GB memory usage
- [ ] 60 FPS rendering with 1000+ units

## 🙏 Acknowledgments

- **Bevy Engine**: Modern ECS game engine for Rust
- **Rust Community**: Excellent crates and documentation
- **Strategy Game Classics**: Inspiration from Civilization, Europa Universalis, and similar games

# Menu System Documentation

## Overview

This document describes the menu system added to Dominion Earth, based on the bevy_new_2d template structure. The menu system provides a professional game flow with multiple screens and menus.

## Architecture

### Screen States (`screens/mod.rs`)

The game now uses a state-based screen system with the following states:

- **Splash**: Initial splash screen shown at startup (2 seconds)
- **MainMenu**: Main menu screen where players start
- **Gameplay**: The main game screen

### Menu States (`menus/mod.rs`)

Menus are overlays that can appear on top of screens:

- **None**: No menu active (default during gameplay)
- **Main**: Main menu (shown on MainMenu screen)
- **Pause**: Pause menu (accessible during gameplay with Escape)
- **Settings**: Settings menu (accessible from Main or Pause)
- **Credits**: Credits menu (accessible from Main menu)

## Components

### Theme System (`theme/`)

#### `palette.rs`

- Color constants for consistent UI theming
- Helper functions: `px()` and `percent()` for UI values
- Button, panel, and text colors

#### `interaction.rs`

- InteractionPalette component for button hover/press effects
- Automatically updates button colors based on interaction state

#### `widget.rs`

- Reusable UI widget functions:
  - `ui_root()`: Full-screen centered container
  - `header()`: Large text headers
  - `label()`: Standard text labels
  - `button()`: Large rounded buttons
  - `button_small()`: Small square buttons
- `ButtonAction` enum for defining button behaviors
- `ButtonText` component for button labels

### Button System

Buttons use a centralized action system:

1. Create buttons with `widget::button()` or `widget::button_small()`
2. Assign a `ButtonAction` enum variant
3. The `handle_button_interactions` system processes all button clicks
4. Button text is automatically spawned as children

Available button actions:

- `EnterGameplay`: Start the game
- `OpenSettings`: Open settings menu
- `OpenCredits`: Open credits menu
- `ExitApp`: Exit application (desktop only)
- `CloseMenu`: Close current menu
- `OpenPauseMenu`: Open pause menu
- `QuitToMenu`: Return to main menu
- `GoBack`: Return to previous menu
- `LowerVolume` / `RaiseVolume`: Adjust audio volume

## Game Flow

```
Splash Screen (2s)
    ↓
Main Menu
    ├── Play → Gameplay
    ├── Settings → Settings Menu → Back to Main
    ├── Credits → Credits Menu → Back to Main
    └── Exit (desktop only)

Gameplay
    ├── ESC → Pause Menu
    │   ├── Continue → Resume gameplay
    │   ├── Settings → Settings Menu → Back to Pause
    │   └── Quit to Menu → Main Menu
    └── (game continues)
```

## Integration with Existing Systems

### Core Simulation

- Game setup moved from `Startup` to `OnEnter(Screen::Gameplay)`
- All gameplay systems now run only when `in_state(Screen::Gameplay)`

### Rendering

- Tilemap and sprite systems scoped to Gameplay screen
- Fog of war rendering scoped to Gameplay screen

### Input Handling

- Player input systems only active during Gameplay
- Menu navigation uses Escape key

### UI Integration

- bevy_hui UI panels only display during Gameplay screen
- UI setup triggers on entering Gameplay state

## Usage

### Creating New Menus

1. Create a new file in `dominion_earth/src/menus/`
2. Define a plugin function
3. Add systems for `OnEnter(Menu::YourMenu)` to spawn UI
4. Use `widget` functions to build the menu
5. Add your menu to the `Menu` enum
6. Register the plugin in `menus/mod.rs`

Example:

```rust
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::MyMenu), spawn_my_menu);
}

fn spawn_my_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("My Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::MyMenu),
    ))
    .with_children(|parent| {
        parent.spawn(widget::header("My Menu Title"));
        parent.spawn(widget::button("Action", ButtonAction::SomeAction));
    });
}
```

### Adding New Button Actions

1. Add variant to `ButtonAction` enum in `theme/widget.rs`
2. Handle the action in `handle_button_interactions` in `theme/mod.rs`

## Files Modified

### New Files

- `dominion_earth/src/theme/` (mod.rs, palette.rs, interaction.rs, widget.rs)
- `dominion_earth/src/screens/` (mod.rs, splash.rs, main_menu.rs, gameplay.rs)
- `dominion_earth/src/menus/` (mod.rs, main.rs, pause.rs, settings.rs, credits.rs)
- `dominion_earth/src/plugins/menu.rs`

### Modified Files

- `dominion_earth/src/main.rs`: Added theme, screens, and menus modules
- `dominion_earth/src/plugins/mod.rs`: Added MenuPlugin
- `dominion_earth/src/plugins/core_simulation.rs`: Scoped to Gameplay screen
- `dominion_earth/src/plugins/rendering.rs`: Scoped to Gameplay screen
- `dominion_earth/src/plugins/input_handling.rs`: Scoped to Gameplay screen
- `dominion_earth/src/plugins/camera.rs`: Camera centering on Gameplay enter
- `dominion_earth/src/plugins/ui_integration.rs`: UI scoped to Gameplay screen
- `dominion_earth/src/ui/bevy_hui/mod.rs`: Added screen-scoped setup method

## Features

- ✅ Professional splash screen
- ✅ Main menu with Play, Settings, Credits, Exit
- ✅ In-game pause menu (Escape key)
- ✅ Settings menu with master volume control
- ✅ Credits menu
- ✅ Smooth state transitions
- ✅ Consistent UI theming
- ✅ Button hover and press effects
- ✅ Keyboard navigation (Escape to go back)
- ✅ StateScoped cleanup (menus auto-despawn on state change)

## Running the Game

The game now starts with a splash screen, then shows the main menu. Click "Play" to enter the game, or press Escape during gameplay to access the pause menu.

```bash
cargo run -- --seed 1756118413
```

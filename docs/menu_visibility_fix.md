# Menu System Visibility Fix

## Problem

The splash screen text and menus were not visible, or wouldn't disappear when clicking "Play".

## Root Cause

1. Game UI panels (top, left, right panels) were spawned without `StateScoped(Screen::Gameplay)` component
2. This meant they persisted across all screen states, covering the splash and menus
3. Menu z-index was too low (2) compared to potential game elements

## Solutions Applied

### 1. Added StateScoped to UI Panels

Modified `/dominion_earth/src/ui/bevy_hui/main_ui.rs`:

- Added `StateScoped(crate::screens::Screen::Gameplay)` to `spawn_top_panel()`
- Added `StateScoped(crate::screens::Screen::Gameplay)` to `spawn_right_side_panel()`
- Added `StateScoped(crate::screens::Screen::Gameplay)` to `spawn_left_side_panel()`

This ensures UI panels only exist during gameplay and are automatically cleaned up when leaving the Gameplay screen.

### 2. Increased Menu Z-Index

Changed GlobalZIndex from 2 to 100 for all menus:

- Main menu (`menus/main.rs`)
- Pause menu (`menus/pause.rs`)
- Settings menu (`menus/settings.rs`)
- Credits menu (`menus/credits.rs`)
- Splash screen (`screens/splash.rs`)

This ensures menus always render on top of game elements.

## Expected Behavior After Fix

1. Splash screen appears with "Dominion Earth" text visible for 2 seconds
2. Transitions to main menu with visible buttons
3. Clicking "Play" enters gameplay and spawns UI panels
4. Pressing Escape opens pause menu (visible on top of game)
5. Returning to main menu cleans up all game UI panels

## Files Modified

- `dominion_earth/src/ui/bevy_hui/main_ui.rs`
- `dominion_earth/src/screens/splash.rs`
- `dominion_earth/src/menus/main.rs`
- `dominion_earth/src/menus/pause.rs`
- `dominion_earth/src/menus/settings.rs`
- `dominion_earth/src/menus/credits.rs`

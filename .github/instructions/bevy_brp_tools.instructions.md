---
applyTo: "**"
---

# Bevy BRP (Bevy Remote Protocol) Tools Instructions

This project is configured with `bevy_brp_mcp` and `bevy_brp_extras` to enable powerful debugging, inspection, and manipulation capabilities through the Bevy Remote Protocol. The MCP server provides AI coding assistants with direct access to running Bevy applications.

## Setup Status

- ✅ `bevy_brp_mcp` installed locally in `./bin/bevy_brp_mcp`
- ✅ MCP server configured in `.vscode/mcp.json`
- ✅ `bevy_brp_extras` dependency added to workspace
- ✅ Ready for use with VS Code Copilot Chat in agent mode

## Required App Configuration

To enable full BRP functionality in the Dominion Earth game, ensure your `main.rs` includes:

```rust
use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dominion Earth".to_string(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BrpExtrasPlugin) // Enables BRP with enhanced features
        // ... other plugins
        .run();
}
```

**Required Cargo.toml features:**

```toml
bevy = { version = "0.16", features = ["default", "bevy_remote", "png"] }
bevy_brp_extras = { workspace = true }
```

## Available Tools & Commands

### 1. Core BRP Operations

#### Entity Management

- `bevy_spawn` - Create new entities with components
- `bevy_destroy` - Remove entities from the world
- `bevy_query` - Search for entities with specific components
- `bevy_get_component` - Get component data from specific entities
- `bevy_insert_component` - Add components to existing entities
- `bevy_remove_component` - Remove components from entities
- `bevy_mutate_component` - Modify component data

#### Resource Management

- `bevy_get_resource` - Access global resources
- `bevy_insert_resource` - Add global resources
- `bevy_mutate_resource` - Modify global resources

#### Component Discovery

- `bevy_list` - List all registered components and resources
- `bevy_registry_schema` - Get component type schemas

### 2. Application Discovery & Management

#### App Discovery

- `list_bevy_apps` - Find all Bevy applications in workspace
- `get_app_build_status` - Check which apps are built and ready
- `launch_bevy_app` - Start applications with proper logging setup

#### Process Management

- `check_app_running` - Verify if apps are running with BRP enabled
- `list_running_apps` - Show all currently running Bevy processes

### 3. Real-time Monitoring

#### Component Watching

- `brp_get_watch` - Monitor component changes on specific entities
- `brp_stop_watch` - Stop monitoring specific entities
- `brp_list_watches` - Show all active watches

#### Log Management

- `read_log` - Read application log files
- `list_logs` - Show available log files
- `clean_logs` - Remove old log files

### 4. Enhanced BRP Features (via bevy_brp_extras)

#### Screenshot Capture

- `brp_extras_screenshot` - Take screenshots of the running application
  - Parameters: `path` (required) - where to save the image
  - Requires `png` feature in Bevy

#### Format Discovery

- `brp_extras_discover_format` - Get correct JSON formats for BRP operations
  - Parameters: `types` (array of fully-qualified type paths)
  - Returns exact format needed for spawn/insert/mutate operations
  - Essential for avoiding trial-and-error with complex component structures

#### Keyboard Input Simulation

- `brp_extras_send_keys` - Send keyboard input to the application
  - Parameters: `keys` (array of key codes), `duration_ms` (optional)
  - Useful for automated testing and interaction

#### Graceful Shutdown

- `brp_extras_shutdown` - Cleanly shut down the application
  - No parameters required

### 5. Key Code Discovery

- `brp_extras_list_key_codes` - Get all available keyboard key codes for input operations

## Common Workflows

### 1. Debug Entity Issues

```
1. Use `launch_bevy_app` to start the game with logging
2. Use `bevy_query` to find problematic entities
3. Use `bevy_get_component` to inspect component data
4. Use `brp_get_watch` to monitor entity changes in real-time
5. Use `read_log` to examine application logs
```

### 2. Test Game Mechanics

```
1. Launch the game with `launch_bevy_app`
2. Use `brp_extras_send_keys` to simulate player input
3. Use `bevy_query` to verify game state changes
4. Use `brp_extras_screenshot` to document results
```

### 3. Modify Game State

```
1. Use `brp_extras_discover_format` to get correct component formats
2. Use `bevy_mutate_component` to modify entity data
3. Use `bevy_insert_component` to add new behaviors
4. Use `brp_get_watch` to observe the effects
```

### 4. Performance Analysis

```
1. Use `bevy_list` to see all registered components
2. Use `bevy_query` with filters to count entities
3. Use `read_log` to check for performance warnings
4. Use multiple `brp_get_watch` to monitor key entities
```

## Log Files Location

All BRP operations create detailed logs in `/tmp/` with predictable naming:

- `bevy_brp_mcp_dominion_earth_<timestamp>.log` - Application logs
- `bevy_brp_mcp_watch_<entity_id>_<component>_<timestamp>.log` - Monitoring logs

Use `list_logs` and `read_log` to access these files for debugging.

## Best Practices

### Format Discovery First

Always use `brp_extras_discover_format` before attempting complex component operations. It provides the exact JSON structure needed and prevents trial-and-error.

### Use Fully-Qualified Type Names

When working with component types, use full paths like:

- `bevy_transform::components::transform::Transform`
- `core_sim::components::Civilization`
- NOT just `Transform` or `Civilization`

### Monitor Before Modifying

Use `brp_get_watch` to observe entity state before making changes, so you can see the effects of your modifications.

### Clean Up Logs

Regularly use `clean_logs` to remove old log files and keep the tmp directory manageable.

### Take Screenshots for Documentation

Use `brp_extras_screenshot` to capture visual evidence of bugs, features, or game state for documentation.

## Integration with Dominion Earth

### Key Components to Monitor

- `core_sim::Civilization` - Civilization state and AI behavior
- `bevy_transform::components::transform::Transform` - Entity positions
- `core_sim::resources::CurrentTurn` - Turn progression
- `core_sim::resources::GameConfig` - Game configuration
- `core_sim::resources::WorldMap` - World state

### Common Debug Scenarios

1. **AI Not Acting**: Watch civilization entities and check AI coordinator logs
2. **Turn Not Advancing**: Monitor `CurrentTurn` resource and turn management systems
3. **UI Issues**: Check entity queries and component data for UI elements
4. **Performance Problems**: Monitor entity counts and component changes

## Port Configuration

- Default BRP port: 15702
- Can be overridden with `BRP_PORT` environment variable
- MCP server connects automatically to the running application

## Error Handling

The BRP tools provide detailed error messages. Common issues:

- App not running: Use `launch_bevy_app` first
- Wrong component format: Use `brp_extras_discover_format`
- Entity not found: Use `bevy_query` to find correct entity IDs
- Component not registered: Check with `bevy_list`

Remember: These tools provide live access to the running game. Changes made through BRP immediately affect the application state.

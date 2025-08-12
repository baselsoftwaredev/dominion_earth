# Bevy BRP MCP Tools Quick Reference

## Launch Commands

```bash
# Start game with BRP enabled
cargo run -- --enable-remote

# Start with custom port
cargo run -- --enable-remote --remote-port 8080

# Start in auto-advance mode with BRP
cargo run -- --enable-remote --auto-advance
```

## Core Entity Operations

- `bevy_spawn` - Create entities with components
- `bevy_destroy` - Remove entities
- `bevy_query` - Find entities by component filters
- `bevy_get_component` - Get component data
- `bevy_insert_component` - Add components to entities
- `bevy_remove_component` - Remove components
- `bevy_mutate_component` - Modify component data

## Resource Operations

- `bevy_get_resource` - Access global resources
- `bevy_insert_resource` - Add global resources
- `bevy_mutate_resource` - Modify global resources

## Discovery & Schema

- `bevy_list` - List all registered components/resources
- `bevy_registry_schema` - Get type schemas
- `brp_extras_discover_format` - Get exact JSON formats for operations

## App Management

- `list_bevy_apps` - Find Bevy apps in workspace
- `get_app_build_status` - Check build status
- `launch_bevy_app` - Start apps with logging
- `check_app_running` - Verify BRP connection
- `list_running_apps` - Show running processes

## Real-time Monitoring

- `brp_get_watch` - Monitor entity component changes
- `brp_stop_watch` - Stop watching entities
- `brp_list_watches` - Show active watches

## Enhanced Features (bevy_brp_extras)

- `brp_extras_screenshot` - Take screenshots (requires `png` feature)
- `brp_extras_send_keys` - Send keyboard input
- `brp_extras_shutdown` - Graceful app shutdown
- `brp_extras_list_key_codes` - Available key codes

## Logging

- `read_log` - Read application logs
- `list_logs` - Show available log files
- `clean_logs` - Remove old logs

## Key Dominion Earth Components

- `core_sim::Civilization` - Civ state & AI
- `bevy_transform::components::transform::Transform` - Positions
- `core_sim::resources::CurrentTurn` - Turn counter
- `core_sim::resources::GameConfig` - Game settings
- `core_sim::resources::WorldMap` - World state

## Common Workflows

### Debug Entity Issue

1. `launch_bevy_app dominion_earth --enable-remote`
2. `bevy_query` to find problematic entities
3. `brp_get_watch` to monitor changes
4. `read_log` to check application logs

### Test Game Mechanics

1. Launch with BRP enabled
2. `brp_extras_send_keys` to simulate input
3. `bevy_query` to verify state changes
4. `brp_extras_screenshot` to document

### Modify Game State

1. `brp_extras_discover_format` for correct JSON
2. `bevy_mutate_component` to change data
3. `brp_get_watch` to observe effects

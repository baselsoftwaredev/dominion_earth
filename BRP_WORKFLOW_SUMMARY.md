# BRP Two-Terminal Workflow Summary

## The Critical Rule: Game First, Then Curl

**⚠️ CRITICAL FOR AI DEVELOPMENT: The user MUST run their application first before any AI agent can debug or add features. AI agents cannot perform BRP operations on applications that are not running.**

The Bevy Remote Protocol (BRP) requires a specific two-terminal workflow:

## Terminal 1: Start the Game

```bash
cd /Users/basel/development/dominion_earth
cargo run -- --enable-remote
```

**Wait for this confirmation message:**

```
BRP extras enabled on http://localhost:15702
```

## Terminal 2: Make BRP Calls

Only AFTER seeing the confirmation message above, you can make curl requests:

```bash
# Test basic connection (should return a number)
curl -X POST http://localhost:15702/bevy/list \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "id": 1, "method": "bevy/list"}' | jq '.result | length'

# Query for all entities with Transform components
curl -X POST http://localhost:15702/bevy/query \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "id": 1, "method": "bevy/query", "params": {"data": {"components": ["bevy_transform::components::transform::Transform"]}}}' \
  | jq '.result | length'

# Get current game turn
curl -X POST http://localhost:15702/bevy/get \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "id": 1, "method": "bevy/get", "params": {"data": {"resource": "core_sim::resources::CurrentTurn"}}}' \
  | jq '.result'

# Take a screenshot for debugging
curl -X POST http://localhost:15702/brp_extras/screenshot \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "id": 1, "method": "brp_extras/screenshot", "params": {"path": "/tmp/dominion_earth_debug.png"}}'

# Discover component format (essential for complex operations)
curl -X POST http://localhost:15702/brp_extras/discover_format \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "id": 1, "method": "brp_extras/discover_format", "params": {"types": ["core_sim::components::Civilization"]}}' \
  | jq '.result'
```

## Why This Matters

- **Connection Refused Error:** Means the game isn't running yet
- **Timeout/Hanging:** Game is starting up, wait for the BRP enabled message
- **JSON Parse Errors:** Game is running but BRP command syntax is wrong

**🚨 FOR AI AGENTS: You CANNOT debug issues, inspect game state, or add features unless the user has started their application with `--enable-remote` first. Always instruct the user to run their application before attempting any BRP operations.**

## For VS Code MCP Tools

The same rule applies to the MCP tools in VS Code:

1. Start the game first: `cargo run -- --enable-remote`
2. Wait for "BRP extras enabled" message
3. Then the MCP tools (bevy_spawn, bevy_query, etc.) will work in Copilot Chat

## Essential BRP Commands for Development

Once the game is running, these are the most useful commands:

1. **List all available components:** `curl ... /bevy/list`
2. **Find entities:** `curl ... /bevy/query`
3. **Read component data:** `curl ... /bevy/get`
4. **Modify component data:** `curl ... /bevy/insert` or `/bevy/mutate`
5. **Take screenshots:** `curl ... /brp_extras/screenshot`
6. **Get correct JSON format:** `curl ... /brp_extras/discover_format`

This workflow is now documented in:

- `.github/instructions/bevy_brp_tools.instructions.md`
- `brp_workflow_guide.sh` (executable guide)
- `demo_brp_workflow.sh` (simple demo)

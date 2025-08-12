#!/bin/bash

# Demo script showing the proper BRP workflow
echo "=== BRP Workflow Demo ==="
echo "This demonstrates the correct way to interact with Bevy Remote Protocol"
echo ""

echo "Step 1: Start the game with BRP enabled"
echo "Command: cargo run -- --enable-remote"
echo "This will start the game and enable BRP on port 15702"
echo ""

echo "Step 2: In a separate terminal, make BRP calls"
echo "Command examples:"
echo ""

echo "# List all registered components and resources:"
echo "curl -X POST http://localhost:15702/bevy/list -H 'Content-Type: application/json' -d '{}' | jq '.result | keys | length'"
echo ""

echo "# Query for entities with Transform component:"
echo "curl -X POST http://localhost:15702/bevy/query -H 'Content-Type: application/json' -d '{\"data\": {\"components\": [\"bevy_transform::components::transform::Transform\"]}}' | jq '.result | length'"
echo ""

echo "# Get current turn resource:"
echo "curl -X POST http://localhost:15702/bevy/get -H 'Content-Type: application/json' -d '{\"data\": {\"resource\": \"core_sim::resources::CurrentTurn\"}}' | jq '.result'"
echo ""

echo "# Take a screenshot:"
echo "curl -X POST http://localhost:15702/brp_extras/screenshot -H 'Content-Type: application/json' -d '{\"path\": \"/tmp/dominion_earth_screenshot.png\"}'"
echo ""

echo "# Format discovery for components:"
echo "curl -X POST http://localhost:15702/brp_extras/discover_format -H 'Content-Type: application/json' -d '{\"types\": [\"bevy_transform::components::transform::Transform\"]}' | jq '.result'"
echo ""

echo "=== IMPORTANT ==="
echo "The game MUST be running with --enable-remote BEFORE making curl requests!"
echo "If you get 'Connection refused', the game is not running or BRP is disabled."

#!/bin/bash

# BRP Two-Terminal Workflow Example
# This script demonstrates the proper way to work with Bevy Remote Protocol

echo "🎮 Bevy Remote Protocol (BRP) Two-Terminal Workflow Demo"
echo "========================================================"
echo
echo "🚨 CRITICAL FOR AI DEVELOPMENT:"
echo "   The USER must run their application FIRST before AI can debug or add features!"
echo "   AI agents CANNOT access BRP tools without the application running!"
echo
echo "📋 SETUP REQUIRED:"
echo "1. Open TWO terminal windows/tabs"
echo "2. Navigate both to the dominion_earth project directory"
echo "3. Follow the steps below in order"
echo

echo "🖥️  TERMINAL 1 - Start the Game:"
echo "-------------------------------"
echo "Run this command in Terminal 1:"
echo
echo "    cargo run -- --enable-remote"
echo
echo "⚠️  WAIT for this message before proceeding:"
echo "    'BRP extras enabled on http://localhost:15702'"
echo

echo "🌐 TERMINAL 2 - Make BRP Calls:"
echo "------------------------------"
echo "Once the game is running, use Terminal 2 for these commands:"
echo

echo "# Test basic connection:"
echo "curl -X POST http://localhost:15702/bevy/list \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{}' | jq '.result | keys | length'"
echo

echo "# Query entities with Transform:"
echo "curl -X POST http://localhost:15702/bevy/query \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{\"data\": {\"components\": [\"bevy_transform::components::transform::Transform\"]}}' \\"
echo "  | jq '.result | length'"
echo

echo "# Get current turn:"
echo "curl -X POST http://localhost:15702/bevy/get \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{\"data\": {\"resource\": \"core_sim::resources::CurrentTurn\"}}' \\"
echo "  | jq '.result'"
echo

echo "# Take a screenshot:"
echo "curl -X POST http://localhost:15702/brp_extras/screenshot \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{\"path\": \"/tmp/dominion_earth_debug.png\"}'"
echo

echo "# Discover component format:"
echo "curl -X POST http://localhost:15702/brp_extras/discover_format \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{\"types\": [\"core_sim::components::Civilization\"]}' \\"
echo "  | jq '.result'"
echo

echo "🔧 VS CODE MCP TOOLS:"
echo "---------------------"
echo "Once the game is running, the MCP tools in VS Code will automatically work:"
echo "- bevy_spawn, bevy_query, bevy_get_component, etc."
echo "- brp_extras_screenshot, brp_extras_send_keys, etc."
echo

echo "⚠️  TROUBLESHOOTING:"
echo "-------------------"
echo "• 'Connection refused' → Game not running, start Terminal 1 first"
echo "• 'Timeout' → Game still starting, wait for BRP enabled message"
echo "• 'JSON parse error' → Check command syntax and quotes"
echo
echo "🤖 FOR AI AGENTS:"
echo "----------------"
echo "• CANNOT debug without user running application first"
echo "• CANNOT add features without BRP connection active"
echo "• MUST instruct user to start game before attempting BRP operations"
echo

echo "🛑 TO STOP:"
echo "----------"
echo "Press Ctrl+C in Terminal 1 to stop the game"
echo "All BRP calls in Terminal 2 will then fail (expected)"
echo

echo "✅ Remember: Game MUST be running before BRP tools work!"

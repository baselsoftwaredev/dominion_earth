#!/bin/bash
# Install bevy_brp_mcp locally to this project
cargo install --root . bevy_brp_mcp
echo "bevy_brp_mcp installed to ./bin/bevy_brp_mcp"
echo "MCP server configured in .vscode/mcp.json"

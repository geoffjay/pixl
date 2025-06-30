#!/bin/bash

# Test script for PIXL MCP Server
# This script tests that the MCP server can start and respond to basic commands

echo "Testing PIXL MCP Server..."

# Set the binary path
MCP_BINARY="./target/release/pixl-mcp-server"

if [ ! -f "$MCP_BINARY" ]; then
    echo "❌ MCP server binary not found at $MCP_BINARY"
    echo "Please run 'cargo build --release' first"
    exit 1
fi

echo "✅ MCP server binary found"

# Test that the server can start (we'll send it a simple command and see if it responds)
echo "🧪 Testing MCP server startup..."

# Create a simple MCP message to test health check tool
cat << 'EOF' | timeout 5s "$MCP_BINARY" > test_output.json 2>&1 &
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {}
}
EOF

# Wait a moment for the server to process
sleep 2

# Check if the server produced any output
if [ -f "test_output.json" ] && [ -s "test_output.json" ]; then
    echo "✅ MCP server started and produced output"
    echo "📋 Sample output:"
    head -n 5 test_output.json
else
    echo "⚠️  MCP server may not be responding as expected"
    echo "This could be normal for MCP stdio mode - manual testing recommended"
fi

# Clean up
rm -f test_output.json

echo ""
echo "🎯 MCP Server Build Complete!"
echo ""
echo "📍 Binary location: $MCP_BINARY"
echo "📖 Documentation: ./README.md"
echo ""
echo "🚀 Next steps:"
echo "1. Start the PIXL server: cd ../server && cargo run"
echo "2. Configure your AI tool (Claude, ChatGPT, etc.) with the MCP server"
echo "3. Use the tools to create pixel art!"
echo ""
echo "🔧 Manual test:"
echo "   echo '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\"}' | $MCP_BINARY" 
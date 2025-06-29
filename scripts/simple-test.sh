#!/bin/bash

# Simple test script to test one drawing operation

SERVER_URL="http://localhost:3000"
BOOK_NAME="test-artwork.pxl"

echo "🧪 Simple Test: Drawing One Pixel"
echo "================================="

# Check if server is running
if ! curl -s "$SERVER_URL" > /dev/null; then
    echo "❌ Error: PIXL server is not running at $SERVER_URL"
    exit 1
fi

echo "✅ Server is running"

# Send one drawing operation
echo "🖌️  Drawing a red pixel at (5,5) on frame 0..."
curl -v -X PUT "$SERVER_URL/books/$BOOK_NAME" \
    -H "Content-Type: application/json" \
    -d '{
        "operations": [
            {
                "type": "draw_pixel",
                "frame": 0,
                "x": 5,
                "y": 5,
                "color": [255, 0, 0, 255]
            }
        ]
    }'

echo ""
echo "✅ Test completed!" 
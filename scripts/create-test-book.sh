#!/bin/bash

# PIXL Test Book Creator
# This script creates a sample pixel book with various drawing operations
# to test the viewer application

set -e

SERVER_URL="http://localhost:3000"
BOOK_NAME="test-artwork.pxl"

echo "🎨 PIXL Test Book Creator"
echo "========================="

# Function to check if server is running
check_server() {
    echo "📡 Checking if PIXL server is running..."
    if curl -s "$SERVER_URL/" > /dev/null 2>&1; then
        echo "✅ Server is running at $SERVER_URL"
    else
        echo "❌ Server is not running. Please start the server first:"
        echo "   cd server && cargo run"
        exit 1
    fi
}

# Function to create a new pixel book
create_book() {
    echo "📖 Creating new pixel book: $BOOK_NAME"
    curl -s -X POST "$SERVER_URL/books" \
        -H "Content-Type: application/json" \
        -d '{
            "filename": "'$BOOK_NAME'",
            "width": 32,
            "height": 32,
            "frames": 3
        }' | jq '.'
    
    if [ $? -eq 0 ]; then
        echo "✅ Pixel book created successfully"
    else
        echo "❌ Failed to create pixel book"
        exit 1
    fi
}

# Function to add drawing operations
add_drawings() {
    echo "🎨 Adding artwork to the pixel book..."
    
    # Frame 0: Geometric shapes and patterns
    echo "🖼️  Frame 1: Geometric Shapes"
    curl -s -X PUT "$SERVER_URL/books/$BOOK_NAME" \
        -H "Content-Type: application/json" \
        -d '{
            "operations": [
                {
                    "type": "draw_shape",
                    "frame": 0,
                    "shape": "rectangle",
                    "position": {"x": 2, "y": 2},
                    "size": {"width": 12, "height": 8},
                    "filled": false,
                    "color": [255, 0, 0, 255]
                },
                {
                    "type": "draw_shape",
                    "frame": 0,
                    "shape": "rectangle",
                    "position": {"x": 4, "y": 4},
                    "size": {"width": 8, "height": 4},
                    "filled": true,
                    "color": [0, 255, 0, 128]
                },
                {
                    "type": "draw_shape",
                    "frame": 0,
                    "shape": "circle",
                    "position": {"x": 18, "y": 2},
                    "size": {"width": 12, "height": 12},
                    "filled": false,
                    "color": [0, 0, 255, 255]
                },
                {
                    "type": "draw_shape",
                    "frame": 0,
                    "shape": "circle",
                    "position": {"x": 20, "y": 4},
                    "size": {"width": 8, "height": 8},
                    "filled": true,
                    "color": [255, 255, 0, 200]
                },
                {
                    "type": "draw_shape",
                    "frame": 0,
                    "shape": "triangle",
                    "position": {"x": 8, "y": 16},
                    "size": {"width": 16, "height": 12},
                    "filled": true,
                    "color": [255, 0, 255, 180]
                },
                {
                    "type": "draw_line",
                    "frame": 0,
                    "start": {"x": 0, "y": 0},
                    "end": {"x": 31, "y": 31},
                    "line_type": "straight",
                    "color": [128, 128, 128, 255]
                },
                {
                    "type": "draw_line",
                    "frame": 0,
                    "start": {"x": 31, "y": 0},
                    "end": {"x": 0, "y": 31},
                    "line_type": "straight",
                    "color": [128, 128, 128, 255]
                }
            ]
        }' > /dev/null
    
    # Frame 1: Pixel art pattern
    echo "🖼️  Frame 2: Pixel Art Pattern"
    curl -s -X PUT "$SERVER_URL/books/$BOOK_NAME" \
        -H "Content-Type: application/json" \
        -d '{
            "operations": [
                {
                    "type": "fill_area",
                    "frame": 1,
                    "x": 0,
                    "y": 0,
                    "color": [32, 32, 64, 255]
                },
                {
                    "type": "draw_pixel",
                    "frame": 1,
                    "x": 8,
                    "y": 8,
                    "color": [255, 255, 255, 255]
                },
                {
                    "type": "draw_pixel",
                    "frame": 1,
                    "x": 9,
                    "y": 8,
                    "color": [255, 255, 255, 255]
                },
                {
                    "type": "draw_pixel",
                    "frame": 1,
                    "x": 10,
                    "y": 8,
                    "color": [255, 255, 255, 255]
                },
                {
                    "type": "draw_pixel",
                    "frame": 1,
                    "x": 8,
                    "y": 9,
                    "color": [255, 255, 255, 255]
                },
                {
                    "type": "draw_pixel",
                    "frame": 1,
                    "x": 10,
                    "y": 9,
                    "color": [255, 255, 255, 255]
                },
                {
                    "type": "draw_pixel",
                    "frame": 1,
                    "x": 8,
                    "y": 10,
                    "color": [255, 255, 255, 255]
                },
                {
                    "type": "draw_pixel",
                    "frame": 1,
                    "x": 9,
                    "y": 10,
                    "color": [255, 255, 255, 255]
                },
                {
                    "type": "draw_pixel",
                    "frame": 1,
                    "x": 10,
                    "y": 10,
                    "color": [255, 255, 255, 255]
                },
                {
                    "type": "draw_shape",
                    "frame": 1,
                    "shape": "oval",
                    "position": {"x": 14, "y": 6},
                    "size": {"width": 10, "height": 14},
                    "filled": true,
                    "color": [255, 192, 64, 255]
                },
                {
                    "type": "draw_shape",
                    "frame": 1,
                    "shape": "circle",
                    "position": {"x": 16, "y": 8},
                    "size": {"width": 6, "height": 6},
                    "filled": true,
                    "color": [64, 192, 255, 255]
                }
            ]
        }' > /dev/null
    
    # Frame 2: Complex polygon and patterns
    echo "🖼️  Frame 3: Polygons and Patterns"
    curl -s -X PUT "$SERVER_URL/books/$BOOK_NAME" \
        -H "Content-Type: application/json" \
        -d '{
            "operations": [
                {
                    "type": "fill_area",
                    "frame": 2,
                    "x": 0,
                    "y": 0,
                    "color": [20, 20, 20, 255]
                },
                {
                    "type": "draw_polygon",
                    "frame": 2,
                    "points": [
                        {"x": 16, "y": 4},
                        {"x": 24, "y": 8},
                        {"x": 28, "y": 16},
                        {"x": 24, "y": 24},
                        {"x": 16, "y": 28},
                        {"x": 8, "y": 24},
                        {"x": 4, "y": 16},
                        {"x": 8, "y": 8}
                    ],
                    "filled": true,
                    "color": [200, 100, 255, 200]
                },
                {
                    "type": "draw_polygon",
                    "frame": 2,
                    "points": [
                        {"x": 16, "y": 8},
                        {"x": 20, "y": 12},
                        {"x": 20, "y": 20},
                        {"x": 16, "y": 24},
                        {"x": 12, "y": 20},
                        {"x": 12, "y": 12}
                    ],
                    "filled": true,
                    "color": [255, 255, 100, 255]
                },
                {
                    "type": "draw_shape",
                    "frame": 2,
                    "shape": "circle",
                    "position": {"x": 14, "y": 14},
                    "size": {"width": 4, "height": 4},
                    "filled": true,
                    "color": [255, 50, 50, 255]
                }
            ]
        }' > /dev/null
    
    echo "✅ Artwork added successfully"
}

# Function to verify the created book
verify_book() {
    echo "🔍 Verifying created pixel book..."
    echo "📋 Book information:"
    curl -s "$SERVER_URL/books/$BOOK_NAME" | jq '{filename, width, height, frames: (.frames | length)}'
    
    echo ""
    echo "📚 Available books:"
    curl -s "$SERVER_URL/books" | jq '.books[] | {filename, frames}'
}

# Function to display usage instructions
show_usage() {
    echo ""
    echo "🎮 How to test the viewer:"
    echo "========================="
    echo "1. Start the viewer application:"
    echo "   cd viewer && cargo run"
    echo ""
    echo "2. In the viewer window:"
    echo "   - Press Ctrl+O to open a file"
    echo "   - Use arrow keys to navigate between frames"
    echo "   - Press Escape to exit"
    echo ""
    echo "📁 Created book: $BOOK_NAME"
    echo "   - 32x32 pixels"
    echo "   - 3 frames with different artwork"
    echo "   - Various shapes, colors, and transparency effects"
    echo ""
    echo "🔗 API endpoints to test manually:"
    echo "   GET  $SERVER_URL/books"
    echo "   GET  $SERVER_URL/books/$BOOK_NAME"
    echo "   GET  $SERVER_URL/books/$BOOK_NAME/events (SSE stream)"
}

# Main execution
main() {
    check_server
    echo ""
    create_book
    echo ""
    add_drawings
    echo ""
    verify_book
    show_usage
    echo ""
    echo "🎉 Test book creation complete!"
}

# Check if jq is available
if ! command -v jq &> /dev/null; then
    echo "⚠️  jq is not installed. JSON output will be raw."
    echo "   Install with: brew install jq (macOS) or apt-get install jq (Ubuntu)"
    echo ""
fi

# Run main function
main 
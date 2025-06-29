#!/bin/bash

# Test script for real-time pixel book updates
# This script performs various drawing operations with delays to test real-time viewer updates
# Uses randomized positions and colors for different results on each run

SERVER_URL="http://localhost:3000"
BOOK_NAME="test-artwork.pxl"
DELAY=0.1  # 100ms delay between operations

# Generate random values
random_color() {
    echo $((RANDOM % 256))
}

random_pos() {
    local max=$1
    echo $((RANDOM % max))
}

random_size() {
    local min=$1
    local max=$2
    echo $(((RANDOM % (max - min + 1)) + min))
}

echo "🎨 PIXL Real-time Update Test Script"
echo "===================================="
echo ""
echo "💡 For best testing, start the server with debug logging:"
echo "   cd server && RUST_LOG=debug cargo run"
echo ""

# Check if server is running
if ! curl -s "$SERVER_URL" > /dev/null; then
    echo "❌ Error: PIXL server is not running at $SERVER_URL"
    echo "Please start the server first:"
    echo "   cd server && RUST_LOG=debug cargo run"
    exit 1
fi

# Check if test book exists
echo "📚 Checking for test book..."
if ! curl -s "$SERVER_URL/books/$BOOK_NAME" > /dev/null; then
    echo "❌ Error: Test book '$BOOK_NAME' not found"
    echo "Please create it first: ./scripts/create-test-book.sh"
    exit 1
fi

echo "✅ Found test book: $BOOK_NAME"
echo "🎬 Starting real-time update test..."
echo "   Open the viewer and press Ctrl+O to load the book"
echo "   You should see changes appear in real-time!"
echo ""

# Function to send drawing operation
send_operation() {
    local operation="$1"
    echo "🖌️  $2"
    curl -s -X PUT "$SERVER_URL/books/$BOOK_NAME" \
        -H "Content-Type: application/json" \
        -d "$operation" > /dev/null
    
    if [ $? -eq 0 ]; then
        echo "   ✅ Operation sent"
    else
        echo "   ❌ Failed to send operation"
    fi
    
    sleep $DELAY
}

# Test 1: Draw individual pixels (frame 0)
echo "🔹 Test 1: Drawing random pixels on frame 0..."
for i in {1..3}; do
    x=$(random_pos 32)
    y=$(random_pos 32)
    r=$(random_color)
    g=$(random_color)
    b=$(random_color)
    send_operation '{
        "operations": [
            {
                "DrawPixel": {
                    "frame": 0,
                    "x": '$x',
                    "y": '$y',
                    "color": ['$r', '$g', '$b', 255]
                }
            }
        ]
    }' "Random pixel at ($x,$y) with color ($r,$g,$b)"
done

# Test 2: Draw a line (frame 0)
echo ""
echo "🔹 Test 2: Drawing a random line..."
x1=$(random_pos 25)
y1=$(random_pos 25)
x2=$((x1 + $(random_size 3 8)))
y2=$((y1 + $(random_size 3 8)))
r=$(random_color)
g=$(random_color)
b=$(random_color)
send_operation '{
    "operations": [
        {
            "DrawLine": {
                "frame": 0,
                "start": {"x": '$x1', "y": '$y1'},
                "end": {"x": '$x2', "y": '$y2'},
                "line_type": "Straight",
                "color": ['$r', '$g', '$b', 255]
            }
        }
    ]
}' "Random line from ($x1,$y1) to ($x2,$y2)"

# Test 3: Draw shapes (frame 0)
echo ""
echo "🔹 Test 3: Drawing random shapes..."
# Random rectangle
rect_x=$(random_pos 25)
rect_y=$(random_pos 25)
rect_w=$(random_size 3 6)
rect_h=$(random_size 3 6)
rect_r=$(random_color)
rect_g=$(random_color)
rect_b=$(random_color)
send_operation '{
    "operations": [
        {
            "DrawShape": {
                "frame": 0,
                "shape": "Rectangle",
                "position": {"x": '$rect_x', "y": '$rect_y'},
                "size": {"width": '$rect_w', "height": '$rect_h'},
                "filled": false,
                "color": ['$rect_r', '$rect_g', '$rect_b', 255]
            }
        }
    ]
}' "Random rectangle at ($rect_x,$rect_y)"

sleep 0.2

# Random circle
circ_x=$(random_pos 25)
circ_y=$(random_pos 25)
circ_size=$(random_size 3 7)
circ_r=$(random_color)
circ_g=$(random_color)
circ_b=$(random_color)
send_operation '{
    "operations": [
        {
            "DrawShape": {
                "frame": 0,
                "shape": "Circle",
                "position": {"x": '$circ_x', "y": '$circ_y'},
                "size": {"width": '$circ_size', "height": '$circ_size'},
                "filled": true,
                "color": ['$circ_r', '$circ_g', '$circ_b', 255]
            }
        }
    ]
}' "Random filled circle at ($circ_x,$circ_y)"

# Test 4: Update frame 1
echo ""
echo "🔹 Test 4: Updating frame 1..."
tri_x=$(random_pos 20)
tri_y=$(random_pos 20)
tri_size=$(random_size 4 8)
tri_r=$(random_color)
tri_g=$(random_color)
tri_b=$(random_color)
send_operation '{
    "operations": [
        {
            "DrawShape": {
                "frame": 1,
                "shape": "Triangle",
                "position": {"x": '$tri_x', "y": '$tri_y'},
                "size": {"width": '$tri_size', "height": '$tri_size'},
                "filled": true,
                "color": ['$tri_r', '$tri_g', '$tri_b', 255]
            }
        }
    ]
}' "Random triangle on frame 1 at ($tri_x,$tri_y)"

# Test 5: Multiple operations at once
echo ""
echo "🔹 Test 5: Multiple operations in one request..."
multi_x=$(random_pos 25)
multi_y=$(random_pos 25)
multi_r=$(random_color)
multi_g=$(random_color)
multi_b=$(random_color)
send_operation '{
    "operations": [
        {
            "DrawPixel": {
                "frame": 1,
                "x": '$multi_x',
                "y": '$multi_y',
                "color": ['$multi_r', '$multi_g', '$multi_b', 255]
            }
        },
        {
            "DrawPixel": {
                "frame": 1,
                "x": '$((multi_x + 1))',
                "y": '$multi_y',
                "color": ['$multi_r', '$multi_g', '$multi_b', 255]
            }
        },
        {
            "DrawPixel": {
                "frame": 1,
                "x": '$((multi_x + 2))',
                "y": '$multi_y',
                "color": ['$multi_r', '$multi_g', '$multi_b', 255]
            }
        }
    ]
}' "Three random colored pixels in a row"

# Test 6: Fill area (frame 2)
echo ""
echo "🔹 Test 6: Fill area on frame 2..."
send_operation '{
    "operations": [
        {
            "DrawPixel": {
                "frame": 2,
                "x": 12,
                "y": 12,
                "color": [0, 0, 0, 0]
            }
        }
    ]
}' "Clear a pixel for fill test"

sleep 0.2

send_operation '{
    "operations": [
        {
            "FillArea": {
                "frame": 2,
                "x": 12,
                "y": 12,
                "color": [128, 64, 192, 255]
            }
        }
    ]
}' "Purple fill from (12,12)"

# Test 7: Animation-like updates
echo ""
echo "🔹 Test 7: Animation-like pixel updates..."
start_x=$(random_pos 20)
start_y=$(random_pos 20)
anim_r=$(random_color)
anim_g=$(random_color)
anim_b=$(random_color)
for i in {0..5}; do
    x=$((start_x + i))
    send_operation '{
        "operations": [
            {
                "DrawPixel": {
                    "frame": 0,
                    "x": '$x',
                    "y": '$start_y',
                    "color": ['$anim_r', '$anim_g', '$anim_b', 255]
                }
            }
        ]
    }' "Moving pixel to ($x, $start_y)"
done

# Clear the moving pixels
sleep 0.3
for i in {0..5}; do
    x=$((start_x + i))
    send_operation '{
        "operations": [
            {
                "DrawPixel": {
                    "frame": 0,
                    "x": '$x',
                    "y": '$start_y',
                    "color": [0, 0, 0, 0]
                }
            }
        ]
    }' "Clearing pixel at ($x, $start_y)"
done

# Test 8: Draw a polygon (frame 1)
echo ""
echo "🔹 Test 8: Drawing a random polygon..."
poly_x=$(random_pos 15)
poly_y=$(random_pos 15)
poly_r=$(random_color)
poly_g=$(random_color)
poly_b=$(random_color)
send_operation '{
    "operations": [
        {
            "DrawPolygon": {
                "frame": 1,
                "points": [
                    {"x": '$poly_x', "y": '$poly_y'},
                    {"x": '$((poly_x + 5))', "y": '$((poly_y - 2))'},
                    {"x": '$((poly_x + 8))', "y": '$((poly_y + 3))'},
                    {"x": '$((poly_x + 2))', "y": '$((poly_y + 5))'}
                ],
                "filled": true,
                "color": ['$poly_r', '$poly_g', '$poly_b', 255]
            }
        }
    ]
}' "Random polygon on frame 1"

echo ""
echo "🎉 Real-time update test completed!"
echo "   Check the viewer to see all the changes that were applied."
echo "   Use left/right arrows to navigate between frames 0, 1, and 2."
echo ""
echo "💡 Tips for testing:"
echo "   - Keep the viewer window open during the test"
echo "   - Navigate between frames to see updates on different frames"
echo "   - The updates should appear automatically without manual refresh"
echo "   - If you don't see updates, check that the server supports real-time events" 
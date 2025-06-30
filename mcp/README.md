# PIXL MCP Server

The PIXL MCP (Model Context Protocol) Server enables AI models to create and manipulate pixel art images through a comprehensive set of drawing tools. This server acts as a bridge between AI assistants and the PIXL API, providing seamless integration for AI-powered pixel art creation.

## Features

The PIXL MCP Server provides the following tools:

### File Management
- **health_check**: Verify PIXL server connectivity
- **get_path**: Get current file system path
- **set_path**: Set file system path for pixel books
- **list_books**: List all available pixel books
- **create_book**: Create new pixel books with specified dimensions
- **get_book**: Get information about specific pixel books

### Drawing Operations
- **draw_pixel**: Draw individual pixels with RGBA color
- **set_color**: Set the current drawing color
- **draw_line**: Draw straight or curved lines
- **draw_shape**: Draw rectangles, circles, ovals, and triangles
- **draw_polygon**: Draw custom polygons from point arrays
- **fill_area**: Flood fill areas with color
- **batch_operations**: Apply multiple operations in a single command

## Prerequisites

1. **PIXL Server**: The main PIXL server must be running (typically on `http://localhost:3000`)
2. **Rust**: Rust toolchain for building the MCP server
3. **AI Tool**: Claude Desktop, ChatGPT, Zed, or Cursor with MCP support

## Quick Start

1. **Build the MCP server**:
   ```bash
   cd mcp
   cargo build --release
   ./test-mcp.sh  # Optional: verify build
   ```

2. **Start the PIXL server**:
   ```bash
   cd ../server
   cargo run
   ```

3. **Configure your AI tool** (see configuration sections below)

4. **Start creating pixel art** with AI assistance!

## Installation

### Building the MCP Server

```bash
cd mcp
cargo build --release
```

The compiled binary will be available at `target/release/pixl-mcp-server`.

### Environment Configuration

The server connects to the PIXL API server. Configure the server URL:

```bash
export PIXL_SERVER_URL="http://localhost:3000"  # Default if not set
```

## Configuration for AI Tools

### Claude Desktop

Add the following to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "pixl": {
      "command": "/path/to/pixl/mcp/target/release/pixl-mcp-server",
      "env": {
        "PIXL_SERVER_URL": "http://localhost:3000"
      }
    }
  }
}
```

For debugging, add `"PIXL_MCP_DEBUG": "1"` to the `env` section.

### ChatGPT (with MCP support)

If using ChatGPT with MCP support, add the server configuration:

```json
{
  "mcp_servers": [
    {
      "name": "pixl",
      "command": "/path/to/pixl/mcp/target/release/pixl-mcp-server",
      "env": {
        "PIXL_SERVER_URL": "http://localhost:3000"
      }
    }
  ]
}
```

### Zed Editor

Add to your Zed configuration (`~/.config/zed/settings.json`):

```json
{
  "mcp": {
    "servers": {
      "pixl": {
        "command": "/path/to/pixl/mcp/target/release/pixl-mcp-server",
        "env": {
          "PIXL_SERVER_URL": "http://localhost:3000"
        }
      }
    }
  }
}
```

### Cursor Editor

Add to your Cursor configuration:

```json
{
  "mcp.servers": {
    "pixl": {
      "command": "/path/to/pixl/mcp/target/release/pixl-mcp-server",
      "env": {
        "PIXL_SERVER_URL": "http://localhost:3000"
      }
    }
  }
}
```

## Usage Examples

### Basic Workflow

1. **Start the PIXL server**: 
   ```bash
   cd server
   cargo run
   ```

2. **Start your AI tool** (Claude, ChatGPT, etc.) with MCP configuration

3. **Create a new pixel book**:
   ```
   Use the create_book tool to create a new 32x32 pixel book called "my_art.pxl" with 1 frame
   ```

4. **Draw some pixels**:
   ```
   Draw a red pixel at position (5, 5) in frame 0 of "my_art.pxl"
   ```

5. **Create more complex art**:
   ```
   Draw a blue circle at position (10, 10) with width 8 and height 8 in "my_art.pxl"
   ```

### Advanced Examples

#### Creating a Simple Sprite

```
Create a 16x16 pixel book called "sprite.pxl", then:
1. Draw a filled rectangle for the body (green color)
2. Draw circles for eyes (white with black centers)
3. Draw a line for the mouth (black)
4. Add some decorative pixels around the edges
```

#### Drawing a Landscape

```
Create a 64x32 pixel book called "landscape.pxl", then:
1. Fill the bottom half with green (grass)
2. Fill the top half with blue (sky)
3. Draw brown triangles for mountains
4. Add white circles for clouds
5. Draw a yellow circle for the sun
```

## Tool Reference

### File Management Tools

#### `health_check()`
Checks if the PIXL server is running and responsive.

#### `get_path()`
Returns the current directory path where pixel books are stored.

#### `set_path(path: String)`
Sets the directory path where pixel books should be stored.

#### `list_books()`
Lists all available pixel books in the current directory.

#### `create_book(filename: String, width: u16, height: u16, frames: usize)`
Creates a new pixel book with the specified dimensions and frame count.

Parameters:
- `filename`: Name of the pixel book file (e.g., "my_art.pxl")
- `width`: Width in pixels (1-65535)
- `height`: Height in pixels (1-65535) 
- `frames`: Number of animation frames (1-1000)

#### `get_book(filename: String)`
Retrieves information about a specific pixel book.

### Drawing Tools

#### `draw_pixel(filename: String, frame: usize, x: u16, y: u16, r: u8, g: u8, b: u8, a: u8)`
Draws a single pixel at the specified coordinates.

Parameters:
- `filename`: Target pixel book
- `frame`: Frame index (0-based)
- `x`, `y`: Pixel coordinates
- `r`, `g`, `b`, `a`: RGBA color values (0-255)

#### `draw_line(filename: String, frame: usize, start_x: u16, start_y: u16, end_x: u16, end_y: u16, line_type: String, r: u8, g: u8, b: u8, a: u8)`
Draws a line between two points.

Parameters:
- `line_type`: "straight" or "curved"
- Other parameters as above

#### `draw_shape(filename: String, frame: usize, shape_type: String, x: u16, y: u16, width: u16, height: u16, filled: bool, r: u8, g: u8, b: u8, a: u8)`
Draws geometric shapes.

Parameters:
- `shape_type`: "rectangle", "circle", "oval", or "triangle"
- `x`, `y`: Position of the shape
- `width`, `height`: Size of the shape
- `filled`: Whether to fill the shape or just draw outline

#### `draw_polygon(filename: String, frame: usize, points_json: String, filled: bool, r: u8, g: u8, b: u8, a: u8)`
Draws a polygon from a list of points.

Parameters:
- `points_json`: JSON array of points, e.g., `[{"x": 10, "y": 20}, {"x": 15, "y": 25}, {"x": 5, "y": 30}]`
- `filled`: Whether to fill the polygon

#### `fill_area(filename: String, frame: usize, x: u16, y: u16, r: u8, g: u8, b: u8, a: u8)`
Performs flood fill starting from the specified point.

#### `batch_operations(filename: String, operations_json: String)`
Applies multiple drawing operations in a single command for better performance.

Parameters:
- `operations_json`: JSON array of drawing operations

## Color Guidelines

Colors are specified as RGBA values (Red, Green, Blue, Alpha):
- **Red**: 0-255
- **Green**: 0-255
- **Blue**: 0-255
- **Alpha**: 0-255 (0 = transparent, 255 = opaque)

Common colors:
- Black: (0, 0, 0, 255)
- White: (255, 255, 255, 255)
- Red: (255, 0, 0, 255)
- Green: (0, 255, 0, 255)
- Blue: (0, 0, 255, 255)
- Transparent: (0, 0, 0, 0)

## Troubleshooting

### Common Issues

1. **"PIXL server is not healthy"**
   - Ensure the PIXL server is running on the correct port
   - Check the `PIXL_SERVER_URL` environment variable
   - Verify network connectivity

2. **"Failed to create book"**
   - Check that the filename is valid (ends with .pxl)
   - Ensure dimensions are within valid ranges
   - Verify write permissions in the target directory

3. **"Invalid filename"**
   - Filenames must end with `.pxl`
   - Avoid special characters in filenames
   - Use alphanumeric characters and underscores

4. **"Drawing operation failed"**
   - Check that coordinates are within the pixel book dimensions
   - Verify frame index is valid (0-based)
   - Ensure the pixel book exists

### Debug Mode

The MCP server runs silently by default to avoid interfering with JSON-RPC communication. To enable debug logging (outputs to stderr), set:
```bash
export PIXL_MCP_DEBUG=1
```

Then run your AI tool. Debug logs will appear in the terminal where you started the AI application.

## Real-time Viewing

While using the MCP server, you can run the PIXL viewer to see your creations in real-time:

```bash
cd viewer
cargo run
```

The viewer will automatically update as you make changes through the MCP server.

## Performance Tips

1. **Use batch operations** for multiple drawing commands to reduce API calls
2. **Create appropriate canvas sizes** - larger canvases require more memory
3. **Limit frame counts** for animations to reasonable numbers
4. **Use the viewer** to preview changes without repeatedly calling get_book

## Support

For issues or questions:
1. Check the PIXL server logs for error details
2. Verify MCP server configuration in your AI tool
3. Test individual tools using the health_check command
4. Ensure all prerequisites are properly installed

## License

This MCP server is part of the PIXL project and follows the same licensing terms. 
# Viewer Application PRD

## Overview
The PIXL viewer is a minimal desktop application that displays pixel art from pixel books with real-time updates. It uses minifb for cross-platform window management and rendering.

## Core Requirements

### Rendering System
- **Framework**: minifb for cross-platform rendering
- **Display**: Pixel-perfect scaling of bitmap data
- **Alpha Channel**: Checkerboard pattern for transparent pixels
- **Performance**: Real-time updates with minimal latency

### User Interface
- **Design**: Minimal UI with focus on pixel art display
- **Window**: Resizable window with pixel art centered
- **Scaling**: Automatic scaling to fit window size
- **Background**: Black background for non-matching aspect ratios

## Functional Requirements

### Display Logic

#### Pixel Scaling
- Calculate scale factor: `min(window_width / image_width, window_height / image_height)`
- Use integer scaling only (no fractional scaling)
- Center image in window if aspect ratios don't match
- Fill remaining space with black

**Example Scaling:**
- 32x32 pixel image in 128x128 window = 4x scale (each pixel becomes 4x4)
- 32x32 pixel image in 100x100 window = 3x scale (image centered with black borders)

#### Alpha Channel Rendering
- Transparent pixels (alpha < 255) show checkerboard pattern
- Checkerboard: alternating light gray (240, 240, 240) and dark gray (200, 200, 200)
- Checkerboard square size: 8x8 pixels at 1x scale, scaled proportionally

### File Management

#### File Selection (Ctrl+O)
- Trigger: Ctrl+O keyboard shortcut
- Action: Query server for available pixel books via `GET /books`
- Dialog: Cross-platform file selection using rfd crate
- Display: List of `.pxl` files with metadata (name, frames, dimensions)

#### File Loading
- Request pixel book data via `GET /books/{filename}`
- Parse RGBA pixel data
- Initialize display with first frame
- Connect to SSE stream for real-time updates

### Real-Time Updates

#### Server-Sent Events
- Connect to `GET /books/{filename}/events` endpoint
- Handle update events in real-time
- Update pixel data without full reload
- Maintain rendering performance during updates

#### Event Types
```
update: Single pixel update
frame_added: New frame added to pixel book
frame_removed: Frame removed from pixel book
book_updated: Full book refresh required
```

## Technical Requirements

### Window Management
- **Initial Size**: 512x512 pixels
- **Minimum Size**: 256x256 pixels
- **Resizable**: Yes
- **Title**: `PIXL Viewer - {filename}` or `PIXL Viewer` if no file loaded

### Keyboard Controls
- `Ctrl+O`: Open file dialog
- `Escape`: Close application
- `Left/Right Arrow`: Navigate frames (if multiple frames)
- `Space`: Play/pause animation (future feature)

### Performance Targets
- **Frame Rate**: 60 FPS rendering
- **Update Latency**: < 50ms from server event to display
- **Memory Usage**: < 100MB for typical pixel books
- **Startup Time**: < 2 seconds

## Data Structures

### Pixel Book
```rust
struct PixelBook {
    filename: String,
    width: u32,
    height: u32,
    frames: Vec<Frame>,
    current_frame: usize,
}

struct Frame {
    index: usize,
    pixels: Vec<Vec<[u8; 4]>>, // RGBA pixels as 2D array
}
```

### Rendering State
```rust
struct RenderState {
    window_buffer: Vec<u32>, // minifb pixel buffer
    scale_factor: u32,
    offset_x: i32,
    offset_y: i32,
    checkerboard_pattern: Vec<u32>,
}
```

## Error Handling

### Server Connection
- Display error message if server unreachable
- Retry connection with exponential backoff
- Show connection status in window title

### File Loading
- Handle file not found gracefully
- Display error dialog for invalid pixel books
- Fall back to empty state if loading fails

### Rendering Errors
- Continue operation if single frame fails to render
- Log errors without crashing application
- Provide visual feedback for rendering issues

## Platform Support
- **Primary**: macOS, Linux, Windows
- **Architecture**: x86_64, ARM64
- **Dependencies**: Minimal system requirements
- **Distribution**: Single binary executable

## Future Considerations
- Frame animation playback
- Zoom controls
- Export functionality
- Multi-book viewing
- Editing capabilities (read-only for now) 
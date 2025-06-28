# PIXL Development Plan

## Overview
This document outlines the development plan for the PIXL pixel art API and viewer application. The implementation is divided into phases to ensure steady progress and testable milestones.

## Development Phases

### Phase 1: Core Infrastructure (Week 1-2)
**Goal**: Establish basic project structure, data models, and file operations

#### Server Tasks
1. **Project Setup**
   - Update Cargo.toml with required dependencies
   - Set up proper error handling types
   - Create basic project structure

2. **Pixel Book File Format**
   - Implement binary file format reader/writer
   - Create Pixel, Frame, and PixelBook data structures
   - Add comprehensive unit tests for file operations

3. **Basic API Endpoints**
   - `GET /path` - Get current file system path
   - `PUT /path` - Set file system path
   - `GET /books` - List available pixel books
   - `POST /books` - Create new pixel book

#### Viewer Tasks
1. **Project Setup**
   - Update Cargo.toml with minifb and HTTP client dependencies
   - Set up basic window creation and event handling
   - Create basic project structure

2. **Server Communication**
   - Implement HTTP client for API communication
   - Create data structures matching server API
   - Add error handling for network operations

**Deliverables:**
- Working file system operations
- Basic API endpoints operational
- Viewer window opens and displays placeholder
- Unit tests passing for file operations

### Phase 2: Pixel Book Operations (Week 3-4)
**Goal**: Implement pixel book loading, display, and basic drawing operations

#### Server Tasks
1. **Pixel Book Management**
   - `GET /books/{filename}` - Load and return pixel book data
   - Implement file validation and error handling
   - Add comprehensive error responses

2. **Basic Drawing Operations**
   - `PUT /books/{filename}` - Handle pixel book updates
   - Implement `draw_pixel` operation
   - Implement `set_color` operation
   - Add coordinate and color validation

#### Viewer Tasks
1. **Pixel Book Display**
   - Load pixel book data from server
   - Implement pixel-perfect scaling algorithm
   - Render RGBA pixels to minifb buffer
   - Handle window resizing

2. **Alpha Channel Support**
   - Implement checkerboard pattern for transparency
   - Optimize rendering performance
   - Add proper color blending

**Deliverables:**
- Pixel books can be loaded and displayed
- Basic pixel drawing operations work
- Viewer correctly scales and displays pixel art
- Alpha channel rendering functional

### Phase 3: Advanced Drawing & Real-time Updates (Week 5-6)
**Goal**: Complete drawing operations and implement real-time updates via SSE

#### Server Tasks
1. **Advanced Drawing Operations**
   - Implement shape drawing (rectangle, circle, oval, triangle)
   - Implement line drawing (straight and curved)
   - Implement polygon drawing
   - Implement flood fill operation

2. **Server-Sent Events**
   - `GET /books/{filename}/events` - SSE endpoint
   - Real-time update broadcasting
   - Event serialization and optimization

#### Viewer Tasks
1. **Real-time Updates**
   - Connect to SSE stream
   - Handle pixel update events
   - Optimize rendering for real-time updates
   - Maintain rendering performance

2. **File Selection**
   - Implement Ctrl+O keyboard shortcut
   - Integrate rfd dialog for cross-platform file selection
   - Handle file loading and switching

**Deliverables:**
- All drawing operations implemented and tested
- Real-time updates working between server and viewer
- File selection dialog functional
- Performance targets met

### Phase 4: Testing & Polish (Week 7-8)
**Goal**: Comprehensive testing, error handling, and user experience polish

#### Server Tasks
1. **Testing & Validation**
   - Comprehensive unit tests for all operations
   - Integration tests for API endpoints
   - Performance testing and optimization
   - Error handling improvements

2. **API Polish**
   - Improve error messages and codes
   - Add request validation
   - Optimize file I/O operations
   - Documentation improvements

#### Viewer Tasks
1. **Testing & Polish**
   - Unit tests for rendering and scaling
   - Integration tests with server
   - Error handling and user feedback
   - Performance optimization

2. **User Experience**
   - Improve keyboard shortcuts
   - Better error dialogs
   - Window management improvements
   - Visual feedback enhancements

**Deliverables:**
- Comprehensive test suite passing
- Error handling robust and user-friendly
- Performance optimized
- Documentation complete

## Build System Setup

### Root Level Makefile.toml
```toml
[config]
default_to_workspace = false

[tasks.build]
dependencies = ["build-server", "build-viewer"]

[tasks.build-server]
command = "cargo"
args = ["make", "--cwd", "server", "--makefile", "tasks.toml", "build"]

[tasks.build-viewer]
command = "cargo"
args = ["make", "--cwd", "viewer", "--makefile", "tasks.toml", "build"]

[tasks.test]
dependencies = ["test-server", "test-viewer"]

[tasks.test-server]
command = "cargo"
args = ["make", "--cwd", "server", "--makefile", "tasks.toml", "test"]

[tasks.test-viewer]
command = "cargo"
args = ["make", "--cwd", "viewer", "--makefile", "tasks.toml", "test"]

[tasks.clean]
dependencies = ["clean-server", "clean-viewer"]

[tasks.clean-server]
command = "cargo"
args = ["make", "--cwd", "server", "--makefile", "tasks.toml", "clean"]

[tasks.clean-viewer]
command = "cargo"
args = ["make", "--cwd", "viewer", "--makefile", "tasks.toml", "clean"]

[tasks.run-server]
command = "cargo"
args = ["make", "--cwd", "server", "--makefile", "tasks.toml", "run"]

[tasks.run-viewer]
command = "cargo"
args = ["make", "--cwd", "viewer", "--makefile", "tasks.toml", "run"]
```

## Testing Strategy

### Unit Tests
- File format operations (read/write/validate)
- Drawing operations (pixel manipulation)
- API endpoint handlers
- Rendering algorithms
- Scaling calculations

### Integration Tests
- Full API workflow tests
- Server-viewer communication
- File system operations
- SSE event handling

### Performance Tests
- Large pixel book loading
- Real-time update latency
- Memory usage monitoring
- Rendering frame rate

## Quality Assurance

### Code Quality
- Clippy lints enabled and passing
- Rustfmt for consistent formatting
- Documentation for public APIs
- Error handling best practices

### Testing Coverage
- Minimum 80% test coverage
- Critical paths 100% covered
- Edge cases thoroughly tested
- Error conditions tested

## Risk Management

### Technical Risks
- **Performance**: Large pixel books may impact rendering
- **Memory**: Multiple frames could consume excessive memory
- **Network**: SSE connection stability
- **Cross-platform**: File dialog and rendering differences

### Mitigation Strategies
- Implement frame caching and lazy loading
- Add memory usage monitoring and limits
- Implement connection retry logic
- Test on all target platforms early

## Success Criteria

### Functional Requirements
- ✅ All API endpoints operational
- ✅ All drawing operations working
- ✅ Real-time updates functional
- ✅ File selection dialog working
- ✅ Pixel-perfect rendering with scaling

### Quality Requirements
- ✅ Unit tests passing (>80% coverage)
- ✅ Integration tests passing
- ✅ Performance targets met
- ✅ Error handling robust
- ✅ Cross-platform compatibility

### User Experience
- ✅ Responsive UI (< 50ms update latency)
- ✅ Intuitive keyboard controls
- ✅ Clear error messages
- ✅ Stable operation under normal use 
# System Architecture PRD

## Overview
PIXL is a pixel art creation and viewing application designed as a development tool for AI-assisted game development. The system consists of two main components: a REST API server for pixel art manipulation and a viewer application for real-time display.

## System Components

### 1. Server (API)
- **Technology**: Rust with Poem web framework
- **Purpose**: Provides REST API for pixel art operations
- **Location**: `server/` directory
- **Responsibilities**:
  - File system operations for pixel books
  - Pixel art manipulation operations
  - Real-time updates via Server-Sent Events (SSE)
  - Path management for pixel book storage

### 2. Viewer Application
- **Technology**: Rust with minifb for rendering
- **Purpose**: Displays pixel art with real-time updates
- **Location**: `viewer/` directory
- **Responsibilities**:
  - Render pixel art frames with scaling
  - Handle file selection dialogs
  - Receive real-time updates from server
  - Alpha channel visualization (checkerboard pattern)

### 3. Pixel Book File Format
- **Extension**: `.pxl`
- **Storage**: Bitmapped data format
- **Structure**: Multiple frames per book
- **Pixel Format**: RGBA values

## Data Flow

```
AI Tools/Client → Server API → Pixel Book Files (.pxl)
                      ↓
                 SSE Updates
                      ↓
                  Viewer App → Visual Display
```

## System Requirements

### Performance
- Real-time updates via SSE
- Efficient bitmap rendering with scaling
- Fast file I/O operations

### Scalability
- Single-user development tool
- File-based storage (no database)
- Configurable file system path

### Security
- File system access limited to configured path
- No authentication required (development tool)

## Build System
- **Primary**: `cargo make` for root-level tasks
- **Component**: `cargo make --cwd server|viewer --makefile tasks.toml` for individual components
- **Testing**: Unit tests for all components

## Development Phases
1. **Phase 1**: Core API and basic viewer
2. **Phase 2**: Drawing operations and real-time updates
3. **Phase 3**: Advanced features and optimization 
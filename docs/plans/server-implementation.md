# Server Implementation Plan

## Overview
This document provides detailed implementation tasks for the PIXL server API. The server is built using Poem web framework and implements a REST API for pixel book management and drawing operations.

## Project Structure
```
server/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library exports
│   ├── api/              # API endpoint handlers
│   │   ├── mod.rs        # API module exports
│   │   ├── path.rs       # Path management endpoints
│   │   ├── books.rs      # Book management endpoints
│   │   └── events.rs     # SSE event handling
│   ├── models/           # Data structures
│   │   ├── mod.rs        # Model exports
│   │   ├── pixel_book.rs # PixelBook, Frame, Pixel structs
│   │   ├── operations.rs # Drawing operation types
│   │   └── errors.rs     # Error types
│   ├── services/         # Business logic
│   │   ├── mod.rs        # Service exports
│   │   ├── file_service.rs     # File I/O operations
│   │   ├── drawing_service.rs  # Drawing operations
│   │   └── event_service.rs    # SSE event management
│   └── utils/            # Utility functions
│       ├── mod.rs        # Utility exports
│       └── validation.rs # Input validation
├── tests/                # Integration tests
│   ├── api_tests.rs      # API endpoint tests
│   ├── drawing_tests.rs  # Drawing operation tests
│   └── file_tests.rs     # File format tests
├── Cargo.toml            # Dependencies
└── tasks.toml            # Build tasks
```

## Phase 1: Core Infrastructure

### Task 1.1: Update Dependencies
Update `Cargo.toml` with required dependencies:

```toml
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[dependencies]
poem = { version = "3.1", features = ["sse"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.45", features = ["full"] }
futures-util = "0.3"
tokio-stream = "0.1"
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
dirs = "5.0"

[dev-dependencies]
tokio-test = "0.4"
tempfile = "3.0"
```

### Task 1.2: Define Data Models
Create `src/models/pixel_book.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub index: usize,
    pub pixels: Vec<Vec<Pixel>>, // [y][x] indexing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelBook {
    pub filename: String,
    pub width: u16,
    pub height: u16,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelBookInfo {
    pub filename: String,
    pub size: u64,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub frames: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePixelBookRequest {
    pub filename: String,
    pub width: u16,
    pub height: u16,
    pub frames: usize,
}
```

### Task 1.3: Define Error Types
Create `src/models/errors.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PixelError {
    #[error("File not found: {filename}")]
    FileNotFound { filename: String },
    
    #[error("Invalid file format: {details}")]
    InvalidFormat { details: String },
    
    #[error("Invalid coordinates: x={x}, y={y} for image size {width}x{height}")]
    InvalidCoordinates { x: u16, y: u16, width: u16, height: u16 },
    
    #[error("Invalid color values: {details}")]
    InvalidColor { details: String },
    
    #[error("Invalid path: {path}")]
    InvalidPath { path: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, PixelError>;
```

### Task 1.4: Implement File Format
Create `src/services/file_service.rs`:

```rust
use crate::models::{PixelBook, Pixel, Frame, PixelBookInfo, Result, PixelError};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};

const MAGIC_NUMBER: u32 = 0x504958; // "PIX"
const FORMAT_VERSION: u16 = 1;

pub struct FileService {
    base_path: PathBuf,
}

impl FileService {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
    
    pub fn set_path(&mut self, path: PathBuf) -> Result<()> {
        if !path.exists() || !path.is_dir() {
            return Err(PixelError::InvalidPath { 
                path: path.to_string_lossy().to_string() 
            });
        }
        self.base_path = path;
        Ok(())
    }
    
    pub fn get_path(&self) -> &Path {
        &self.base_path
    }
    
    pub fn list_books(&self) -> Result<Vec<PixelBookInfo>> {
        // Implementation for listing .pxl files
        todo!()
    }
    
    pub fn load_book(&self, filename: &str) -> Result<PixelBook> {
        // Implementation for loading binary .pxl file
        todo!()
    }
    
    pub fn save_book(&self, book: &PixelBook) -> Result<()> {
        // Implementation for saving binary .pxl file
        todo!()
    }
    
    pub fn create_book(&self, filename: &str, width: u16, height: u16, frames: usize) -> Result<PixelBook> {
        // Implementation for creating new pixel book
        todo!()
    }
}
```

### Task 1.5: Basic API Endpoints
Create `src/api/path.rs`:

```rust
use crate::services::FileService;
use poem::{handler, web::Json, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize)]
pub struct SetPathRequest {
    pub path: String,
}

#[derive(Serialize)]
pub struct PathResponse {
    pub path: String,
}

#[handler]
pub async fn get_path(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
) -> Result<Json<PathResponse>> {
    // Implementation
    todo!()
}

#[handler]
pub async fn set_path(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
    request: Json<SetPathRequest>,
) -> Result<Json<PathResponse>> {
    // Implementation
    todo!()
}
```

## Phase 2: Pixel Book Operations

### Task 2.1: Book Management Endpoints
Create `src/api/books.rs`:

```rust
use crate::models::{PixelBook, PixelBookInfo, CreatePixelBookRequest, Result};
use crate::services::FileService;
use poem::{handler, web::{Json, Path}, Result as PoemResult};
use std::sync::Arc;
use tokio::sync::RwLock;

#[handler]
pub async fn list_books(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
) -> PoemResult<Json<Vec<PixelBookInfo>>> {
    // Implementation
    todo!()
}

#[handler]
pub async fn get_book(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
    filename: Path<String>,
) -> PoemResult<Json<PixelBook>> {
    // Implementation
    todo!()
}

#[handler]
pub async fn create_book(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
    request: Json<CreatePixelBookRequest>,
) -> PoemResult<Json<serde_json::Value>> {
    // Implementation
    todo!()
}
```

### Task 2.2: Drawing Operations
Create `src/models/operations.rs`:

```rust
use serde::{Deserialize, Serialize};
use crate::models::Pixel;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DrawingOperation {
    #[serde(rename = "draw_pixel")]
    DrawPixel {
        frame: usize,
        x: u16,
        y: u16,
        color: [u8; 4],
    },
    #[serde(rename = "set_color")]
    SetColor {
        color: [u8; 4],
    },
    #[serde(rename = "draw_line")]
    DrawLine {
        frame: usize,
        start: Point,
        end: Point,
        line_type: LineType,
        color: [u8; 4],
    },
    #[serde(rename = "draw_shape")]
    DrawShape {
        frame: usize,
        shape: ShapeType,
        position: Point,
        size: Size,
        filled: bool,
        color: [u8; 4],
    },
    #[serde(rename = "draw_polygon")]
    DrawPolygon {
        frame: usize,
        points: Vec<Point>,
        filled: bool,
        color: [u8; 4],
    },
    #[serde(rename = "fill_area")]
    FillArea {
        frame: usize,
        x: u16,
        y: u16,
        color: [u8; 4],
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LineType {
    #[serde(rename = "straight")]
    Straight,
    #[serde(rename = "curved")]
    Curved,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShapeType {
    #[serde(rename = "rectangle")]
    Rectangle,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "oval")]
    Oval,
    #[serde(rename = "triangle")]
    Triangle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePixelBookRequest {
    pub operations: Vec<DrawingOperation>,
}
```

### Task 2.3: Drawing Service
Create `src/services/drawing_service.rs`:

```rust
use crate::models::{PixelBook, Pixel, DrawingOperation, Point, Result, PixelError};

pub struct DrawingService;

impl DrawingService {
    pub fn apply_operations(book: &mut PixelBook, operations: &[DrawingOperation]) -> Result<()> {
        for operation in operations {
            Self::apply_operation(book, operation)?;
        }
        Ok(())
    }
    
    fn apply_operation(book: &mut PixelBook, operation: &DrawingOperation) -> Result<()> {
        match operation {
            DrawingOperation::DrawPixel { frame, x, y, color } => {
                Self::draw_pixel(book, *frame, *x, *y, *color)?;
            }
            DrawingOperation::DrawLine { frame, start, end, line_type, color } => {
                Self::draw_line(book, *frame, start, end, line_type, *color)?;
            }
            // ... other operations
        }
        Ok(())
    }
    
    fn draw_pixel(book: &mut PixelBook, frame: usize, x: u16, y: u16, color: [u8; 4]) -> Result<()> {
        // Implementation
        todo!()
    }
    
    fn draw_line(book: &mut PixelBook, frame: usize, start: &Point, end: &Point, line_type: &LineType, color: [u8; 4]) -> Result<()> {
        // Implementation using Bresenham's algorithm
        todo!()
    }
    
    // ... other drawing methods
}
```

## Phase 3: Real-time Updates

### Task 3.1: Event Service
Create `src/services/event_service.rs`:

```rust
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum PixelBookEvent {
    #[serde(rename = "update")]
    Update {
        frame: usize,
        x: u16,
        y: u16,
        color: [u8; 4],
    },
    #[serde(rename = "frame_added")]
    FrameAdded {
        frame: usize,
    },
    #[serde(rename = "frame_removed")]
    FrameRemoved {
        frame: usize,
    },
    #[serde(rename = "book_updated")]
    BookUpdated,
}

pub struct EventService {
    channels: std::collections::HashMap<String, broadcast::Sender<PixelBookEvent>>,
}

impl EventService {
    pub fn new() -> Self {
        Self {
            channels: std::collections::HashMap::new(),
        }
    }
    
    pub fn subscribe(&mut self, filename: &str) -> broadcast::Receiver<PixelBookEvent> {
        let sender = self.channels
            .entry(filename.to_string())
            .or_insert_with(|| broadcast::channel(100).0);
        sender.subscribe()
    }
    
    pub fn publish(&mut self, filename: &str, event: PixelBookEvent) {
        if let Some(sender) = self.channels.get(filename) {
            let _ = sender.send(event);
        }
    }
}
```

### Task 3.2: SSE Endpoint
Create `src/api/events.rs`:

```rust
use crate::services::EventService;
use poem::{handler, web::{Path, sse::{Event, SSE}}};
use tokio_stream::wrappers::BroadcastStream;
use futures_util::StreamExt;

#[handler]
pub async fn pixel_book_events(
    event_service: poem::web::Data<&Arc<RwLock<EventService>>>,
    filename: Path<String>,
) -> SSE {
    let receiver = {
        let mut service = event_service.write().await;
        service.subscribe(&filename)
    };
    
    let stream = BroadcastStream::new(receiver)
        .filter_map(|result| async move {
            match result {
                Ok(event) => {
                    let json = serde_json::to_string(&event).ok()?;
                    Some(Event::message(json))
                }
                Err(_) => None,
            }
        });
    
    SSE::new(stream).keep_alive(Duration::from_secs(30))
}
```

## Phase 4: Testing & Integration

### Task 4.1: Unit Tests
Create comprehensive unit tests for:
- File format operations
- Drawing algorithms
- API endpoint handlers
- Validation logic

### Task 4.2: Integration Tests
Create `tests/api_tests.rs`:

```rust
use tokio_test;
use tempfile::TempDir;

#[tokio::test]
async fn test_create_and_load_pixel_book() {
    // Test full workflow
}

#[tokio::test]
async fn test_drawing_operations() {
    // Test drawing operations
}

#[tokio::test]
async fn test_sse_events() {
    // Test real-time events
}
```

### Task 4.3: Build Tasks
Create `server/tasks.toml`:

```toml
[tasks.build]
command = "cargo"
args = ["build"]

[tasks.test]
command = "cargo"
args = ["test"]

[tasks.run]
command = "cargo"
args = ["run"]

[tasks.clean]
command = "cargo"
args = ["clean"]

[tasks.check]
command = "cargo"
args = ["check"]

[tasks.clippy]
command = "cargo"
args = ["clippy", "--", "-D", "warnings"]

[tasks.fmt]
command = "cargo"
args = ["fmt"]
```

## Implementation Notes

### Error Handling
- Use `thiserror` for structured error types
- Implement proper HTTP status codes
- Provide detailed error messages for debugging

### Performance Considerations
- Use async/await throughout
- Implement efficient file I/O with buffering
- Consider memory usage for large pixel books
- Optimize drawing algorithms for performance

### Security
- Validate all input data
- Restrict file system access to configured path
- Implement proper bounds checking
- Handle malformed requests gracefully

### Testing Strategy
- Unit tests for individual functions
- Integration tests for API workflows
- Performance tests for large operations
- Error condition testing 
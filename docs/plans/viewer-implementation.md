# Viewer Implementation Plan

## Overview
This document provides detailed implementation tasks for the PIXL viewer application. The viewer uses minifb for cross-platform rendering and provides real-time pixel art display with file selection capabilities.

## Implementation Tasks

### Phase 1: Core Infrastructure
- Update Cargo.toml with dependencies (minifb, reqwest, rfd, etc.)
- Create project structure with modules for app, rendering, models, services
- Define data structures for PixelBook, Frame, and Pixel
- Implement basic window creation with minifb

### Phase 2: Rendering System
- Implement pixel scaling algorithm for integer scaling
- Create checkerboard pattern for alpha channel visualization
- Build main renderer with pixel buffer management
- Add support for real-time frame updates

### Phase 3: File Management
- Integrate rfd for cross-platform file selection dialogs
- Implement API client for server communication
- Add keyboard shortcuts (Ctrl+O for file open)
- Connect to Server-Sent Events for real-time updates

### Phase 4: Testing and Polish
- Add unit tests for rendering and scaling
- Implement error handling and user feedback
- Optimize performance for 60 FPS rendering
- Add frame navigation controls

## Technical Requirements
- Window management with minifb
- HTTP client for API communication
- Real-time SSE event handling
- Cross-platform file dialogs using rfd
- Efficient pixel buffer rendering

## Build Configuration
- tasks.toml with build, test, run, clean tasks
- Integration with root-level cargo make system
- Unit and integration test suites

## Project Structure
```
viewer/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library exports
│   ├── app/              # Application logic
│   │   ├── mod.rs        # App module exports
│   │   ├── viewer.rs     # Main viewer application
│   │   ├── input.rs      # Input handling
│   │   └── state.rs      # Application state
│   ├── rendering/        # Rendering system
│   │   ├── mod.rs        # Rendering exports
│   │   ├── renderer.rs   # Main renderer
│   │   ├── scaling.rs    # Pixel scaling logic
│   │   └── checkerboard.rs # Alpha channel pattern
│   ├── models/           # Data structures
│   │   ├── mod.rs        # Model exports
│   │   ├── pixel_book.rs # PixelBook, Frame, Pixel structs
│   │   └── events.rs     # SSE event types
│   ├── services/         # Business logic
│   │   ├── mod.rs        # Service exports
│   │   ├── api_client.rs # HTTP client for server API
│   │   ├── event_client.rs # SSE client
│   │   └── file_dialog.rs  # File selection dialog
│   └── utils/            # Utility functions
│       ├── mod.rs        # Utility exports
│       └── color.rs      # Color conversion utilities
├── tests/                # Integration tests
│   ├── rendering_tests.rs # Rendering tests
│   ├── scaling_tests.rs   # Scaling algorithm tests
│   └── integration_tests.rs # Full integration tests
├── Cargo.toml            # Dependencies
└── tasks.toml            # Build tasks
```

## Phase 1: Core Infrastructure

### Task 1.1: Update Dependencies
Update `Cargo.toml` with required dependencies:

```toml
[package]
name = "viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
minifb = "0.28.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.45", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "stream"] }
futures-util = "0.3"
rfd = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tokio-test = "0.4"
```

### Task 1.2: Define Data Models
Create `src/models/pixel_book.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn to_rgba32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    
    pub fn is_transparent(&self) -> bool {
        self.a < 255
    }
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
```

### Task 1.3: Define Event Types
Create `src/models/events.rs`:

```rust
use serde::{Serialize, Deserialize};

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
```

### Task 1.4: API Client Service
Create `src/services/api_client.rs`:

```rust
use crate::models::{PixelBook, PixelBookInfo};
use reqwest::Client;
use std::error::Error;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn list_books(&self) -> Result<Vec<PixelBookInfo>, Box<dyn Error>> {
        let url = format!("{}/books", self.base_url);
        let response = self.client.get(&url).send().await?;
        let books: Vec<PixelBookInfo> = response.json().await?;
        Ok(books)
    }
    
    pub async fn get_book(&self, filename: &str) -> Result<PixelBook, Box<dyn Error>> {
        let url = format!("{}/books/{}", self.base_url, filename);
        let response = self.client.get(&url).send().await?;
        let book: PixelBook = response.json().await?;
        Ok(book)
    }
    
    pub async fn get_path(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/path", self.base_url);
        let response = self.client.get(&url).send().await?;
        let path_response: serde_json::Value = response.json().await?;
        Ok(path_response["path"].as_str().unwrap_or("").to_string())
    }
}
```

### Task 1.5: Basic Window Setup
Create `src/app/viewer.rs`:

```rust
use minifb::{Key, Window, WindowOptions};
use crate::models::PixelBook;
use crate::rendering::Renderer;
use crate::services::ApiClient;
use std::time::Duration;

const WINDOW_WIDTH: usize = 512;
const WINDOW_HEIGHT: usize = 512;

pub struct Viewer {
    window: Window,
    renderer: Renderer,
    api_client: ApiClient,
    current_book: Option<PixelBook>,
    current_frame: usize,
}

impl Viewer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut window = Window::new(
            "PIXL Viewer",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default(),
        )?;
        
        window.limit_update_rate(Some(Duration::from_millis(16))); // ~60 FPS
        
        let renderer = Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let api_client = ApiClient::new("http://localhost:3000".to_string());
        
        Ok(Self {
            window,
            renderer,
            api_client,
            current_book: None,
            current_frame: 0,
        })
    }
    
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.handle_input().await?;
            self.render();
            
            let buffer = self.renderer.get_buffer();
            self.window.update_with_buffer(buffer, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        }
        
        Ok(())
    }
    
    async fn handle_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.window.is_key_pressed(Key::O, minifb::KeyRepeat::No) 
            && (self.window.is_key_down(Key::LeftCtrl) || self.window.is_key_down(Key::RightCtrl)) {
            self.open_file_dialog().await?;
        }
        
        // Frame navigation
        if let Some(book) = &self.current_book {
            if self.window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) && self.current_frame > 0 {
                self.current_frame -= 1;
            }
            if self.window.is_key_pressed(Key::Right, minifb::KeyRepeat::No) && self.current_frame < book.frames.len() - 1 {
                self.current_frame += 1;
            }
        }
        
        Ok(())
    }
    
    async fn open_file_dialog(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation using rfd and API client
        todo!()
    }
    
    fn render(&mut self) {
        if let Some(book) = &self.current_book {
            let (width, height) = self.window.get_size();
            self.renderer.update_size(width, height);
            
            if let Some(frame) = book.frames.get(self.current_frame) {
                self.renderer.render_frame(frame, book.width, book.height);
            }
        } else {
            self.renderer.clear();
        }
    }
}
```

## Phase 2: Rendering System

### Task 2.1: Pixel Scaling Algorithm
Create `src/rendering/scaling.rs`:

```rust
pub struct ScalingCalculator;

impl ScalingCalculator {
    pub fn calculate_scale_and_offset(
        image_width: u16,
        image_height: u16,
        window_width: usize,
        window_height: usize,
    ) -> (u32, i32, i32) {
        let scale_x = window_width / image_width as usize;
        let scale_y = window_height / image_height as usize;
        let scale = std::cmp::min(scale_x, scale_y).max(1) as u32;
        
        let scaled_width = (image_width as u32 * scale) as i32;
        let scaled_height = (image_height as u32 * scale) as i32;
        
        let offset_x = (window_width as i32 - scaled_width) / 2;
        let offset_y = (window_height as i32 - scaled_height) / 2;
        
        (scale, offset_x, offset_y)
    }
    
    pub fn pixel_to_screen_coords(
        pixel_x: u16,
        pixel_y: u16,
        scale: u32,
        offset_x: i32,
        offset_y: i32,
    ) -> (i32, i32) {
        let screen_x = offset_x + (pixel_x as u32 * scale) as i32;
        let screen_y = offset_y + (pixel_y as u32 * scale) as i32;
        (screen_x, screen_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_perfect_scale() {
        let (scale, offset_x, offset_y) = ScalingCalculator::calculate_scale_and_offset(
            32, 32, 128, 128
        );
        assert_eq!(scale, 4);
        assert_eq!(offset_x, 0);
        assert_eq!(offset_y, 0);
    }
    
    #[test]
    fn test_non_perfect_scale() {
        let (scale, offset_x, offset_y) = ScalingCalculator::calculate_scale_and_offset(
            32, 32, 100, 100
        );
        assert_eq!(scale, 3);
        assert_eq!(offset_x, 2);
        assert_eq!(offset_y, 2);
    }
}
```

### Task 2.2: Checkerboard Pattern
Create `src/rendering/checkerboard.rs`:

```rust
pub struct CheckerboardPattern {
    light_color: u32,
    dark_color: u32,
    square_size: u32,
}

impl CheckerboardPattern {
    pub fn new() -> Self {
        Self {
            light_color: 0xF0F0F0, // Light gray (240, 240, 240)
            dark_color: 0xC8C8C8,  // Dark gray (200, 200, 200)
            square_size: 8,
        }
    }
    
    pub fn get_color_at(&self, x: u32, y: u32, scale: u32) -> u32 {
        let checker_size = self.square_size * scale;
        let checker_x = x / checker_size;
        let checker_y = y / checker_size;
        
        if (checker_x + checker_y) % 2 == 0 {
            self.light_color
        } else {
            self.dark_color
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_checkerboard_pattern() {
        let pattern = CheckerboardPattern::new();
        
        // Test at scale 1
        assert_eq!(pattern.get_color_at(0, 0, 1), 0xF0F0F0);
        assert_eq!(pattern.get_color_at(8, 0, 1), 0xC8C8C8);
        assert_eq!(pattern.get_color_at(0, 8, 1), 0xC8C8C8);
        assert_eq!(pattern.get_color_at(8, 8, 1), 0xF0F0F0);
    }
}
```

### Task 2.3: Main Renderer
Create `src/rendering/renderer.rs`:

```rust
use crate::models::{Frame, Pixel};
use crate::rendering::{ScalingCalculator, CheckerboardPattern};

pub struct Renderer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    checkerboard: CheckerboardPattern,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
            checkerboard: CheckerboardPattern::new(),
        }
    }
    
    pub fn update_size(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.buffer.resize(width * height, 0);
        }
    }
    
    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
    
    pub fn clear(&mut self) {
        self.buffer.fill(0x000000); // Black
    }
    
    pub fn render_frame(&mut self, frame: &Frame, image_width: u16, image_height: u16) {
        self.clear();
        
        let (scale, offset_x, offset_y) = ScalingCalculator::calculate_scale_and_offset(
            image_width,
            image_height,
            self.width,
            self.height,
        );
        
        for (y, row) in frame.pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                self.render_pixel(x as u16, y as u16, pixel, scale, offset_x, offset_y);
            }
        }
    }
    
    fn render_pixel(&mut self, x: u16, y: u16, pixel: &Pixel, scale: u32, offset_x: i32, offset_y: i32) {
        let (screen_x, screen_y) = ScalingCalculator::pixel_to_screen_coords(x, y, scale, offset_x, offset_y);
        
        // Check bounds
        if screen_x < 0 || screen_y < 0 {
            return;
        }
        
        let screen_x = screen_x as usize;
        let screen_y = screen_y as usize;
        
        if screen_x + scale as usize > self.width || screen_y + scale as usize > self.height {
            return;
        }
        
        // Render the scaled pixel
        for dy in 0..scale {
            for dx in 0..scale {
                let px = screen_x + dx as usize;
                let py = screen_y + dy as usize;
                
                if px < self.width && py < self.height {
                    let index = py * self.width + px;
                    
                    let color = if pixel.is_transparent() {
                        // Blend with checkerboard
                        let bg_color = self.checkerboard.get_color_at(px as u32, py as u32, scale);
                        self.blend_colors(bg_color, pixel.to_rgba32(), pixel.a)
                    } else {
                        pixel.to_rgba32()
                    };
                    
                    self.buffer[index] = color;
                }
            }
        }
    }
    
    fn blend_colors(&self, background: u32, foreground: u32, alpha: u8) -> u32 {
        if alpha == 255 {
            return foreground;
        }
        if alpha == 0 {
            return background;
        }
        
        let alpha_f = alpha as f32 / 255.0;
        let inv_alpha = 1.0 - alpha_f;
        
        let bg_r = ((background >> 16) & 0xFF) as f32;
        let bg_g = ((background >> 8) & 0xFF) as f32;
        let bg_b = (background & 0xFF) as f32;
        
        let fg_r = ((foreground >> 16) & 0xFF) as f32;
        let fg_g = ((foreground >> 8) & 0xFF) as f32;
        let fg_b = (foreground & 0xFF) as f32;
        
        let r = (fg_r * alpha_f + bg_r * inv_alpha) as u32;
        let g = (fg_g * alpha_f + bg_g * inv_alpha) as u32;
        let b = (fg_b * alpha_f + bg_b * inv_alpha) as u32;
        
        (r << 16) | (g << 8) | b
    }
}
```

## Phase 3: File Selection & SSE

### Task 3.1: File Dialog Service
Create `src/services/file_dialog.rs`:

```rust
use crate::models::PixelBookInfo;
use crate::services::ApiClient;
use rfd::FileDialog;
use std::error::Error;

pub struct FileDialogService {
    api_client: ApiClient,
}

impl FileDialogService {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }
    
    pub async fn show_open_dialog(&self) -> Result<Option<String>, Box<dyn Error>> {
        // Get list of available books from server
        let books = self.api_client.list_books().await?;
        
        if books.is_empty() {
            rfd::MessageDialog::new()
                .set_title("No Pixel Books")
                .set_description("No pixel books found in the configured path.")
                .set_buttons(rfd::MessageButtons::Ok)
                .show();
            return Ok(None);
        }
        
        // Create file dialog with .pxl filter
        let current_path = self.api_client.get_path().await?;
        
        let file = FileDialog::new()
            .add_filter("Pixel Books", &["pxl"])
            .set_directory(&current_path)
            .set_title("Select Pixel Book")
            .pick_file();
        
        if let Some(path) = file {
            if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    return Ok(Some(filename_str.to_string()));
                }
            }
        }
        
        Ok(None)
    }
}
```

### Task 3.2: SSE Event Client
Create `src/services/event_client.rs`:

```rust
use crate::models::events::PixelBookEvent;
use futures_util::StreamExt;
use reqwest::Client;
use std::error::Error;
use tokio::sync::mpsc;

pub struct EventClient {
    client: Client,
    base_url: String,
}

impl EventClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn subscribe(&self, filename: &str) -> Result<mpsc::Receiver<PixelBookEvent>, Box<dyn Error>> {
        let url = format!("{}/books/{}/events", self.base_url, filename);
        let response = self.client.get(&url).send().await?;
        
        let (tx, rx) = mpsc::channel(100);
        let mut stream = response.bytes_stream();
        
        tokio::spawn(async move {
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        let text = String::from_utf8_lossy(&chunk);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let json_data = &line[6..]; // Remove "data: " prefix
                                if let Ok(event) = serde_json::from_str::<PixelBookEvent>(json_data) {
                                    if tx.send(event).await.is_err() {
                                        break; // Receiver dropped
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        
        Ok(rx)
    }
}
```

## Phase 4: Integration & Polish

### Task 4.1: Complete Viewer Integration
Update `src/app/viewer.rs` with complete functionality:

```rust
// Add SSE integration
use crate::services::EventClient;
use tokio::sync::mpsc;

impl Viewer {
    async fn load_pixel_book(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let book = self.api_client.get_book(filename).await?;
        
        // Update window title
        let title = format!("PIXL Viewer - {}", filename);
        self.window.set_title(&title);
        
        // Start SSE subscription
        let event_client = EventClient::new("http://localhost:3000".to_string());
        let mut event_receiver = event_client.subscribe(filename).await?;
        
        // Handle events in background task
        let book_clone = book.clone();
        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                // Handle real-time updates
                // This would need to be communicated back to the main thread
                println!("Received event: {:?}", event);
            }
        });
        
        self.current_book = Some(book);
        self.current_frame = 0;
        
        Ok(())
    }
    
    async fn open_file_dialog(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let dialog_service = FileDialogService::new(self.api_client.clone());
        
        if let Some(filename) = dialog_service.show_open_dialog().await? {
            self.load_pixel_book(&filename).await?;
        }
        
        Ok(())
    }
}
```

### Task 4.2: Build Tasks
Create `viewer/tasks.toml`:

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

### Task 4.3: Testing
Create comprehensive tests for:
- Scaling calculations
- Rendering accuracy
- Event handling
- File dialog integration

```rust
// tests/rendering_tests.rs
use viewer::rendering::ScalingCalculator;

#[test]
fn test_scaling_edge_cases() {
    // Test minimum scale
    let (scale, _, _) = ScalingCalculator::calculate_scale_and_offset(100, 100, 50, 50);
    assert_eq!(scale, 1);
    
    // Test large scale
    let (scale, _, _) = ScalingCalculator::calculate_scale_and_offset(1, 1, 1000, 1000);
    assert_eq!(scale, 1000);
}
```

## Implementation Notes

### Performance Optimization
- Use efficient pixel buffer operations
- Minimize memory allocations in render loop
- Cache checkerboard patterns
- Optimize SSE event processing

### Error Handling
- Graceful degradation when server unavailable
- User-friendly error messages
- Retry logic for network operations
- Proper cleanup on shutdown

### Cross-Platform Considerations
- Test file dialogs on all platforms
- Handle different keyboard layouts
- Ensure rendering consistency
- Test window management behavior

### User Experience
- Smooth animations during frame transitions
- Clear visual feedback for loading states
- Intuitive keyboard shortcuts
- Responsive UI even during network operations 
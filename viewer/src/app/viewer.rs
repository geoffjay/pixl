use minifb::{Key, Window, WindowOptions};
use crate::models::PixelBook;
use crate::rendering::Renderer;
use crate::services::{ApiClient, EventClient, FileDialogService};
use crate::app::{AppState, InputHandler};
use std::time::Duration;

const WINDOW_WIDTH: usize = 512;
const WINDOW_HEIGHT: usize = 512;

pub struct Viewer {
    window: Window,
    renderer: Renderer,
    api_client: ApiClient,
    event_client: EventClient,
    file_dialog: FileDialogService,
    state: AppState,
}

impl Viewer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut window = Window::new(
            "PIXL Viewer",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default(),
        )?;
        
        window.limit_update_rate(Some(Duration::from_millis(16))); // ~60 FPS
        
        let renderer = Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let api_client = ApiClient::new("http://localhost:3000".to_string());
        let event_client = EventClient::new("http://localhost:3000".to_string());
        let file_dialog = FileDialogService::new(api_client.clone());
        let state = AppState::new();
        
        Ok(Self {
            window,
            renderer,
            api_client,
            event_client,
            file_dialog,
            state,
        })
    }
    
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check server connection
        match self.api_client.health_check().await {
            Ok(true) => {
                self.state.is_connected = true;
                println!("Connected to PIXL server");
            }
            _ => {
                self.state.is_connected = false;
                self.state.set_error("Cannot connect to PIXL server at http://localhost:3000".to_string());
                println!("Warning: Cannot connect to PIXL server");
            }
        }
        
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.handle_input().await?;
            self.handle_real_time_updates().await?;
            self.render();
            
            let buffer = self.renderer.get_buffer();
            self.window.update_with_buffer(buffer, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        }
        
        Ok(())
    }
    
    async fn handle_input(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Ctrl+O for file open
        if InputHandler::is_ctrl_o_pressed(&self.window) {
            if self.state.is_connected {
                self.open_file_dialog().await?;
            } else {
                println!("Cannot open file dialog: server not connected");
                self.state.set_error("Server not connected".to_string());
            }
        }
        
        // Frame navigation
        if InputHandler::is_left_arrow_pressed(&self.window) {
            self.state.prev_frame();
        }
        
        if InputHandler::is_right_arrow_pressed(&self.window) {
            self.state.next_frame();
        }
        
        Ok(())
    }
    
    async fn open_file_dialog(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Opening file dialog...");
        
        // First, get list of available books from server
        match self.api_client.list_books().await {
            Ok(books) => {
                if books.is_empty() {
                    self.state.set_error("No pixel books found on server".to_string());
                    return Ok(());
                }
                
                // For now, just load the first book as a placeholder
                // In a real implementation, this would show a proper file selection dialog
                if let Some(book_info) = books.first() {
                    self.load_book(&book_info.filename).await?;
                }
            }
            Err(e) => {
                self.state.set_error(format!("Failed to list books: {}", e));
            }
        }
        
        Ok(())
    }
    
    async fn load_book(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self.api_client.get_book(filename).await {
            Ok(book) => {
                self.state.set_book(book);
                
                // Start listening for real-time updates for this book
                if let Err(e) = self.event_client.connect(filename).await {
                    println!("Warning: Could not connect to real-time updates: {}", e);
                }
                
                println!("Loaded pixel book: {}", filename);
            }
            Err(e) => {
                self.state.set_error(format!("Failed to load book: {}", e));
            }
        }
        
        Ok(())
    }
    
    async fn handle_real_time_updates(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Poll for real-time updates
        if let Some(events) = self.event_client.poll_events().await? {
            for event in events {
                match &event.event_type {
                    crate::models::EventType::DrawingOperation { .. } => {
                        // Reload the current book to get the latest changes
                        if let Some(book) = &self.state.current_book {
                            let filename = book.filename.clone();
                            self.load_book(&filename).await?;
                        }
                    }
                    crate::models::EventType::BookSaved => {
                        println!("Book saved remotely");
                    }
                    crate::models::EventType::FrameChanged { frame_index } => {
                        self.state.set_frame(*frame_index);
                    }
                    crate::models::EventType::Heartbeat => {
                        // Keep connection alive
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    fn render(&mut self) {
        let (width, height) = self.window.get_size();
        self.renderer.update_size(width, height);
        
        if let Some(book) = &self.state.current_book {
            if let Some(frame) = book.frames.get(self.state.current_frame) {
                self.renderer.render_frame(frame, book.width, book.height);
                
                // Update window title with current frame info
                let title = format!("PIXL Viewer - {} (Frame {}/{})", 
                    book.filename, 
                    self.state.current_frame + 1,
                    book.frames.len()
                );
                self.window.set_title(&title);
            }
        } else {
            self.renderer.clear();
            
            let title = if self.state.is_connected {
                "PIXL Viewer - Press Ctrl+O to open a pixel book"
            } else {
                "PIXL Viewer - Server not connected"
            };
            self.window.set_title(title);
        }
        
        // Show error message if any
        if let Some(error) = &self.state.last_error {
            // In a real implementation, this would overlay the error on the screen
            // For now, just print to console
            println!("Error: {}", error);
        }
    }
    
    // For testing purposes - load a demo pixel book
    pub async fn load_demo_book(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.state.is_connected {
            return Err("Server not connected".into());
        }
        
        // Try to load the first available book
        match self.api_client.list_books().await {
            Ok(books) => {
                if let Some(book_info) = books.first() {
                    self.load_book(&book_info.filename).await?;
                } else {
                    self.state.set_error("No pixel books found on server".to_string());
                }
            }
            Err(e) => {
                self.state.set_error(format!("Failed to list books: {}", e));
            }
        }
        
        Ok(())
    }
} 
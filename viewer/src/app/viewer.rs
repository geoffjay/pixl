use minifb::{Key, Window, WindowOptions};
use crate::models::PixelBook;
use crate::rendering::Renderer;
use crate::services::ApiClient;
use crate::app::{AppState, InputHandler};
use std::time::Duration;

const WINDOW_WIDTH: usize = 512;
const WINDOW_HEIGHT: usize = 512;

pub struct Viewer {
    window: Window,
    renderer: Renderer,
    api_client: ApiClient,
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
        let state = AppState::new();
        
        Ok(Self {
            window,
            renderer,
            api_client,
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
            self.render();
            
            let buffer = self.renderer.get_buffer();
            self.window.update_with_buffer(buffer, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        }
        
        Ok(())
    }
    
    async fn handle_input(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Ctrl+O for file open (placeholder for Phase 3)
        if InputHandler::is_ctrl_o_pressed(&self.window) {
            if self.state.is_connected {
                println!("File dialog will be implemented in Phase 3");
                // TODO: Implement file dialog in Phase 3
            } else {
                println!("Cannot open file dialog: server not connected");
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
                    match self.api_client.get_book(&book_info.filename).await {
                        Ok(book) => {
                            self.state.set_book(book);
                            println!("Loaded demo book: {}", book_info.filename);
                        }
                        Err(e) => {
                            self.state.set_error(format!("Failed to load book: {}", e));
                        }
                    }
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
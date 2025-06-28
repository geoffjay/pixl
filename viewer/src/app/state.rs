use crate::models::PixelBook;

#[derive(Debug)]
pub struct AppState {
    pub current_book: Option<PixelBook>,
    pub current_frame: usize,
    pub is_connected: bool,
    pub last_error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_book: None,
            current_frame: 0,
            is_connected: false,
            last_error: None,
        }
    }
    
    pub fn set_book(&mut self, book: PixelBook) {
        self.current_book = Some(book);
        self.current_frame = 0;
        self.last_error = None;
    }
    
    pub fn clear_book(&mut self) {
        self.current_book = None;
        self.current_frame = 0;
    }
    
    pub fn set_frame(&mut self, frame: usize) {
        if let Some(book) = &self.current_book {
            if frame < book.frames.len() {
                self.current_frame = frame;
            }
        }
    }
    
    pub fn next_frame(&mut self) {
        if let Some(book) = &self.current_book {
            if self.current_frame + 1 < book.frames.len() {
                self.current_frame += 1;
            }
        }
    }
    
    pub fn prev_frame(&mut self) {
        if self.current_frame > 0 {
            self.current_frame -= 1;
        }
    }
    
    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error);
    }
    
    pub fn clear_error(&mut self) {
        self.last_error = None;
    }
} 
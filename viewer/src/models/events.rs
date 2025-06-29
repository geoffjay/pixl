use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelBookEvent {
    pub filename: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventType {
    #[serde(rename = "drawing_operation")]
    DrawingOperation { operation: DrawingOperation },
    #[serde(rename = "book_saved")]
    BookSaved,
    #[serde(rename = "book_loaded")]
    BookLoaded,
    #[serde(rename = "frame_changed")]
    FrameChanged { frame_index: usize },
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "heartbeat")]
    Heartbeat,
}

// Simplified drawing operation for viewer
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    // Add other operations as needed
} 
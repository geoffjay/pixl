use crate::models::DrawingOperation;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
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
    #[serde(rename = "heartbeat")]
    Heartbeat,
}

pub struct EventService {
    // In a real implementation, this would use a proper event store/database
    events: Arc<RwLock<HashMap<String, Vec<PixelBookEvent>>>>,
}

impl EventService {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn emit_event(&self, filename: &str, event_type: EventType) {
        let event = PixelBookEvent {
            filename: filename.to_string(),
            timestamp: Utc::now(),
            event_type,
        };
        
        let mut events = self.events.write().await;
        events.entry(filename.to_string())
            .or_insert_with(Vec::new)
            .push(event);
    }
    
    pub async fn get_recent_events(&self, filename: &str, since: DateTime<Utc>) -> Vec<PixelBookEvent> {
        let events = self.events.read().await;
        
        if let Some(file_events) = events.get(filename) {
            file_events
                .iter()
                .filter(|event| event.timestamp > since)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    pub async fn clear_old_events(&self, filename: &str, older_than: DateTime<Utc>) {
        let mut events = self.events.write().await;
        
        if let Some(file_events) = events.get_mut(filename) {
            file_events.retain(|event| event.timestamp > older_than);
        }
    }
    
    // Global event handlers for integration
    pub async fn on_drawing_operation(&self, filename: &str, operation: DrawingOperation) {
        self.emit_event(filename, EventType::DrawingOperation { operation }).await;
    }
    
    pub async fn on_book_saved(&self, filename: &str) {
        self.emit_event(filename, EventType::BookSaved).await;
    }
    
    pub async fn on_book_loaded(&self, filename: &str) {
        self.emit_event(filename, EventType::BookLoaded).await;
    }
    
    pub async fn on_frame_changed(&self, filename: &str, frame_index: usize) {
        self.emit_event(filename, EventType::FrameChanged { frame_index }).await;
    }
} 
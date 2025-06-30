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
        
        println!("📤 EventService: Emitting event for {}: {:?}", filename, event.event_type);
        
        let mut events = self.events.write().await;
        events.entry(filename.to_string())
            .or_insert_with(Vec::new)
            .push(event);
        
        println!("📊 EventService: Total events for {}: {}", filename, 
            events.get(filename).map(|v| v.len()).unwrap_or(0));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DrawingOperation, Point, ShapeType, Size};

    #[tokio::test]
    async fn test_emit_and_get_events() {
        let service = EventService::new();
        let filename = "test.pxl";
        
        // Emit a drawing operation event
        let operation = DrawingOperation::DrawPixel {
            frame: 0,
            x: 5,
            y: 5,
            color: [255, 0, 0, 255],
        };
        service.on_drawing_operation(filename, operation.clone()).await;
        
        // Emit a book saved event
        service.on_book_saved(filename).await;
        
        // Get recent events
        let start_time = Utc::now() - chrono::Duration::milliseconds(1000);
        let events = service.get_recent_events(filename, start_time).await;
        
        assert_eq!(events.len(), 2);
        
        // Check the drawing operation event
        if let EventType::DrawingOperation { operation: op } = &events[0].event_type {
            match op {
                DrawingOperation::DrawPixel { frame, x, y, color } => {
                    assert_eq!(*frame, 0);
                    assert_eq!(*x, 5);
                    assert_eq!(*y, 5);
                    assert_eq!(*color, [255, 0, 0, 255]);
                }
                _ => panic!("Expected DrawPixel operation"),
            }
        } else {
            panic!("Expected DrawingOperation event type");
        }
        
        // Check the book saved event
        match &events[1].event_type {
            EventType::BookSaved => (),
            _ => panic!("Expected BookSaved event type"),
        }
    }

    #[tokio::test]
    async fn test_get_events_for_different_files() {
        let service = EventService::new();
        
        // Emit events for different files
        service.on_book_saved("file1.pxl").await;
        service.on_book_saved("file2.pxl").await;
        
        let start_time = Utc::now() - chrono::Duration::milliseconds(1000);
        
        // Get events for file1
        let events1 = service.get_recent_events("file1.pxl", start_time).await;
        assert_eq!(events1.len(), 1);
        assert_eq!(events1[0].filename, "file1.pxl");
        
        // Get events for file2
        let events2 = service.get_recent_events("file2.pxl", start_time).await;
        assert_eq!(events2.len(), 1);
        assert_eq!(events2[0].filename, "file2.pxl");
        
        // Get events for non-existent file
        let events3 = service.get_recent_events("nonexistent.pxl", start_time).await;
        assert_eq!(events3.len(), 0);
    }

    #[tokio::test]
    async fn test_time_filtering() {
        let service = EventService::new();
        let filename = "test.pxl";
        
        // Emit an event
        service.on_book_saved(filename).await;
        
        // Get events from the future (should be empty)
        let future_time = Utc::now() + chrono::Duration::seconds(1);
        let events = service.get_recent_events(filename, future_time).await;
        assert_eq!(events.len(), 0);
        
        // Get events from the past (should include the event)
        let past_time = Utc::now() - chrono::Duration::seconds(1);
        let events = service.get_recent_events(filename, past_time).await;
        assert_eq!(events.len(), 1);
    }

    #[tokio::test]
    async fn test_all_event_types() {
        let service = EventService::new();
        let filename = "test.pxl";
        
        // Test all event type handlers
        let operation = DrawingOperation::DrawShape {
            frame: 0,
            shape: ShapeType::Circle,
            position: Point { x: 10, y: 10 },
            size: Size { width: 5, height: 5 },
            filled: true,
            color: [0, 255, 0, 255],
        };
        
        service.on_drawing_operation(filename, operation).await;
        service.on_book_saved(filename).await;
        service.on_book_loaded(filename).await;
        service.on_frame_changed(filename, 2).await;
        
        let start_time = Utc::now() - chrono::Duration::milliseconds(1000);
        let events = service.get_recent_events(filename, start_time).await;
        
        assert_eq!(events.len(), 4);
        
        // Verify event types
        let event_types: Vec<_> = events.iter().map(|e| &e.event_type).collect();
        assert!(matches!(event_types[0], EventType::DrawingOperation { .. }));
        assert!(matches!(event_types[1], EventType::BookSaved));
        assert!(matches!(event_types[2], EventType::BookLoaded));
        assert!(matches!(event_types[3], EventType::FrameChanged { frame_index: 2 }));
    }

    #[tokio::test]
    async fn test_clear_old_events() {
        let service = EventService::new();
        let filename = "test.pxl";
        
        // Emit some events
        service.on_book_saved(filename).await;
        service.on_book_loaded(filename).await;
        
        // Clear events older than now (should clear all)
        let now = Utc::now();
        service.clear_old_events(filename, now).await;
        
        // Should have no events now
        let past_time = Utc::now() - chrono::Duration::seconds(1);
        let events = service.get_recent_events(filename, past_time).await;
        assert_eq!(events.len(), 0);
    }

    #[tokio::test]
    async fn test_event_serialization() {
        let service = EventService::new();
        let filename = "test.pxl";
        
        let operation = DrawingOperation::DrawPixel {
            frame: 1,
            x: 3,
            y: 7,
            color: [128, 64, 192, 255],
        };
        
        service.on_drawing_operation(filename, operation).await;
        
        let start_time = Utc::now() - chrono::Duration::milliseconds(1000);
        let events = service.get_recent_events(filename, start_time).await;
        
        // Test that the event can be serialized to JSON
        let json_result = serde_json::to_string(&events[0]);
        assert!(json_result.is_ok());
        
        let json = json_result.unwrap();
        assert!(json.contains("draw_pixel"));
        assert!(json.contains("\"x\":3"));
        assert!(json.contains("\"y\":7"));
    }
} 
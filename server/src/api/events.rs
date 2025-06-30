use poem::{handler, web::Path, web::sse::{SSE, Event}};
use crate::services::EventService;
use poem::{Result, Error};
use std::time::Duration;
use tokio::time::interval;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use poem::{web::Data, Response};
use futures::stream::Stream;

#[handler]
pub async fn pixel_book_events(
    filename: Path<String>,
    event_service: poem::web::Data<&Arc<RwLock<EventService>>>,
) -> Result<SSE> {
    if !crate::utils::validation::validate_filename(&filename) {
        return Err(Error::from_string(
            "Invalid filename",
            poem::http::StatusCode::BAD_REQUEST,
        ));
    }
    
    let filename = filename.to_string();
    let event_service = event_service.clone();
    
    let stream = async_stream::stream! {
        let mut interval = interval(Duration::from_millis(500)); // Check for updates every 500ms
        let mut last_check = Utc::now();
        
        // Send initial connection event
        yield Event::message(format!(
            r#"{{"type":"connected","filename":"{}","timestamp":"{}"}}"#,
            filename,
            chrono::Utc::now().to_rfc3339()
        ));
        
        println!("📡 SSE client connected for book: {}", filename);
        
        loop {
            interval.tick().await;
            
            // Get recent events from the event service
            let service = event_service.read().await;
            let recent_events = service.get_recent_events(&filename, last_check).await;
            
            if !recent_events.is_empty() {
                println!("📨 Sending {} events for book: {}", recent_events.len(), filename);
                
                for event in recent_events {
                    // Convert PixelBookEvent to JSON and send via SSE
                    match serde_json::to_string(&event) {
                        Ok(json_event) => {
                            println!("📤 Sending event: {}", json_event);
                            yield Event::message(json_event);
                        },
                        Err(e) => {
                            println!("❌ Failed to serialize event: {}", e);
                        }
                    }
                }
            }
            
            last_check = Utc::now();
            
            // Send periodic heartbeat every 10 seconds
            if last_check.timestamp() % 10 == 0 {
                yield Event::message(format!(
                    r#"{{"type":"heartbeat","filename":"{}","timestamp":"{}"}}"#,
                    filename,
                    last_check.to_rfc3339()
                ));
            }
        }
    };
    
    Ok(SSE::new(stream))
} 
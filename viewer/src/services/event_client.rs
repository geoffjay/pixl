// SSE Event client will be implemented in Phase 3
use crate::models::events::PixelBookEvent;
use std::error::Error;
use tokio::sync::mpsc;

pub struct EventClient {
    #[allow(dead_code)]
    base_url: String,
}

impl EventClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    
    pub async fn subscribe(&self, _filename: &str) -> Result<mpsc::Receiver<PixelBookEvent>, Box<dyn Error + Send + Sync>> {
        // TODO: Implement SSE subscription in Phase 3
        let (_tx, rx) = mpsc::channel(100);
        Ok(rx)
    }
} 
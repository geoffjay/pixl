use crate::models::events::PixelBookEvent;
use reqwest::Client;
use std::error::Error;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct EventClient {
    base_url: String,
    client: Client,
    current_filename: Option<String>,
    event_buffer: Arc<Mutex<VecDeque<PixelBookEvent>>>,
}

impl EventClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
            current_filename: None,
            event_buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    pub async fn connect(&mut self, filename: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.current_filename = Some(filename.to_string());
        
        // Start SSE connection in background
        let url = format!("{}/books/{}/events", self.base_url, filename);
        let client = self.client.clone();
        let event_buffer = self.event_buffer.clone();
        let filename_clone = filename.to_string();
        
        println!("🔌 Connecting to SSE endpoint: {}", url);
        
        tokio::spawn(async move {
            match Self::sse_listener(client, url, event_buffer, filename_clone).await {
                Ok(_) => println!("📡 SSE connection closed"),
                Err(e) => println!("❌ SSE connection error: {}", e),
            }
        });
        
        Ok(())
    }
    
    async fn sse_listener(
        client: Client,
        url: String,
        event_buffer: Arc<Mutex<VecDeque<PixelBookEvent>>>,
        filename: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("🎯 Starting SSE listener for: {}", filename);
        
        let response = client
            .get(&url)
            .header("Accept", "text/event-stream")
            .header("Cache-Control", "no-cache")
            .send()
            .await?;
        
        println!("📻 SSE response status: {}", response.status());
        
        if !response.status().is_success() {
            return Err(format!("SSE connection failed: {}", response.status()).into());
        }
        
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();
        
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    buffer.push_str(&text);
                    
                    // Process complete SSE events
                    while let Some(pos) = buffer.find("\n\n") {
                        let event_text = buffer[..pos].to_string();
                        buffer = buffer[pos + 2..].to_string();
                        
                        if let Some(event) = Self::parse_sse_event(&event_text) {
                            println!("📨 Received SSE event: {:?}", event);
                            let mut events = event_buffer.lock().await;
                            events.push_back(event);
                            
                            // Keep buffer size manageable
                            while events.len() > 100 {
                                events.pop_front();
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("❌ SSE stream error: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        Ok(())
    }
    
    fn parse_sse_event(event_text: &str) -> Option<PixelBookEvent> {
        // Parse SSE format: "data: {json}"
        for line in event_text.lines() {
            if let Some(data) = line.strip_prefix("data: ") {
                match serde_json::from_str::<PixelBookEvent>(data) {
                    Ok(event) => return Some(event),
                    Err(e) => {
                        // Skip heartbeat and connection events that don't match PixelBookEvent format
                        if !data.contains("heartbeat") && !data.contains("connected") {
                            println!("⚠️ Failed to parse SSE event: {} - Data: {}", e, data);
                        }
                    }
                }
            }
        }
        None
    }
    
    pub async fn disconnect(&mut self) {
        self.current_filename = None;
        println!("🔌 Disconnected from real-time updates");
    }
    
    pub async fn poll_events(&self) -> Result<Option<Vec<PixelBookEvent>>, Box<dyn Error + Send + Sync>> {
        let mut events = self.event_buffer.lock().await;
        if events.is_empty() {
            Ok(None)
        } else {
            let all_events: Vec<PixelBookEvent> = events.drain(..).collect();
            Ok(Some(all_events))
        }
    }
    
    pub fn is_connected(&self) -> bool {
        self.current_filename.is_some()
    }
    
    pub fn current_filename(&self) -> Option<&str> {
        self.current_filename.as_deref()
    }
} 